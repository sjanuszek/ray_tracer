use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material, ray::Ray, vec3::random_unit_vector};

pub struct Lambertian {
    tex: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new_from_color(albedo: Color) -> Self {
        Self::new_from_texture(Arc::new(SolidColor::new_from_color(albedo)))
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Self {
        Self {
            tex
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(Color, crate::ray::Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.tex.value(rec.u, rec.v, rec.p), Ray::new_with_time(rec.p, scatter_direction, r_in.time())))
    }
}
