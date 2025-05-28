pub type Color = crate::Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    // gamma 2
    linear_component.max(0.0).sqrt()
}

pub fn write_color<T: std::io::Write>(file: &mut T, pixel_color: Color) -> std::io::Result<()> {
    // let r = (pixel_color.x * 255.999) as u32;
    // let g = (pixel_color.y * 255.999) as u32;
    // let b = (pixel_color.z * 255.999) as u32;

    let intensity = 0.0..0.999;
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);
    let r = (256.0 * r.clamp(intensity.start, intensity.end)) as u32;
    let g = (256.0 * g.clamp(intensity.start, intensity.end)) as u32;
    let b = (256.0 * b.clamp(intensity.start, intensity.end)) as u32;

    writeln!(file, "{} {} {}", r, g, b)
}
