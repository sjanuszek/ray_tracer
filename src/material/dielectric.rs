use crate::{color::Color, material::material::Material, ray::Ray, vec3::{refract, unit_vector}};

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(crate::color::Color, crate::ray::Ray)> {
        let ri = if rec.front_face { 1./self.refraction_index } else { self.refraction_index };

        let unit_direction = unit_vector(*r_in.direction());
        let refracted = refract(&unit_direction, &rec.normal, ri);

        Some((Color::new(1., 1., 1.), Ray::new(rec.p, refracted)))
    }
}
