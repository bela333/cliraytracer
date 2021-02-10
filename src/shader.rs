use crate::{raytracer::{RayTracer, Sphere, Types}, types::Parameters, utilities::{Matrix3, Vector3}};

const EPSILON: f32 = 0.0001;

pub fn get_params(time: f32) -> Parameters{
    let fanim = time*2.0;
    let fcam = time;

    let camera_pos = Vector3::new(
        (fcam.sin())*10f32,
        0f32,
        (1.0-fcam.cos())*10f32);

    let raytracer = RayTracer{
        spheres: vec![
            //"Plane". Really, it's a sphere with a really large radius.
            //Making that radius too large would cause glitches
            Sphere::new(Vector3::new(0f32, -100000.0 - 10.0, 0f32), 100000f32, 0.1f32, Types::Diffuse),
            Sphere::new(Vector3::new(0f32, -10.0, 6f32), 6f32, 0.75f32, Types::Diffuse),
            Sphere::new(Vector3::new(20f32, -10.0, 15f32), 10f32, 2f32, Types::Reflective),
            Sphere::new(Vector3::new(fanim.sin()*3.0, fanim.cos()*3.0, 10f32), 2f32, 0.25f32, Types::Diffuse),
            Sphere::new(Vector3::new(fanim.sin()*3.0, 0f32, 10f32+fanim.cos()*3.0), 1f32, 1f32, Types::Diffuse),
        ],
        lamp: Vector3::new(1.0, 1.0, 0.0).multiply(10.0),
        //look_at_matrix sets up a matrix where k points in the specified direction
        camera_matrix: Matrix3::look_at_matrix(Vector3::new(0f32, 0.0, 10f32).subtract(camera_pos).normalized()), 
        camera_pos,
    };
    Parameters{
        raytracer,
        time
    }
}

fn get_diffuse(param: &Parameters, sphere: &Sphere, normal: Vector3, lamp_dir: Vector3, ray_direction: Vector3, hit: Vector3, depth: u32, max_depth: u32) -> f32{
    match sphere.sphere_type{
        Types::Diffuse => normal.dot(lamp_dir),
        Types::Reflective => {
            let out = ray_direction.negate().reflect(normal);
            render(param, hit.add(normal.multiply(EPSILON)), out, depth + 1, max_depth)
        },
    }
}

pub fn render(param: &Parameters, ray_start: Vector3, ray_direction: Vector3, depth: u32, max_depth: u32) -> f32{
    if depth > max_depth {
        return 0.0;
    }
    if let Some((sphere, dist)) = param.raytracer.intersect(ray_start, ray_direction){
        //If ray hits
        let hit = ray_direction.multiply(dist).add(ray_start); //Calculate hit position using received distance
        let normal = hit.subtract(sphere.center).multiply(1.0/sphere.radius); //Calculate normal using center of hit sphere
        let lamp_dir = param.raytracer.lamp.subtract(hit); //Vector pointed at the lamp
        let lamp_dist_squared = lamp_dir.length_squared(); //Distance to the lamp, squared
        let lamp_dir = lamp_dir.normalized(); //Make the lamp direction normalized

        
        let ambient = 0.005;
        
        let diffuse = {
            let ray_start = hit.add(normal.multiply(EPSILON)); //Send shadow ray from above the surface of the sphere
            let result = param.raytracer.intersect(ray_start, lamp_dir);
            match result {
                //If it didn't hit anything, that means that it reached the lamp. Return diffuse color
                None =>                                                 get_diffuse(param, sphere, normal, lamp_dir, ray_direction, hit, depth, max_depth),
                //If it did hit something, but that is behind the lamp, the ray still managed to reach the lamp. Return diffuse color
                Some((_, dist)) if dist*dist > lamp_dist_squared => get_diffuse(param, sphere, normal, lamp_dir, ray_direction, hit, depth, max_depth), 
                //Ray hit an object -> Shadow
                _ => 0.0                   
            }
        };

        sphere.value * (ambient + diffuse)
        //sphere.value
        

    }else{
        //If ray doesn't hit
        0f32
    }
}

//This function runs for every "pixel".
pub fn eval(x: f32, y: f32, aspect_ratio: f32, param: &Parameters) -> f32{
    //Move origin into middle of screen, account for aspect ratio
    let x = (x*2.0-1.0) * aspect_ratio; 
    let y = (1.0-y)*2.0-1.0;
    //Set up ray parameters
    let ray_start = param.raytracer.camera_pos;
    let ray_direction = param.raytracer.camera_matrix.multiply(Vector3::new(x, y, 2f32).normalized());
    render(param, ray_start, ray_direction, 0, 3)
}