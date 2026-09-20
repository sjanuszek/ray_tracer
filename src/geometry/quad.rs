use std::sync::Arc;

use crate::{bvh::aabb::AABB, geometry::{hittable::{HitRecord, Hittable}, hittable_list::HittableList}, interval::Interval, material::material::Material, ray::Ray, utility::{INFINITY, random_double}, vec3::{Point3, Vec3, cross, dot, unit_vector}};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    area: f64
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
            d: dot(normal, q),
            area: n.length()
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

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(*origin, *direction), Interval::new(0.001, INFINITY), &mut rec) {
            return 0.;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (dot(*direction, rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - *origin
    }
}

pub fn quad_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0., 0.);
    let dy = Vec3::new(0., max.y() - min.y(), 0.);
    let dz = Vec3::new(0., 0., max.z() - min.z());

    sides.add(Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone()));
    sides.add(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, mat.clone()));
    sides.add(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, mat.clone()));
    sides.add(Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone()));
    sides.add(Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, mat.clone()));
    sides.add(Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, mat.clone()));

    sides
}
