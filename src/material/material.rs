use std::sync::Arc;

use crate::{color::Color, geometry::hittable::HitRecord, pdf::{pdf::PDF, sphere_pdf::SpherePDF}, ray::Ray, vec3::Point3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray
}

impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            attenuation: Color::default(),
            pdf_ptr: None,
            skip_pdf: bool::default(),
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.
    }
}
