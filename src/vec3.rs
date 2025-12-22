//! Vec3 向量库
//!
//! 这是一个简单的 3D 向量数学库，用于《Ray Tracing in one weekend》项目。
//!
//! 实现了向量的基础运算、点积、叉积、归一化等常用操作。

use crate::utils::{random_double, random_double_range};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 三维向量，可以用来表示三维空间中的点、向量或颜色。
///
/// # 示例
/// ```
/// use vec3::Vec3;
///
/// let vec = Vec3::new(1.0, 2.0, 3.0);
/// assert_eq!(vec.x, 1.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// 创建一个新的向量。
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 创建一个零向量。
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// 创建一个全是 1 的向量。
    #[inline]
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// 创建一个单位向量，向量方向为 X 轴。
    #[inline]
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    /// 创建一个单位向量，向量方向为 Y 轴。
    #[inline]
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// 创建一个单位向量，向量方向为 Z 轴。
    #[inline]
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// 创建一个随机向量，向量的每个分量都在 [0, 1) 范围内。
    #[inline]
    pub fn random() -> Self {
        Self {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    /// 创建一个随机向量，向量的每个分量都在 [min, max) 范围内。
    #[inline]
    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    /// 创建一个随机单位向量，向量的每个分量都在 [-1, 1) 范围内。
    #[inline]
    pub fn random_unit() -> Self {
        loop {
            let random_vector = Self::random_range(-1.0, 1.0);
            let length_squared = random_vector.length_squared();
            if length_squared >= 1e-160 && length_squared <= 1.0 {
                break random_vector / length_squared.sqrt();
            }
        }
    }

    /// 创建一个随机单位向量，向量的每个分量都在 [-1, 1) 范围内，且与给定法线的点积大于 0。
    #[inline]
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// 计算向量的模长平方，在某些场景下可以简化向量的比较，避免开方运算造成性能损失。
    #[inline]
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// 计算向量的模长，即向量的长度。
    #[inline]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// 计算两个向量的点积。
    #[inline]
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 叉积，返回两个向量的叉积向量。
    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// 向量归一化
    ///
    /// # 安全性
    /// 如果向量模长接近于 0，则返回零向量，这里参考了 Unity 的实现。
    #[inline]
    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();
        // 如果模长太小，为了避免除以 0 产生 NaN，直接返回零向量
        // 1e-8 是一个很小的数，足以应对浮点误差
        if len > 1e-8 { self / len } else { Vec3::zero() }
    }
}

/// 创建一个默认的零向量。
impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

/// 对向量进行负运算，即它在各个坐标的值取负。
impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

/// 实现两个向量的加法运算。
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// 实现两个向量的减法运算
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// 实现向量乘以一个标量。
impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// 实现标量乘以向量。
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// 实现两个向量的乘法运算，即各个方向分量按位相乘。
/// 注意，这不是点积运算！
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

/// 实现向量除以一个标量。
impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// 实现两个向量的加法运算，并赋值给当前向量（+=）。
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// 实现两个向量的减法运算，并赋值给当前向量（-=）。
impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// 实现向量乘以一个标量，并赋值给当前向量（*=）。
impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

/// 实现向量除以一个标量，并赋值给当前向量（/=）。
impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
