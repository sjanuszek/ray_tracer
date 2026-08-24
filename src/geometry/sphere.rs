use std::sync::Arc;

use crate::{bvh::aabb::AABB, geometry::hittable::Hittable, interval::Interval, material::material::Material, ray::Ray, vec3::{Point3, Vec3, dot}};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::new(0., 0., 0.)),
            radius: radius.max(0.),
            mat,
            bbox: AABB::new_from_points(static_center - rvec, static_center + rvec)
        }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1);
        let box1 = AABB::new_from_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box2 = AABB::new_from_points(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius: radius.max(0.),
            mat,
            bbox: AABB::new_from_boxes(&box1, &box2)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(*r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        return true;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
} 
