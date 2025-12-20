#![allow(dead_code)]

mod color;
mod ray;
mod vec3;

use crate::color::{Color, write_color};
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

const P3_MAGIC_NUMBER: &str = "P3";
const MAX_COLOR_VALUE: i32 = 255;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

/// 判断光线是否与球体发生碰撞
fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(r.direction, r.direction);
    let b = -2.0 * Vec3::dot(r.direction, oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

/// 计算射线颜色
fn ray_color(r: Ray) -> Color {
    // 如果命中了球体，那么返回红色
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    // 这里实现一个从蓝色到白色的线性差值
    let unit_direction = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // 设置画布比例
    let aspect_ration = 16.0 / 9.0;

    // 计算画布宽高
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ration) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // 计算视窗宽高
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // 计算视窗边缘的向量
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // 计算每个像素的 Delta 向量
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // 计算图像左上角的坐标
    let viewport_up_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_up_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // 开始渲染
    println!(
        "{}\n{} {}\n{}",
        P3_MAGIC_NUMBER, image_width, image_height, MAX_COLOR_VALUE
    );

    let mut stdout = std::io::stdout();

    for y in 0..image_height {
        eprint!("\rScan lines remaining: {}", image_height - y);
        for x in 0..image_width {
            let pixel_center = pixel_00_loc + x as f64 * pixel_delta_u + y as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(ray);
            write_color(&mut stdout, color).expect("Failed to write color to stdout");
        }
    }
    eprintln!("\nDone.");
}
