use std::sync::Arc;

use crate::{bvh::aabb::AABB, geometry::hittable::Hittable, ray::Ray, utility::{INFINITY, degrees_to_radians}, vec3::{Point3, Vec3}};

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = *object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            let fi = i as f64;
            for j in 0..2 {
                let fj = j as f64;
                for k in 0..2 {
                    let fk = k as f64;

                    let x = fi*bbox.x.max + (1.-fi)*bbox.x.min;
                    let y = fj*bbox.y.max + (1.-fj)*bbox.y.min;
                    let z = fk*bbox.z.max + (1.-fk)*bbox.z.min;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = AABB::new_from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z())
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z())
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z())
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z())
        );

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
