//! 相机和Builder的定义以及相关工具方法。

use crate::geometry::Hittable;
use crate::material::ScatterRecord;
use crate::math::{
    color::{write_color_p3, write_color_p6}, Color, Interval, Point3, Ray,
    Vec3,
};
use crate::sampling::{HittablePdf, MixturePdf, Pdf};
use crate::utils::random_double;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 相机构建参数
///
/// 使用方法：
/// ```
/// use ray_tracing_in_one_weekend::camera::CameraBuilder;
/// let camera = CameraBuilder::default().build();
/// /// camera.xxx()
/// ```
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    vfov: f64,
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    background_color: Color,
    use_ascii: bool,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::zero(),
            look_at: -Point3::unit_z(),
            up: Vec3::unit_y(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::zero(),
            use_ascii: false,
        }
    }
}

impl CameraBuilder {
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_width(mut self, image_width: i32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn use_ascii(mut self, use_ascii: bool) -> Self {
        self.use_ascii = use_ascii;
        self
    }

    pub fn build(self) -> Camera {
        // 计算画布高度
        let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        // 计算视窗宽高
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);
        let center = self.look_from;

        // 计算相机的 w u v 坐标单位向量
        let w = (self.look_from - self.look_at).unit_vector();
        let u = Vec3::cross(self.up, w).unit_vector();
        let v = Vec3::cross(w, u);

        // 计算视窗边缘的向量
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // 计算每个像素的 Delta 向量
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // 计算图像左上角的坐标
        let viewport_up_left = center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_up_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // 计算散焦基础向量
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        // 计算 sqrt_spp 和 recip_sqrt_spp
        let sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        let recip_sqrt_spp = 1.0 / (sqrt_spp as f64);

        Camera {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            samples_per_pixel: self.samples_per_pixel,
            sqrt_spp,
            recip_sqrt_spp,
            max_depth: self.max_depth,
            vfov: self.vfov,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_scale: 1.0 / ((sqrt_spp * sqrt_spp) as f64), // 每个像素的样本数缩放比例，按照实际样本数的平方根来缩放
            defocus_angle: self.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background_color: self.background_color,
            use_ascii: self.use_ascii,
        }
    }
}

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
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
    max_depth: i32,
    vfov: f64,
    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_scale: f64,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background_color: Color,
    use_ascii: bool,
}

impl Camera {
    const P3_MAGIC_NUMBER: &str = "P3";
    const P6_MAGIC_NUMBER: &str = "P6";
    const MAX_COLOR_VALUE: i32 = 255;

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
    fn ray_color(&self, r: Ray, world: &dyn Hittable, lights: &dyn Hittable, depth: i32) -> Color {
        // 如果递归深度为 0，那么返回黑色
        if depth <= 0 {
            return Color::zero();
        }

        // 如果没有命中任何物体，返回背景颜色
        let Some(rec) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) else {
            return self.background_color;
        };

        // 计算自发光颜色
        let emitted = rec.mat.emitted(rec.uv, &rec.p, &r, &rec);

        // 如果材质没有散射，那么直接返回自发光颜色
        let Some(scatter_record) = rec.mat.scatter(&r, &rec) else {
            return emitted;
        };

        // 计算散射后的光线颜色 (递归)
        let color_from_scatter = match scatter_record {
            ScatterRecord::Specular { attenuation, ray } => {
                attenuation * self.ray_color(ray, world, lights, depth - 1)
            }

            ScatterRecord::Diffuse { attenuation, pdf } => {
                let light_pdf = HittablePdf::new(lights, rec.p);
                let mixture_pdf = MixturePdf::new(&light_pdf, pdf.as_ref());

                let scattered = Ray::new_with_time(rec.p, mixture_pdf.generate(), r.time);
                let pdf_value = mixture_pdf.value(scattered.direction);

                let scattering_pdf = pdf.value(scattered.direction);

                let li = self.ray_color(scattered, world, lights, depth - 1);

                if pdf_value > 0.0 {
                    attenuation * li * scattering_pdf / pdf_value
                } else {
                    Color::zero()
                }
            }
        };

        // 返回最终得到的颜色
        emitted + color_from_scatter
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
    fn get_ray(&self, x: i32, y: i32, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_center = self.pixel_00_loc
            + (x as f64 + offset.x) * self.pixel_delta_u
            + (y as f64 + offset.y) * self.pixel_delta_v;

        let ray_direction = pixel_center - self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_time = random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        // 计算每个样本的偏移量，范围 [-0.5, 0.5]
        let x = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let y = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        Vec3::new(x, y, 0.0)
    }

    /// 生成镜头上的随机采样点
    fn defocus_disk_sample(&self) -> Vec3 {
        let vec = Vec3::random_in_unit_disk();
        self.center + vec.x * self.defocus_disk_u + vec.y * self.defocus_disk_v
    }

    /// 渲染场景并输出图像。
    ///
    /// # 参数
    ///
    /// * `world` - 场景中的可命中对象。
    pub fn render(
        &self,
        world: &(dyn Hittable + Send + Sync),
        lights: &(dyn Hittable + Send + Sync),
        out: &mut impl Write,
    ) {
        let magic_number = if self.use_ascii {
            Self::P3_MAGIC_NUMBER
        } else {
            Self::P6_MAGIC_NUMBER
        };

        // 开始渲染
        writeln!(
            out,
            "{}\n{} {}\n{}",
            magic_number,
            self.image_width,
            self.image_height,
            Self::MAX_COLOR_VALUE
        )
        .expect("Failed to write PPM header");

        // 使用 AtomicUsize 跨线程安全地跟踪进度
        let rows_remaining = AtomicUsize::new(self.image_height as usize);

        // 1. 并行计算阶段
        // 使用 map 而不是 for_each，这样我们会得到一个包含所有像素数据的 Vec<Vec<Color>>
        // Rayon 的 collect 会自动保证结果的顺序与输入范围 (0..height) 的顺序一致
        let scan_lines: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                // 进度条逻辑
                let remaining = rows_remaining.fetch_sub(1, Ordering::Relaxed);
                if remaining % 10 == 0 {
                    eprint!("\rScan lines remaining: {:>4}", remaining);
                }

                let mut row = Vec::with_capacity(self.image_width as usize);
                for x in 0..self.image_width {
                    // 生成多条射线，对得到的颜色取平均值
                    let mut color = Color::zero();
                    for s_j in 0..self.sqrt_spp {
                        for s_i in 0..self.sqrt_spp {
                            let ray = self.get_ray(x, y, s_i, s_j);
                            color += self.ray_color(ray, world, lights, self.max_depth);
                        }
                    }
                    color *= self.samples_per_scale;
                    row.push(color);
                }
                row // 返回这一行的数据
            })
            .collect(); // 收集所有行

        // 2. 串行输出阶段
        for row in scan_lines {
            for color in row {
                if self.use_ascii {
                    write_color_p3(out, color).expect("Failed to write color to writer");
                } else {
                    write_color_p6(out, color).expect("Failed to write color to writer");
                }
            }
        }

        eprintln!("\nDone.");
    }
}
