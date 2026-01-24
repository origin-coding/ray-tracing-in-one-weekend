//! 球面上的均匀 PDF 实现。

use crate::math::{PdfValue, Vec3};
use crate::sampling::Pdf;

/// 球面上的均匀 PDF 实现。
pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _: Vec3) -> PdfValue {
        1.0 / (4.0 * std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit()
    }
}
