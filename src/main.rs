mod color;
mod vec3;

use crate::color::{write_color, Color};

const P3_MAGIC_NUMBER: &str = "P3";
const MAX_COLOR_VALUE: i32 = 255;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    println!(
        "{}\n{} {}\n{}",
        P3_MAGIC_NUMBER, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR_VALUE
    );

    let mut stdout = std::io::stdout();

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScan lines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let r = x as f64 / IMAGE_WIDTH as f64;
            let g = y as f64 / IMAGE_HEIGHT as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            write_color(&mut stdout, &pixel_color).expect("Failed to write color to stdout");
        }
    }
    eprintln!("\nDone.");
}
