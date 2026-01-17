//! 纯色纹理模块，定义了纯色纹理的行为。

use crate::math::{Color, Point3};
use crate::texture::Texture;

/// 纯色纹理，每个点的颜色都是相同的。
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// 创建一个新的纯色纹理。
    ///
    /// # Arguments
    ///
    /// * `albedo` - 纹理的颜色。
    ///
    /// # Returns
    ///
    /// 新的纯色纹理。
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// 创建一个新的纯色纹理，使用 RGB 值。
    ///
    /// # Arguments
    ///
    /// * `r` - 红色通道的值。
    /// * `g` - 绿色通道的值。
    /// * `b` - 蓝色通道的值。
    ///
    /// # Returns
    ///
    /// 新的纯色纹理。
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: (f64, f64), _p: &Point3) -> Color {
        self.albedo
    }
}
