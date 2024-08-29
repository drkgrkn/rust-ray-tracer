use crate::{
    hittable::HitRecord,
    random::random_f64,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Copy, Clone, Debug)]
pub enum MaterialKind {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Material for MaterialKind {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            MaterialKind::Metal(m) => m.scatter(ray_in, hit_rec),
            MaterialKind::Lambertian(l) => l.scatter(ray_in, hit_rec),
            MaterialKind::Dielectric(d) => d.scatter(ray_in, hit_rec),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = ray_in.dir.reflect(hit_rec.normal);
        reflected = reflected.normalized() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_rec.point, reflected);
        if scattered.dir.dot(hit_rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }

        Some((Ray::new(hit_rec.point, scatter_direction), self.albedo))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if hit_rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.dir.normalized();
        let cos_theta = (-unit_direction).dot(hit_rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f64() {
            unit_direction.reflect(hit_rec.normal)
        } else {
            unit_direction.refract(hit_rec.normal, ri)
        };

        Some((Ray::new(hit_rec.point, direction), Color::WHITE))
    }
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r0_2 = r0 * r0;
    r0_2 + (1. - r0_2) * (1. - cosine).powi(5)
}
