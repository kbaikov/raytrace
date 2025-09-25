use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::random_f64,
    vec3::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: usize,
    pub max_depth: usize, // Maximum number of ray bounces into scene
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    image_height: u64,
    pixel_samples_scale: f64,
    center: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        }
    }
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        // Render
        let mut out = std::io::stdout();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                write_color(&mut out, self.pixel_samples_scale * pixel_color);
            }
        }
        eprint!("\rDone                         \n")
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2. - viewport_v / 2.;

        self.pixel100_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn ray_color(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0., 0.);
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attentuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attentuation * self.ray_color(&scattered, depth - 1, world);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
    pub fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel100_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }
}
