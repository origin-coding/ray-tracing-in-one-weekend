//! 棋盘纹理模块，定义了棋盘纹理的行为。

use crate::math::{Color, Point3};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

/// 棋盘纹理，由两种不同的纹理交替组成。
pub struct Checker {
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
    inv_scale: f32,
}

impl Checker {
    /// 创建一个新的棋盘纹理。
    ///
    /// # Arguments
    /// * `even` - 偶数索引的纹理。
    /// * `odd` - 奇数索引的纹理。
    /// * `inv_scale` - 纹理的缩放因子，用于控制棋盘的大小。
    ///
    /// # Returns
    ///
    /// 新的棋盘纹理。
    pub fn new(
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
        scale: f32,
    ) -> Self {
        Self {
            even,
            odd,
            inv_scale: 1.0 / scale,
        }
    }

    /// 创建一个新的棋盘纹理，使用两种不同的颜色。
    ///
    /// # Arguments
    ///
    /// * `even` - 偶数索引的颜色。
    /// * `odd` - 奇数索引的颜色。
    /// * `inv_scale` - 纹理的缩放因子，用于控制棋盘的大小。
    ///
    /// # Returns
    ///
    /// 新的棋盘纹理。
    pub fn from_color(even: Color, odd: Color, scale: f32) -> Self {
        Self::new(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
            scale,
        )
    }
}

impl Texture for Checker {
    fn value(&self, uv: (f32, f32), p: &Point3) -> Color {
        let x = (self.inv_scale * p.x).floor();
        let y = (self.inv_scale * p.y).floor();
        let z = (self.inv_scale * p.z).floor();

        // 计算点 p 在棋盘上的索引，判断是否为偶数
        let is_even = (x + y + z) as i32 % 2 == 0;

        if is_even {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}
