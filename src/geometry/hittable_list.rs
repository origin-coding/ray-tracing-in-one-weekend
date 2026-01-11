//! 可碰撞物体列表
use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::math::{Interval, Ray};

pub struct HittableList {
    pub bounding_box: Aabb,
    /// 物体列表
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    /// 创建一个新的空 HittableList 实例。
    pub fn new() -> Self {
        Self {
            bounding_box: Aabb::empty(),
            objects: Vec::new(),
        }
    }

    /// 添加一个物体到列表中
    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.bounding_box = Aabb::surrounding(self.bounding_box, object.bounding_box());
        self.objects.push(object);
    }

    /// 清空物体列表
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        let mut hit_record = None;
        let mut closest_so_far = interval.max;
        for object in &self.objects {
            if let Some(record) = object.hit(r, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
