//! BVH 节点定义。

use crate::Ray;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use std::cmp::Ordering;

pub struct BvhNode {
    pub bounding_box: Aabb,
    pub left: Box<dyn Hittable + Send + Sync>,
    pub right: Box<dyn Hittable + Send + Sync>,
}

impl BvhNode {
    /// 创建一个由多个 hittable 对象组成的 BVH 节点。
    pub fn new(
        mut objects: Vec<Box<dyn Hittable + Send + Sync>>,
    ) -> Box<dyn Hittable + Send + Sync> {
        let axis = rand::random_range(0..=2);
        let comparator = |a: &Box<dyn Hittable + Send + Sync>,
                          b: &Box<dyn Hittable + Send + Sync>| {
            let box_a = a.bounding_box();
            let box_b = b.bounding_box();

            let (min_a, min_b) = match axis {
                0 => (box_a.x.min, box_b.x.min),
                1 => (box_a.y.min, box_b.y.min),
                _ => (box_a.z.min, box_b.z.min),
            };

            min_a.partial_cmp(&min_b).unwrap_or(Ordering::Equal)
        };

        let object_span = objects.len();
        if object_span == 1 {
            objects.pop().unwrap()
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Greater {
                objects.swap(0, 1);
            }
            let right = objects.pop().unwrap();
            let left = objects.pop().unwrap();
            let bounding_box = Aabb::surrounding(left.bounding_box(), right.bounding_box());

            Box::new(BvhNode {
                bounding_box,
                left,
                right,
            })
        } else {
            objects.sort_by(comparator);

            let middle_index = object_span / 2;
            let right_objects = objects.split_off(middle_index);
            let left_objects = objects;

            let left = Self::new(left_objects);
            let right = Self::new(right_objects);
            let bounding_box = Aabb::surrounding(left.bounding_box(), right.bounding_box());

            Box::new(BvhNode {
                bounding_box,
                left,
                right,
            })
        }
    }

    /// 从 HittableList 创建 BVH 节点。
    pub fn from_hittable_list(hittable_list: HittableList) -> Box<dyn Hittable + Send + Sync> {
        Self::new(hittable_list.objects)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        if !self.bounding_box.hit(&r, interval) {
            return None;
        }

        let hit_left = self.left.hit(r, interval);
        let hit_right = self.right.hit(r, interval);

        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
