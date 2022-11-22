use std::io::{self, Write};

use util::{ray::Ray, vec3::Vec3};
use Vec3 as Point3;

use crate::util::color::Color;
mod util;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_cornet =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let r = Ray::new(
                &origin,
                &(lower_left_cornet + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);
            let s = pixel_color.as_color_repr();

            print!("{}", s);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = r.dir().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
