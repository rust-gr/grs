use core::ffi::c_int;
use core::mem::MaybeUninit;
use gr_sys::gr::*;
use paste::paste;
use std::ffi::CStr;

macro_rules! impl_primitive_set_inq {
    ($name:ident, $type:ty) => {
        paste! {
            pub fn [<set$name>](val: impl Into<$type>) {
                let val = val.into();
                unsafe { [<gr_set$name>](val) };
            }

            pub fn [<inq$name>]() -> $type {
                let mut val = MaybeUninit::uninit();
                let p = val.as_mut_ptr();
                unsafe {
                    [<gr_inq$name>](p);
                    val.assume_init()
                }
            }
        }
    };
}

pub fn inqtext((x, y): (f64, f64), s: impl AsRef<CStr>) -> (f64, f64) {
    let s = s.as_ref().as_ptr().cast_mut();
    let mut tbx = MaybeUninit::uninit();
    let mut tby = MaybeUninit::uninit();
    let tbx_ptr = tbx.as_mut_ptr();
    let tby_ptr = tby.as_mut_ptr();
    unsafe {
        gr_inqtext(x, y, s, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

pub(super) fn textx_opts(world_cooridnates: bool, inline_math: bool) -> c_int {
    match (world_cooridnates, inline_math) {
        (true, true) => GR_TEXT_USE_WC | GR_TEXT_ENABLE_INLINE_MATH,
        (true, false) => GR_TEXT_USE_WC,
        (false, true) => GR_TEXT_ENABLE_INLINE_MATH,
        (false, false) => 0,
    }
}

pub fn inqtextx(
    (x, y): (f64, f64),
    s: impl AsRef<CStr>,
    world_cooridnates: bool,
    inline_math: bool,
) -> (f64, f64) {
    let s = s.as_ref().as_ptr().cast_mut();
    let f = textx_opts(world_cooridnates, inline_math);
    let mut tbx = MaybeUninit::uninit();
    let mut tby = MaybeUninit::uninit();
    let tbx_ptr = tbx.as_mut_ptr();
    let tby_ptr = tby.as_mut_ptr();
    unsafe {
        gr_inqtextx(x, y, s, f, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

pub use crate::gks::out::GksLinetype as GrLinetype;
impl_primitive_set_inq! { linetype, c_int }
impl_primitive_set_inq! { linewidth, f64 }
impl_primitive_set_inq! { linecolorind, c_int }
impl_primitive_set_inq! { markertype, c_int }
impl_primitive_set_inq! { markersize, f64 }
impl_primitive_set_inq! { markercolorind, c_int }
