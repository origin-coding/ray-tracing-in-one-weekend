//! 轴对齐边界框（Axis-Aligned Bounding Box）
//! 是一个简单的矩形框，用于快速判断射线是否与场景中的物体相交。
//! 它的边与坐标轴平行，且不考虑物体的旋转。
//! 它的主要作用是将场景中的物体分组，减少射线与物体的交点计算次数。

use crate::math::{Interval, Point3, Ray, Vec3};
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    const DELTA: f32 = 0.0001;

    /// 创建一个新的轴对齐边界框。
    #[inline]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// 创建一个空的轴对齐边界框。
    #[inline]
    pub fn empty() -> Self {
        Self::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY)
    }

    /// 创建一个由两个点定义的轴对齐边界框。
    #[inline]
    pub fn from_two_points(a: Point3, b: Point3) -> Self {
        let mut aabb = Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        };
        aabb.pad_if_needed();
        aabb
    }

    /// 创建一个包含两个 Aabb 的新 Aabb。
    #[inline]
    pub fn surrounding(a: Self, b: Self) -> Self {
        // 在构建原始 Aabb 时已经按需扩展了一次，这里无需再扩展
        Self {
            x: Interval::enclosing(a.x, b.x),
            y: Interval::enclosing(a.y, b.y),
            z: Interval::enclosing(a.z, b.z),
        }
    }

    /// 检测光线是否与边界框相交
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        // 遍历三个轴 (x=0, y=1, z=2)
        for axis in 0..3 {
            let ax_interval = match axis {
                0 => &self.x,
                1 => &self.y,
                _ => &self.z,
            };

            let ax_origin = match axis {
                0 => r.origin.x,
                1 => r.origin.y,
                _ => r.origin.z,
            };

            let ax_dir = match axis {
                0 => r.direction.x,
                1 => r.direction.y,
                _ => r.direction.z,
            };

            // 1. 计算两个交点 t0, t1
            // 技巧：预先计算 1.0 / dir 可以稍快一点，但在现代 CPU 上差异不大
            let t0 = (ax_interval.min - ax_origin) / ax_dir;
            let t1 = (ax_interval.max - ax_origin) / ax_dir;

            // 2. & 3. 处理负方向 & 收缩区间
            // 无论 t0 和 t1 谁大谁小，min(t0, t1) 永远是进入点，max(t0, t1) 永远是离开点。
            t_min = t0.min(t1).max(t_min);
            t_max = t0.max(t1).min(t_max);

            // 4. 提前退出判断
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    /// 如果边界框的任何轴的大小小于 DELTA，则扩展该轴。
    fn pad_if_needed(&mut self) {
        if self.x.size() < Self::DELTA {
            self.x = self.x.expand(Self::DELTA);
        }
        if self.y.size() < Self::DELTA {
            self.y = self.y.expand(Self::DELTA);
        }
        if self.z.size() < Self::DELTA {
            self.z = self.z.expand(Self::DELTA);
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
