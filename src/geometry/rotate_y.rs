use crate::geometry::{Aabb, HitRecord, Hittable};
use crate::math::{Interval, Point3, Ray, Vec3};
use std::sync::Arc;

pub struct RotateY {
    hittable: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f32,
    cos_theta: f32,

    bounding_box: Aabb,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable + Send + Sync>, angle: f32) -> Self {
        // 1. 预计算 sin 和 cos
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // 2. 计算旋转后的包围盒
        // 因为 AABB 必须是轴对齐的，旋转后的物体通常需要一个更大的盒子来包围
        let bbox = hittable.bounding_box();
        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        // 遍历包围盒的 8 个顶点 (000, 001, 010, ..., 111)
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    // 获取当前顶点的 x, y, z
                    let x = if i == 0 { bbox.x.min } else { bbox.x.max };
                    let y = if j == 0 { bbox.y.min } else { bbox.y.max };
                    let z = if k == 0 { bbox.z.min } else { bbox.z.max };

                    // 应用旋转公式
                    // new_x = cos(theta)*x + sin(theta)*z
                    // new_z = -sin(theta)*x + cos(theta)*z
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    // 更新新的 AABB 的边界
                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        let bounding_box = Aabb::from_two_points(min, max);

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord<'_>> {
        // [变换 1: 世界空间 -> 物体空间]
        // 将光线反向旋转 (ray_t 不变)

        // Origin 变换
        let origin = Point3::new(
            (self.cos_theta * r.origin.x) - (self.sin_theta * r.origin.z),
            r.origin.y,
            (self.sin_theta * r.origin.x) + (self.cos_theta * r.origin.z),
        );

        // Direction 变换
        let direction = Vec3::new(
            (self.cos_theta * r.direction.x) - (self.sin_theta * r.direction.z),
            r.direction.y,
            (self.sin_theta * r.direction.x) + (self.cos_theta * r.direction.z),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time);

        // 在物体空间进行正常的碰撞检测
        if let Some(mut rec) = self.hittable.hit(&rotated_r, interval) {
            // [变换 2: 物体空间 -> 世界空间]
            // 将击中点 p 和法线 normal 正向旋转回世界空间

            // 旋转点 p
            let p = rec.p;
            rec.p.x = (self.cos_theta * p.x) + (self.sin_theta * p.z);
            rec.p.z = (-self.sin_theta * p.x) + (self.cos_theta * p.z);

            // 旋转法线 normal
            let normal = rec.normal;
            rec.normal.x = (self.cos_theta * normal.x) + (self.sin_theta * normal.z);
            rec.normal.z = (-self.sin_theta * normal.x) + (self.cos_theta * normal.z);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
