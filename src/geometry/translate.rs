//! 平移变换的定义及其实现。

use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::math::{Interval, Ray, Vec3};
use std::sync::Arc;

/// 平移变换。
///
/// 平移变换将一个物体在空间中平移一个固定的向量。
pub struct Translate {
    offset: Vec3,
    hittable: Arc<dyn Hittable + Send + Sync>,
    aabb: Aabb,
}

impl Translate {
    /// 创建一个新的平移变换。
    ///
    /// # Arguments
    ///
    /// * `offset` - 平移向量。
    /// * `hittable` - 要平移的物体。
    ///
    /// # Returns
    ///
    /// 新的平移变换。
    pub fn new(hittable: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let aabb = hittable.bounding_box() + offset;
        Self {
            offset,
            hittable,
            aabb,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        let r = Ray::new_with_time(r.origin - self.offset, r.direction, r.time);

        if let Some(mut record) = self.hittable.hit(&r, interval) {
            record.p += self.offset;
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
