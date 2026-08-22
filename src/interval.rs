use crate::utility::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn new_from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max)
        }
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
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding
        }
    }

    pub fn empty() -> Self {
        Self::new(INFINITY, -INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(-INFINITY, INFINITY)
    }
}
