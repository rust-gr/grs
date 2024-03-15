use core::ffi::{c_int, c_uint};
use core::mem::MaybeUninit;
use gr_sys::gr::*;
use paste::paste;
use std::ffi::CStr;

macro_rules! impl_primitive_set {
    ($name:ident, $t:ty)                         => { impl_primitive_set! { @impl $name, {val, $t} } };
    ($name:ident, $t:ty, $t2:ty)                 => { impl_primitive_set! { @impl $name, {val, $t}, {val2, $t2} } };
    ($name:ident, $t:ty, $t2:ty, $t3:ty)         => { impl_primitive_set! { @impl $name, {val, $t}, {val2, $t2}, {val3, $t3} } };
    ($name:ident, $t:ty, $t2:ty, $t3:ty, $t4:ty) => { impl_primitive_set! { @impl $name, {val, $t}, {val2, $t2}, {val3, $t3}, {val4, $t4} } };

    (@impl $name:ident, $({$n:ident, $t:ty}),+) => {
        pub fn $name($($n: impl Into<$t>),+) {
            $(let $n = $n.into();)+
            unsafe { paste!([<gr_$name>])($($n),+) }
        }
    };
}

macro_rules! impl_primitive_inq {
    ($name:ident, $type:ty) => {
        pub fn $name() -> $type {
            let mut val = MaybeUninit::uninit();
            let p = val.as_mut_ptr();
            unsafe {
                paste!([<gr_$name>])(p);
                val.assume_init()
            }
        }
    };
}

macro_rules! impl_primitive_set_inq {
    ($name:ident, $type:ty) => {
        paste! {
            impl_primitive_set! { [<set$name>], $type }
            impl_primitive_inq! { [<inq$name>], $type }
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

pub fn setscale(val: impl Into<c_int>) -> Result<(), GrError> {
    let val = val.into();
    let result = unsafe { gr_setscale(val) };
    match result {
        0 => Ok(()),
        _ => Err(GrError),
    }
}

pub fn inqregenflags() -> c_int {
    unsafe { gr_inqregenflags() }
}

pub use crate::gks::out::GksLinetype as GrLinetype;

use super::GrError;
impl_primitive_set_inq! { linetype, c_int }
impl_primitive_set_inq! { linewidth, f64 }
impl_primitive_set_inq! { linecolorind, c_int }
impl_primitive_set_inq! { markertype, c_int }
impl_primitive_set_inq! { markersize, f64 }
impl_primitive_set_inq! { markercolorind, c_int }
impl_primitive_set_inq! { textcolorind, c_int }
impl_primitive_set_inq! { fillintstyle, c_int }
impl_primitive_set_inq! { fillstyle, c_int }
impl_primitive_set_inq! { fillcolorind, c_int }
impl_primitive_set_inq! { resizebehaviour, c_int }
impl_primitive_set_inq! { colormap, c_int }
impl_primitive_set_inq! { resamplemethod, c_uint }
impl_primitive_set_inq! { borderwidth, f64 }
impl_primitive_set_inq! { bordercolorind, c_int }
impl_primitive_set_inq! { projectiontype, c_int }
impl_primitive_set_inq! { textencoding, c_int }
impl_primitive_set_inq! { charheight, f64 }
impl_primitive_set! { settextfontprec, c_int, c_int }
impl_primitive_set! { selectclipxform, c_int }
impl_primitive_inq! { inqclipxform, c_int }
impl_primitive_set! { setregenflags, c_int }
impl_primitive_set! { setcharexpan, f64 }
impl_primitive_set! { setcharspace, f64 }
impl_primitive_set! { settextpath, c_int }
impl_primitive_set! { selntran, c_int }
impl_primitive_set! { setclip, c_int }
impl_primitive_inq! { inqscale, c_int }
