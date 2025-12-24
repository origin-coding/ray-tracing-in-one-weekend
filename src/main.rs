use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use ray_tracing_in_one_weekend::camera::CameraBuilder;
use ray_tracing_in_one_weekend::hittable_list::HittableList;
use ray_tracing_in_one_weekend::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing_in_one_weekend::ray::Point3;
use ray_tracing_in_one_weekend::sphere::Sphere;
use ray_tracing_in_one_weekend::utils::{random_double, random_double_range};
use ray_tracing_in_one_weekend::{Color, Vec3};
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    // 生成随机的球体
    generate_random_balls(&mut world);

    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_first = Rc::new(Dielectric::new(1.5));
    let material_second = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_third = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_first.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_second.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_third.clone(),
    )));

    // 创建相机，渲染场景
    let camera = CameraBuilder::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.06)
        .focus_dist(10.0)
        .build();
    camera.render(&world);
}

/// 生成一些随机的球体
fn generate_random_balls(world: &mut HittableList) {
    // 每个闭包负责生成一种特定类型的随机材质
    let material_generators: Vec<Box<dyn Fn() -> Rc<dyn Material>>> = vec![
        // 漫反射生成器
        Box::new(|| {
            let albedo = Color::random() * Color::random();
            Rc::new(Lambertian::new(albedo))
        }),
        // 金属生成器
        Box::new(|| {
            let albedo = Color::random_range(0.5, 1.0);
            let fuzz = random_double_range(0.0, 0.5);
            Rc::new(Metal::new(albedo, fuzz))
        }),
        // 玻璃生成器
        Box::new(|| Rc::new(Dielectric::new(1.5))),
    ];

    let weights = [85, 15, 5];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let generator_index = dist.sample(&mut rng);
                let material = material_generators[generator_index]();
                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }
}
