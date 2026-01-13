//! 几何形状定义以及相关工具方法。

pub mod aabb;
pub mod bvh_node;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
mod quadrilateral;

pub use aabb::Aabb;
pub use bvh_node::BvhNode;
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use quadrilateral::Quadrilateral;
