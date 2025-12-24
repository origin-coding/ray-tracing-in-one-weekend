//! 放置项目内常用的工具函数，如随机数生成等。

use rand::Rng;

/// 生成一个 [0,1) 之间的随机浮点数。
#[inline]
pub fn random_double() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

/// 生成一个 [min,max) 之间的随机浮点数。
#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}

/// 生成一个 [min,max] 之间的随机浮点数。
#[inline]
pub fn random_double_range_inclusive(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..=max)
}

/// 将线性空间的分量转换为 gamma 空间的分量。
#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component >= 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

// 将角度转换为弧度
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
