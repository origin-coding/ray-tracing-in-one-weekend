//! 数学计算相关类型及其对应的操作。

pub mod color;
pub mod interval;
mod onb;
mod perlin;
pub mod ray;
pub mod vec3;

pub use color::Color;
pub use interval::Interval;
pub use onb::Onb;
pub use perlin::Perlin;
pub use ray::{Point3, Ray};
pub use vec3::Vec3;
