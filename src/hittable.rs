//! 光线与物体碰撞检测相关逻辑，包含碰撞记录、Hittable Trait。

use crate::interval::Interval;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

/// 碰撞记录
#[allow(dead_code)]
pub struct HitRecord {
    /// 碰撞点
    pub p: Point3,
    /// 碰撞点对应的法线向量
    pub normal: Vec3,
    /// 碰撞时间
    pub t: f64,
    /// 碰撞点是否在物体的正前面
    pub front_face: bool,
}

impl HitRecord {
    /// 创建一个新的碰撞记录实例。
    ///
    /// # 参数
    ///
    /// * `p` - 碰撞点
    /// * `output_normal` - 物体的几何法线（始终指向外）
    /// * `t` - 碰撞时间
    /// * `ray` - 碰撞时的光线
    pub fn new(p: Point3, output_normal: Vec3, t: f64, ray: Ray) -> Self {
        let front_face = ray.direction.dot(output_normal) < 0.0;
        // 在 front_face 为 false 时，翻转法线向量，存储最终的法线向量
        let normal = if front_face {
            output_normal
        } else {
            -output_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

/// 碰撞检测接口
pub trait Hittable {
    /// 检测光线在给定时间范围内能否与物体发生碰撞
    fn hit(&self, r: Ray, interval: Interval) -> Option<HitRecord>;
}
