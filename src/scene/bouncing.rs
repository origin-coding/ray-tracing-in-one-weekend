use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use crate::texture::Checker;
use crate::utils::{random_double, random_double_range};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;
use std::sync::Arc;

/// 生成随机弹跳小球场景。
pub struct BouncingScene;

impl Scene for BouncingScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
        let mut world = HittableList::new();
        generate_fixed_balls(&mut world);
        generate_random_balls(&mut world);

        let camera_config = CameraConfig {
            look_from: Point3::new(13.0, 2.0, 3.0),
            defocus_angle: 0.06,
            background_color: Color::new(0.7, 0.8, 1.0),
            ..Default::default()
        };

        SceneContext {
            world,
            lights: HittableList::new(),
            camera_config,
        }
    }
}

/// 生成固定的球体。
fn generate_fixed_balls(world: &mut HittableList) {
    let texture_ground =
        Checker::from_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 0.32);
    let material_ground = Arc::new(Lambertian::new(Arc::new(texture_ground)));
    let material_first = Arc::new(Dielectric::new(1.5));
    let material_second = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_third = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_first.clone(),
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_second.clone(),
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_third.clone(),
    )));
}

/// 生成一些随机的球体。
fn generate_random_balls(world: &mut HittableList) {
    // 每个闭包负责生成一种特定类型的随机材质
    let material_generators: Vec<Box<dyn Fn() -> Arc<dyn Material + Send + Sync>>> = vec![
        // 漫反射生成器
        Box::new(|| {
            let albedo = Color::random() * Color::random();
            Arc::new(Lambertian::from_color(albedo))
        }),
        // 金属生成器
        Box::new(|| {
            let albedo = Color::random_range(0.5, 1.0);
            let fuzz = random_double_range(0.0, 0.5);
            Arc::new(Metal::new(albedo, fuzz))
        }),
        // 玻璃生成器
        Box::new(|| Arc::new(Dielectric::new(1.5))),
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
                if generator_index == 0 {
                    let center_end = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::moving(center, center_end, 0.2, material)));
                } else {
                    world.add(Box::new(Sphere::stationary(center, 0.2, material)));
                }
            }
        }
    }
}
