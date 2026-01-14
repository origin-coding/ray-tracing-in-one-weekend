use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Sphere};
use crate::material::Lambertian;
use crate::material::texture::NoiseTexture;
use crate::math::{Color, Point3};
use crate::scene::{Scene, SceneContext};
use std::sync::Arc;

/// 生成柏林噪声纹理场景。
pub struct PerlinScene;

impl Scene for PerlinScene {
    fn generate(&self, _config: &SceneConfig) -> SceneContext {
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

        let camera_config = CameraConfig {
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
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
