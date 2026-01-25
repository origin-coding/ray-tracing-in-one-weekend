use crate::geometry::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Color, Point3, Ray};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

/// 漫反射光源纹理，将颜色映射到点上。
pub struct DiffuseLight {
    /// 光源的颜色纹理。
    texture: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    /// 创建一个新的漫反射光源纹理。
    ///
    /// # Arguments
    ///
    /// * `texture` - 光源的颜色纹理。
    ///
    /// # Returns
    ///
    /// 新的漫反射光源纹理。
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
    }

    /// 创建一个新的漫反射光源纹理，使用纯色颜色。
    ///
    /// # Arguments
    ///
    /// * `color` - 光源的颜色。
    ///
    /// # Returns
    ///
    /// 新的漫反射光源纹理。
    pub fn from_color(color: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(color)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord<'_>) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, uv: (f32, f32), p: &Point3, _r_in: &Ray, rec: &HitRecord<'_>) -> Color {
        if !rec.front_face {
            Color::ZERO
        } else {
            self.texture.value(uv, p)
        }
    }
}
