use crate::config::{CameraConfig, SceneConfig};
use crate::geometry::{HittableList, Sphere};
use crate::material::Lambertian;
use crate::math::{Color, Point3};
use crate::scene::{Scene, SceneContext};
use crate::texture::Image;
use std::sync::Arc;

/// 生成地球纹理场景。
pub struct EarthScene;

impl Scene for EarthScene {
    fn generate(&self, config: &SceneConfig) -> SceneContext {
        let mut world = HittableList::new();

        let path = config
            .image_texture_path
            .as_ref()
            .expect("Earth scene requires --image-texture-path");
        eprintln!("Loading image texture from: {:?}", path);

        let earth_texture = Arc::new(Image::new(path));
        let earth_surface = Arc::new(Lambertian::new(earth_texture));

        world.add(Box::new(Sphere::stationary(
            Point3::new(0.0, 0.0, 0.0),
            2.0,
            earth_surface,
        )));

        // 地球需要离得远一点，且通常不需要景深模糊
        let camera_config = CameraConfig {
            look_from: Point3::new(0.0, 0.0, 12.0),
            background_color: Color::new(0.7, 0.8, 1.0),
            ..Default::default()
        };

        SceneContext {
            world,
            camera_config,
        }
    }
}
