use crate::geometry::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::Ray;
use crate::sampling::SpherePdf;
use crate::texture::Texture;
use std::sync::Arc;

/// 全反射材质
pub struct Isotropic {
    texture: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    /// 创建一个新的全反射材质实例。
    /// # Arguments
    ///
    /// * `texture` - 材质的颜色纹理。
    ///
    /// # Returns
    ///
    /// 新的全反射材质实例。
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord<'_>) -> Option<ScatterRecord> {
        let attenuation = self.texture.value(rec.uv, &rec.p);
        let pdf = Box::new(SpherePdf);

        Some(ScatterRecord::Diffuse { attenuation, pdf })
    }
}
