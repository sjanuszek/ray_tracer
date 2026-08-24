use std::sync::Arc;

use crate::{bvh::aabb::AABB, interval::Interval, material::material::Material, ray::Ray, vec3::{Point3, Vec3, dot}};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(*r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;
}
