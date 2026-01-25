use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Quadrilateral, RotateY, Translate};
use crate::material::{DiffuseLight, Lambertian};
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use std::sync::Arc;

///  Cornell Box 场景
pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    //noinspection DuplicatedCode
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
        let mut world = HittableList::new();
        let mut lights = HittableList::new();

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

        let light = Arc::new(Quadrilateral::new(
            Point3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            material_light,
        ));

        world.add(Box::new(light.clone()));
        lights.add(Box::new(light));

        // 添加两个平行四面体，并将其平移 + 旋转
        let box1 = Quadrilateral::create_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 330.0, 165.0),
            material_white.clone(),
        );
        let box1 = RotateY::new(Arc::new(box1), 15.0);
        let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
        world.add(Box::new(box1));

        let box2 = Quadrilateral::create_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 165.0, 165.0),
            material_white.clone(),
        );
        let box2 = RotateY::new(Arc::new(box2), -18.0);
        let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
        world.add(Box::new(box2));

        // （在运行时记得设置 --aspect-ratio 1，将图片渲染为正方形）
        let camera_config = CameraConfig {
            look_from: Point3::new(278.0, 278.0, -800.0),
            look_at: Point3::new(278.0, 278.0, 0.0),
            vfov: 40.0,
            background_color: Color::ZERO,
            aspect_ratio: Some(1.0),
            ..Default::default()
        };

        SceneContext {
            world,
            lights,
            camera_config,
        }
    }
}
