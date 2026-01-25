//! 混合 PDF 实现。

use crate::math::{PdfValue, Vec3};
use crate::sampling::Pdf;
use crate::utils::random_float;

/// 混合 PDF 实现。
pub struct MixturePdf<'a> {
    p0: &'a dyn Pdf,
    p1: &'a dyn Pdf,
}

impl<'a> MixturePdf<'a> {
    /// 创建一个新的混合 PDF 实例。
    ///
    /// # 参数
    ///
    /// * `p0` - 第一个 PDF 实例
    /// * `p1` - 第二个 PDF 实例
    ///
    /// # 返回值
    ///
    /// 一个新的混合 PDF 实例。
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf<'_> {
    fn value(&self, direction: Vec3) -> PdfValue {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_float() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
