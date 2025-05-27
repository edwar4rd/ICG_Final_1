pub type Color = crate::Vec3;

pub fn write_color<T: std::io::Write>(file: &mut T, pixel_color: Color) -> std::io::Result<()> {
    // let r = (pixel_color.x * 255.999) as u32;
    // let g = (pixel_color.y * 255.999) as u32;
    // let b = (pixel_color.z * 255.999) as u32;

    let intensity = 0.0..0.999;
    let r = (256.0 * pixel_color.x.clamp(intensity.start, intensity.end)) as u32;
    let g = (256.0 * pixel_color.y.clamp(intensity.start, intensity.end)) as u32;
    let b = (256.0 * pixel_color.z.clamp(intensity.start, intensity.end)) as u32;

    writeln!(file, "{} {} {}", r, g, b)
}
