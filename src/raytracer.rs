use std::cmp::Ordering;

use crate::utilities::{Matrix3, Vector3};

pub struct RayTracer{
    pub spheres: Vec<Box<dyn Sphere>>,
    pub lamp: Vector3,
    pub camera_pos: Vector3,
    pub camera_matrix: Matrix3
}

pub trait Sphere{
    fn get_center_radius(&self) -> (Vector3, f32);
    fn get_value(&self, normal: Vector3, position: Vector3) -> f32;
    //https://gist.github.com/wwwtyro/beecc31d65d1004f5a9d
    fn intersect(&self, r0: Vector3, rd: Vector3) -> f32{
        let (center, radius) = self.get_center_radius();
        let a = rd.length_squared();
        let s0_r0 = r0.subtract(center);
        let b = 2f32 * rd.dot(s0_r0);
        let c = s0_r0.length_squared() - (radius * radius);
        if b*b - 4f32*a*c < 0f32 {
            return -1f32;
        }
        return (-b - ((b*b)-4f32*a*c).sqrt())/(2f32*a);
    }
}

pub struct BasicSphere{
    pub center: Vector3,
    pub radius: f32,
    pub value: f32
}

impl Sphere for BasicSphere {
    fn get_center_radius(&self) -> (Vector3, f32) {
        return (self.center, self.radius)
    }

    fn get_value(&self, normal: Vector3, position: Vector3) -> f32 {
        return self.value
    }
}

impl BasicSphere {
    pub fn new(center: Vector3, radius: f32, value: f32) -> Self{
        Self{
            center,
            radius,
            value
        }
    }
}

pub struct CheckerboardSphere{
    pub center: Vector3,
    pub radius: f32,
    pub value: f32,
    pub size: f32
}

impl Sphere for CheckerboardSphere {
    fn get_center_radius(&self) -> (Vector3, f32) {
        return (self.center, self.radius)
    }

    fn get_value(&self, normal: Vector3, position: Vector3) -> f32 {
        let position = position.multiply(self.size);
        return ((position.x as i32 ^ position.z as i32)&1) as f32 * self.value
    }
}

impl CheckerboardSphere {
    pub fn new(center: Vector3, radius: f32, value: f32, size: f32) -> Self{
        Self{
            center,
            radius,
            value,
            size
        }
    }
}

impl RayTracer {
    pub fn intersect(&self, r0: Vector3, rd: Vector3) -> Option<(&Box<dyn Sphere>, f32)>{
        return self.spheres.iter()
            .map(|s|(s, s.intersect(r0, rd))) //Associate each sphere with the ray distance
            .filter(|s|s.1 > 0f32) //Filter out successful hits
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)); //Find closest hit
    }
}