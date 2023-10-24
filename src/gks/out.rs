use super::ActiveGks;
use crate::ffi::gks::{gks_fillarea, gks_polyline, gks_polymarker, gks_text};
use ::core::ffi::CStr;
use ::core::fmt;
use ::core::num::TryFromIntError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GksError;
type Result = ::core::result::Result<(), GksError>;

impl fmt::Display for GksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("error in GKS")
    }
}

impl From<TryFromIntError> for GksError {
    fn from(_value: TryFromIntError) -> Self {
        GksError
    }
}

fn check_that(cond: bool) -> Result {
    match cond {
        true => Ok(()),
        false => Err(Default::default()),
    }
}

impl ActiveGks {
    pub fn polyline(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr() as *mut f64;
        let y = y.as_ptr() as *mut f64;
        Ok(unsafe { gks_polyline(n, x, y) })
    }

    pub fn polymarker(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr() as *mut f64;
        let y = y.as_ptr() as *mut f64;
        Ok(unsafe { gks_polymarker(n, x, y) })
    }

    pub fn text(&mut self, (x, y): (f64, f64), s: impl AsRef<CStr>) {
        let p = s.as_ref().as_ptr().cast_mut();
        unsafe { gks_text(x, y, p) }
    }

    pub fn fillarea(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result {
        let n = n.try_into()?;
        let x = x.as_ptr() as *mut f64;
        let y = y.as_ptr() as *mut f64;
        Ok(unsafe { gks_fillarea(n, x, y) })
    }

    // TODO gks_cellarray
}
