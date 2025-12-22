//! 时间区间的类型定义、相关常量和方法。

/// 时间区间的类型定义，包含一个最小时间和一个最大时间。
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    /// 空区间，任何时间点都无法被包含在其中。
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };
    /// 全局时间区间，包含所有时间点。
    pub const UNIVERSE: Self = Self {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };

    /// 创建一个新的时间区间。
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// 判断一个时间点是否被区间所包含（包含边界）。
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// 判断一个时间点是否被区间所包围（不包含边界）。
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// 将一个时间点限制在区间内。
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    /// 默认时间区间为空区间。
    fn default() -> Self {
        Self::EMPTY
    }
}
