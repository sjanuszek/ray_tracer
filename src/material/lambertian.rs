use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material, ray::Ray, utility::PI, vec3::{dot, random_on_hemisphere, random_unit_vector, unit_vector}};

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
        let mut scatter_direction = random_on_hemisphere(&rec.normal);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.tex.value(rec.u, rec.v, rec.p), Ray::new_with_time(rec.p, scatter_direction, r_in.time())))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, scattered: &Ray) -> f64 {
        1. / (2. * PI)
    }
}
