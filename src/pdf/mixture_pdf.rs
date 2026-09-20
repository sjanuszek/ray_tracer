use crate::{pdf::pdf::PDF, utility::random_double};

pub struct MixturePDF<'a> {
    p0: &'a dyn PDF,
    p1: &'a dyn PDF
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self {
            p0,
            p1
        }
    }
}

impl<'a> PDF for MixturePDF<'a> {
    fn value(&self, direction: &crate::vec3::Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> crate::vec3::Vec3 {
        if random_double() < 0.5 {
            return self.p0.generate();
        } else {
            return self.p1.generate();
        }
    }
}
