use crate::vec3::Vec3;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.
    }

    fn generate(&self) -> Vec3 {
        Vec3::default()
    }
}
