use super::util::impl_primitive_function;
use core::ffi::c_int;
use std::mem::MaybeUninit;
use gr_sys::gr::*;
use paste::paste;

pub fn samplelocator() -> (f64, f64, c_int) {
    let mut x = MaybeUninit::uninit();
    let mut y = MaybeUninit::uninit();
    let mut buttons = MaybeUninit::uninit();
    let xp = x.as_mut_ptr();
    let yp = y.as_mut_ptr();
    let bp = buttons.as_mut_ptr();
    unsafe {
        gr_samplelocator(xp, yp, bp);
        (x.assume_init(), y.assume_init(), buttons.assume_init())
    }
}

// Segments
impl_primitive_function! { createseg(segment: c_int) }
impl_primitive_function! { copysegws(segment: c_int) }
impl_primitive_function! { redrawsegws() }
impl_primitive_function! { setsegtran(segment: c_int, fx: f64, fy: f64, transx: f64, transy: f64, phi: f64, scalex: f64, scaley: f64) }
impl_primitive_function! { closeseg() }

impl_primitive_function! { updategks() }
impl_primitive_function! { emergencyclosegks() }
