use super::util::{impl_primitive_function, textx_opts};
use super::GrError;
use crate::util::f64range::F64Range;
use core::ffi::{c_int, c_uint, CStr};
use core::mem;
use core::mem::MaybeUninit;
use core::ptr;
use gr_sys::gkscore::MAX_COLOR;
use gr_sys::gr::*;
use gr_sys::strlib::*;
use paste::paste;

#[rustfmt::skip]
macro_rules! impl_primitive_set {
    ($name:ident, $t:ty)                         => { impl_primitive_function! { $name(val: $t) } };
    ($name:ident, $t:ty, $t2:ty)                 => { impl_primitive_function! { $name(val: $t, val2: $t2) } };
    ($name:ident, $t:ty, $t2:ty, $t3:ty)         => { impl_primitive_function! { $name(val: $t, val2: $t2, val3: $t3) } };
    ($name:ident, $t:ty, $t2:ty, $t3:ty, $t4:ty) => { impl_primitive_function! { $name(val: $t, val2: $t2, val3: $t3, val4: $t4) } };
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

    ($name:ident 3f64) => {
        pub fn $name() -> (f64, f64, f64) {
            let mut x = MaybeUninit::uninit();
            let mut y = MaybeUninit::uninit();
            let mut z = MaybeUninit::uninit();
            let xp = x.as_mut_ptr();
            let yp = y.as_mut_ptr();
            let zp = z.as_mut_ptr();
            unsafe {
                paste!([<gr_$name>])(xp, yp, zp);
                (x.assume_init(), y.assume_init(), z.assume_init())
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

    ($name:ident 3f64) => {
        paste! {
            impl_primitive_set! { [<set$name>], f64, f64, f64 }
            impl_primitive_inq! { [<inq$name>] 3f64 }
        }
    };
}

// TODO move to appropriate location
pub fn startlistener() -> Result<(), GrError> {
    match unsafe { gr_startlistener() } {
        -1 => Err(GrError),
        _ => Ok(()),
    }
}

pub fn inqtext((x, y): (f64, f64), s: impl AsRef<CStr>) -> ([f64; 4], [f64; 4]) {
    let s = s.as_ref().as_ptr().cast_mut();
    let mut tbx = MaybeUninit::<[f64; 4]>::uninit();
    let mut tby = MaybeUninit::<[f64; 4]>::uninit();
    let tbx_ptr = tbx.as_mut_ptr().cast();
    let tby_ptr = tby.as_mut_ptr().cast();
    unsafe {
        gr_inqtext(x, y, s, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

pub fn inqtextx(
    (x, y): (f64, f64),
    s: impl AsRef<CStr>,
    world_cooridnates: bool,
    inline_math: bool,
) -> ([f64; 4], [f64; 4]) {
    let s = s.as_ref().as_ptr().cast_mut();
    let f = textx_opts(world_cooridnates, inline_math);
    let mut tbx = MaybeUninit::<[f64; 4]>::uninit();
    let mut tby = MaybeUninit::<[f64; 4]>::uninit();
    let tbx_ptr = tbx.as_mut_ptr().cast();
    let tby_ptr = tby.as_mut_ptr().cast();
    unsafe {
        gr_inqtextx(x, y, s, f, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

pub fn inqtextext((x, y): (f64, f64), s: impl AsRef<CStr>) -> ([f64; 4], [f64; 4]) {
    let s = s.as_ref().as_ptr().cast_mut();
    let mut tbx = MaybeUninit::<[f64; 4]>::uninit();
    let mut tby = MaybeUninit::<[f64; 4]>::uninit();
    let tbx_ptr = tbx.as_mut_ptr().cast();
    let tby_ptr = tby.as_mut_ptr().cast();
    unsafe {
        gr_inqtextext(x, y, s, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

/// `axis` specifies, in which direction the text is drawn (1: YX-plane, 2: XY plane, 3: YZ plane, 4: XZ plane). Negative values invert shearing.
pub fn inqtext3d(
    (x, y, z): (f64, f64, f64),
    s: impl AsRef<CStr>,
    axis: impl Into<c_int>,
) -> ([[[f64; 4]; 2]; 2], [[[f64; 4]; 2]; 2]) {
    // [MaybeUninit::<f64>::uninit(); 16] compiles but is wrong!
    let mut xout: [MaybeUninit<f64>; 16] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut yout: [MaybeUninit<f64>; 16] = unsafe { MaybeUninit::uninit().assume_init() };
    let p = s.as_ref().as_ptr().cast_mut();
    let axis = axis.into();
    let xp = xout.as_mut_ptr().cast();
    let yp = yout.as_mut_ptr().cast();
    unsafe {
        gr_inqtext3d(x, y, z, p, axis, xp, yp);
        (mem::transmute(xout), mem::transmute(yout))
    }
}

pub fn inqmathtex((x, y): (f64, f64), s: impl AsRef<CStr>) -> (f64, f64) {
    let s = s.as_ref().as_ptr().cast_mut();
    let mut tbx = MaybeUninit::uninit();
    let mut tby = MaybeUninit::uninit();
    let tbx_ptr = tbx.as_mut_ptr();
    let tby_ptr = tby.as_mut_ptr();
    unsafe {
        gr_inqmathtex(x, y, s, tbx_ptr, tby_ptr);
        (tbx.assume_init(), tby.assume_init())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScientificFormatOption {
    E = SCIENTIFIC_FORMAT_OPTION_E as _,
    TexTex = SCIENTIFIC_FORMAT_OPTION_TEXTEX as _,
    MathTex = SCIENTIFIC_FORMAT_OPTION_MATHTEX as _,
}

pub fn setscientificformat(opt: ScientificFormatOption) {
    let opt = opt as _;
    unsafe { gr_setscientificformat(opt) };
}

pub fn setscale(val: impl Into<c_int>) -> Result<(), GrError> {
    let val = val.into();
    let result = unsafe { gr_setscale(val) };
    match result {
        0 => Ok(()),
        _ => Err(GrError),
    }
}

pub fn setspace(z: F64Range, rotation: c_int, tilt: c_int) -> Result<(), GrError> {
    let (zmin, zmax) = z.into();
    let err = unsafe { gr_setspace(zmin, zmax, rotation, tilt) };
    match err {
        0 => Ok(()),
        -1 => Err(GrError),
        _ => unreachable!(),
    }
}

pub fn inqspace() -> (F64Range, c_int, c_int) {
    let mut zmin = MaybeUninit::uninit();
    let mut zmax = MaybeUninit::uninit();
    let mut rotation = MaybeUninit::uninit();
    let mut tilt = MaybeUninit::uninit();
    let zmin_ptr = zmin.as_mut_ptr();
    let zmax_ptr = zmax.as_mut_ptr();
    let rotation_ptr = rotation.as_mut_ptr();
    let tilt_ptr = tilt.as_mut_ptr();
    unsafe {
        gr_inqspace(zmin_ptr, zmax_ptr, rotation_ptr, tilt_ptr);
        (
            F64Range::new_unchecked(zmin.assume_init(), zmax.assume_init()),
            rotation.assume_init(),
            tilt.assume_init(),
        )
    }
}

pub fn inqspace3d() -> Option<(f64, f64, f64, f64)> {
    let mut used = MaybeUninit::uninit();
    let mut azimuth = MaybeUninit::uninit();
    let mut polar = MaybeUninit::uninit();
    let mut fov = MaybeUninit::uninit();
    let mut cam = MaybeUninit::uninit();
    let used_ptr = used.as_mut_ptr();
    let azimuth_ptr = azimuth.as_mut_ptr();
    let polar_ptr = polar.as_mut_ptr();
    let fov_ptr = fov.as_mut_ptr();
    let cam_ptr = cam.as_mut_ptr();
    unsafe { gr_inqspace3d(used_ptr, azimuth_ptr, polar_ptr, fov_ptr, cam_ptr) }
    let used = unsafe { used.assume_init() };
    match used {
        1 => Some(unsafe {
            (
                azimuth.assume_init(),
                polar.assume_init(),
                fov.assume_init(),
                cam.assume_init(),
            )
        }),
        0 => None,
        _ => unreachable!(),
    }
}

#[allow(clippy::unit_arg)]
pub fn setcolormapfromrgb(
    n: usize,
    r: &[f64],
    g: &[f64],
    b: &[f64],
    x: Option<&[f64]>,
) -> Result<(), GrError> {
    if n > r.len() || n > g.len() || n > b.len() || x.map_or(false, |x| n > x.len()) {
        return Err(GrError);
    }
    let n = n.try_into()?;
    let r = r.as_ptr().cast_mut();
    let g = g.as_ptr().cast_mut();
    let b = b.as_ptr().cast_mut();
    let x = x.map_or(ptr::null(), |x| x.as_ptr()).cast_mut();
    Ok(unsafe { gr_setcolormapfromrgb(n, r, g, b, x) })
}

pub fn inqcolor(n: usize) -> Result<c_int, GrError> {
    match n.try_into() {
        Ok(n) if n < MAX_COLOR => {
            let mut col = MaybeUninit::uninit();
            let cp = col.as_mut_ptr();
            Ok(unsafe {
                gr_inqcolor(n, cp);
                col.assume_init()
            })
        }
        _ => Err(GrError),
    }
}

pub fn inqcolormapinds() -> (c_int, c_int) {
    let mut fst = MaybeUninit::uninit();
    let mut snd = MaybeUninit::uninit();
    let fp = fst.as_mut_ptr();
    let sp = snd.as_mut_ptr();
    unsafe {
        gr_inqcolormapinds(fp, sp);
        (fst.assume_init(), snd.assume_init())
    }
}

pub fn inqcolorfromrgb(r: f64, g: f64, b: f64) -> c_int {
    unsafe { gr_inqcolorfromrgb(r, g, b) }
}

pub fn hsvtorgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let mut r = MaybeUninit::uninit();
    let mut g = MaybeUninit::uninit();
    let mut b = MaybeUninit::uninit();
    let rp = r.as_mut_ptr();
    let gp = g.as_mut_ptr();
    let bp = b.as_mut_ptr();
    unsafe {
        gr_hsvtorgb(h, s, v, rp, gp, bp);
        (r.assume_init(), g.assume_init(), b.assume_init())
    }
}

pub fn tick(range: F64Range) -> f64 {
    let (min, max) = range.into();
    unsafe { gr_tick(min, max) }
}

pub fn validaterange(min: f64, max: f64) -> bool {
    0 != unsafe { gr_validaterange(min, max) }
}

pub fn adjustlimits(limits: F64Range) -> F64Range {
    let (mut min, mut max) = limits.into();
    unsafe {
        gr_adjustrange(&mut min, &mut max);
        F64Range::new_unchecked(min, max)
    }
}

pub fn adjustrange(mut min: f64, mut max: f64) -> (f64, f64) {
    unsafe { gr_adjustrange(&mut min, &mut max) }
    (min, max)
}

pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(gr_version()) }
}

pub fn beginprint(pathname: impl AsRef<CStr>) {
    let p = pathname.as_ref().as_ptr().cast_mut();
    unsafe { gr_beginprint(p) }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrColorMode {
    GrayScale,
    Color,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrOrientation {
    Landscape,
    Portrait,
}

pub fn beginprintext(
    pathname: impl AsRef<CStr>,
    mode: GrColorMode,
    format: impl AsRef<CStr>,
    orientation: GrOrientation,
) {
    let p = pathname.as_ref().as_ptr().cast_mut();
    let m = match mode {
        GrColorMode::GrayScale => c"GrayScale",
        GrColorMode::Color => c"Color",
    }
    .as_ptr()
    .cast_mut();
    let f = format.as_ref().as_ptr().cast_mut();
    let o = match orientation {
        GrOrientation::Landscape => c"Landscape",
        GrOrientation::Portrait => c"Portrait",
    }
    .as_ptr()
    .cast_mut();
    unsafe { gr_beginprintext(p, m, f, o) }
}

pub fn endprint() {
    unsafe { gr_endprint() }
}

pub fn ndctowc(mut x: f64, mut y: f64) -> (f64, f64) {
    unsafe { gr_ndctowc(&mut x, &mut y) }
    (x, y)
}

pub fn wctondc(mut x: f64, mut y: f64) -> (f64, f64) {
    unsafe { gr_wctondc(&mut x, &mut y) }
    (x, y)
}

pub fn wc3towc(mut x: f64, mut y: f64, mut z: f64) -> (f64, f64, f64) {
    unsafe { gr_wc3towc(&mut x, &mut y, &mut z) }
    (x, y, z)
}

pub fn setbboxcallback(id: impl Into<c_int>, f: unsafe extern "C" fn(c_int, f64, f64, f64, f64)) {
    let id = id.into();
    let f = Some(f);
    unsafe { gr_setbboxcallback(id, f) }
}

pub fn uselinespec(linespec: impl AsRef<CStr>) -> c_int {
    let s = linespec.as_ref().as_ptr().cast_mut();
    unsafe { gr_uselinespec(s) }
}

pub fn setwindows3d(x: (f64, f64), y: (f64, f64), z: (f64, f64)) {
    let (xmin, xmax) = x;
    let (ymin, ymax) = y;
    let (zmin, zmax) = z;
    unsafe { gr_setwindow3d(xmin, xmax, ymin, ymax, zmin, zmax) }
}

pub fn inqwindows3d() -> ((f64, f64), (f64, f64), (f64, f64)) {
    let mut xmin = MaybeUninit::uninit();
    let mut xmax = MaybeUninit::uninit();
    let mut ymin = MaybeUninit::uninit();
    let mut ymax = MaybeUninit::uninit();
    let mut zmin = MaybeUninit::uninit();
    let mut zmax = MaybeUninit::uninit();
    let xmin_ptr = xmin.as_mut_ptr();
    let xmax_ptr = xmax.as_mut_ptr();
    let ymin_ptr = ymin.as_mut_ptr();
    let ymax_ptr = ymax.as_mut_ptr();
    let zmin_ptr = zmin.as_mut_ptr();
    let zmax_ptr = zmax.as_mut_ptr();
    unsafe { gr_inqwindow3d(xmin_ptr, xmax_ptr, ymin_ptr, ymax_ptr, zmin_ptr, zmax_ptr) }
    unsafe {
        (
            (xmin.assume_init(), xmax.assume_init()),
            (ymin.assume_init(), ymax.assume_init()),
            (zmin.assume_init(), zmax.assume_init()),
        )
    }
}

pub fn settransformationparameters(
    camera_pos: (f64, f64, f64),
    up: (f64, f64, f64),
    focus_point: (f64, f64, f64),
) {
    let (cpx, cpy, cpz) = camera_pos;
    let (upx, upy, upz) = up;
    let (fpx, fpy, fpz) = focus_point;
    unsafe { gr_settransformationparameters(cpx, cpy, cpz, upx, upy, upz, fpx, fpy, fpz) }
}

pub fn inqtransformationparameters() -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
    let mut arr: [MaybeUninit<f64>; 9] = unsafe { MaybeUninit::uninit().assume_init() };
    let arr: [f64; 9] = unsafe {
        gr_inqtransformationparameters(
            arr[0].as_mut_ptr(),
            arr[1].as_mut_ptr(),
            arr[2].as_mut_ptr(),
            arr[3].as_mut_ptr(),
            arr[4].as_mut_ptr(),
            arr[5].as_mut_ptr(),
            arr[6].as_mut_ptr(),
            arr[7].as_mut_ptr(),
            arr[7].as_mut_ptr(),
        );
        mem::transmute(arr)
    };
    (
        (arr[0], arr[1], arr[2]),
        (arr[3], arr[4], arr[5]),
        (arr[6], arr[7], arr[8]),
    )
}

pub fn setorthographicprojection(
    (left, right): (f64, f64),
    (bottom, top): (f64, f64),
    (near, far): (f64, f64),
) {
    unsafe { gr_setorthographicprojection(left, right, bottom, top, near, far) }
}

pub fn inqorthographicprojection() -> ((f64, f64), (f64, f64), (f64, f64)) {
    let mut left = MaybeUninit::uninit();
    let mut right = MaybeUninit::uninit();
    let mut bottom = MaybeUninit::uninit();
    let mut top = MaybeUninit::uninit();
    let mut near = MaybeUninit::uninit();
    let mut far = MaybeUninit::uninit();
    let leftp = left.as_mut_ptr();
    let rightp = right.as_mut_ptr();
    let bottomp = bottom.as_mut_ptr();
    let topp = top.as_mut_ptr();
    let nearp = near.as_mut_ptr();
    let farp = far.as_mut_ptr();
    unsafe {
        gr_inqorthographicprojection(leftp, rightp, bottomp, topp, nearp, farp);
        (
            (left.assume_init(), right.assume_init()),
            (bottom.assume_init(), top.assume_init()),
            (near.assume_init(), far.assume_init()),
        )
    }
}

pub fn inqvpsize() -> (c_int, c_int, f64) {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();
    let mut device_pixel_ratio = MaybeUninit::uninit();
    let wp = width.as_mut_ptr();
    let hp = height.as_mut_ptr();
    let dpr = device_pixel_ratio.as_mut_ptr();
    unsafe {
        gr_inqvpsize(wp, hp, dpr);
        (
            width.assume_init(),
            height.assume_init(),
            device_pixel_ratio.assume_init(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VolumeFlags {
    border: c_int,
    max_threads: c_int,
    picture_width: c_int,
    picture_height: c_int,
    approximative_calculation: c_int,
}

pub fn inqvolumeflags() -> VolumeFlags {
    let mut vf = MaybeUninit::<VolumeFlags>::uninit();
    unsafe {
        gr_inqvolumeflags(
            ptr::addr_of_mut!((*vf.as_mut_ptr()).border),
            ptr::addr_of_mut!((*vf.as_mut_ptr()).max_threads),
            ptr::addr_of_mut!((*vf.as_mut_ptr()).picture_width),
            ptr::addr_of_mut!((*vf.as_mut_ptr()).picture_height),
            ptr::addr_of_mut!((*vf.as_mut_ptr()).approximative_calculation),
        );
        vf.assume_init()
    }
}

macro_rules! impl_set_size {
    ($name:ident) => {
        pub fn $name(x: F64Range, y: F64Range) {
            let (xmin, xmax) = x.into();
            let (ymin, ymax) = y.into();
            unsafe { paste!([<gr_$name>])(xmin, xmax, ymin, ymax) }
        }
    };

    ($($n:ident),+ $(,)?) => {
        $(impl_set_size! { $n })+
    };
}

macro_rules! impl_inq_size {
    ($name:ident) => {
        pub fn $name() -> (F64Range, F64Range) {
            let mut xmin = MaybeUninit::uninit();
            let mut xmax = MaybeUninit::uninit();
            let mut ymin = MaybeUninit::uninit();
            let mut ymax = MaybeUninit::uninit();
            let xmin_ptr = xmin.as_mut_ptr();
            let xmax_ptr = xmax.as_mut_ptr();
            let ymin_ptr = ymin.as_mut_ptr();
            let ymax_ptr = ymax.as_mut_ptr();
            unsafe { paste!([<gr_$name>])(xmin_ptr, xmax_ptr, ymin_ptr, ymax_ptr) }
            #[cfg(debug_assertions)]
            {
                let x = unsafe { F64Range::new_unchecked(xmin.assume_init(), xmax.assume_init()) };
                let y = unsafe { F64Range::new_unchecked(ymin.assume_init(), ymax.assume_init()) };
                (x, y)
            }
            #[cfg(not(debug_assertions))]
            {
                let x = F64Range::new(unsafe { xmin.assume_init() }, unsafe { xmax.assume_init() }).unwrap();
                let y = F64Range::new(unsafe { ymin.assume_init() }, unsafe { ymax.assume_init() }).unwrap();
                (x, y)
            }
        }
    };

    ($($n:ident),+ $(,)?) => {
        $(impl_inq_size! { $n })+
    };
}

impl_set_size! { setwindow, setviewport, setwswindow, setwsviewport }
impl_inq_size! { inqwindow, inqviewport, inqbbox }

pub use crate::gks::GksLinetype as GrLinetype;

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
impl_primitive_set_inq! { transparency, f64 }
impl_primitive_set_inq! { scalefactors3d 3f64 }
impl_primitive_set_inq! { perspectiveprojection 3f64 }
impl_primitive_set! { selectclipxform, c_int }
impl_primitive_inq! { inqclipxform, c_int }
impl_primitive_set! { settextfontprec, c_int, c_int }
impl_primitive_set! { setregenflags, c_int }
impl_primitive_set! { setcharexpan, f64 }
impl_primitive_set! { setcharspace, f64 }
impl_primitive_set! { settextpath, c_int }
impl_primitive_set! { selntran, c_int }
impl_primitive_set! { setclip, c_int }
impl_primitive_set! { setarrowstyle, c_int }
impl_primitive_set! { setarrowsize, f64 }
impl_primitive_set! { setwscharheight, f64, f64 }
impl_primitive_set! { setcharup, f64, f64 }
impl_primitive_set! { setcolorrep, c_int, f64, f64, f64 }
impl_primitive_set! { setshadow, f64, f64, f64 }
impl_primitive_set! { setthreadnumber, c_int }
impl_primitive_set! { setpicturesizeforvolume, c_int, c_int }
impl_primitive_set! { setvolumebordercalculation, c_int }
impl_primitive_set! { setapproximativecalculation, c_int }
impl_primitive_inq! { inqscale, c_int }
impl_primitive_function! { inqregenflags() -> c_int }
impl_primitive_function! { precision() -> f64 }
impl_primitive_function! { text_maxsize() -> c_int }
impl_primitive_function! { setspace3d(azimuth: f64, polar: f64, fov: f64, cam: f64) }
impl_primitive_function! { beginselection(index: c_int, type_: c_int) }
impl_primitive_function! { moveselection(x: f64, y: f64) }
impl_primitive_function! { resizeselection(type_: c_int, x: f64, y: f64) }
impl_primitive_function! { endselection() }
impl_primitive_function! { savestate() }
impl_primitive_function! { restorestate() }
impl_primitive_function! { savecontext(context: c_int) }
impl_primitive_function! { selectcontext(context: c_int) }
impl_primitive_function! { destroycontext(context: c_int) }
impl_primitive_function! { unselectcontext() }
impl_primitive_function! { cancelbboxcallback() }
