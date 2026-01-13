//! 四边形类型的定义，以及它的光线碰撞检测逻辑。

use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::material::Material;
use crate::math::{Interval, Point3, Ray, Vec3};
use std::sync::Arc;

/// 四边形类型定义，包含起始点 Q 和两个向量 U、V，以及材质。
pub struct Quadrilateral {
    // 几何参数 (私有，防止外部直接修改导致缓存失效)
    q: Point3,
    u: Vec3,
    v: Vec3,

    // 材质和包围盒
    material: Arc<dyn Material + Send + Sync>,
    bounding_box: Aabb,

    // 预计算的优化参数 (对应 C++ 中的 n, D, w)
    normal: Vec3,
    d: f64,
    w: Vec3, // 用于快速计算平面坐标 alpha, beta
}

impl Quadrilateral {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        // 1. 计算向量 n (未归一化的法线)
        let n = u.cross(v);

        // 2. 计算法线 normal (归一化)
        let normal = n.unit_vector();

        // 3. 计算平面方程常数 D = n . Q
        let d = normal.dot(q);

        // 4. 计算 w 向量 (优化项：n / (n . n))
        // 这个向量用于在 hit 函数中快速解出 alpha 和 beta
        let w = n / n.dot(n);

        // 5. 计算包围盒
        // 四边形的包围盒就是对角线两个点 (Q) 和 (Q + u + v) 构成的 AABB
        // 注意：我们刚才修改了 AABB，现在它会自动处理 padding
        let bounding_box = Aabb::from_two_points(q, q + u + v);

        Self {
            q,
            u,
            v,
            material,
            bounding_box,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quadrilateral {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        // 如果光线与四边形的法线垂直，直接返回 None
        let denom = self.normal.dot(r.direction);
        if denom.abs() < 1e-8 {
            return None;
        }

        // 如果 t 不在范围内，直接返回 None
        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !interval.contains(t) {
            return None;
        }

        // 计算光线与平面的交点 p = r(t) 是否在四边形内
        // 如果 alpha, beta 都在 [0, 1] 范围内，说明 p 在四边形内
        let intersection = r.at(t);
        let planer_hit_point_vector = intersection - self.q;
        let alpha = self.w.dot(planer_hit_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planer_hit_point_vector));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            t,
            (alpha, beta),  // 四边形的 uv 一般在 [0, 1] 范围内，使用 alpha, beta 表示
            r,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
