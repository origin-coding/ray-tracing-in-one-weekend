//! 相机定义以及相关工具方法。

use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::{Point3, Ray};
use crate::utils::random_double_range_inclusive;
use crate::vec3::Vec3;

/// 相机类型定义。
///
/// 示例：
/// ```
/// use crate::camera::Camera;
/// let camera = Camera::new(16.0 / 9.0, 400, 100);
/// ```
#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,

    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_scale: f64,
}

impl Camera {
    const P3_MAGIC_NUMBER: &str = "P3";
    const MAX_COLOR_VALUE: i32 = 255;

    /// 创建一个新的相机实例。
    ///
    /// # 参数
    ///
    /// * `aspect_ratio` - 相机的宽高比，通常为 16:9 或 4:3。
    /// * `image_width` - 相机输出的图像宽度，单位为像素。
    /// * `samples_per_pixel` - 每个像素采样的次数，用于抗锯齿。
    /// * `max_depth` - 递归深度，用于控制反射次数。
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        // 计算画布高度
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        // 计算视窗宽高
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::zero();

        // 计算视窗边缘的向量
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // 计算每个像素的 Delta 向量
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // 计算图像左上角的坐标
        let viewport_up_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_up_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_scale: 1.0 / (samples_per_pixel as f64),
        }
    }

    /// 计算一条射线的颜色。
    ///
    /// # 参数
    ///
    /// * `r` - 要计算颜色的射线。
    /// * `world` - 场景中的可命中对象。
    ///
    /// # 返回值
    ///
    /// 射线的颜色。
    fn ray_color(&self, r: Ray, world: &dyn Hittable, depth: i32) -> Color {
        // 如果递归深度为 0，那么返回黑色
        if depth <= 0 {
            return Color::zero();
        }

        // 如果命中了物体，那么计算物体颜色
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            return if let Some((albedo, scattered)) = rec.mat.scatter(r, &rec) {
                albedo * self.ray_color(scattered, world, depth - 1)
            } else {
                Color::zero()
            };
        }

        // 这里实现一个从蓝色到白色的线性差值
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
    }

    /// 生成一条射线。
    ///
    /// # 参数
    ///
    /// * `x` - 像素的 x 坐标。
    /// * `y` - 像素的 y 坐标。
    ///
    /// # 返回值
    ///
    /// 从相机中心到像素采样点的射线。
    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel_00_loc
            + (x as f64 + offset.x) * self.pixel_delta_u
            + (y as f64 + offset.y) * self.pixel_delta_v;
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }

    /// 生成一个随机偏移量，用于抗锯齿。
    ///
    /// # 返回值
    ///
    /// 一个随机向量，用于偏移像素采样点。
    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            random_double_range_inclusive(-0.5, 0.5),
            random_double_range_inclusive(-0.5, 0.5),
            0.0,
        )
    }

    /// 渲染场景并输出图像。
    ///
    /// # 参数
    ///
    /// * `world` - 场景中的可命中对象。
    pub fn render(&self, world: &dyn Hittable) {
        // 开始渲染
        println!(
            "{}\n{} {}\n{}",
            Self::P3_MAGIC_NUMBER,
            self.image_width,
            self.image_height,
            Self::MAX_COLOR_VALUE
        );

        let mut stdout = std::io::stdout();

        for y in 0..self.image_height {
            eprint!("\rScan lines remaining: {:>3}", self.image_height - y);
            for x in 0..self.image_width {
                // 生成多条射线，对得到的颜色取平均值
                let mut color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(ray, world, self.max_depth);
                }
                color *= self.samples_per_scale;

                write_color(&mut stdout, color).expect("Failed to write color to stdout");
            }
        }
        eprintln!("\nDone.");
    }
}

impl Default for Camera {
    /// 创建一个默认的相机实例。
    ///
    /// 默认的相机宽高比为 16:9，图像宽度为 100 像素，每个像素采样 100 次。
    fn default() -> Self {
        Self::new(16.0 / 9.0, 100, 100, 10)
    }
}
