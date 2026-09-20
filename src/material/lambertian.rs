use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material, onb::ONB, ray::Ray, utility::PI, vec3::{dot, random_cosine_direction, unit_vector}};

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
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(Color, crate::ray::Ray, f64)> {
        let uvw = ONB::new(&rec.normal);
        let scatter_direction = uvw.transform(&random_cosine_direction());

        let scattered = Ray::new_with_time(rec.p, unit_vector(scatter_direction), r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        let pdf = dot(*uvw.w(), *scattered.direction()) / PI;

        Some((attenuation, scattered, pdf))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, scattered: &Ray) -> f64 {
        1. / (2. * PI)
    }
}
