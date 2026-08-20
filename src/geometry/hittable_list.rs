use crate::geometry::hittable::{HitRecord, Hittable};

pub struct HittableList<T: Hittable> {
    pub objects: Vec<T>
}

impl <T: Hittable>HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl <T: Hittable>Hittable for HittableList<T> {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut super::hittable::HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // this is bad
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.p = temp_rec.p;
            }
        }
        return hit_anything;
    }
}
