use crate::geometry::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Color, Ray, Vec3, Vec3Ext};

/// 金属材质
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    /// 创建一个新的金属材质实例。
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<ScatterRecord> {
        let reflected = r_in.direction.reflect(rec.normal);
        let scattered = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * Vec3::random_unit(),
            r_in.time,
        );
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord::Specular {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
