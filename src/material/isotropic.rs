use crate::geometry::HitRecord;
use crate::material::Material;
use crate::math::{Color, Ray, Vec3};
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let scattered = Ray::new_with_time(rec.p, Vec3::random_unit(), r_in.time);
        let attenuation = self.texture.value(rec.uv, &rec.p);
        Some((attenuation, scattered))
    }
}
