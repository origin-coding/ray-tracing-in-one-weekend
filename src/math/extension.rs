//! 为 Vec3A 实现的扩展 trait，提供一些常用的方法。

use crate::utils::{random_float, random_float_range};
use glam::Vec3A;

/// 为 Vec3A 实现的扩展 trait，提供一些常用的方法。
pub trait Vec3Ext {
    /// 判断向量是否接近于零向量。
    /// 当向量的所有分量的绝对值都小于一个很小的阈值时，认为该向量接近于零向量。
    fn near_zero(&self) -> bool;

    /// 生成一个随机向量，其分量在 [0,1) 之间。
    fn random() -> Self;

    /// 生成一个随机向量，其分量在 [min,max) 之间。
    fn random_range(min: f32, max: f32) -> Self;

    /// 生成一个随机向量，其分量在单位圆盘内。
    fn random_in_unit_disk() -> Self;

    /// 创建一个随机单位向量，向量的每个分量都在 [-1, 1) 范围内。
    fn random_unit() -> Self;

    /// 生成一个余弦分布的随机单位向量（位于 Z 轴半球）
    /// 用于重要性采样
    fn random_cosine_direction() -> Self;
}

impl Vec3Ext for Vec3A {
    #[inline]
    fn near_zero(&self) -> bool {
        const EPSILON: f32 = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    #[inline]
    fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    #[inline]
    fn random_range(min: f32, max: f32) -> Self {
        Self::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    #[inline]
    fn random_in_unit_disk() -> Self {
        loop {
            let random_vector = Self::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );
            let distance_squared = random_vector.length_squared();
            if distance_squared < 1.0 {
                break random_vector;
            }
        }
    }

    #[inline]
    fn random_unit() -> Self {
        loop {
            let random_vector = Self::random_range(-1.0, 1.0);
            let length_squared = random_vector.length_squared();
            if length_squared >= 1e-30 && length_squared <= 1.0 {
                break random_vector / length_squared.sqrt();
            }
        }
    }

    /// 生成一个余弦分布的随机单位向量（位于 Z 轴半球）
    /// 用于重要性采样
    #[inline]
    fn random_cosine_direction() -> Self {
        let r1 = random_float();
        let r2 = random_float();

        // 极坐标转换
        let z = (1.0 - r2).sqrt(); // z 轴分量，越接近 1 概率越大

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Self::new(x, y, z)
    }
}
