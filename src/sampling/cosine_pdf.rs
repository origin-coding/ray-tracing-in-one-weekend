//! 余弦 PDF 实现。

use crate::math::{Onb, PdfValue, Vec3, Vec3Ext};
use crate::sampling::Pdf;

/// 余弦 PDF 实现。
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    /// 创建一个新的余弦 PDF 实例。
    ///
    /// # 参数
    ///
    /// * `w` - 余弦 PDF 对应的方向向量
    ///
    /// # 返回值
    ///
    /// 一个新的余弦 PDF 实例。
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> PdfValue {
        let cos_theta = direction.normalize().dot(self.uvw.w());
        if cos_theta <= 0.0 {
            0.0
        } else {
            cos_theta / std::f32::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(Vec3::random_cosine_direction())
    }
}
