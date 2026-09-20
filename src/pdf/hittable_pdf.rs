use crate::{geometry::hittable::Hittable, pdf::pdf::PDF, vec3::Point3};

pub struct HittablePDF<'a> {
    objects: &'a dyn Hittable,
    origin: Point3
}

impl<'a> HittablePDF<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: &Point3) -> Self {
        Self {
            objects,
            origin: *origin
        }
    }
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: &crate::vec3::Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> crate::vec3::Vec3 {
        self.objects.random(&self.origin)
    }
}
