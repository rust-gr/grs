use ::core::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct F64Range {
    min: f64,
    max: f64,
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

// (min, max) ∈ f64Range ⇒ min < max ⇒ NaN ∉ {min, max}
impl Eq for F64Range {}

impl PartialOrd for F64Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let min = self.min.partial_cmp(&other.min);
        let max = self.max.partial_cmp(&other.max);
        let min = unsafe { min.unwrap_unchecked() };
        let max = unsafe { max.unwrap_unchecked() };
        match min == max {
            true => Some(min),
            false => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidRangeError;

impl TryFrom<(f64, f64)> for F64Range {
    type Error = InvalidRangeError;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1).ok_or(InvalidRangeError)
    }
}
