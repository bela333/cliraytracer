use std::cmp::Ordering;

use crate::utilities::{Matrix3, Vector3};

pub struct RayTracer{
    pub spheres: Vec<Sphere>,
    pub lamp: Vector3,
    pub camera_pos: Vector3,
    pub camera_matrix: Matrix3
}

pub enum Types{
    Diffuse,
    Reflective
}

pub struct Sphere{
    pub center: Vector3,
    pub radius: f32,
    pub value: f32,
    pub sphere_type: Types,
}

impl Sphere {
    //https://gist.github.com/wwwtyro/beecc31d65d1004f5a9d
    pub fn intersect(&self, r0: Vector3, rd: Vector3) -> f32{
        let a = rd.length_squared();
        let s0_r0 = r0.subtract(self.center);
        let b = 2f32 * rd.dot(s0_r0);
        let c = s0_r0.length_squared() - (self.radius * self.radius);
        if b*b - 4f32*a*c < 0f32 {
            return -1f32;
        }
        return (-b - ((b*b)-4f32*a*c).sqrt())/(2f32*a);
    }
    pub fn new(center: Vector3, radius: f32, value: f32, sphere_type: Types) -> Self{
        Self{
            center,
            radius,
            value,
            sphere_type
        }
    }
}

impl RayTracer {
    pub fn intersect(&self, r0: Vector3, rd: Vector3) -> Option<(&Sphere, f32)>{
        return self.spheres.iter()
            .map(|s|(s, s.intersect(r0, rd))) //Associate each sphere with the ray distance
            .filter(|s|s.1 > 0f32) //Filter out successful hits
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)); //Find closest hit
    }
}