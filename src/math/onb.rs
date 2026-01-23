/// 正交基（ONB）的定义和实现。
use crate::math::Vec3;

/// 正交基（Onb）
/// 正交基（Onb）是一个包含三个互相正交的单位向量的结构体。
/// 通常用于将一个向量从一个坐标系变换到另一个坐标系。
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
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
        let unit_w = w.unit_vector();

        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = unit_w.cross(a).unit_vector();
        let u = v.cross(unit_w);

        Self { u, v, w: unit_w }
    }

    /// 将一个向量从本地坐标系变换到世界坐标系
    ///
    /// # Arguments
    ///
    /// * `a` - 本地坐标系的 x 分量
    /// * `b` - 本地坐标系的 y 分量
    /// * `c` - 本地坐标系的 z 分量
    ///
    /// # Returns
    ///
    /// * `Vec3` - 变换后的向量
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u + b * self.v + c * self.w
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
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
