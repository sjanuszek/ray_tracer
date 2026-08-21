use crate::{color::Color, geometry::hittable::HitRecord, ray::Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
