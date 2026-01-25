//! 数学计算相关类型及其对应的操作。

pub mod color;
pub mod extension;
pub mod interval;
mod onb;
mod perlin;
pub mod ray;

pub use color::Color;
use glam::Vec3A;
pub use interval::Interval;
pub use onb::Onb;
pub use perlin::Perlin;
pub use ray::{Point3, Ray};

/// 概率密度函数（PDF）的值类型。
pub type PdfValue = f32;

/// 使用 Glam 库中的 Vec3A 类型替代原有的 Vec3 类型。
pub type Vec3 = Vec3A;
pub use extension::Vec3Ext;
