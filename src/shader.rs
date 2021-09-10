use crate::{ray_resolvers::{bvh::aabb::AABBRayResolver, ray_resolver::RayResolver}, raytracer::{SceneInformation}, types::Parameters, utilities::{Matrix3, Vector3}};

const EPSILON: f32 = 0.0001;

pub fn get_params(time: f32, camera_pos: Vector3, direction: Matrix3, mesh: &AABBRayResolver) -> Parameters{
    let fanim = time*2.0;

    let info = SceneInformation{
        lamp: Vector3::new(1.0, 1.0, 0.0).multiply(10.0),
        camera_matrix: direction,
        camera_pos,
    };
    
    Parameters{
        mesh,
        time,
        sceneInformation: info
    }
}

//This function runs for every "pixel".
pub fn eval(x: f32, y: f32, aspect_ratio: f32, param: &Parameters) -> f32{
    //Move origin into middle of screen, account for aspect ratio
    let x = (x*2.0-1.0) * aspect_ratio; 
    let y = (1.0-y)*2.0-1.0;
    //Set up ray parameters
    let ray_start = param.sceneInformation.camera_pos;
    let ray_direction = param.sceneInformation.camera_matrix.multiply(Vector3::new(x, y, 2f32).normalized());
    //if let Some((sphere, dist)) = param.raytracer.intersect(ray_start, ray_direction){
    if let Some(ray) = param.mesh.resolve(ray_start, ray_direction, false){
        //If ray hits
        let hit = ray.pos; //Calculate hit position using received distance
        let normal = ray.normal; //Calculate normal using center of hit sphere
        let lamp_dir = param.sceneInformation.lamp.subtract(hit); //Vector pointed at the lamp
        let lamp_dir = lamp_dir.normalized(); //Make the lamp direction normalized

        
        let ambient = 0.000;
        
        let diffuse = {
            let ray_start = hit.add(normal.multiply(EPSILON)); //Send shadow ray from above the surface of the sphere
            let ray = param.mesh.resolve(ray_start, lamp_dir, false);
            match ray {
                //If it didn't hit anything, that means that it reached the lamp. Return diffuse color
                None =>                                                 normal.dot(lamp_dir),
                //Ray hit an object -> Shadow
                _ => 0.0                   
            }
        };

        (ray.color.x+ray.color.y+ray.color.z)/3.0 * (ambient + diffuse)
        //sphere.value
        

    }else{
        //If ray doesn't hit
        0f32
    }
}