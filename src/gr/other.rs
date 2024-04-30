use super::{util::impl_primitive_function, GrError};
use core::{
    ffi::{c_int, CStr},
    mem::MaybeUninit,
    slice,
};
use gr_sys::{gkscore::gks_free, gr::*};
use paste::paste;
use std::alloc;

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

#[derive(Debug)]
pub struct GrImage {
    width: usize,
    height: usize,
    data: *mut c_int,
}

pub enum GrColorModel {
    RGB = color_model_t_GR_MODEL_RGB as _,
    HSV = color_model_t_GR_MODEL_HSV as _,
}

impl GrImage {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[c_int] {
        unsafe { slice::from_raw_parts(self.data, self.width * self.height) }
    }

    pub fn draw(&self, x: (f64, f64), y: (f64, f64), model: GrColorModel) {
        let &GrImage {
            width,
            height,
            data,
        } = self;
        // usize -> c_int cast is safe because of provenance
        let w = width as _;
        let h = height as _;
        let m = model as _;
        unsafe { gr_drawimage(x.0, x.1, y.0, y.1, w, h, data, m) }
    }
}

impl Drop for GrImage {
    fn drop(&mut self) {
        let p = self.data.cast();
        unsafe { gks_free(p) }
    }
}

pub fn readimage(path: impl AsRef<CStr>) -> Option<GrImage> {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();
    let mut data = MaybeUninit::uninit();
    let p = path.as_ref().as_ptr().cast_mut();
    let w = width.as_mut_ptr();
    let h = height.as_mut_ptr();
    let d = data.as_mut_ptr();
    match unsafe { gr_readimage(p, w, h, d) } {
        -1 => return None,
        _ => Some(unsafe {
            GrImage {
                width: width.assume_init().try_into().ok()?,
                height: height.assume_init().try_into().ok()?,
                data: data.assume_init(),
            }
        }),
    }
}

pub fn delaunay(n: usize, x: &[f64], y: &[f64]) -> Result<Box<[c_int], alloc::System>, GrError> {
    let mut ntri = MaybeUninit::uninit();
    let mut tri = MaybeUninit::uninit();
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let ntrip = ntri.as_mut_ptr();
    let trip = tri.as_mut_ptr();
    unsafe { gr_delaunay(n, x, y, ntrip, trip) }
    let ntri = unsafe { ntri.assume_init() }.try_into();
    let ntri = match cfg!(debug_assertions) {
        true => ntri.unwrap(),
        false => unsafe { ntri.unwrap_unchecked() },
    };
    Ok(unsafe {
        Box::from_raw_in(
            slice::from_raw_parts_mut(tri.assume_init(), ntri),
            alloc::System,
        )
    })
}

#[allow(clippy::unit_arg)]
pub fn reducepoints(
    x: &[f64],
    y: &[f64],
    new_x: &mut [f64],
    new_y: &mut [f64],
) -> Result<(), GrError> {
    if x.len() != y.len() || new_x.len() != new_y.len() {
        return Err(GrError);
    }
    let n = x.len().try_into()?;
    let x = x.as_ptr();
    let y = y.as_ptr();
    let points = new_x.len().try_into()?;
    let new_x = new_x.as_mut_ptr();
    let new_y = new_y.as_mut_ptr();
    Ok(unsafe { gr_reducepoints(n, x, y, points, new_x, new_y) })
}

// Segments
impl_primitive_function! { createseg(segment: c_int) }
impl_primitive_function! { copysegws(segment: c_int) }
impl_primitive_function! { redrawsegws() }
impl_primitive_function! { setsegtran(segment: c_int, fx: f64, fy: f64, transx: f64, transy: f64, phi: f64, scalex: f64, scaley: f64) }
impl_primitive_function! { closeseg() }

impl_primitive_function! { updategks() }
impl_primitive_function! { emergencyclosegks() }
