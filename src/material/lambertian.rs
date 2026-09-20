use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::{Material, ScatterRecord}, onb::ONB, pdf::cosine_pdf::CosinePDF, ray::Ray, utility::PI, vec3::{dot, random_cosine_direction, unit_vector}};

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
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePDF::new(&rec.normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(rec.normal, unit_vector(*scattered.direction()));
        if cos_theta < 0. { 0. } else { cos_theta / PI }
    }
}
