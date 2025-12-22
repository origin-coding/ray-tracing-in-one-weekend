use ray_tracing_in_one_weekend::Color;
use ray_tracing_in_one_weekend::camera::Camera;
use ray_tracing_in_one_weekend::hittable_list::HittableList;
use ray_tracing_in_one_weekend::material::{Lambertian, Metal};
use ray_tracing_in_one_weekend::ray::Point3;
use ray_tracing_in_one_weekend::sphere::Sphere;
use std::rc::Rc;

fn main() {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // 创建世界，添加两个球体
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // 创建相机，渲染场景
    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
