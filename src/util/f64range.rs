use core::cmp::Ordering;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct F64Range {
    min: f64,
    max: f64,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvalidRangeError;

impl From<F64Range> for (f64, f64) {
    fn from(value: F64Range) -> Self {
        (value.min, value.max)
    }
}

impl F64Range {
    pub fn new(min: f64, max: f64) -> Option<F64Range> {
        match min < max {
            true => Some(F64Range { min, max }),
            false => None,
        }
    }

    pub const unsafe fn new_unchecked(min: f64, max: f64) -> F64Range {
        F64Range { min, max }
    }

    pub const fn min(&self) -> f64 {
        self.min
    }

    pub const fn max(&self) -> f64 {
        self.max
    }
}

// (min, max) ∈ F64Range ⇒ min < max ⇒ NaN ∉ {min, max}
impl Eq for F64Range {}

impl fmt::Display for F64Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

impl PartialOrd for F64Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let min = self.min.partial_cmp(&other.min);
        let max = self.max.partial_cmp(&other.max);
        let (min, max) = (min?, max?);
        match min == max {
            true => Some(min),
            false => None,
        }
    }
}

impl TryFrom<(f64, f64)> for F64Range {
    type Error = InvalidRangeError;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1).ok_or(InvalidRangeError)
    }
}

impl fmt::Display for InvalidRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("constructing range with invalid numbers")
    }
}
