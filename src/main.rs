mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use hittable_list::HittableList;
use sphere::Sphere;

use crate::{
    camera::Camera,
    color::Color,
    material::{Dielectric, Lambertian, Metal},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0., -1.0), 0.5, material_left));
    world.add(Sphere::new(
        Point3::new(-1.0, 0., -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(Point3::new(1.0, 0., -1.0), 0.5, material_right));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.render(&world);
}
