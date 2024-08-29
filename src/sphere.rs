use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::MaterialKind,
    ray::Ray,
    vec3::Point,
};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: MaterialKind,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: MaterialKind) -> Sphere {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = h * h - a * c;
        if disc < 0. {
            return None;
        }

        let sqrt_disc = disc.sqrt();
        let mut root = (h - sqrt_disc) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_disc) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(
            ray,
            &point,
            t,
            outward_normal,
            self.material,
        ))
    }
}
