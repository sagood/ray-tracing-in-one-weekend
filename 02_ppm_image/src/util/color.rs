use super::vec3::Vec3;

trait Color {
    fn as_color_repr(color: &Vec3) -> String;
}

impl Color for Vec3 {
    fn as_color_repr(color: &Vec3) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * color.x()) as i32,
            (255.999 * color.y()) as i32,
            (255.999 * color.z()) as i32
        )
    }
}
