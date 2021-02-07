use crate::{raytracer::{RayTracer, Sphere}, types::Parameters, utilities::{Matrix3, Vector3}};

const EPSILON: f32 = 0.0001;

pub fn get_raytracer(time: f32) -> RayTracer{
    let f = time*2.0;
    let fcam = time;
    let camx = (fcam.sin())*10f32;
    let camz = (1.0-fcam.cos())*10f32;

    let raytracer = RayTracer{
        spheres: vec![
            Sphere::new(Vector3::new(0f32, -100000.0 - 10.0, 0f32), 100000f32, 0.1f32),
            Sphere::new(Vector3::new(0f32, -10.0, 6f32), 6f32, 0.75f32),
            Sphere::new(Vector3::new(20f32, -10.0, 15f32), 10f32, 2f32),
            Sphere::new(Vector3::new(f.sin()*3.0, f.cos()*3.0, 10f32), 2f32, 0.25f32),
            Sphere::new(Vector3::new(f.sin()*3.0, 0f32, 10f32+f.cos()*3.0), 1f32, 1f32),
        ],
        lamp: Vector3::new(1.0, 1.0, 0.0).multiply(10.0),
        camera_pos: Vector3::new(camx, 0f32, camz),
        camera_matrix: Matrix3::look_at_matrix(Vector3::new(0f32, 0.0, 10f32).subtract(Vector3::new(camx, 0f32, camz)).normalized())
    };
    raytracer
}

pub fn eval(x: f32, y: f32, aspect_ratio: f32, param: &Parameters) -> f32{
    let x = (x*2.0-1.0) * aspect_ratio;
    let y = (1.0-y)*2.0-1.0;
    let ray_center = param.raytracer.camera_pos;
    let ray_direction = param.raytracer.camera_matrix.multiply(Vector3::new(x, y, 2f32).normalized());
    if let Some((sphere, dist)) = param.raytracer.intersect(ray_center, ray_direction){
        let hit = ray_direction.multiply(dist).add(ray_center);
        let lamp_dir = param.raytracer.lamp.subtract(hit).normalized();
        let normal = hit.subtract(sphere.center).normalized();
        let diff = if let None = param.raytracer.intersect(hit.add(normal.multiply(EPSILON)), lamp_dir){
            let diff = normal.dot(lamp_dir);
            diff
        }else{0.0};
        (diff+0.005)*sphere.value

    }else{
        0f32
    }
}