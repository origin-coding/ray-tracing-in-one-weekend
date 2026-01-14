use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Quadrilateral};
use crate::material::Lambertian;
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use std::sync::Arc;

/// 四边形场景
pub struct QuadScene;

impl Scene for QuadScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
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

        SceneContext {
            world,
            camera_config,
        }
    }
}
