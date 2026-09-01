use std::sync::Arc;

use crate::{bvh::aabb::AABB, geometry::hittable::Hittable, ray::Ray, vec3::Vec3};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box().clone() + offset;
        Self {
            object,
            offset,
            bbox
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let offset_r = Ray::new_with_time(*r.origin() - self.offset, *r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &crate::bvh::aabb::AABB {
       &self.bbox
    }
}
