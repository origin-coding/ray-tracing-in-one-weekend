use ray_tracing_in_one_weekend::camera::CameraBuilder;
use ray_tracing_in_one_weekend::hittable_list::HittableList;
use ray_tracing_in_one_weekend::material::{Dielectric, Lambertian, Metal};
use ray_tracing_in_one_weekend::ray::Point3;
use ray_tracing_in_one_weekend::sphere::Sphere;
use ray_tracing_in_one_weekend::{Color, Vec3};
use std::rc::Rc;

fn main() {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // 创建相机，渲染场景
    let camera = CameraBuilder::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .max_depth(100)
        .samples_per_pixel(50)
        .vfov(20.0)
        .look_from(Point3::new(-2.0, 2.0, 1.0))
        .look_at(Point3::new(0.0, 0.0, -1.0))
        .up(Vec3::new(0.0, 1.0, 0.0))
        .build();
    camera.render(&world);
}
