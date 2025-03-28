use super::GrColorModel;
use crate::gks::{GksColorIndexArray, GksPrimitive};
use crate::gr::util::textx_opts;
use crate::util::f64range::F64Range;
use crate::util::region::Region;
use core::ffi::{c_int, c_uint, CStr};
use core::fmt;
use core::mem::MaybeUninit;
use core::num::TryFromIntError;
use core::ptr;
use gr_sys::gr::*;
use paste::paste;
use std::ffi::CString;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GrError;
type Result<T> = ::core::result::Result<T, GrError>;

impl fmt::Display for GrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid arguments to GR")
    }
}

impl std::error::Error for GrError {}

#[doc(hidden)]
impl From<TryFromIntError> for GrError {
    fn from(_value: TryFromIntError) -> Self {
        GrError
    }
}

pub type GrColorIndexArray<'a> = GksColorIndexArray<'a>;
pub type GrPrimitive = GksPrimitive;
pub type GrVertex = vertex_t;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrSurfaceOption {
    Lines = surface_option_t_GR_OPTION_LINES as _,
    Mesh = surface_option_t_GR_OPTION_MESH as _,
    FilledMesh = surface_option_t_GR_OPTION_FILLED_MESH as _,
    ZShadedMesh = surface_option_t_GR_OPTION_Z_SHADED_MESH as _,
    ColoredMesh = surface_option_t_GR_OPTION_COLORED_MESH as _,
    CellArray = surface_option_t_GR_OPTION_CELL_ARRAY as _,
    ShadedMesh = surface_option_t_GR_OPTION_SHADED_MESH as _,
    Mesh3D = surface_option_t_GR_OPTION_3D_MESH as _,
}

#[derive(Clone, Copy, Debug)]
pub enum GrContourHeights<'a> {
    Default,
    N(usize),
    Custom(&'a [f64]),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrPathCode {
    Stop = path_code_t_GR_STOP as _,
    MoveTo = path_code_t_GR_MOVETO as _,
    LineTo = path_code_t_GR_LINETO as _,
    Curve3 = path_code_t_GR_CURVE3 as _,
    Curve4 = path_code_t_GR_CURVE4 as _,
    ClosePoly = path_code_t_GR_CLOSEPOLY as _,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum XForm {
    Boolean = xform_types_t_GR_XFORM_BOOLEAN as _,
    Linear = xform_types_t_GR_XFORM_LINEAR as _,
    Log = xform_types_t_GR_XFORM_LOG as _,
    LogLog = xform_types_t_GR_XFORM_LOGLOG as _,
    Cubic = xform_types_t_GR_XFORM_CUBIC as _,
    Equalized = xform_types_t_GR_XFORM_EQUALIZED as _,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Interp2Method {
    Nearest = interp2_method_t_GR_INTERP2_NEAREST as _,
    Linear = interp2_method_t_GR_INTERP2_LINEAR as _,
    Spline = interp2_method_t_GR_INTERP2_SPLINE as _,
    Cubic = interp2_method_t_GR_INTERP2_CUBIC as _,
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[rustfmt::skip]
    let   wkid =   wkid.into();
    let wstype = wstype.into();
    let conn = connection.unwrap_or(c"").as_ptr().cast_mut();
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
    let f = textx_opts(world_cooridnates, inline_math);
    unsafe { gr_textx(x, y, p, f) }
}

pub fn textext((x, y): (f64, f64), s: impl AsRef<CStr>) -> Result<()> {
    let p = s.as_ref().as_ptr().cast_mut();
    let r = unsafe { gr_textext(x, y, p) };
    match r {
        0 => Err(GrError),
        _ => Ok(()),
    }
}

// axis has sensible values in range [-4,4], but all are allowed
pub fn text3d((x, y, z): (f64, f64, f64), s: impl AsRef<CStr>, axis: impl Into<c_int>) {
    let p = s.as_ref().as_ptr().cast_mut();
    let axis = axis.into();
    unsafe { gr_text3d(x, y, z, p, axis) }
}

pub fn mathtex((x, y): (f64, f64), s: impl AsRef<CStr>) {
    let p = s.as_ref().as_ptr().cast_mut();
    unsafe { gr_mathtex(x, y, p) }
}

#[allow(clippy::unit_arg)]
pub fn fillarea(n: usize, x: &[f64], y: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    Ok(unsafe { gr_polymarker(n, x, y) })
}

#[allow(clippy::unit_arg)]
pub fn cellarray(
    ((px, py), start): ((f64, f64), (usize, usize)),
    ((qx, qy), end): ((f64, f64), (usize, usize)),
    color_array: GrColorIndexArray,
) -> Result<()> {
    let sx = (start.0 + 1).try_into()?;
    let sy = (start.1 + 1).try_into()?;
    let nx = (end.0 - start.0).try_into()?;
    let ny = (end.1 - start.1).try_into()?;
    let GrColorIndexArray {
        data,
        dimensions: (dx, dy),
        slice: _,
    } = color_array;
    Ok(unsafe { gr_cellarray(px, py, qx, qy, dx, dy, sx, sy, nx, ny, data) })
}

#[allow(clippy::unit_arg)]
pub fn nonuniformcellarray(
    (x, edges_x): (&[f64], bool),
    (y, edges_y): (&[f64], bool),
    start: (usize, usize),
    end: (usize, usize),
    color_array: GrColorIndexArray,
) -> Result<()> {
    let nx = end.0 - start.0;
    let ny = end.1 - start.1;
    check_that(
        (x.len() > nx || !edges_x && x.len() == nx) && (y.len() > ny || !edges_y && y.len() == ny),
    )?;
    let sx = (start.0 + 1).try_into()?;
    let sy = (start.1 + 1).try_into()?;
    let nx = nx.try_into()?;
    let ny = ny.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let GrColorIndexArray {
        data,
        dimensions: (dx, dy),
        slice: _,
    } = color_array;
    let dx = if edges_x { dx } else { -dx };
    let dy = if edges_y { dy } else { -dy };
    Ok(unsafe { gr_nonuniformcellarray(x, y, dx, dy, sx, sy, nx, ny, data) })
}

#[allow(clippy::unit_arg)]
pub fn polarcellarray(
    (x_org, y_org): (f64, f64),
    angle_range: F64Range,
    radius_range: F64Range,
    start: (usize, usize),
    end: (usize, usize),
    color_array: GrColorIndexArray,
) -> Result<()> {
    let sx = (start.0 + 1).try_into()?;
    let sy = (start.1 + 1).try_into()?;
    let nx = (end.0 - start.0).try_into()?;
    let ny = (end.1 - start.1).try_into()?;
    let (phimin, phimax) = angle_range.into();
    let (rmin, rmax) = radius_range.into();
    let GrColorIndexArray {
        data,
        dimensions: (dx, dy),
        slice: _,
    } = color_array;
    Ok(unsafe {
        gr_polarcellarray(
            x_org, y_org, phimin, phimax, rmin, rmax, dx, dy, sx, sy, nx, ny, data,
        )
    })
}

#[allow(clippy::unit_arg)]
pub fn nonuniformpolarcellarray(
    (x_org, y_org): (f64, f64),
    (angles, edges_angles): (&[f64], bool),
    (radii, edges_radii): (&[f64], bool),
    start: (usize, usize),
    end: (usize, usize),
    color_array: GrColorIndexArray,
) -> Result<()> {
    let (x, y) = (angles, radii);
    let nx = end.0 - start.0;
    let ny = end.1 - start.1;
    check_that(x.len() > nx || !edges_angles && x.len() == nx)?;
    check_that(y.len() > ny || !edges_radii && y.len() == ny)?;
    let sx = (start.0 + 1).try_into()?;
    let sy = (start.1 + 1).try_into()?;
    let nx = nx.try_into()?;
    let ny = ny.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let GrColorIndexArray {
        data,
        dimensions: (dx, dy),
        slice: _,
    } = color_array;
    let dx = if edges_angles { dx } else { -dx };
    let dy = if edges_angles { dy } else { -dy };
    Ok(unsafe { gr_nonuniformpolarcellarray(x_org, y_org, x, y, dx, dy, sx, sy, nx, ny, data) })
}

#[allow(clippy::unit_arg)]
pub fn gdp(
    n: usize,
    x: &[f64],
    y: &[f64],
    primitive: GrPrimitive,
    data_records: &[c_int],
) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let prim = primitive as c_int;
    let ldr = data_records.len().try_into()?;
    let dr = data_records.as_ptr().cast_mut();
    Ok(unsafe { gr_gdp(n, x, y, prim, ldr, dr) })
}

#[allow(clippy::unit_arg)]
pub fn spline(n: usize, x: &[f64], y: &[f64], m: usize, method: c_int) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n: c_int = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let m = m.try_into()?;
    Ok(unsafe { gr_spline(n, x, y, m, method) })
}

#[allow(clippy::unit_arg, clippy::too_many_arguments)]
pub fn gridit(
    nd: usize,
    xd: &[f64],
    yd: &[f64],
    zd: &[f64],
    nx: usize,
    ny: usize,
    x: &mut [f64],
    y: &mut [f64],
    z: &mut [f64],
) -> Result<()> {
    check_that(nd <= xd.len() && nd <= yd.len() && nd <= zd.len())?;
    check_that(nx <= x.len() && ny <= y.len() && nx * ny <= z.len())?;
    let nd = nd.try_into()?;
    let nx = nx.try_into()?;
    let ny = ny.try_into()?;
    let xd = xd.as_ptr().cast_mut();
    let yd = yd.as_ptr().cast_mut();
    let zd = zd.as_ptr().cast_mut();
    let x = x.as_mut_ptr();
    let y = y.as_mut_ptr();
    let z = z.as_mut_ptr();
    Ok(unsafe { gr_gridit(nd, xd, yd, zd, nx, ny, x, y, z) })
}

#[allow(clippy::unit_arg)]
pub fn axes(
    tick_interval: (f64, f64),
    origin: (f64, f64),
    major: (Option<c_uint>, Option<c_uint>),
    size: f64,
) -> Result<()> {
    let (x_tick, y_tick) = tick_interval;
    let (x, y) = origin;
    let major_x = major.0.map_or(Ok(-1), TryInto::try_into)?;
    let major_y = major.1.map_or(Ok(-1), TryInto::try_into)?;
    Ok(unsafe { gr_axes(x_tick, y_tick, x, y, major_x, major_y, size) })
}

#[allow(clippy::unit_arg)]
pub fn grid(tick_interval: (f64, f64), origin: (f64, f64), major: (c_uint, c_uint)) -> Result<()> {
    let (x_tick, y_tick) = tick_interval;
    let (x, y) = origin;
    let major_x = major.0.try_into()?;
    let major_y = major.1.try_into()?;
    Ok(unsafe { gr_grid(x_tick, y_tick, x, y, major_x, major_y) })
}

#[allow(clippy::unit_arg)]
pub fn grid3d(
    tick_interval: (f64, f64, f64),
    origin: (f64, f64, f64),
    major: (c_uint, c_uint, c_uint),
) -> Result<()> {
    let (x_tick, y_tick, z_tick) = tick_interval;
    let (x, y, z) = origin;
    let major_x = major.0.try_into()?;
    let major_y = major.1.try_into()?;
    let major_z = major.2.try_into()?;
    Ok(unsafe { gr_grid3d(x_tick, y_tick, z_tick, x, y, z, major_x, major_y, major_z) })
}

#[allow(clippy::unit_arg)]
pub fn verrorbars(n: usize, x: &[f64], y: &[f64], lo: &[f64], hi: &[f64]) -> Result<()> {
    #[rustfmt::skip]
    check_that(n <=  x.len() && n <=  y.len())?;
    check_that(n <= lo.len() && n <= hi.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let lo = lo.as_ptr().cast_mut();
    let hi = hi.as_ptr().cast_mut();
    Ok(unsafe { gr_verrorbars(n, x, y, lo, hi) })
}

#[allow(clippy::unit_arg)]
pub fn herrorbars(n: usize, x: &[f64], y: &[f64], lo: &[f64], hi: &[f64]) -> Result<()> {
    #[rustfmt::skip]
    check_that(n <=  x.len() && n <=  y.len())?;
    check_that(n <= lo.len() && n <= hi.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let lo = lo.as_ptr().cast_mut();
    let hi = hi.as_ptr().cast_mut();
    Ok(unsafe { gr_herrorbars(n, x, y, lo, hi) })
}

#[allow(clippy::unit_arg)]
pub fn polyline3d(n: usize, x: &[f64], y: &[f64], z: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len() && n <= z.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    Ok(unsafe { gr_polyline3d(n, x, y, z) })
}

#[allow(clippy::unit_arg)]
pub fn polymarker3d(n: usize, x: &[f64], y: &[f64], z: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len() && n <= z.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    Ok(unsafe { gr_polymarker3d(n, x, y, z) })
}

#[allow(clippy::unit_arg)]
pub fn axes3d(
    tick_interval: (f64, f64, f64),
    origin: (f64, f64, f64),
    major: (c_uint, c_uint, c_uint),
    size: f64,
) -> Result<()> {
    let (x_tick, y_tick, z_tick) = tick_interval;
    let (x, y, z) = origin;
    let major_x = major.0.try_into()?;
    let major_y = major.1.try_into()?;
    let major_z = major.2.try_into()?;
    Ok(unsafe {
        gr_axes3d(
            x_tick, y_tick, z_tick, x, y, z, major_x, major_y, major_z, size,
        )
    })
}

#[allow(clippy::unit_arg)]
pub fn titles3d(x: impl AsRef<CStr>, y: impl AsRef<CStr>, z: impl AsRef<CStr>) {
    let x = x.as_ref().as_ptr().cast_mut();
    let y = y.as_ref().as_ptr().cast_mut();
    let z = z.as_ref().as_ptr().cast_mut();
    unsafe { gr_titles3d(x, y, z) }
}

#[allow(clippy::unit_arg)]
pub fn surface(x: &[f64], y: &[f64], z: &[f64], option: GrSurfaceOption) -> Result<()> {
    check_that(x.len() * y.len() <= z.len())?;
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    let opt = option as _;
    Ok(unsafe { gr_surface(nx, ny, x, y, z, opt) })
}

#[allow(clippy::unit_arg)]
pub fn contour(
    x: &[f64],
    y: &[f64],
    h: Option<&[f64]>,
    z: &[f64],
    major_h: usize,
    color: bool,
) -> Result<()> {
    check_that(x.len() * y.len() <= z.len())?;
    check_that(major_h < 1000)?;
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let nh = h
        .map(|h| h.len().try_into())
        .transpose()?
        .unwrap_or_default();
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let h = h.map_or(ptr::null(), |h| h.as_ptr()).cast_mut();
    let z = z.as_ptr().cast_mut();
    let major_h = major_h.try_into()?; // optimizer, pls see the check above ;)
    let major_h = if color { major_h + 1000 } else { major_h };
    Ok(unsafe { gr_contour(nx, ny, nh, x, y, h, z, major_h) })
}

#[allow(clippy::unit_arg)]
pub fn contourf(
    x: &[f64],
    y: &[f64],
    h: GrContourHeights,
    z: &[f64],
    major_h: Option<usize>,
    color: bool, // doesn't actually do anything
) -> Result<()> {
    check_that(x.len() * y.len() <= z.len())?;
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    let (nh, h) = match h {
        GrContourHeights::Default => (0, ptr::null_mut()),
        GrContourHeights::N(nh) => (nh.try_into()?, ptr::null_mut()),
        GrContourHeights::Custom(h) => (h.len().try_into()?, h.as_ptr().cast_mut()),
    };
    let major_h = major_h.map(TryInto::try_into).transpose()?.unwrap_or(-1);
    check_that(major_h < 1000)?;
    let major_h = if color { major_h + 1000 } else { major_h };
    Ok(unsafe { gr_contourf(nx, ny, nh, x, y, h, z, major_h) })
}

#[allow(clippy::unit_arg)]
pub fn tricontour(n: usize, x: &[f64], y: &[f64], z: &[f64], levels: &[f64]) -> Result<()> {
    check_that(n <= x.len() && n <= y.len() && n <= z.len())?;
    let n = n.try_into()?;
    let ln = levels.len().try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    let l = levels.as_ptr().cast_mut();
    Ok(unsafe { gr_tricontour(n, x, y, z, ln, l) })
}

pub fn hexbin(n: usize, x: &[f64], y: &[f64], nbins: usize) -> Result<c_int> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let nbins = nbins.try_into()?;
    Ok(unsafe { gr_hexbin(n, x, y, nbins) })
}

pub fn colorbar() {
    unsafe { gr_colorbar() }
}

pub fn drawrect(x: (f64, f64), y: (f64, f64)) {
    unsafe { gr_drawrect(x.0, x.1, y.0, y.1) }
}

pub fn fillrect(x: (f64, f64), y: (f64, f64)) {
    unsafe { gr_fillrect(x.0, x.1, y.0, y.1) }
}

pub fn drawarc(x: (f64, f64), y: (f64, f64), angle: (f64, f64)) {
    unsafe { gr_drawarc(x.0, x.1, y.0, y.1, angle.0, angle.1) }
}

pub fn fillarc(x: (f64, f64), y: (f64, f64), angle: (f64, f64)) {
    unsafe { gr_fillarc(x.0, x.1, y.0, y.1, angle.0, angle.1) }
}

#[allow(clippy::unit_arg)]
pub fn drawpath(n: usize, verts: &[GrVertex], codes: &[GrPathCode], fill: bool) -> Result<()> {
    check_that(n <= verts.len() && n <= codes.len())?;
    let n = n.try_into()?;
    let v = verts.as_ptr().cast_mut();
    let c = codes.as_ptr().cast_mut().cast();
    let f = fill.into();
    Ok(unsafe { gr_drawpath(n, v, c, f) })
}

pub fn drawarrow(tail: (f64, f64), head: (f64, f64)) {
    unsafe { gr_drawarrow(tail.0, tail.1, head.0, head.1) }
}

#[allow(clippy::unit_arg)]
pub fn drawimage(
    x: (f64, f64),
    y: (f64, f64),
    (width, height, data): (usize, usize, &[c_int]),
    model: GrColorModel,
) -> Result<()> {
    check_that(width * height <= data.len())?;
    let w = width.try_into()?;
    let h = height.try_into()?;
    let d = data.as_ptr().cast_mut();
    let m = model as _;
    Ok(unsafe { gr_drawimage(x.0, x.1, y.0, y.1, w, h, d, m) })
}

pub fn importgraphics(path: impl AsRef<CStr>) -> bool {
    let p = path.as_ref().as_ptr().cast_mut();
    -1 != unsafe { gr_importgraphics(p) }
}

pub fn drawgraphics(xml_string: impl AsRef<CStr>) {
    let s = xml_string.as_ref().as_ptr().cast_mut();
    unsafe {
        gr_drawgraphics(s);
    }
}

pub fn begingraphics(path: impl AsRef<CStr>) {
    let p = path.as_ref().as_ptr().cast_mut();
    unsafe { gr_begingraphics(p) }
}

pub fn endgraphics() {
    unsafe { gr_endgraphics() }
}

pub fn getgraphics() -> CString {
    // copy cause GR's buffer might get reallocated
    unsafe { CString::from_raw(gr_getgraphics()) }
}

#[allow(clippy::unit_arg)]
pub fn trisurface(n: usize, x: &[f64], y: &[f64], z: &[f64]) -> Result<()> {
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let z = z.as_ptr().cast_mut();
    Ok(unsafe { gr_trisurface(n, x, y, z) })
}

#[allow(clippy::unit_arg)]
pub fn quiver(x: &[f64], y: &[f64], u: &mut [f64], v: &mut [f64], color: bool) -> Result<()> {
    let n = x.len() * y.len();
    check_that(n <= u.len() && n <= v.len())?;
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let u = u.as_ptr().cast_mut();
    let v = v.as_ptr().cast_mut();
    Ok(unsafe { gr_quiver(nx, ny, x, y, u, v, color as _) })
}

#[allow(clippy::too_many_arguments, clippy::unit_arg)]
pub fn shade<'a>(
    n: usize,
    x: &[f64],
    y: &[f64],
    lines: bool,
    xform: XForm,
    roi: Region,
    (width, height): (usize, usize),
    bins: &'a mut [MaybeUninit<c_int>],
) -> Result<&'a mut [c_int]> {
    check_that(n <= x.len() && n <= y.len() && width * height <= bins.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let roi = roi.as_ref().as_ptr().cast_mut();
    let w = width.try_into()?;
    let h = height.try_into()?;
    let bins_ptr = bins.as_mut_ptr().cast();
    unsafe { gr_shade(n, x, y, lines as _, xform as _, roi, w, h, bins_ptr) }
    Ok(unsafe { std::mem::transmute(bins) })
}

pub fn shade_alloc(
    n: usize,
    x: &[f64],
    y: &[f64],
    lines: bool,
    xform: XForm,
    roi: Region,
    (width, height): (usize, usize),
) -> Result<Vec<c_int>> {
    let mut bins = vec![];
    bins.resize(width * height, MaybeUninit::uninit());
    shade(n, x, y, lines, xform, roi, (width, height), bins.as_mut())?;
    Ok(unsafe { std::mem::transmute(bins) })
    // #![feature(vec_into_raw_parts)]
    // let (ptr, len, cap) = bins.into_raw_parts();
    // Ok(unsafe { Vec::from_raw_parts(ptr.cast(), len, cap) })
}

macro_rules! impl_shade_fn {
    ($name:ident) => {
        #[allow(clippy::unit_arg)]
        pub fn $name(n: usize, x: &[f64], y: &[f64], xform: XForm, w: usize, h: usize) -> Result<()> {
            check_that(n <= x.len() && n <= y.len())?;
            let n = n.try_into()?;
            let x = x.as_ptr().cast_mut();
            let y = y.as_ptr().cast_mut();
            let w = w.try_into()?;
            let h = h.try_into()?;
            Ok(unsafe { paste!([<gr_$name>])(n, x, y, xform as _, w, h) })
        }
    };
}

impl_shade_fn! { shadepoints }
impl_shade_fn! { shadelines }

#[allow(clippy::unit_arg)]
pub fn path(n: usize, x: &[f64], y: &[f64], codes: impl AsRef<CStr>) -> Result<()> {
    check_that(n <= x.len() && n <= y.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr().cast_mut();
    let y = y.as_ptr().cast_mut();
    let codes = codes.as_ref().as_ptr();
    Ok(unsafe { gr_path(n, x, y, codes) })
}

#[allow(clippy::too_many_arguments, clippy::unit_arg)]
pub fn interp2<'a>(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    target_x: &[f64],
    target_y: &[f64],
    zout: &'a mut [MaybeUninit<f64>],
    method: Interp2Method,
    extrapolation_value: f64,
) -> Result<&'a mut [f64]> {
    check_that(x.len() * y.len() <= z.len())?;
    check_that(target_x.len() * target_y.len() <= zout.len())?;
    let nx = x.len().try_into()?;
    let ny = y.len().try_into()?;
    let x = x.as_ptr();
    let y = y.as_ptr();
    let z = z.as_ptr();
    let tnx = target_x.len().try_into()?;
    let tny = target_y.len().try_into()?;
    let tx = target_x.as_ptr();
    let ty = target_y.as_ptr();
    let zout_ptr = zout.as_mut_ptr().cast();
    unsafe {
        gr_interp2(
            nx,
            ny,
            x,
            y,
            z,
            tnx,
            tny,
            tx,
            ty,
            zout_ptr,
            method as _,
            extrapolation_value,
        )
    }
    Ok(unsafe { std::mem::transmute(zout) })
}

pub fn interp2_alloc(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    target_x: &[f64],
    target_y: &[f64],
    method: Interp2Method,
    extrapolation_value: f64,
) -> Result<Vec<f64>> {
    let mut zout = Vec::with_capacity(x.len() * y.len());
    zout.resize(x.len() * y.len(), MaybeUninit::uninit());
    interp2(
        x,
        y,
        z,
        target_x,
        target_y,
        &mut zout,
        method,
        extrapolation_value,
    )?;
    Ok(unsafe { std::mem::transmute(zout) })
    // #![feature(vec_into_raw_parts)]
    // let (ptr, len, cap) = zout.into_raw_parts();
    // Ok(unsafe { Vec::from_raw_parts(ptr.cast(), len, cap) })
}

#[allow(clippy::unit_arg)]
pub fn polygonmesh3d(
    n: usize,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    connections: &[c_int],
    colors: &[c_int],
) -> Result<()> {
    check_that(n <= x.len() && n <= y.len() && n <= z.len())?;
    check_that(connections.len() <= colors.len())?;
    let n = n.try_into()?;
    let x = x.as_ptr();
    let y = y.as_ptr();
    let z = z.as_ptr();
    let nc = connections.len().try_into()?;
    let conn = connections.as_ptr();
    let col = colors.as_ptr();
    Ok(unsafe { gr_polygonmesh3d(n, x, y, z, nc, conn, col) })
}
