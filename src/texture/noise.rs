//! 噪声纹理模块，定义了噪声纹理的行为。

use crate::math::{Color, Perlin, Point3};
use crate::texture::Texture;

/// 噪声纹理，使用 Perlin 噪声生成颜色。
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    /// 创建一个新的噪声纹理。
    ///
    /// # Arguments
    ///
    /// * `scale` - 噪声的缩放因子，用于控制噪声的范围。
    ///
    /// # Returns
    ///
    /// 新的噪声纹理。
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: (f64, f64), p: &Point3) -> Color {
        // 计算点 p 的噪声值，范围 [-1, 1]
        let turbulence = self.noise.turbulence(*p, 7);

        // 对 z 坐标进行扰动，生成大理石纹路
        // scale * z 负责制造基础的条纹
        // 10.0 * turbulence 负责添加细节，增加.marble 效果
        let s = self.scale * p.z + 10.0 * turbulence;

        // 将 s 映射到 [0, 1] 范围，用于颜色插值
        Color::one() * 0.5 * (1.0 + s.sin())
    }
}
