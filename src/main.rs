const P3_MAGIC_NUMBER: &str = "P3";
const MAX_COLOR_VALUE: i32 = 255;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    println!(
        "{}\n{} {}\n{}",
        P3_MAGIC_NUMBER, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR_VALUE
    );

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScan lines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (r * MAX_COLOR_VALUE as f64) as i32;
            let ig = (g * MAX_COLOR_VALUE as f64) as i32;
            let ib = (b * MAX_COLOR_VALUE as f64) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
}
