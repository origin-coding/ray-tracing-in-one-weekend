//! 球体类型的定义，以及它的光线碰撞检测逻辑。

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

/// 球体类型定义，包含球心和半径。
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    /// 创建一个新的球体实例。
    pub fn new(center: Point3, radius: f64) -> Self {
        // 防止半径为负数
        let radius = if radius < 0.0 { 0.0 } else { radius };
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, interval: Interval) -> Option<HitRecord> {
        // 首先计算判别式，并在没有解的情况下直接返回 None
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        };

        // 有解，那么尝试计算两个解，并且找出在 t_min 和 t_max 之间的解，如果没有，那么返回 None
        let mut root = (h - discriminant.sqrt()) / a;
        if !interval.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !interval.surrounds(root) {
                return None;
            };
        }

        // 有解，并且在 t_min 和 t_max 之间，计算 HitRecord
        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, outward_normal, root, r))
    }
}
