//! 放置项目内常用的工具函数，如随机数生成等。

use rand::prelude::{Rng, SeedableRng, SmallRng};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng())
}

/// 生成一个 [0,1) 之间的随机浮点数。
#[inline]
pub fn random_float() -> f32 {
    RNG.with(|rng| rng.borrow_mut().random_range(0.0..1.0))
}

/// 生成一个 [min,max) 之间的随机浮点数。
#[inline]
pub fn random_float_range(min: f32, max: f32) -> f32 {
    RNG.with(|rng| rng.borrow_mut().random_range(min..max))
}

/// 生成一个 [min,max] 之间的随机浮点数。
#[inline]
pub fn random_float_range_inclusive(min: f32, max: f32) -> f32 {
    RNG.with(|rng| rng.borrow_mut().random_range(min..=max))
}

/// 生成一个 [min, max) 之间的随机 usize 整数。
/// 主要用于从数组或列表中随机选择元素。
#[inline]
pub fn random_usize_range(min: usize, max: usize) -> usize {
    // random_range 对整数也是左闭右开区间 [min, max)
    RNG.with(|rng| rng.borrow_mut().random_range(min..max))
}

/// 将线性空间的分量转换为 gamma 空间的分量。
#[inline]
pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component >= 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
