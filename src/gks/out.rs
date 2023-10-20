use super::Result;
use super::ws::GksActiveWs;
use crate::ffi::gks::{gks_polyline, gks_polymarker, gks_text, gks_fillarea, GKS_K_TEXT_MAX_SIZE, gks_cellarray};
use ::core::ffi::{c_char, c_int, CStr};

impl GksActiveWs<'_> {
    pub fn polyline(&self, n: c_int, x: &[f64], y: &[f64]) -> Result<()> {
        match n >= 2 && n as usize <= x.len() && n as usize <= y.len() {
            true => Ok({
                let x = x.as_ptr() as *mut f64;
                let y = y.as_ptr() as *mut f64;
                unsafe { gks_polyline(n, x, y) };
            }),
            false => Err(Default::default()),
        }
    }

    pub fn polymarker(&self, n: c_int, x: &[f64], y: &[f64]) -> Result<()> {
        match n >= 1 {
            true => Ok({
                let x = x.as_ptr() as *mut f64;
                let y = y.as_ptr() as *mut f64;
                unsafe { gks_polymarker(n, x, y) };
            }),
            false => Err(Default::default()),
        }
    }

    pub fn text(&self, (x, y): (f64, f64), s: impl AsRef<CStr>) -> Result<()> {
        let s = s.as_ref();
        match s.to_bytes().len() < GKS_K_TEXT_MAX_SIZE as usize {
            true => Ok({
                let p = s.as_ptr() as *mut c_char;
                unsafe { gks_text(x, y, p) }
            }),
            false => Err(Default::default()),
        }
    }

    pub fn fillarea(n: c_int, x: &[f64], y: &[f64]) -> Result<()> {
        match n >= 3 {
            true => Ok({
                let x = x.as_ptr() as *mut f64;
                let y = y.as_ptr() as *mut f64;
                unsafe { gks_fillarea(n, x, y) };
            }),
            false => Err(Default::default()),
        }
    }
}
