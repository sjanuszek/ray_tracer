use crate::{color::Color, material::material::Material, ray::Ray, vec3::{dot, random_unit_vector, reflect, unit_vector}};

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::geometry::hittable::HitRecord) -> Option<(Color, crate::ray::Ray, f64)> {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new_with_time(rec.p, reflected, r_in.time());

        if dot(*scattered.direction(), rec.normal) > 0. {
            Some((self.albedo, scattered, 0.))
        } else {
            None
        }
    }
}
