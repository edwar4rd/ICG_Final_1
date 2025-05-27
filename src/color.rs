pub type Color = crate::Vec3;

pub fn write_color<T: std::io::Write>(file: &mut T, pixel_color: Color) -> std::io::Result<()> {
    let r = (pixel_color.x * 255.999) as u32;
    let g = (pixel_color.y * 255.999) as u32;
    let b = (pixel_color.z * 255.999) as u32;

    writeln!(file, "{} {} {}", r, g, b)
}
