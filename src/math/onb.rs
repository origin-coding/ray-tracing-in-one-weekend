/// 正交基（ONB）的定义和实现。
use crate::math::Vec3;
use glam::Mat3A;

/// 正交基（Onb）
/// 正交基（Onb）是一个包含三个互相正交的单位向量的结构体。
/// 通常用于将一个向量从一个坐标系变换到另一个坐标系。
pub struct Onb {
    mat: Mat3A,
}

impl Onb {
    /// 从 w 构建 ONB
    ///
    /// # Arguments
    ///
    /// * `w` - 法向量 w
    ///
    /// # Returns
    ///
    /// * `Onb` - 从 w 构建的 ONB
    pub fn build_from_w(w: Vec3) -> Self {
        let unit_w = w.normalize();

        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = unit_w.cross(a).normalize();
        let u = v.cross(unit_w);

        Self {
            mat: Mat3A::from_cols(u, v, unit_w),
        }
    }

    /// 将一个向量从本地坐标系变换到世界坐标系
    ///
    /// # Arguments
    ///
    /// * `a` - 本地坐标系的向量
    ///
    /// # Returns
    ///
    /// * `Vec3` - 变换后的向量
    pub fn local_vec(&self, a: Vec3) -> Vec3 {
        self.mat * a
    }

    /// 获取 ONB 的 u 向量
    ///
    /// # Returns
    ///
    /// * `Vec3` - ONB 的 u 向量
    #[inline]
    pub fn u(&self) -> Vec3 {
        self.mat.col(0)
    }

    /// 获取 ONB 的 v 向量
    ///
    /// # Returns
    ///
    /// * `Vec3` - ONB 的 v 向量
    #[inline]
    pub fn v(&self) -> Vec3 {
        self.mat.col(1)
    }

    /// 获取 ONB 的 w 向量
    ///
    /// # Returns
    ///
    /// * `Vec3` - ONB 的 w 向量
    #[inline]
    pub fn w(&self) -> Vec3 {
        self.mat.col(2)
    }
}
