use core::fmt;
use std::{cmp::Ordering, path::Path, usize};

use crate::{
    error::Error,
    ray_resolvers::{
        bvh::{
            multi_ray_resolver::MultiRayResolver,
            triangle::{TriangleMaterial, TriangleResolver},
        },
        ray_resolver::MaterialType,
    },
    utilities::Vector3,
};

use self::{
    aabb::{AABBRayResolver, AABB},
    triangle::Triangle,
};

pub mod aabb;
pub mod dummy;
pub mod multi_ray_resolver;
pub mod triangle;

pub fn generate_bvh_from_file<P: AsRef<Path> + fmt::Debug>(
    filename: P,
) -> Result<AABBRayResolver, Error> {
    let triangles = triangles_from_file(filename)?;
    generate_bvh(triangles)
}

pub fn triangles_from_file<P: AsRef<Path> + fmt::Debug>(
    filename: P,
) -> Result<Vec<Triangle>, Error> {
    let (models, materials) = tobj::load_obj(
        filename,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )?;
    //Use default material if no material file can be loaded
    let materials = materials.unwrap_or(vec![tobj::Material::default()]);
    //Use default material if no materials were loaded
    let materials = if materials.len() <= 0 {
        vec![tobj::Material::default()]
    } else {
        materials
    };
    let mut triangles: Vec<Triangle> = Vec::new();
    for model in models {
        let material_index = model.mesh.material_id.unwrap_or(0);
        let material = materials.get(material_index).unwrap_or(&materials[0]);
        let material = TriangleMaterial {
            color: Vector3::from_slice(&material.diffuse),
            emit: Vector3::zero(),
            t: MaterialType::Diffuse,
        };
        /*//Organize positions into Vector3s
        let positions: Vec<(Vector3,Vector3)> = model.mesh.positions
            .chunks(3)
            .zip(normals) //Zip in the normals as well
            .map(|p| match p{
                ([x, y, z], [nx, ny, nz]) =>
                (Vector3::new(-*x, *y, *z), Vector3::new(-*nx, *ny, *nz)),
                _ => panic!("Couldn't load OBJ positions")
            }).collect();

        //Generate triangles
        let mut t: Vec<Triangle> = model.mesh.indices
            .chunks(3)
            .map(|i| match i  {
                [i0, i1, i2] => {
                let (v0, n0) = positions[*i0 as usize];
                let (v1, n1) = positions[*i1 as usize];
                let (v2, n2) = positions[*i2 as usize];
                Triangle::new_with_normal(v2, v1, v0,n1, n0, n2, material.clone())},
                _ => panic!("Couldn't load mesh")
            }).collect();
        triangles.append(&mut t);*/
        let has_normals = model.mesh.normals.len() > 0;
        for _f in 0..model.mesh.indices.len() / 3 {
            let i0 = model.mesh.indices[3 * _f + 0] as usize;
            let i1 = model.mesh.indices[3 * _f + 1] as usize;
            let i2 = model.mesh.indices[3 * _f + 2] as usize;
            let v0 = Vector3::new(
                -model.mesh.positions[i0 * 3 + 0],
                model.mesh.positions[i0 * 3 + 1],
                model.mesh.positions[i0 * 3 + 2],
            );
            let v1 = Vector3::new(
                -model.mesh.positions[i1 * 3 + 0],
                model.mesh.positions[i1 * 3 + 1],
                model.mesh.positions[i1 * 3 + 2],
            );
            let v2 = Vector3::new(
                -model.mesh.positions[i2 * 3 + 0],
                model.mesh.positions[i2 * 3 + 1],
                model.mesh.positions[i2 * 3 + 2],
            );
            if has_normals {
                let n0 = Vector3::new(
                    -model.mesh.normals[i0 * 3 + 0],
                    model.mesh.normals[i0 * 3 + 1],
                    model.mesh.normals[i0 * 3 + 2],
                );
                let n1 = Vector3::new(
                    -model.mesh.normals[i1 * 3 + 0],
                    model.mesh.normals[i1 * 3 + 1],
                    model.mesh.normals[i1 * 3 + 2],
                );
                let n2 = Vector3::new(
                    -model.mesh.normals[i2 * 3 + 0],
                    model.mesh.normals[i2 * 3 + 1],
                    model.mesh.normals[i2 * 3 + 2],
                );
                triangles.push(Triangle::new_with_normal(
                    v2,
                    v1,
                    v0,
                    n1,
                    n0,
                    n2,
                    material.clone(),
                ))
            } else {
                triangles.push(Triangle::new(v2, v1, v0, material.clone()))
            }
        }
    }
    Ok(triangles)
}

fn get_bounding_box(triangles: &Vec<Triangle>) -> AABB {
    let mut aabb = triangles[0].bounds();
    for triangle in triangles {
        aabb = aabb.union(&triangle.bounds())
    }
    aabb
}

pub fn generate_bvh(mut triangles: Vec<Triangle>) -> Result<AABBRayResolver, Error> {
    if triangles.len() < 1 {
        return Err(Error::new(
            "BVH generation requires atleast 2 triangles".to_string(),
        ));
    }
    if triangles.len() == 1 {
        //Return single triangle
        let triangle = triangles[0].clone();
        let aabb = triangle.bounds();
        let inner = TriangleResolver { triangle };
        let resolver = AABBRayResolver::new(aabb, inner);
        return Ok(resolver);
    }
    let bounds = get_bounding_box(&triangles);
    let orientation = bounds.size().largest_component(); //Divide among the longest dimension
                                                         //Divide triangles among the median
    let (t1, t2) = {
        let index = triangles.len() / 2;
        triangles.select_nth_unstable_by(index, |a, b| {
            let a_pos = a.centroid.get_component(orientation);
            let b_pos = b.centroid.get_component(orientation);
            a_pos.partial_cmp(&b_pos).unwrap_or(Ordering::Equal)
        });
        let (t1, t2) = triangles.split_at(index);
        (t1.to_vec(), t2.to_vec())
    };

    //Recursive generate BVH for each branch
    let bvh1 = generate_bvh(t1)?;
    let bvh2 = generate_bvh(t2)?;
    let bvh1 = Box::new(bvh1);
    let bvh2 = Box::new(bvh2);
    //Create MultiRayResolver
    let multi = MultiRayResolver {
        inner: vec![bvh1, bvh2],
    };
    //Create current AABB and return
    Ok(AABBRayResolver::new(bounds, multi))
}
