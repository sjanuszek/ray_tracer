use crate::{pdf::pdf::PDF, utility::PI, vec3::random_unit_vector};

pub struct SpherePDF {}

impl SpherePDF {
    pub fn new() -> Self {
        Self {}
    }
}

impl PDF for SpherePDF {
    fn value(&self, direction: &crate::vec3::Vec3) -> f64 {
        1. / (4. * PI)
    }

    fn generate(&self) -> crate::vec3::Vec3 {
        random_unit_vector()
    }
}
