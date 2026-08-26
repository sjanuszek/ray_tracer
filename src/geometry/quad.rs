use std::sync::Arc;

use crate::{bvh::aabb::AABB, geometry::hittable::Hittable, interval::Interval, material::material::Material, vec3::{Point3, Vec3, cross, dot, unit_vector}};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);

        let mut quad = Self {
            q,
            u,
            v,
            w: n / dot(n, n),
            mat,
            bbox: AABB::default(),
            normal,
            d: dot(normal, q)
        };
        
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::new_from_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = AABB::new_from_points(self.q + self.u, self.q + self.v);
        self.bbox = AABB::new_from_boxes(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(&self, a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0., 1.);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let denom = dot(self.normal, *r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - dot(self.normal, *r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if !self.is_interior(alpha, beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;
        
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
