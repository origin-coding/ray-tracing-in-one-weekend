use crate::geometry::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Color, Ray};
use crate::utils::random_double;

/// 电介质材质
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    /// 创建一个新的电介质材质实例。
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    // Schlick 近似计算反射率
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // 计算基础反射率 r0
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        // 套用 Schlick 公式
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<ScatterRecord> {
        let attenuation = Color::one();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        let scattered = Ray::new_with_time(rec.p, direction, r_in.time);
        Some(ScatterRecord::Specular {
            attenuation,
            ray: scattered,
        })
    }
}
