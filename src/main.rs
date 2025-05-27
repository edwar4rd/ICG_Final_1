use icg_final_1::color::{Color, write_color};
use log::info;
use std::io::stdout;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    env_logger::init();

    println!("P3\n{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    for y in 0..IMAGE_HEIGHT {
        info!("Scanlines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let color = Color::new(
                x as f64 / (IMAGE_WIDTH - 1) as f64,
                y as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            write_color(&mut stdout(), color).unwrap();
        }
    }
    info!("Done.");
}
