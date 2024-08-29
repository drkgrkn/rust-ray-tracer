use core::f64;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn default() -> Interval {
        return Interval {
            max: f64::INFINITY,
            min: f64::NEG_INFINITY,
        };
    }

    pub fn new(min: f64, max: f64) -> Interval {
        return Interval { max, min };
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x <= self.min {
            self.min
        } else if x >= self.max {
            self.max
        } else {
            x
        }
    }

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const EMPTY: Interval = Interval {
        max: f64::NEG_INFINITY,
        min: f64::INFINITY,
    };

    pub const INTENSITY: Interval = Interval {
        min: 0.,
        max: 0.999,
    };
}
