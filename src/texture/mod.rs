//! 纹理模块，定义了纹理的行为，以及模块下各种不同的纹理实现。

use crate::math::{Color, Point3};

mod checker;
mod image;
mod noise;
mod solid_color;

pub use checker::Checker;
pub use image::Image;
pub use noise::Noise;
pub use solid_color::SolidColor;

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
