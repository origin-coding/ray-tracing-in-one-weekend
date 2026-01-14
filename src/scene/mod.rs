//! 场景模块，包含场景相关的类型和函数。

mod bouncing;
mod checkered;
mod cornell_box;
mod earth;
mod light;
mod perlin;
mod quad;

use crate::config::{CameraConfig, SceneConfig, SceneType};
use crate::geometry::HittableList;

/// 场景上下文，包含场景中的物体和相机配置。
pub struct SceneContext {
    pub world: HittableList,
    pub camera_config: CameraConfig,
}

/// 场景生成 Trait
/// 任何一个具体的场景，比如 Cornell Box 场景，都需要实现这个 Trait。
pub trait Scene: Send + Sync {
    /// 生成场景上下文。
    ///
    /// # Arguments
    ///
    /// * `config` - 场景配置。
    ///
    /// # Returns
    ///
    /// 场景上下文，包含场景中的物体和相机配置。
    fn generate(&self, _config: &SceneConfig) -> SceneContext;
}

/// 获取场景实例。
///
/// # Arguments
///
/// * `scene_type` - 场景类型。
///
/// # Returns
///
/// 场景实例，实现了 Scene Trait。
pub fn get_scene(scene_type: SceneType) -> Box<dyn Scene> {
    match scene_type {
        SceneType::Bouncing => Box::new(bouncing::BouncingScene),
        SceneType::Checkered => Box::new(checkered::CheckeredScene),
        SceneType::Earth => Box::new(earth::EarthScene),
        SceneType::Perlin => Box::new(perlin::PerlinScene),
        SceneType::Quad => Box::new(quad::QuadScene),
        SceneType::Light => Box::new(light::LightScene),
        SceneType::CornellBox => Box::new(cornell_box::CornellBoxScene),
    }
}
