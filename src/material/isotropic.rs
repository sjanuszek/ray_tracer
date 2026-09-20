use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material, ray::Ray, utility::PI, vec3::random_unit_vector};

pub struct Isotropic {
    tex: Arc<dyn Texture>
}

impl Isotropic {
    pub fn new_from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new_from_color(albedo))
        }
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Self {
        Self {
            tex
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(Color, crate::ray::Ray, f64)> {
        Some((self.tex.value(rec.u, rec.v, rec.p), Ray::new_with_time(rec.p, random_unit_vector(), r_in.time()), 1. / (4. * PI)))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, scattered: &Ray) -> f64 {
        1. / (4. * PI)
    }
}
