use core::ffi::{c_char, c_uint, CStr};
use core::mem;
use gr_sys::gr::gr_axeslbl;
use std::sync::{Mutex, PoisonError};

pub type AxesFunction<'a> = &'a mut dyn FnMut((f64, f64), &CStr, f64);

// functions don't really have lifetime 'static!
static mut FPX: Option<AxesFunction> = None;
static mut FPY: Option<AxesFunction> = None;
static AXES_MUTEX: Mutex<()> = Mutex::new(());

macro_rules! impl_axes_fn {
    ($name:ident, $global:ident) => {
        unsafe extern "C" fn $name(x: f64, y: f64, s: *const c_char, v: f64) {
            let f = unsafe { $global.take() }.unwrap(); // should never fail because axeslbl should set this
            f((x, y), unsafe { CStr::from_ptr(s) }, v);
            unsafe {
                $global = Some(f);
            }
        }
    };
}
impl_axes_fn! {c_fpx, FPX}
impl_axes_fn! {c_fpy, FPY}

pub fn axeslbl(
    tick_interval: (f64, f64),
    origin: (f64, f64),
    major: (Option<c_uint>, Option<c_uint>),
    tick_size: f64,
    fpx: Option<AxesFunction>,
    fpy: Option<AxesFunction>,
) {
    struct DropGuard;
    impl Drop for DropGuard {
        fn drop(&mut self) {
            unsafe {
                FPX = None;
                FPY = None;
            }
        }
    }
    let (x_tick, y_tick) = tick_interval;
    let (x_org, y_org) = origin;
    let major_x = major.0.map_or(-1, |m| m as _);
    let major_y = major.1.map_or(-1, |m| m as _);
    let (fpx, c_fpx) = fpx
        .map(|f| (unsafe { mem::transmute(f) }, c_fpx as _))
        .unzip();
    let (fpy, c_fpy) = fpy
        .map(|f| (unsafe { mem::transmute(f) }, c_fpy as _))
        .unzip();
    let _guard = AXES_MUTEX.lock().unwrap_or_else(PoisonError::into_inner);
    let _drop_guard = DropGuard;
    unsafe {
        FPX = fpx;
        FPY = fpy;
        gr_axeslbl(
            x_tick, y_tick, x_org, y_org, major_x, major_y, tick_size, c_fpx, c_fpy,
        )
    }
}
