use ::core::ffi::{c_int, CStr};
use ::core::mem::MaybeUninit;

use crate::ffi::gr::{
    gr_activatews, gr_clearws, gr_closegks, gr_closews, gr_configurews, gr_deactivatews, gr_debug,
    gr_initgr, gr_inqdspsize, gr_opengks, gr_openws, gr_updatews,
};

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
    pub pixels: (isize, isize),
}

#[rustfmt::skip]
pub fn inqdspsize() -> DisplaySize {
    let mut mwidth  = MaybeUninit::uninit();
    let mut mheight = MaybeUninit::uninit();
    let mut  width  = MaybeUninit::uninit();
    let mut  height = MaybeUninit::uninit();
    unsafe { gr_inqdspsize(mwidth.as_mut_ptr(), mheight.as_mut_ptr(), width.as_mut_ptr(), height.as_mut_ptr()) }
    DisplaySize {
        meters: (unsafe { mwidth.assume_init() }     , unsafe { mheight.assume_init() }),
        pixels: (unsafe {  width.assume_init() } as _, unsafe {  height.assume_init() } as _),
    }
}

pub fn openws(wkid: impl Into<c_int>, connection: Option<&CStr>, wstype: impl Into<c_int>) {
    #[rustfmt::skip]
    let   wkid =   wkid.into();
    let wstype = wstype.into();
    #[rustfmt::skip]
    let   conn = connection.unwrap_or(c"").as_ptr().cast_mut();
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
