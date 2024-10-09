use super::GrError;
use core::ffi::{c_char, c_uint, CStr};
use core::mem;
use gr_sys::gr::gr_axeslbl;
use std::sync::{Mutex, PoisonError};

type AxesFunction<'a, 'b> = &'a mut dyn FnMut((f64, f64), &'b CStr, f64);

// references don't really have lifetime 'static!
static mut FPX: Option<AxesFunction> = None;
static mut FPY: Option<AxesFunction> = None;
static AXES_MUTEX: Mutex<()> = Mutex::new(());
pub const NO_LABEL_FUNCTION: Option<AxesFunction> = None;

macro_rules! impl_axes_fn {
    ($name:ident, $global:ident) => {
        // only called while thread holds mutex
        unsafe extern "C" fn $name(x: f64, y: f64, s: *const c_char, v: f64) {
            #[allow(static_mut_refs)]
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

#[allow(clippy::unit_arg)]
pub fn axeslbl<'a, F1: FnMut((f64, f64), &'a CStr, f64), F2: FnMut((f64, f64), &'a CStr, f64)>(
    tick_interval: (f64, f64),
    origin: (f64, f64),
    major: (Option<c_uint>, Option<c_uint>),
    tick_size: f64,
    mut fpx: Option<F1>,
    mut fpy: Option<F2>,
) -> Result<(), GrError> {
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
    let major_x = major.0.map_or(Ok(-1), TryInto::try_into)?;
    let major_y = major.1.map_or(Ok(-1), TryInto::try_into)?;
    let (fpx, c_fpx) = fpx
        .as_mut()
        .map(|f| f as AxesFunction)
        .map(|f| unsafe { mem::transmute(f) }) // extend lifetimes
        .map(|f| (f, c_fpx as _))
        .unzip();
    let (fpy, c_fpy) = fpy
        .as_mut()
        .map(|f| f as AxesFunction)
        .map(|f| unsafe { mem::transmute(f) }) // extend lifetimes
        .map(|f| (f, c_fpy as _))
        .unzip();
    let _guard = AXES_MUTEX.lock().unwrap_or_else(PoisonError::into_inner);
    let _drop_guard = DropGuard;
    Ok(unsafe {
        FPX = fpx;
        FPY = fpy;
        gr_axeslbl(
            x_tick, y_tick, x_org, y_org, major_x, major_y, tick_size, c_fpx, c_fpy,
        )
    })
}
