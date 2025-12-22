//! 材质定义以及相关工具方法。

use crate::hittable::HitRecord;
use crate::{Color, Ray, Vec3};

/// 材质定义
pub trait Material {
    /// 散射光线
    ///
    /// # 参数
    ///
    /// * `r_in` - 入射光线
    /// * `rec` - 碰撞记录
    ///
    /// # 返回值
    ///
    /// 如果散射成功，返回散射后的颜色和光线；否则返回 None。
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)>;
}

/// 朗伯材质
///
/// 朗伯材质是一种基于反射率的材质，它的反射率与入射光线的角度无关。
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    /// 创建一个新的朗伯材质实例。
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_unit();
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

/// 金属材质
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    /// 创建一个新的金属材质实例。
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let reflected = r_in.direction.reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit());
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
