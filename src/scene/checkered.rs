use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Sphere};
use crate::material::Lambertian;
use crate::math::{Color, Point3};
use crate::scene::{Scene, SceneContext};
use crate::texture::Checker;
use std::sync::Arc;

/// 生成棋盘格纹理场景。
pub struct CheckeredScene;

impl Scene for CheckeredScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
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

        let camera_config = CameraConfig {
            look_from: Point3::new(13.0, 2.0, 3.0),
            defocus_angle: 0.06,
            background_color: Color::new(0.7, 0.8, 1.0),
            ..Default::default()
        };

        SceneContext {
            world,
            camera_config,
        }
    }
}
