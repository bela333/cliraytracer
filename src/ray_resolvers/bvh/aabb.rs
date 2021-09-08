use crate::{
    ray_resolvers::ray_resolver::{RayResolver, RayResult},
    utilities::{Vector3},
};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn trace(&self, pos: &Vector3, dir: &Vector3) -> Option<Vector3> {
        let invdir = dir.reciprocal();
        let (mut tmin, mut tmax) = if invdir.x >= 0.0 {
            let tmin = (self.min.x - pos.x) * invdir.x;
            let tmax = (self.max.x - pos.x) * invdir.x;
            (tmin, tmax)
        } else {
            let tmin = (self.max.x - pos.x) * invdir.x;
            let tmax = (self.min.x - pos.x) * invdir.x;
            (tmin, tmax)
        };
        let (tymin, tymax) = if invdir.y >= 0.0 {
            let tymin = (self.min.y - pos.y) * invdir.y;
            let tymax = (self.max.y - pos.y) * invdir.y;
            (tymin, tymax)
        } else {
            let tymin = (self.max.y - pos.y) * invdir.y;
            let tymax = (self.min.y - pos.y) * invdir.y;
            (tymin, tymax)
        };

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let (tzmin, tzmax) = if invdir.z >= 0.0 {
            let tzmin = (self.min.z - pos.z) * invdir.z;
            let tzmax = (self.max.z - pos.z) * invdir.z;
            (tzmin, tzmax)
        } else {
            let tzmin = (self.max.z - pos.z) * invdir.z;
            let tzmax = (self.min.z - pos.z) * invdir.z;
            (tzmin, tzmax)
        };

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        if tmax < 0.0 {
            return None;
        }
        Some(dir.multiply(t).add(pos.clone()))
    }

    pub fn union(&self, other: &Self) -> Self {
        let xmin = if self.min.x < other.min.x {
            self.min.x
        } else {
            other.min.x
        };
        let ymin = if self.min.y < other.min.y {
            self.min.y
        } else {
            other.min.y
        };
        let zmin = if self.min.z < other.min.z {
            self.min.z
        } else {
            other.min.z
        };

        let xmax = if self.max.x > other.max.x {
            self.max.x
        } else {
            other.max.x
        };
        let ymax = if self.max.y > other.max.y {
            self.max.y
        } else {
            other.max.y
        };
        let zmax = if self.max.z > other.max.z {
            self.max.z
        } else {
            other.max.z
        };

        Self {
            min: Vector3::new(xmin, ymin, zmin),
            max: Vector3::new(xmax, ymax, zmax),
        }
    }
    pub fn size(&self) -> Vector3 {
        self.max.subtract(self.min)
    }
}

pub struct AABBRayResolver {
    pub aabb: AABB,
    pub inner: Box<dyn RayResolver + Sync>,
}

impl AABBRayResolver {
    pub fn new<T: RayResolver + Sync + 'static>(aabb: AABB, inner: T) -> Self {
        let inner = Box::new(inner);
        Self { aabb, inner }
    }
}

impl RayResolver for AABBRayResolver {
    fn resolve(
        &self,
        pos: Vector3,
        dir: Vector3,
        refraction: bool
    ) -> Option<RayResult> {
        match self.aabb.trace(&pos, &dir) {
            Some(_) => self.inner.resolve(pos, dir, refraction),
            None => None,
        }
    }
}
