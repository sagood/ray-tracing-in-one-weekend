use super::vec3::Vec3;

pub trait Color {
    fn as_color_repr(&self) -> String;
}

impl Color for Vec3 {
    fn as_color_repr(&self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32
        )
    }
}
