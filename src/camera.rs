use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

#[allow(dead_code)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    // 私有字段
    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    const P3_MAGIC_NUMBER: &str = "P3";
    const MAX_COLOR_VALUE: i32 = 255;

    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
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
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(&self, r: Ray, world: &dyn Hittable) -> Color {
        // 如果命中了物体，那么计算物体颜色
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            return 0.5 * (rec.normal + Color::one());
        }

        // 这里实现一个从蓝色到白色的线性差值
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
    }

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
            eprint!("\rScan lines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let pixel_center = self.pixel_00_loc
                    + x as f64 * self.pixel_delta_u
                    + y as f64 * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let color = self.ray_color(ray, world);
                write_color(&mut stdout, color).expect("Failed to write color to stdout");
            }
        }
        eprintln!("\nDone.");
    }
}
