use crate::{onb::ONB, pdf::pdf::PDF, utility::PI, vec3::{Vec3, dot, random_cosine_direction, unit_vector}};

pub struct CosinePDF {
    uvw: ONB
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: ONB::new(w)
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(unit_vector(*direction), *self.uvw.w());
        (cosine_theta/PI).max(0.)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}
