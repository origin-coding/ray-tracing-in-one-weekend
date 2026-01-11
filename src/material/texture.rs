//! 纹理的定义和实现。

use crate::math::{Color, Point3};
use std::sync::Arc;

/// 纹理的 trait，定义了纹理的行为。
pub trait Texture {
    /// 获取纹理在点 p 上的颜色。
    ///
    /// # Arguments
    ///
    /// * `u` - 点 p 在纹理上的 u 坐标。
    /// * `v` - 点 p 在纹理上的 v 坐标。
    /// * `p` - 点 p 的坐标。
    ///
    /// # Returns
    ///
    /// 点 p 上的颜色。
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color;
}

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

/// 棋盘纹理，由两种不同的纹理交替组成。
pub struct Checker {
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
    inv_scale: f64,
}

impl Checker {
    /// 创建一个新的棋盘纹理。
    ///
    /// # Arguments
    /// * `even` - 偶数索引的纹理。
    /// * `odd` - 奇数索引的纹理。
    /// * `inv_scale` - 纹理的缩放因子，用于控制棋盘的大小。
    ///
    /// # Returns
    ///
    /// 新的棋盘纹理。
    pub fn new(
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
        scale: f64,
    ) -> Self {
        Self {
            even,
            odd,
            inv_scale: 1.0 / scale,
        }
    }

    /// 创建一个新的棋盘纹理，使用两种不同的颜色。
    ///
    /// # Arguments
    ///
    /// * `even` - 偶数索引的颜色。
    /// * `odd` - 奇数索引的颜色。
    /// * `inv_scale` - 纹理的缩放因子，用于控制棋盘的大小。
    ///
    /// # Returns
    ///
    /// 新的棋盘纹理。
    pub fn from_color(even: Color, odd: Color, scale: f64) -> Self {
        Self::new(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
            scale,
        )
    }
}

impl Texture for Checker {
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color {
        let x = (self.inv_scale * p.x).floor();
        let y = (self.inv_scale * p.y).floor();
        let z = (self.inv_scale * p.z).floor();

        // 计算点 p 在棋盘上的索引，判断是否为偶数
        let is_even = (x + y + z) as i32 % 2 == 0;

        if is_even {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}
