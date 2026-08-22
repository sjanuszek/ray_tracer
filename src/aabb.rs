use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a[0] <= b[0] { Interval::new(a[0], b[0]) } else { Interval::new(b[0], a[0]) },
            y: if a[1] <= b[1] { Interval::new(a[1], b[1]) } else { Interval::new(b[1], a[1]) },
            z: if a[2] <= b[2] { Interval::new(a[2], b[2]) } else { Interval::new(b[2], a[2]) },
        }
    }

    pub fn new_from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::new_from_intervals(box0.x, box1.x),
            y: Interval::new_from_intervals(box0.y, box1.y),
            z: Interval::new_from_intervals(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1. / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0 }
                if t1 < ray_t.max { ray_t.min = t1 }
            } else {
                if t1 > ray_t.min { ray_t.min = t1 }
                if t0 < ray_t.max { ray_t.max = t0 }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
