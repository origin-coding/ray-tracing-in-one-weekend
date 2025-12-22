//! 光线与物体碰撞检测相关逻辑，包含碰撞记录、Hittable Trait。

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

/// 碰撞记录
#[allow(dead_code)]
pub struct HitRecord<'a> {
    /// 碰撞点
    pub p: Point3,
    /// 碰撞点对应的法线向量
    pub normal: Vec3,
    /// 碰撞时间
    pub t: f64,
    /// 碰撞点是否在物体的正前面
    pub front_face: bool,
    /// 碰撞时的材质
    pub mat: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// 创建一个新的碰撞记录实例。
    ///
    /// # 参数
    ///
    /// * `p` - 碰撞点
    /// * `output_normal` - 物体的几何法线（始终指向外），要求是单位向量
    /// * `t` - 碰撞时间
    /// * `ray` - 碰撞时的光线
    pub fn new(p: Point3, output_normal: Vec3, t: f64, ray: Ray, mat: &'a dyn Material) -> Self {
        let front_face = ray.direction.dot(output_normal) < 0.0;
        // 在 front_face 为 false 时，翻转法线向量，存储最终的法线向量
        let normal = if front_face {
            output_normal
        } else {
            -output_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

/// 碰撞检测接口
pub trait Hittable {
    /// 检测光线在给定时间范围内能否与物体发生碰撞
    fn hit(&self, r: Ray, interval: Interval) -> Option<HitRecord<'_>>;
}
