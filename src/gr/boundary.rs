use super::GrError;
use core::ffi::c_int;
use core::mem;
use gr_sys::gr::gr_findboundary;
use std::sync::{Mutex, PoisonError};

type RadiusFunction<'a> = &'a mut dyn FnMut(f64, f64) -> f64;

// references don't really have lifetime 'static!
static mut FP: Option<RadiusFunction> = None;
static MUTEX: Mutex<()> = Mutex::new(());
pub const NO_RADIUS_FUNCTION: Option<RadiusFunction> = None;

unsafe extern "C" fn c_r_function(x: f64, y: f64) -> f64 {
    #[allow(static_mut_refs)]
    let f = unsafe { FP.take().unwrap_unchecked() };
    let r = f(x, y);
    unsafe {
        FP = Some(f);
    }
    r
}

pub fn findboundary<F: FnMut(f64, f64) -> f64>(
    n: usize,
    x: &[f64],
    y: &[f64],
    r: f64,
    mut r_function: Option<F>,
    contour: &mut [c_int],
) -> Result<usize, GrError> {
    if n > x.len() || n > y.len() {
        return Err(GrError);
    }
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let cn = contour.len().try_into()?;
    let c = contour.as_mut_ptr();
    let _guard = MUTEX.lock().unwrap_or_else(PoisonError::into_inner);
    let (rf, c_rf) = r_function
        .as_mut()
        .map(|f| f as RadiusFunction)
        .map(|f| unsafe { mem::transmute(f) }) // extend lifetime
        .map(|f| (f, c_r_function as _))
        .unzip();
    let r = unsafe {
        FP = rf;
        let r = gr_findboundary(n, x, y, r, c_rf, cn, c);
        FP = None;
        r
    };
    Ok(r.try_into().unwrap())
}
