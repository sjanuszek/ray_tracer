use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material, pdf::sphere_pdf::SpherePDF, ray::Ray, utility::PI, vec3::random_unit_vector};

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
    fn scatter(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, srec: &mut super::material::ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf_ptr = Some(Arc::new(SpherePDF::new()));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &crate::geometry::hittable::HitRecord, scattered: &Ray) -> f64 {
        1. / (4. * PI)
    }
}
