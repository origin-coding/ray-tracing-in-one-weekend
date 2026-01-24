//! 采样相关的工具方法。

mod cosine_pdf;
mod hittable_pdf;
mod mixture_pdf;
mod sphere_pdf;

use crate::math::{PdfValue, Vec3};

pub use cosine_pdf::CosinePdf;
pub use hittable_pdf::HittablePdf;
pub use mixture_pdf::MixturePdf;
pub use sphere_pdf::SpherePdf;

/// 概率密度函数（PDF）的 trait 定义。
pub trait Pdf {
    /// 计算方向 direction 上的概率密度值。
    ///
    /// # 参数
    ///
    /// * `direction` - 方向向量
    ///
    /// # 返回值
    ///
    /// 方向 direction 上的概率密度值。
    fn value(&self, direction: Vec3) -> PdfValue;

    /// 根据概率密度函数（PDF）生成一个随机方向向量。
    ///
    /// # 返回值
    ///
    /// 一个随机方向向量。
    fn generate(&self) -> Vec3;
}
