use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Quadrilateral, Sphere};
use crate::material::{DiffuseLight, Lambertian};
use crate::math::{Color, Point3, Vec3};
use crate::scene::{Scene, SceneContext};
use crate::texture::Noise;
use std::sync::Arc;

/// 简单光照场景
pub struct LightScene;

impl Scene for LightScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
        let mut world = HittableList::new();

        let per_text = Arc::new(Noise::new(4.0));
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

        SceneContext {
            world,
            camera_config,
        }
    }
}
