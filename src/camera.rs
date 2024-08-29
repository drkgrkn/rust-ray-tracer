use std::thread;

use crate::{
    hittable::{Hittable, HittableList},
    interval::Interval,
    material::Material,
    random::random_f64,
    ray::Ray,
    vec3::{Color, Point, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn init(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let image_height = if image_width as f64 / aspect_ratio < 1. {
            1
        } else {
            (image_width as f64 / aspect_ratio) as i32
        };
        let center = look_from;

        // camera
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let mut image: Vec<Color> =
            Vec::with_capacity(self.image_height as usize * self.image_width as usize * 3);

        for j in 0..self.image_height {
            if (self.image_height - j) % 50 == 0 {
                eprintln!("remaining scanlines: {}", self.image_height - j);
            }
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.ray_at(i, j);
                    pixel_color += self.ray_color(&ray, world, self.max_depth);
                }

                image.push(pixel_color);
            }
        }

        for pixel in image {
            print!("{}", (self.pixel_samples_scale * pixel).format());
        }
        eprintln!("done");
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable, depth: u32) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            return if let Some((scattered_ray, attenuation)) =
                hit_rec.material.scatter(ray, &hit_rec)
            {
                attenuation * self.ray_color(&scattered_ray, world, depth - 1)
            } else {
                Color::BLACK
            };
        }

        let unit_direction = ray.dir.normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn ray_at(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }
}
