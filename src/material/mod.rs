//! 材质模块，定义了材质的行为，以及各种不同的实现。

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use crate::geometry::HitRecord;
use crate::math::{Color, Point3, Ray};

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

/// 材质定义
pub trait Material {
    /// 散射光线
    ///
    /// # 参数
    ///
    /// * `r_in` - 入射光线
    /// * `rec` - 碰撞记录
    ///
    /// # 返回值
    ///
    /// 如果散射成功，返回散射后的颜色和光线；否则返回 None。
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)>;

    /// 计算材质在点 p 上的发光颜色。
    ///
    /// # 参数
    ///
    /// * `uv` - 点 p 在材质上的纹理坐标。
    /// * `p` - 点 p 的位置。
    ///
    /// # 返回值
    ///
    /// 点 p 上的发光颜色。
    fn emitted(&self, _uv: (f64, f64), _p: &Point3) -> Color {
        Color::zero()
    }

    /// 计算材质在点 p 上的散射概率密度函数（PDF）。
    ///
    /// # 参数
    ///
    /// * `r_in` - 入射光线
    /// * `rec` - 碰撞记录
    /// * `scattered` - 散射后的光线
    ///
    /// # 返回值
    ///
    /// 材质在点 p 上的散射概率密度函数值。
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord<'_>, _scattered: &Ray) -> f64 {
        0.0
    }
}
