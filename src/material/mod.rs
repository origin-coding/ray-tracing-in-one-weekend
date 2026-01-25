//! 材质模块，定义了材质的行为，以及各种不同的实现。

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use crate::geometry::HitRecord;
use crate::math::{Color, Point3, Ray};

use crate::sampling::Pdf;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

/// 散射记录，用于描述光线与物体交互后的结果
pub enum ScatterRecord {
    /// 镜面反射 (Specular)
    /// 比如金属、玻璃。这种类型不需要进行混合概率采样 (PDF)。
    Specular { attenuation: Color, ray: Ray },

    /// 漫反射 (Diffuse)
    /// 比如亚光材质、体积雾。这种类型需要提供一个 PDF 对象来进行重要性采样。
    Diffuse {
        attenuation: Color,
        pdf: Box<dyn Pdf + Send + Sync>,
    },
}

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<'_>) -> Option<ScatterRecord>;

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
    fn emitted(&self, _uv: (f32, f32), _p: &Point3, _r_in: &Ray, _rec: &HitRecord<'_>) -> Color {
        Color::ZERO
    }
}
