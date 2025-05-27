const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3\n{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let r: f64 = x as f64 / ((IMAGE_WIDTH - 1) as f64);
            let g: f64 = y as f64 / ((IMAGE_WIDTH - 1) as f64);
            let b: f64 = 0.0;
            let r = (r * 255.999) as u32;
            let g = (g * 255.999) as u32;
            let b = (b * 255.999) as u32;
            println!("{} {} {}", r, g, b);
        }
    }
}
