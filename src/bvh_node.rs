use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::AABB, geometry::{hittable::Hittable, hittable_list::HittableList}, interval::Interval, utility::random_int};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Self {
        let size = list.objects.len();
        Self::new_from_vec(&mut list.objects, 0, size)
    }
    fn new_from_vec(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for object_index in start..end {
            bbox = AABB::new_from_boxes(&bbox, objects[object_index].bounding_box());
        }
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => BVHNode::box_x_compare,
            1 => BVHNode::box_y_compare,
            _ => BVHNode::box_z_compare,
        };

        let object_span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();

            },
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            },
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span/2;

                left = Arc::new(BVHNode::new_from_vec(objects, start, mid));
                right = Arc::new(BVHNode::new_from_vec(objects, mid, end));
            }
        }

        Self {
            left,
            right,
            bbox
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_min = a.bounding_box().axis_interval(axis_index).min;
        let b_min = b.bounding_box().axis_interval(axis_index).min;
        a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut crate::geometry::hittable::HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
