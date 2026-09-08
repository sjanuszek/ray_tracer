use std::sync::Arc;

use crate::{color::Color, geometry::hittable::{HitRecord, Hittable}, interval::Interval, mapping::texture::Texture, material::{isotropic::Isotropic, material::Material}, utility::{INFINITY, random_double}, vec3::Vec3};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new_from_texture(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1./density,
            phase_function: Arc::new(Isotropic::new_from_texture(tex))
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1./density,
            phase_function: Arc::new(Isotropic::new_from_color(albedo))
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2) {
            return false;
        }

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return false;
        }

        rec1.t = rec1.t.max(0.);

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1., 0., 0.);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> &crate::bvh::aabb::AABB {
        self.boundary.bounding_box()
    }
}
