use crate::{color::Color, geometry::hittable::HitRecord, ray::Ray, vec3::Point3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.
    }
}
