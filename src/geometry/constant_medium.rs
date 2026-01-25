//! 常量介质的定义及其实现。

use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::math::{Color, Interval, Ray, Vec3};
use crate::texture::SolidColor;
use crate::utils::random_float;
use std::sync::Arc;

/// 常量介质。
///
/// 常量介质是一个在空间中均匀分布的介质，其密度是一个常量。
pub struct ConstantMedium {
    hittable: Arc<dyn Hittable + Send + Sync>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    /// 创建一个新的常量介质。
    ///
    /// # Arguments
    ///
    /// * `hittable` - 介质内部的物体。
    /// * `density` - 介质的密度。
    /// * `phase_function` - 介质的相位函数。
    ///
    /// # Returns
    ///
    /// 新的常量介质。
    pub fn new(
        hittable: Arc<dyn Hittable + Send + Sync>,
        density: f32,
        phase_function: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            hittable,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    /// 创建一个新的常量介质，其相位函数是一个颜色纹理。
    ///
    /// # Arguments
    ///
    /// * `hittable` - 介质内部的物体。
    /// * `density` - 介质的密度。
    /// * `color` - 介质的颜色。
    ///
    /// # Returns
    ///
    /// 新的常量介质。
    pub fn new_with_color(
        hittable: Arc<dyn Hittable + Send + Sync>,
        density: f32,
        color: Color,
    ) -> Self {
        Self::new(
            hittable,
            density,
            Arc::new(Isotropic::new(Arc::new(SolidColor::new(color)))),
        )
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        let mut rec1 = self.hittable.hit(r, Interval::UNIVERSE)?;
        let mut rec2 = self
            .hittable
            .hit(r, Interval::new(rec1.t + 0.0001, f32::INFINITY))?;

        rec1.t = interval.min.max(rec1.t);
        rec2.t = interval.max.min(rec2.t);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_float().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let time = rec1.t + hit_distance / ray_length;
        let rec = HitRecord::new(
            r.at(time),
            Vec3::X, // 任意的法线向量都可以，因为常量介质是均匀分布的
            time,
            (0.0, 0.0),
            r,
            self.phase_function.as_ref(),
        );

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.hittable.bounding_box()
    }
}
