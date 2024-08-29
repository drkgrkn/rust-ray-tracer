mod camera;
mod hittable;
mod interval;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

use core::f64;

use camera::Camera;
use hittable::{HittableKind, HittableList};
use material::{Dielectric, Lambertian, MaterialKind, Metal};
use random::{random_f64, random_f64_between};
use sphere::Sphere;
use vec3::{Color, Point, Vec3};

fn old_world() -> HittableList {
    let mat_ground = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.),
    });
    let mat_center = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let mat_left = MaterialKind::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    let mat_bubble = MaterialKind::Dielectric(Dielectric {
        refraction_index: 1. / 1.5,
    });
    let mat_right = MaterialKind::Metal(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.,
    });

    // world
    let mut world = HittableList { objects: vec![] };
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(0.0, -100.5, -1.),
        100.,
        mat_ground,
    )));
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(0., 0., -1.2),
        0.5,
        mat_center,
    )));
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        mat_left,
    )));
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(-1., 0., -1.),
        0.4,
        mat_bubble,
    )));
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        mat_right,
    )));

    world
}

fn new_world() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let mat_left = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(0., 0., 1.),
    });
    let mat_right = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(1., 0., 0.),
    });

    let r = (f64::consts::FRAC_PI_4).cos();
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Vec3::new(-r, 0., -1.),
        r,
        mat_left,
    )));
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Vec3::new(r, 0., -1.),
        r,
        mat_right,
    )));

    world
}

fn final_world() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let ground_material = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        eprintln!("{}/{} spheres done", 23 * (a + 11), 23 * 23);
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_mat = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    MaterialKind::Lambertian(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    let fuzz = random_f64_between(0., 0.5);
                    MaterialKind::Metal(Metal { albedo, fuzz })
                } else {
                    MaterialKind::Dielectric(Dielectric {
                        refraction_index: 1.5,
                    })
                };
                world
                    .objects
                    .push(HittableKind::Sphere(Sphere::new(center, 0.2, sphere_mat)));
            }
        }
    }
    eprintln!("sphere creation done");

    let mat1 = MaterialKind::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(0., 1., 0.),
        1.,
        mat1,
    )));
    let mat2 = MaterialKind::Lambertian(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(-4., 1., 0.),
        1.,
        mat2,
    )));
    let mat3 = MaterialKind::Metal(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.objects.push(HittableKind::Sphere(Sphere::new(
        Point::new(4., 1., 0.),
        1.,
        mat3,
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov: f64 = 20.;
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let defocus_angle = 0.6;
    let focus_dist = 10.;

    let camera = Camera::init(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
    );

    let world = final_world();

    camera.render(&world);
}
