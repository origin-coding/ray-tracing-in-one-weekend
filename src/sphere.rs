//! 球体类型的定义，以及它的光线碰撞检测逻辑。

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use std::sync::Arc;

/// 球体类型定义，包含球心和半径。
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub bounding_box: Aabb,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    /// 创建一个静止的球体实例。
    pub fn stationary(
        center: Point3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bounding_box = Aabb::from_two_points(center - r_vec, center + r_vec);

        // 防止半径为负数
        Self {
            center: Ray::new(center, Vec3::zero()),
            radius: if radius < 0.0 { 0.0 } else { radius },
            bounding_box,
            material,
        }
    }

    /// 创建一个移动的球体实例
    pub fn moving(
        center_start: Point3,
        center_end: Point3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bounding_box_1 = Aabb::from_two_points(center_start - r_vec, center_end + r_vec);
        let bounding_box_2 = Aabb::from_two_points(center_end - r_vec, center_start + r_vec);
        let bounding_box = Aabb::surrounding(bounding_box_1, bounding_box_2);

        Self {
            center: Ray::new(center_start, center_end - center_start),
            radius: if radius < 0.0 { 0.0 } else { radius },
            bounding_box,
            material,
        }
    }

    /// 计算点 p 在球体上的 UV 坐标。
    fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        // p.y 是 -1 到 1。acos(-p.y) 算出从 0 (南极) 到 PI (北极) 的角度
        // 注意：这里是否取负号取决于你的纹理图片上下方向是否颠倒，通常图形学中 v=0 在底部。
        let theta = (-p.y).acos();

        // atan2(-z, x) 算出绕 Y 轴的角度，范围是 -PI 到 PI
        // +PI 是为了把范围移到 0 到 2PI
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        // 计算当前球体的位置
        let current_center = self.center.at(r.time);
        // 首先计算判别式，并在没有解的情况下直接返回 None
        let oc = current_center - r.origin;
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
        let outward_normal = (point - current_center) / self.radius;
        Some(HitRecord::new(
            point,
            outward_normal,
            root,
            Sphere::get_sphere_uv(outward_normal),
            r,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
