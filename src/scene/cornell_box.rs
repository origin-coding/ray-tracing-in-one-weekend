use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Quadrilateral};
use crate::material::material::DiffuseLight;
use crate::material::Lambertian;
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use std::sync::Arc;

///  Cornell Box 场景
pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
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

        SceneContext {
            world,
            camera_config,
        }
    }
}
