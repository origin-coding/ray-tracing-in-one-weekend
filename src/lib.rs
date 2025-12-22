//! 项目跟模块，包含常用的模块和导出类型。

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;
pub mod material;

pub use color::Color;
pub use ray::{Point3, Ray};
pub use vec3::Vec3;
