use crate::{color::Color, material::material::Material, ray::Ray, vec3::reflect};

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(Color, crate::ray::Ray)> {
        let reflected = reflect(r_in.direction(), &rec.normal);

        Some((self.albedo, Ray::new(rec.p, reflected)))
    }
}
