use crate::math::{Color, Point3};
use crate::texture::Texture;
use image::RgbImage;
use std::path::Path;

/// 图像纹理，将图像映射到点上。
pub struct Image {
    image: RgbImage,
}

impl Image {
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

impl Texture for Image {
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
