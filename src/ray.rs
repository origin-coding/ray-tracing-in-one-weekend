#![allow(dead_code)]

//! 光线类型定义和相关方法。

use crate::vec3::Vec3;

/// 三维空间中的点，Vec3 的一个别名。
pub type Point3 = Vec3;

/// 三维空间中的射线，包含起始点以及方向向量。
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// 创建一个新的光线实例。
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// 计算光线在 t 时刻到达的位置。
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
