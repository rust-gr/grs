use super::{util::impl_primitive_function, GrError};
use core::ptr;
use core::{
    ffi::{c_int, CStr},
    mem::MaybeUninit,
    slice,
};
use gr_sys::{gkscore::gks_free, gr::*};
use paste::paste;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        -1 => None,
        _ => Some(unsafe {
            GrImage {
                width: width.assume_init().try_into().ok()?,
                height: height.assume_init().try_into().ok()?,
                data: data.assume_init(),
            }
        }),
    }
}

pub fn delaunay(n: usize, x: &[f64], y: &[f64]) -> Result<Vec<c_int>, GrError> {
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
    let ptr = unsafe { tri.assume_init() };
    let res = unsafe { slice::from_raw_parts_mut(ptr, ntri) }.to_vec();
    unsafe { gks_free(ptr.cast()) }
    Ok(res)
    // #![feature(allocator_api)]
    // Ok(unsafe {
    //     Box::from_raw_in(
    //         slice::from_raw_parts_mut(tri.assume_init(), ntri),
    //         alloc::System,
    //     )
    // })
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

pub fn loadfont(filename: impl AsRef<CStr>) -> c_int {
    let mut font = MaybeUninit::uninit();
    let file = filename.as_ref().as_ptr().cast_mut();
    let fontp = font.as_mut_ptr();
    unsafe {
        gr_loadfont(file, fontp);
        font.assume_init()
    }
}

#[allow(clippy::unit_arg)]
pub fn gradient(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    u: &mut [f64],
    v: &mut [f64],
) -> Result<(), GrError> {
    let n = x.len() * y.len();
    if [z.len(), u.len(), v.len()].into_iter().any(|l| l < n) {
        return Err(GrError);
    }
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    let u = u.as_mut_ptr();
    let v = v.as_mut_ptr();
    Ok(unsafe { gr_gradient(nx, ny, x, y, z, u, v) })
}

pub fn panzoom((x, y): (f64, f64), zoom: (f64, f64)) -> ((f64, f64), (f64, f64)) {
    let mut xmin = MaybeUninit::uninit();
    let mut xmax = MaybeUninit::uninit();
    let mut ymin = MaybeUninit::uninit();
    let mut ymax = MaybeUninit::uninit();
    let (xz, yz) = zoom;
    let xminp = xmin.as_mut_ptr();
    let xmaxp = xmax.as_mut_ptr();
    let yminp = ymin.as_mut_ptr();
    let ymaxp = ymax.as_mut_ptr();
    unsafe {
        gr_panzoom(x, y, xz, yz, xminp, xmaxp, yminp, ymaxp);
        (
            (xmin.assume_init(), xmax.assume_init()),
            (ymin.assume_init(), ymax.assume_init()),
        )
    }
}

pub fn camerainteraction(mouse_start_pos: (f64, f64), mouse_end_pos: (f64, f64)) {
    let (msx, msy) = mouse_start_pos;
    let (mex, mey) = mouse_end_pos;
    unsafe { gr_camerainteraction(msx, msy, mex, mey) }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VolumeAlgorithm {
    Emission = volume_rendering_model_t_GR_VOLUME_EMISSION as _,
    Absorption = volume_rendering_model_t_GR_VOLUME_ABSORPTION as _,
    Mip = volume_rendering_model_t_GR_VOLUME_MIP as _,
}

#[allow(clippy::too_many_arguments)]
pub fn cpubasedvolume(
    nx: usize,
    ny: usize,
    nz: usize,
    data: &[f64],
    algo: VolumeAlgorithm,
    mut min: Option<f64>,
    mut max: Option<f64>,
    min_val: Option<(f64, f64, f64)>,
    max_val: Option<(f64, f64, f64)>,
) -> Result<(Option<f64>, Option<f64>), GrError> {
    if nx * ny * nz > data.len() {
        return Err(GrError);
    }
    let nx = nx.try_into()?;
    let ny = ny.try_into()?;
    let nz = nz.try_into()?;
    let data = data.as_ptr().cast_mut();
    let minp = min.as_mut().map_or(ptr::null_mut(), |d| d as _);
    let maxp = max.as_mut().map_or(ptr::null_mut(), |d| d as _);
    let min_val = min_val.map(Into::<[f64; 3]>::into);
    let max_val = max_val.map(Into::<[f64; 3]>::into);
    let min_val = min_val
        .as_ref()
        .map_or(ptr::null::<f64>(), |t| t as _)
        .cast_mut();
    let max_val = max_val
        .as_ref()
        .map_or(ptr::null::<f64>(), |t| t as _)
        .cast_mut();
    unsafe { gr_cpubasedvolume(nx, ny, nz, data, algo as _, minp, maxp, min_val, max_val) }
    Ok((min, max))
}

// Segments
impl_primitive_function! { createseg(segment: c_int) }
impl_primitive_function! { copysegws(segment: c_int) }
impl_primitive_function! { redrawsegws() }
impl_primitive_function! { setsegtran(segment: c_int, fx: f64, fy: f64, transx: f64, transy: f64, phi: f64, scalex: f64, scaley: f64) }
impl_primitive_function! { closeseg() }

impl_primitive_function! { updategks() }
impl_primitive_function! { emergencyclosegks() }
