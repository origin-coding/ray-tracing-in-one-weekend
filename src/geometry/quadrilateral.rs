//! 四边形类型的定义，以及它的光线碰撞检测逻辑。

use crate::geometry::{Aabb, HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::math::{Interval, PdfValue, Point3, Ray, Vec3};
use crate::utils::random_float;
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
    d: f32,
    w: Vec3, // 用于快速计算平面坐标 alpha, beta

    // 四边形的面积，方便计算 PDF 值
    area: f32,
}

impl Quadrilateral {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        // 1. 计算向量 n (未归一化的法线)
        let n = u.cross(v);

        // 2. 计算法线 normal (归一化)
        let normal = n.normalize();

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
            area: n.length(),
        }
    }

    /// 创建一个盒子，由两个点 a, b 定义的矩形表面，材质为 material。
    ///
    /// # Arguments
    ///
    /// * `a` - 盒子的一个顶点。
    /// * `b` - 盒子的另一个顶点。
    /// * `material` - 盒子的材质。
    ///
    /// # Returns
    ///
    /// 包含六个四边形的 HittableList，构成一个盒子。
    pub fn create_box(
        a: &Point3,
        b: &Point3,
        material: Arc<dyn Material + Send + Sync>,
    ) -> HittableList {
        let mut sides = HittableList::new();

        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Box::new(Quadrilateral::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        )));
        sides.add(Box::new(Quadrilateral::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        )));
        sides.add(Box::new(Quadrilateral::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        )));
        sides.add(Box::new(Quadrilateral::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        )));
        sides.add(Box::new(Quadrilateral::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        )));
        sides.add(Box::new(Quadrilateral::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        )));

        sides
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
            (alpha, beta), // 四边形的 uv 一般在 [0, 1] 范围内，使用 alpha, beta 表示
            r,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> PdfValue {
        // 1. 射线-平面求交
        let denom = self.normal.dot(direction);

        // 如果光线与平面平行，PDF 为 0
        if denom.abs() < 1e-8 {
            return 0.0;
        }

        // 计算 t
        // 理论上，如果是对光源采样，direction = target - origin，所以 t 应该等于 1.0
        // 但这里我们计算出真实的 t，用于后续校验
        let t = (self.d - self.normal.dot(origin)) / denom;

        // 2. 检查 t 是否有效
        // 我们只关心从 origin 发出的光线。
        // 对于光源采样，t 应该在 1.0 附近；对于材质采样，t 只要 > 0.001 即可。
        // 这里为了通用，我们只限制最小距离，防止自相交。
        if t < 0.001 {
            return 0.0;
        }

        // 3. 检查落点是否在四边形内 (这是最关键的一步！)
        let intersection = origin + t * direction;
        let planer_hit_point_vector = intersection - self.q;

        let alpha = self.w.dot(planer_hit_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planer_hit_point_vector));

        // ⚠️ 关键修复：使用宽松的边界检查 (EPSILON)
        // 允许 -0.001 到 1.001 的范围，防止因为浮点误差导致明明在表面上的点被判定为无效
        const EPSILON: f32 = 1e-3;
        if alpha < -EPSILON || alpha > 1.0 + EPSILON || beta < -EPSILON || beta > 1.0 + EPSILON {
            return 0.0;
        }

        // 4. 计算 PDF
        let distance_squared = t * t * direction.length_squared();
        let cosine = (denom / direction.length()).abs();

        if cosine < 1e-8 {
            return 0.0;
        }

        distance_squared / (self.area * cosine)
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let p = self.q + self.u * random_float() + self.v * random_float();
        p - origin
    }
}
