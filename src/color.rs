//! 颜色定义以及相关工具方法。

use std::io::Write;
use crate::vec3::Vec3;

/// 颜色定义，为 Vec3 起一个别名。
pub type Color = Vec3;

/// 将颜色输出到流中。
pub fn write_color<W : Write>(out: &mut W, color: &Color) -> std::io::Result<()>{
    // 获取 r, g, b (假设 Vec3 的 x, y, z 对应 r, g, b)
    let r = color.x;
    let g = color.y;
    let b = color.z;

    // 将 [0,1] 转换为 [0,255]
    // 这里暂时直接转换，书的后面章节会加入 Gamma 校正和 Clamp 限制
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    // 使用 writeln! 宏写入流中
    writeln!(out, "{} {} {}", ir, ig, ib)?;

    Ok(())
}
