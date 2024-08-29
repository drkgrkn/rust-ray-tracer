use std::ops::{self};

use crate::{
    interval::Interval,
    random::{random_f64, random_f64_between},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Self::new(0., 0., 0.)
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn to_ppm_row(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z())
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self {
            e: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        self.x() < S && self.y() < S && self.z() < S
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2. * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-self.dot(n), 1.);
        let r_out_perp = etai_over_etat * (self + (cos_theta * n));
        let r_out_parallel = -f64::sqrt(f64::abs(1. - r_out_perp.length_squared())) * n;

        r_out_perp + r_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_f64(), random_f64(), random_f64()],
        }
    }

    pub fn random_clamped(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_f64_between(min, max),
                random_f64_between(min, max),
                random_f64_between(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_clamped(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Self::new(random_f64_between(-1., 1.), random_f64_between(-1., 1.), 0.);
            if p.length() < 1. {
                return p;
            }
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z());
    }
}

impl ops::MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.e[0] *= 1. / t;
        self.e[1] *= 1. / t;
        self.e[2] *= 1. / t;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        return Vec3 {
            e: [
                self.e[0] * (1. / t),
                self.e[1] * (1. / t),
                self.e[2] * (1. / t),
            ],
        };
    }
}

impl Color {
    pub fn format(self) -> String {
        let rbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.x()))) as i32;
        let gbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.y()))) as i32;
        let bbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.z()))) as i32;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }

    pub fn to_rgb(&self) -> [i32; 3] {
        let rbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.x()))) as i32;
        let gbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.y()))) as i32;
        let bbyte = (255. * Interval::INTENSITY.clamp(linear_to_gamma(self.z()))) as i32;

        [rbyte, gbyte, bbyte]
    }

    pub const BLACK: Color = Color { e: [0., 0., 0.] };
    pub const WHITE: Color = Color { e: [1., 1., 1.] };
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        linear_component.sqrt()
    } else {
        0.
    }
}
