use crate::vec3::{Point, Vec3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        return Ray { origin, dir };
    }

    pub fn at(self, t: f64) -> Point {
        return self.origin + t * self.dir;
    }
}
