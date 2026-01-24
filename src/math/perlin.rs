//! 柏林噪声算法实现。

use crate::math::{Point3, Vec3};
use rand::prelude::{SeedableRng, SliceRandom, SmallRng};

/// 柏林噪声类。
pub struct Perlin {
    /// 随机向量数组。
    random_vec: Vec<Vec3>,

    /// 随机索引数组。
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    /// 创建一个新的柏林噪声实例。
    pub fn new() -> Self {
        let mut rng = SmallRng::from_os_rng();

        // 生成随机向量数组
        let mut random_vec = Vec::with_capacity(Self::POINT_COUNT);
        for _ in 0..Self::POINT_COUNT {
            random_vec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }

        let perm_x = Self::perlin_generate_perm(&mut rng);
        let perm_y = Self::perlin_generate_perm(&mut rng);
        let perm_z = Self::perlin_generate_perm(&mut rng);

        Self {
            random_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// 生成随机索引数组。
    fn perlin_generate_perm(rng: &mut SmallRng) -> Vec<usize> {
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        // 打乱数组
        p.shuffle(rng);
        p
    }

    /// 湍流 (Turbulence)
    /// 将多个频率的噪声叠加在一起，用于制作大理石纹理
    pub fn turbulence(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    /// 计算点 p 处的柏林噪声值。
    pub fn noise(&self, p: Point3) -> f64 {
        // 确定 p 所在晶格的坐标
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        // Hermitian Smoothing (平滑插值)
        // 这一步是为了消除晶格之间的生硬棱角，让过渡更丝滑
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        // 找到包围点 p 的立方体的 8 个顶点，并获取它们的梯度向量
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Self::trilinear_interpolation(c, u, v, w)
    }

    /// 三线性插值
    fn trilinear_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }
}
