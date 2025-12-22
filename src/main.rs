mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::ray::Point3;
use crate::sphere::Sphere;

fn main() {
    // 创建世界，添加两个球体
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // 创建相机，渲染场景
    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
