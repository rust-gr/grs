use ::core::ffi::{c_int, CStr};
use ::core::fmt;
use ::core::mem::MaybeUninit;
use ::std::num::TryFromIntError;

use crate::ffi::gr::{
    gr_activatews, gr_clearws, gr_closegks, gr_closews, gr_configurews, gr_deactivatews, gr_debug,
    gr_initgr, gr_inqdspsize, gr_opengks, gr_openws, gr_polyline, gr_polymarker, gr_text, gr_textx,
    gr_updatews, GR_TEXT_ENABLE_INLINE_MATH, GR_TEXT_USE_WC,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GrError;
type Result<T> = ::core::result::Result<T, GrError>;

impl fmt::Display for GrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid arguments to GR")
    }
}

impl std::error::Error for GrError {}

impl From<TryFromIntError> for GrError {
    fn from(_value: TryFromIntError) -> Self {
        GrError
    }
}

fn check_that(x: bool) -> Result<()> {
    match x {
        true => Ok(()),
        false => Err(GrError),
    }
}

pub fn initgr() {
    unsafe { gr_initgr() }
}

pub fn debug() -> c_int {
    unsafe { gr_debug() }
}

pub fn opengks() {
    unsafe { gr_opengks() }
}

pub fn closegks() {
    unsafe { gr_closegks() }
}

#[derive(Clone, Copy, Debug)]
pub struct DisplaySize {
    pub meters: (f64, f64),
    pub pixels: (c_int, c_int),
}

#[rustfmt::skip]
pub fn inqdspsize() -> DisplaySize {
    let mut mwidth  = MaybeUninit::uninit();
    let mut mheight = MaybeUninit::uninit();
    let mut  width  = MaybeUninit::uninit();
    let mut  height = MaybeUninit::uninit();
    unsafe {
        gr_inqdspsize(
             mwidth.as_mut_ptr(),
            mheight.as_mut_ptr(),
              width.as_mut_ptr(),
             height.as_mut_ptr()
        )
    }
    DisplaySize {
        meters: (unsafe { mwidth.assume_init() }, unsafe { mheight.assume_init() }),
        pixels: (unsafe {  width.assume_init() }, unsafe {  height.assume_init() }),
    }
}

pub fn openws(wkid: impl Into<c_int>, connection: Option<&CStr>, wstype: impl Into<c_int>) {
    let conn = connection.unwrap_or(c"").as_ptr().cast_mut();
    #[rustfmt::skip]
    let   wkid =   wkid.into();
    let wstype = wstype.into();
    unsafe { gr_openws(wkid, conn, wstype) }
}

pub fn closews(wkid: impl Into<c_int>) {
    let wkid = wkid.into();
    unsafe { gr_closews(wkid) }
}

pub fn activatews(wkid: impl Into<c_int>) {
    let wkid = wkid.into();
    unsafe { gr_activatews(wkid) }
}

pub fn deactivatews(wkid: impl Into<c_int>) {
    let wkid = wkid.into();
    unsafe { gr_deactivatews(wkid) }
}

pub fn configurews() {
    unsafe { gr_configurews() }
}

pub fn clearws() {
    unsafe { gr_clearws() }
}

pub fn updatews() {
    unsafe { gr_updatews() }
}

#[allow(clippy::unit_arg)]
pub fn polyline(n: usize, x: &[f64], y: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    Ok(unsafe { gr_polyline(n, x, y) })
}

#[allow(clippy::unit_arg)]
pub fn polymarker(n: usize, x: &[f64], y: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    Ok(unsafe { gr_polymarker(n, x, y) })
}

pub fn text((x, y): (f64, f64), s: impl AsRef<CStr>) {
    let p = s.as_ref().as_ptr().cast_mut();
    unsafe { gr_text(x, y, p) }
}

pub fn textx((x, y): (f64, f64), s: impl AsRef<CStr>, world_cooridnates: bool, inline_math: bool) {
    let p = s.as_ref().as_ptr().cast_mut();
    let f = match (world_cooridnates, inline_math) {
        (true, true) => GR_TEXT_USE_WC | GR_TEXT_ENABLE_INLINE_MATH,
        (true, false) => GR_TEXT_USE_WC,
        (false, true) => GR_TEXT_ENABLE_INLINE_MATH,
        (false, false) => 0,
    };
    unsafe { gr_textx(x, y, p, f) }
}
