//! 朗伯材质模块，定义了朗伯材质的行为。

use crate::geometry::HitRecord;
use crate::material::Material;
use crate::math::{Color, Ray, Vec3};
use crate::texture::{SolidColor, Texture};
use std::f64::consts::PI;
use std::sync::Arc;

/// 朗伯材质
///
/// 朗伯材质是一种基于反射率的材质，它的反射率与入射光线的角度无关。
pub struct Lambertian {
    pub texture: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    /// 创建一个新的朗伯材质实例。
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }

    /// 通过颜色创建朗伯材质
    pub fn from_color(albedo: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let scatter_direction = Vec3::random_on_hemisphere(rec.normal);
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time);
        let attenuation = self.texture.value(rec.uv, &rec.p);
        Some((attenuation, scattered))
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord<'_>, _: &Ray) -> f64 {
        1.0 / (2.0 * PI)
    }
}
