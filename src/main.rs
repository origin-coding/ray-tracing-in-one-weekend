use ray_tracing_in_one_weekend::camera::CameraBuilder;
use ray_tracing_in_one_weekend::config::Config;
use ray_tracing_in_one_weekend::geometry::BvhNode;
use ray_tracing_in_one_weekend::scene::{SceneContext, get_scene};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // 加载配置
    let config = Config::load();
    let render_config = config.render_config;
    let scene_config = config.scene_config;

    // 创建世界，生成场景，转换为 BVH 树
    let scene_generator = get_scene(scene_config.scene_type);
    let SceneContext {
        world,
        camera_config,
    } = scene_generator.generate(&scene_config);

    let world = BvhNode::from_hittable_list(world);

    // 创建/打开输出文件
    let file = File::create(&render_config.output_path).expect("Failed to create output file");
    let mut buffer = BufWriter::new(file);

    // 创建相机，渲染场景
    let camera = CameraBuilder::default()
        .aspect_ratio(render_config.aspect_ratio)
        .image_width(render_config.image_width)
        .samples_per_pixel(render_config.samples_per_pixel)
        .max_depth(render_config.max_depth)
        .vfov(camera_config.vfov)
        .look_from(camera_config.look_from)
        .look_at(camera_config.look_at)
        .up(camera_config.up)
        .defocus_angle(camera_config.defocus_angle)
        .focus_dist(camera_config.focus_dist)
        .background_color(camera_config.background_color)
        .build();
    camera.render(world.as_ref(), &mut buffer);
}
