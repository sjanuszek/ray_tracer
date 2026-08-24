use std::sync::Arc;

use crate::{aabb::AABB, geometry::hittable::{HitRecord, Hittable}, interval::Interval};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: AABB::default()
        }
    }

    pub fn new_from_hittable(object: impl Hittable + 'static) -> Self {
        let bbox = *object.bounding_box();
        Self {
            objects: vec![Arc::new(object)],
            bbox
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = AABB::new_from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
