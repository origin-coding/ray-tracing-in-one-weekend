//! 纹理的定义和实现。

use crate::math::{Color, Perlin, Point3};
use image::RgbImage;
use std::path::Path;
use std::sync::Arc;

/// 纹理的 trait，定义了纹理的行为。
pub trait Texture {
    /// 获取纹理在点 p 上的颜色。
    ///
    /// # Arguments
    ///
    /// * `u` - 点 p 在纹理上的 u 坐标。
    /// * `v` - 点 p 在纹理上的 v 坐标。
    /// * `p` - 点 p 的坐标。
    ///
    /// # Returns
    ///
    /// 点 p 上的颜色。
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color;
}

/// 纯色纹理，每个点的颜色都是相同的。
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// 创建一个新的纯色纹理。
    ///
    /// # Arguments
    ///
    /// * `albedo` - 纹理的颜色。
    ///
    /// # Returns
    ///
    /// 新的纯色纹理。
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// 创建一个新的纯色纹理，使用 RGB 值。
    ///
    /// # Arguments
    ///
    /// * `r` - 红色通道的值。
    /// * `g` - 绿色通道的值。
    /// * `b` - 蓝色通道的值。
    ///
    /// # Returns
    ///
    /// 新的纯色纹理。
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: (f64, f64), _p: &Point3) -> Color {
        self.albedo
    }
}

/// 棋盘纹理，由两种不同的纹理交替组成。
pub struct Checker {
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
    inv_scale: f64,
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
        scale: f64,
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
    pub fn from_color(even: Color, odd: Color, scale: f64) -> Self {
        Self::new(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
            scale,
        )
    }
}

impl Texture for Checker {
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color {
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

/// 图像纹理，将图像映射到点上。
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    /// 创建一个新的图像纹理。
    ///
    /// # Arguments
    ///
    /// * `path` - 图像文件的路径。
    ///
    /// # Returns
    ///
    /// 新的图像纹理。
    pub fn new(path: &Path) -> Self {
        let image = image::open(path)
            .expect("Failed to open image texture.")
            .to_rgb8();
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, (u, v): (f64, f64), _p: &Point3) -> Color {
        // 先判断图片是否为空，防御性编程，返回洋红色
        if self.image.is_empty() {
            return Color::new(1.0, 0.0, 1.0);
        }

        // 将 u, v 映射到 [0, 1] 范围，防止超出边界
        let u = u.clamp(0.0, 1.0);
        // 图片坐标系的原点通常在左上角 (y向下)，而纹理坐标 v 原点在左下角 (y向上)
        let v = 1.0 - v.clamp(0.0, 1.0);

        // 计算像素坐标，确保不超出图片边界
        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let i = i.min(self.image.width() - 1);
        let j = j.min(self.image.height() - 1);

        // 读取像素值
        let pixel = self.image.get_pixel(i, j);
        // 归一化，将像素值转换到 [0, 1] 范围
        let r = (pixel[0] as f64 / 255.0).powf(2.2);
        let g = (pixel[1] as f64 / 255.0).powf(2.2);
        let b = (pixel[2] as f64 / 255.0).powf(2.2);

        Color::new(r, g, b)
    }
}

/// 噪声纹理，使用 Perlin 噪声生成颜色。
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
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

impl Texture for NoiseTexture {
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
