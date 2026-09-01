use std::sync::Arc;

use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}, material::material::Material};

pub struct DiffuseLight {
    tex: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn new_from_color(emit: Color) -> Self {
        Self::new_from_texture(Arc::new(SolidColor::new_from_color(emit)))
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Self {
        Self {
            tex
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(crate::color::Color, crate::ray::Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: crate::vec3::Point3) -> Color {
        return self.tex.value(u, v, p);
    }
}
