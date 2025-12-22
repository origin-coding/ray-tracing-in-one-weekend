use ray_tracing_in_one_weekend::camera::Camera;
use ray_tracing_in_one_weekend::hittable_list::HittableList;
use ray_tracing_in_one_weekend::ray::Point3;
use ray_tracing_in_one_weekend::sphere::Sphere;

fn main() {
    // 创建世界，添加两个球体
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // 创建相机，渲染场景
    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
