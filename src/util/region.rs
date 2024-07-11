use core::borrow::{Borrow, BorrowMut};
use core::convert::{AsMut, AsRef};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region([f64; 4]);

macro_rules! impl_getter {
    ($name:ident,$i:expr) => {
        pub const fn $name(&self) -> f64 {
            self.0[$i]
        }
    };
}

impl Region {
    pub const fn new(left: f64, right: f64, bottom: f64, top: f64) -> Self {
        Self([left, right, bottom, top])
    }
    impl_getter! {   left, 0}
    impl_getter! {  right, 1}
    impl_getter! { bottom, 2}
    impl_getter! {    top, 3}
}

impl From<[f64; 4]> for Region {
    fn from(value: [f64; 4]) -> Self {
        Self(value)
    }
}

impl From<Region> for [f64; 4] {
    fn from(value: Region) -> Self {
        value.0
    }
}

impl AsMut<[f64; 4]> for Region {
    fn as_mut(&mut self) -> &mut [f64; 4] {
        &mut self.0
    }
}

impl AsRef<[f64; 4]> for Region {
    fn as_ref(&self) -> &[f64; 4] {
        &self.0
    }
}

impl Borrow<[f64; 4]> for Region {
    fn borrow(&self) -> &[f64; 4] {
        &self.0
    }
}

impl BorrowMut<[f64; 4]> for Region {
    fn borrow_mut(&mut self) -> &mut [f64; 4] {
        &mut self.0
    }
}
