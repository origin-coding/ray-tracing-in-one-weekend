use crate::math::{Color, Point3, Vec3};
use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

/// 全局配置项，包含渲染配置和场景配置。
#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[command(flatten)]
    pub render_config: RenderConfig,
    #[command(flatten)]
    pub scene_config: SceneConfig,
}

/// 渲染配置项，包含图像尺寸、采样数、最大递归深度等。
#[derive(Args, Debug, Clone)]
pub struct RenderConfig {
    #[arg(
        short = 'w',
        long = "image-width",
        help = "Width of the image",
        default_value_t = 1200
    )]
    pub image_width: i32,

    #[arg(short = 'a', long = "aspect-ratio", help = "Aspect ratio of the image", default_value_t = 16.0 / 9.0)]
    pub aspect_ratio: f32,

    #[arg(
        short = 's',
        long = "samples-per-pixel",
        help = "Number of samples per pixel",
        default_value_t = 500
    )]
    pub samples_per_pixel: i32,

    #[arg(
        short = 'd',
        long = "max-depth",
        help = "Maximum depth of recursion",
        default_value_t = 50
    )]
    pub max_depth: i32,

    #[arg(
        short = 'o',
        long = "output-path",
        help = "Path to the output image file",
        default_value = "image.ppm"
    )]
    pub output_path: PathBuf,

    #[arg(
        short = 'i',
        long = "ascii",
        help = "Output in PPM P3 (ASCII) format instead of P6 (Binary)",
        default_value_t = false
    )]
    pub use_ascii: bool,
}

/// 渲染场景选项，包含跳跃小球，点格球和一个地球。
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum SceneType {
    /// 随机生成的弹跳小球场景 (对应 Chapter 2)
    Bouncing,
    /// 棋盘格纹理场景 (对应 Chapter 3)
    Checkered,
    /// 地球纹理场景 (对应 Chapter 4)
    Earth,
    /// 柏林噪声纹理场景 (对应 Chapter 5)
    Perlin,
    /// 四边形场景 (对应 Chapter 6)
    Quad,
    /// 光照场景 (对应 Chapter 7)
    Light,
    /// Cornell Box 场景 (对应 Chapter 7、8)
    CornellBox,
    /// Cornell Box 烟雾场景 (对应 Chapter 9)
    CornellSmoke,
    /// Cornell Box 最终场景 (对应 Chapter 10)
    CornellFinal,
}

/// 场景配置，包含一个可选的图像纹理路径。
#[derive(Args, Debug, Clone)]
pub struct SceneConfig {
    #[arg(
        short = 't',
        long = "image-texture-path",
        help = "Path to the image texture file"
    )]
    pub image_texture_path: Option<PathBuf>,

    #[arg(
        value_enum,
        long = "scene",
        help = "Type of scene to render",
        default_value_t = SceneType::Bouncing
    )]
    pub scene_type: SceneType,
}

impl Config {
    /// 从命令行参数加载配置。
    ///
    /// # Returns
    ///
    /// 解析后的配置项。
    pub fn load() -> Self {
        Self::parse()
    }
}

/// 相机配置项，包含相机参数。
pub struct CameraConfig {
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    pub background_color: Color,
    pub aspect_ratio: Option<f32>,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            vfov: 20.0,
            up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background_color: Color::ZERO,
            aspect_ratio: None,
        }
    }
}
