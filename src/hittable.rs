use crate::{
    interval::Interval,
    material::MaterialKind,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: MaterialKind,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        point: &Point,
        t: f64,
        outward_normal: Vec3,
        material: MaterialKind,
    ) -> Self {
        let front_face = ray.dir.dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal.to_owned()
        } else {
            -outward_normal.to_owned()
        };

        HitRecord {
            point: point.to_owned(),
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub enum HittableKind {
    Sphere(Sphere),
}

impl Hittable for HittableKind {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            HittableKind::Sphere(sphere) => sphere.hit(ray, ray_t),
        }
    }
}

pub struct HittableList {
    pub objects: Vec<HittableKind>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut current_hit = None;
        let mut interval = ray_t;

        for obj in &self.objects {
            if let Some(hit_rec) = obj.hit(ray, interval) {
                interval.max = hit_rec.t;
                current_hit = Some(hit_rec);
            }
        }

        current_hit
    }
}
