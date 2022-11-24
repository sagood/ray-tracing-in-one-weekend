use std::{
    io::{self, Write},
    sync::Arc,
};

use model::{
    hit::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};
use Vec3 as Point3;

use util::{rtweekend::random_double, rtweekend::INFINITY};

use crate::{
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    model::{camera::Camera, color::Color, hit::HittableList, sphere::Sphere},
    util::rtweekend::PI,
};
mod material;
mod model;
mod util;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let R = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let material_left = Arc::new(Lambertian::new(&Vec3::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(&Vec3::new(1.0, 0.0, 0.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    // Camera
    let camera = Camera::new(90.0, ASPECT_RATIO);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            let s = pixel_color.as_color_repr(SAMPLES_PER_PIXEL);
            print!("{}", s);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
    }

    let unit_direction = r.dir().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}
