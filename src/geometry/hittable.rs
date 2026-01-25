//! 光线与物体碰撞检测相关逻辑，包含碰撞记录、Hittable Trait。

use crate::geometry::Aabb;
use crate::material::Material;
use crate::math::{Interval, PdfValue, Point3, Ray, Vec3, Vec3Ext};
use std::sync::Arc;

/// 碰撞记录
pub struct HitRecord<'a> {
    /// 碰撞点
    pub p: Point3,
    /// 碰撞点对应的法线向量
    pub normal: Vec3,
    /// 碰撞时间
    pub t: f32,
    /// 碰撞点是否在物体的正前面
    pub front_face: bool,
    /// 碰撞时的材质
    pub mat: &'a dyn Material,
    /// 碰撞点对应的纹理坐标
    pub uv: (f32, f32),
}

impl<'a> HitRecord<'a> {
    /// 创建一个新的碰撞记录实例。
    ///
    /// # 参数
    ///
    /// * `p` - 碰撞点
    /// * `output_normal` - 物体的几何法线（始终指向外），要求是单位向量
    /// * `t` - 碰撞时间
    /// * `uv` - 碰撞点对应的纹理坐标
    /// * `ray` - 碰撞时的光线
    pub fn new(
        p: Point3,
        output_normal: Vec3,
        t: f32,
        uv: (f32, f32),
        ray: &Ray,
        mat: &'a dyn Material,
    ) -> Self {
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
            uv,
        }
    }
}

/// 碰撞检测接口
pub trait Hittable {
    /// 检测光线在给定时间范围内能否与物体发生碰撞
    /// # 参数
    ///
    /// * `r` - 碰撞检测时的光线
    /// * `interval` - 碰撞检测的时间范围
    ///
    /// # 返回值
    ///
    /// 如果光线与物体发生碰撞，则返回碰撞记录；否则返回 None。
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>>;

    /// 获取物体的碰撞检测盒
    /// # 返回值
    ///
    /// 物体的碰撞检测盒。
    fn bounding_box(&self) -> Aabb;

    /// 计算从原点到方向向量的 PDF 值
    /// # 参数
    ///
    /// * `origin` - 计算 PDF 值的原点
    /// * `direction` - 计算 PDF 值的方向向量
    ///
    /// # 返回值
    ///
    /// 从原点到方向向量的 PDF 值。
    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> PdfValue {
        0.0
    }

    /// 从原点生成一个随机方向向量
    ///
    /// # 参数
    ///
    /// * `origin` - 生成随机方向向量的原点
    ///
    /// # 返回值
    ///
    /// 一个随机方向向量。
    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::random_unit()
    }
}

/// 实现 Hittable Trait 对 Box 类型的支持
impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        (**self).hit(r, interval)
    }

    fn bounding_box(&self) -> Aabb {
        (**self).bounding_box()
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> PdfValue {
        (**self).pdf_value(origin, direction)
    }

    fn random(&self, origin: Point3) -> Vec3 {
        (**self).random(origin)
    }
}

/// 实现 Hittable Trait 对 Arc 类型的支持
impl<T: Hittable + ?Sized> Hittable for Arc<T> {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        (**self).hit(r, interval)
    }

    fn bounding_box(&self) -> Aabb {
        (**self).bounding_box()
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> PdfValue {
        (**self).pdf_value(origin, direction)
    }

    fn random(&self, origin: Point3) -> Vec3 {
        (**self).random(origin)
    }
}
