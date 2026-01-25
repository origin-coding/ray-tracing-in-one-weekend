//! 球体类型的定义，以及它的光线碰撞检测逻辑。

use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::material::Material;
use crate::math::{Interval, Onb, PdfValue, Point3, Ray, Vec3};
use crate::utils::random_float;
use std::sync::Arc;

/// 球体类型定义，包含球心和半径。
pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    pub bounding_box: Aabb,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    /// 创建一个静止的球体实例。
    pub fn stationary(
        center: Point3,
        radius: f32,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bounding_box = Aabb::from_two_points(center - r_vec, center + r_vec);

        // 防止半径为负数
        Self {
            center: Ray::new(center, Vec3::ZERO),
            radius: if radius < 0.0 { 0.0 } else { radius },
            bounding_box,
            material,
        }
    }

    /// 创建一个移动的球体实例
    pub fn moving(
        center_start: Point3,
        center_end: Point3,
        radius: f32,
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
    fn get_sphere_uv(p: Vec3) -> (f32, f32) {
        // p.y 是 -1 到 1。acos(-p.y) 算出从 0 (南极) 到 PI (北极) 的角度
        // 注意：这里是否取负号取决于你的纹理图片上下方向是否颠倒，通常图形学中 v=0 在底部。
        let theta = (-p.y).acos();

        // atan2(-z, x) 算出绕 Y 轴的角度，范围是 -PI 到 PI
        // +PI 是为了把范围移到 0 到 2PI
        let phi = (-p.z).atan2(p.x) + std::f32::consts::PI;

        let u = phi / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        (u, v)
    }
}

// ... 辅助函数 ...
fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = random_float();
    let r2 = random_float();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
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

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> PdfValue {
        // 这里的逻辑只适用于静止球体
        if self
            .hit(
                &Ray::new(origin, direction),
                Interval::new(0.001, f32::INFINITY),
            )
            .is_none()
        {
            return 0.0;
        }

        let cos_theta_max = (1.0
            - self.radius * self.radius / (self.center.origin - origin).length_squared())
        .sqrt();
        let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.origin - origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::build_from_w(direction);
        uvw.local_vec(random_to_sphere(self.radius, distance_squared))
    }
}
