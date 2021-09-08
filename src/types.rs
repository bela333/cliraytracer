use crate::{ray_resolvers::bvh::aabb::AABBRayResolver, raytracer::SceneInformation};

pub struct Parameters<'a>{
    pub time: f32,
    pub mesh: &'a AABBRayResolver,
    pub sceneInformation: SceneInformation,
}