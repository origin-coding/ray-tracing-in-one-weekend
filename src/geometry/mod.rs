//! 几何形状定义以及相关工具方法。

pub mod aabb;
pub mod bvh_node;
mod constant_medium;
pub mod hittable;
pub mod hittable_list;
mod quadrilateral;
mod rotate_y;
pub mod sphere;
mod translate;

pub use aabb::Aabb;
pub use bvh_node::BvhNode;
pub use constant_medium::ConstantMedium;
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use quadrilateral::Quadrilateral;
pub use rotate_y::RotateY;
pub use sphere::Sphere;
pub use translate::Translate;
