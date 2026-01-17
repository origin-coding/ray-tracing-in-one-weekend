use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{
    BvhNode, ConstantMedium, HittableList, Quadrilateral, RotateY, Sphere, Translate,
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use crate::texture::{Image, Noise};
use crate::utils::random_double_range;
use std::sync::Arc;

/// 《Ray Tracing: The Next Week》最终场景
/// 包含：地面盒子矩阵、体积雾、光照、运动模糊、纹理映射、柏林噪声、玻璃/金属材质等。
pub struct CornellFinalScene;

impl Scene for CornellFinalScene {
    fn generate(&self, config: &SceneConfig) -> SceneContext {
        let mut world = HittableList::new();

        // --- 1. 地面盒子矩阵 (Ground Boxes) ---
        let ground = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

        // 创建一个临时的列表来存放地面上的盒子
        let mut boxes1 = HittableList::new();
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_double_range(1.0, 101.0); // 随机高度
                let z1 = z0 + w;

                let p0 = Point3::new(x0, y0, z0);
                let p1 = Point3::new(x1, y1, z1);

                // Quadrilateral::create_box 返回的是一个包含6个面的 HittableList
                boxes1.add(Box::new(Quadrilateral::create_box(
                    &p0,
                    &p1,
                    ground.clone(),
                )));
            }
        }

        // 将整个地面盒子组构建为一个 BVH 节点，以此来加速渲染
        world.add(BvhNode::from_hittable_list(boxes1));

        // --- 2. 顶部光源 (Top Light) ---
        let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
        world.add(Box::new(Quadrilateral::new(
            Point3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            light,
        )));

        // --- 3. 运动的球体 (Moving Sphere) ---
        let center1 = Point3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0); // 向右移动 30 单位
        let sphere_material = Arc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
        world.add(Box::new(Sphere::moving(
            center1,
            center2,
            50.0,
            sphere_material,
        )));

        // --- 4. 玻璃球 (Glass Sphere) ---
        world.add(Box::new(Sphere::stationary(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectric::new(1.5)),
        )));

        // --- 5. 金属球 (Metal Sphere) ---
        world.add(Box::new(Sphere::stationary(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)), // 大模糊度的金属
        )));

        // --- 6. 蓝色玻璃球与体积雾 (Blue Subsurface Scattering) ---
        let boundary = Arc::new(Sphere::stationary(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        // 先添加玻璃外壳
        world.add(Box::new(boundary.clone())); // 这里复制一份
        // 再添加内部的恒定介质（体积雾）
        world.add(Box::new(ConstantMedium::new_with_color(
            Arc::new(boundary),
            0.2,                       // 密度
            Color::new(0.2, 0.4, 0.9), // 蓝色
        )));

        // --- 7. 全局白雾 (Global Fog) ---
        // 创建一个巨大的球体包围整个场景，作为雾的边界
        let boundary_fog = Arc::new(Sphere::stationary(
            Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        world.add(Box::new(ConstantMedium::new_with_color(
            boundary_fog,
            0.0001,                    // 非常稀薄的密度
            Color::new(1.0, 1.0, 1.0), // 白色
        )));

        // --- 8. 地球纹理球 (Earth Sphere) ---
        // 尝试从配置中读取纹理路径，如果未提供则 panic 提醒用户
        let path = config
            .image_texture_path
            .as_ref()
            .expect("Final scene requires --image-texture-path to render the Earth sphere");

        let earth_mat = Arc::new(Lambertian::new(Arc::new(Image::new(path))));
        world.add(Box::new(Sphere::stationary(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            earth_mat,
        )));

        // --- 9. 柏林噪声球 (Perlin Noise Sphere) ---
        let per_text = Arc::new(Noise::new(0.2)); // 缩放因子 0.2
        world.add(Box::new(Sphere::stationary(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::new(per_text)),
        )));

        // --- 10. 密集的白色小球群 (Cluster of Small Spheres) ---
        let mut boxes2 = HittableList::new();
        let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
        let ns = 1000;
        for _ in 0..ns {
            boxes2.add(Box::new(Sphere::stationary(
                Point3::random_range(0.0, 165.0),
                10.0,
                white.clone(),
            )));
        }

        // 对这组小球进行变换：
        // 1. 构建 BVH (优化碰撞检测)
        let node = BvhNode::from_hittable_list(boxes2);
        // 2. 绕 Y 轴旋转 15 度
        let rotate = RotateY::new(Arc::from(node), 15.0); // 转移所有权到 RotateY
        // 3. 平移到指定位置
        let translate = Translate::new(Arc::new(rotate), Vec3::new(-100.0, 270.0, 395.0));

        world.add(Box::new(translate));

        // --- 相机配置 ---
        // 注意：C++ 原教程中这里使用的是 800x800, 10000 spp。
        // Rust 代码中你可以通过命令行参数覆盖这些值，这里提供的是默认视角配置。
        let camera_config = CameraConfig {
            aspect_ratio: Some(1.0), // 强制 1:1 正方形比例
            vfov: 40.0,
            look_from: Point3::new(478.0, 278.0, -600.0),
            look_at: Point3::new(278.0, 278.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            background_color: Color::zero(), // 黑色背景 (因为场景是封闭的或者是黑暗的)
            defocus_angle: 0.0,              // 无景深模糊
            ..Default::default()
        };

        SceneContext {
            world,
            camera_config,
        }
    }
}
