//! 朗伯材质模块，定义了朗伯材质的行为。

use crate::geometry::HitRecord;
use crate::material::{Material, Pdf};
use crate::math::{Color, Onb, Ray, Vec3};
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray, Option<Pdf>)> {
        let uvw = Onb::build_from_w(rec.normal);

        let scatter_direction = uvw.local_vec(Vec3::random_cosine_direction());

        let scattered = Ray::new_with_time(rec.p, scatter_direction.unit_vector(), r_in.time);
        let attenuation = self.texture.value(rec.uv, &rec.p);

        let pdf = uvw.w.dot(scattered.direction) / PI; // 构造光线时已经求了单位向量，所以这里直接 dot 即可

        Some((attenuation, scattered, Some(pdf)))
    }

    fn scattering_pdf(&self, _: &Ray, rec: &HitRecord<'_>, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(scattered.direction.unit_vector());
        if cos_theta <= 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
