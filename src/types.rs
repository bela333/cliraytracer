use crate::raytracer::RayTracer;

pub struct Parameters<'a>{
    pub time: f32,
    pub raytracer: &'a RayTracer
}