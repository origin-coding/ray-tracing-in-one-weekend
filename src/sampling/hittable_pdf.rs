//! 可命中对象的 PDF 实现。

use crate::geometry::Hittable;
use crate::math::{PdfValue, Point3, Vec3};
use crate::sampling::Pdf;

/// 可命中对象的 PDF 实现。
pub struct HittablePdf<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> HittablePdf<'a> {
    /// 创建一个新的可命中 PDF 实例。
    ///
    /// # 参数
    ///
    /// * `hittable` - 可命中对象
    /// * `origin` - 采样点的原点
    ///
    /// # 返回值
    ///
    /// 一个新的可命中 PDF 实例。
    pub fn new(hittable: &'a dyn Hittable, origin: Point3) -> Self {
        Self {
            objects: hittable,
            origin,
        }
    }
}

impl Pdf for HittablePdf<'_> {
    fn value(&self, direction: Vec3) -> PdfValue {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}
