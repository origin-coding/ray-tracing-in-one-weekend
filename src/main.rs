use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use ray_tracing_in_one_weekend::camera::CameraBuilder;
use ray_tracing_in_one_weekend::config::{CameraConfig, Config, SceneType};
use ray_tracing_in_one_weekend::geometry::{BvhNode, HittableList, Quadrilateral, Sphere};
use ray_tracing_in_one_weekend::material::material::DiffuseLight;
use ray_tracing_in_one_weekend::material::texture::NoiseTexture;
use ray_tracing_in_one_weekend::material::{Checker, ImageTexture};
use ray_tracing_in_one_weekend::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing_in_one_weekend::math::{Color, Point3, Vec3};
use ray_tracing_in_one_weekend::utils::{random_double, random_double_range};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

fn main() {
    // 加载配置
    let config = Config::load();
    let render_config = config.render_config;
    let scene_config = config.scene_config;

    // 创建世界，生成场景，转换为 BVH 树
    let (world, camera_config) = match scene_config.scene_type {
        SceneType::Bouncing => scene_bouncing(),
        SceneType::Checkered => scene_checkered(),
        SceneType::Earth => scene_earth(scene_config.image_texture_path.as_deref()),
        SceneType::Perlin => scene_perlin(),
        SceneType::Quad => scene_quadrilateral(),
        SceneType::Light => scene_light(),
        SceneType::CornellBox => scene_cornell_box(),
    };
    let world = BvhNode::from_hittable_list(world);

    // 创建/打开输出文件
    let file = File::create(&render_config.output_path).expect("Failed to create output file");
    let mut buffer = BufWriter::new(file);

    // 创建相机，渲染场景
    let camera = CameraBuilder::default()
        .aspect_ratio(render_config.aspect_ratio)
        .image_width(render_config.image_width)
        .samples_per_pixel(render_config.samples_per_pixel)
        .max_depth(render_config.max_depth)
        .vfov(camera_config.vfov)
        .look_from(camera_config.look_from)
        .look_at(camera_config.look_at)
        .up(camera_config.up)
        .defocus_angle(camera_config.defocus_angle)
        .focus_dist(camera_config.focus_dist)
        .background_color(camera_config.background_color)
        .build();
    camera.render(world.as_ref(), &mut buffer);
}

// 不同场景的生成函数

//noinspection DuplicatedCode
/// 生成随机弹跳小球场景。

fn scene_bouncing() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();
    generate_fixed_balls(&mut world);
    generate_random_balls(&mut world);

    let config = CameraConfig {
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        defocus_angle: 0.06,
        background_color: Color::new(0.7, 0.8, 1.0),
        ..Default::default()
    };

    (world, config)
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

/// 生成棋盘格纹理场景。
fn scene_checkered() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let checker = Arc::new(Checker::from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
        0.32,
    ));

    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    let config = CameraConfig {
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vfov: 20.0,
        defocus_angle: 0.06,
        background_color: Color::new(0.7, 0.8, 1.0),
        ..Default::default()
    };

    (world, config)
}

/// 生成地球纹理场景。
fn scene_earth(path: Option<&Path>) -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let path = path.expect("Earth scene requires --image-texture-path");
    eprintln!("Loading image texture from: {:?}", path);

    let earth_texture = Arc::new(ImageTexture::new(path));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));

    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    // 地球需要离得远一点，且通常不需要景深模糊
    let config = CameraConfig {
        look_from: Point3::new(0.0, 0.0, 12.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        defocus_angle: 0.0, // 清晰的地球
        background_color: Color::new(0.7, 0.8, 1.0),
        ..Default::default()
    };

    (world, config)
}

//noinspection DuplicatedCode
/// 生成柏林噪声纹理场景。
fn scene_perlin() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let ground_texture = Arc::new(NoiseTexture::new(4.0));
    let ground_material = Arc::new(Lambertian::new(ground_texture));
    let small_sphere_texture = Arc::new(NoiseTexture::new(2.0));
    let small_sphere_material = Arc::new(Lambertian::new(small_sphere_texture));

    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        small_sphere_material,
    )));

    let config = CameraConfig {
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        defocus_angle: 0.06,
        background_color: Color::new(0.7, 0.8, 1.0),
        ..Default::default()
    };

    (world, config)
}

/// 四边形场景
fn scene_quadrilateral() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let material_left_red = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let material_back_green = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let material_right_blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let material_upper_orange = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let material_lower_teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Box::new(Quadrilateral::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        material_left_red,
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        material_back_green,
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        material_right_blue,
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        material_upper_orange,
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        material_lower_teal,
    )));

    let camera_config = CameraConfig {
        look_from: Point3::new(0.0, 0.0, 9.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vfov: 80.0,
        background_color: Color::new(0.7, 0.8, 1.0),
        ..Default::default()
    };

    (world, camera_config)
}

/// 简单光照场景
fn scene_light() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let per_text = Arc::new(NoiseTexture::new(4.0));
    let per_material = Arc::new(Lambertian::new(per_text));

    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        per_material.clone(),
    )));
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        per_material.clone(),
    )));

    let light_material = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        light_material.clone(),
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        light_material.clone(),
    )));

    let camera_config = CameraConfig {
        vfov: 20.0,
        look_from: Point3::new(26.0, 3.0, 6.0),
        look_at: Point3::new(0.0, 2.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        background_color: Color::zero(),
        ..Default::default()
    };

    (world, camera_config)
}

///  Cornell Box 场景
fn scene_cornell_box() -> (HittableList, CameraConfig) {
    let mut world = HittableList::new();

    let material_red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let material_white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let material_green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let material_light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Box::new(Quadrilateral::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_red,
    )));

    world.add(Box::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_green,
    )));

    world.add(Box::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_white.clone(),
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        material_white.clone(),
    )));
    world.add(Box::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        material_white.clone(),
    )));

    world.add(Box::new(Quadrilateral::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        material_light,
    )));

    // （在运行时记得设置 --aspect-ratio 1，将图片渲染为正方形）
    let camera_config = CameraConfig {
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        vfov: 40.0,
        background_color: Color::zero(),
        ..Default::default()
    };

    (world, camera_config)
}
