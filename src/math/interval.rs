//! 时间区间的类型定义、相关常量和方法。

use std::ops::Add;

/// 时间区间的类型定义，包含一个最小时间和一个最大时间。
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

#[allow(dead_code)]
impl Interval {
    /// 空区间，任何时间点都无法被包含在其中。
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: -f32::INFINITY,
    };
    /// 全局时间区间，包含所有时间点。
    pub const UNIVERSE: Self = Self {
        min: -f32::INFINITY,
        max: f32::INFINITY,
    };

    /// 创建一个新的时间区间。
    #[inline]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    /// 创建一个时间区间，包含给定的两个区间。
    #[inline]
    pub fn enclosing(i1: Self, i2: Self) -> Self {
        Self {
            min: i1.min.min(i2.min),
            max: i1.max.max(i2.max),
        }
    }

    /// 判断一个时间点是否被区间所包含（包含边界）。
    #[inline]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    /// 判断一个时间点是否被区间所包围（不包含边界）。
    #[inline]
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    /// 将一个时间点限制在区间内。
    #[inline]
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    /// 扩展时间范围
    #[inline]
    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// 返回时间区间的大小（宽度）。
    #[inline]
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
}

impl Default for Interval {
    /// 默认时间区间为空区间。
    #[inline]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f32 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}
