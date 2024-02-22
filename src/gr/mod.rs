use crate::ffi::gr::{
    gr_activatews, gr_cellarray, gr_clearws, gr_closegks, gr_closews, gr_configurews,
    gr_deactivatews, gr_debug, gr_gdp, gr_gridit, gr_initgr, gr_inqdspsize, gr_nonuniformcellarray,
    gr_nonuniformpolarcellarray, gr_opengks, gr_openws, gr_polarcellarray, gr_polyline,
    gr_polymarker, gr_spline, gr_text, gr_textx, gr_updatews, GR_TEXT_ENABLE_INLINE_MATH,
    GR_TEXT_USE_WC,
};
use crate::util::f64range::F64Range;
use core::ffi::{c_int, CStr};
use core::fmt;
use core::mem::MaybeUninit;
use core::num::TryFromIntError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GrError;
type Result<T> = ::core::result::Result<T, GrError>;

impl fmt::Display for GrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid arguments to GR")
    }
}

impl std::error::Error for GrError {}

impl From<TryFromIntError> for GrError {
    fn from(_value: TryFromIntError) -> Self {
        GrError
    }
}

pub type GrColorIndexArray<'a> = super::gks::out::GksColorIndexArray<'a>;
pub type GrPrimitive = super::gks::out::GksPrimitive;

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

#[derive(Clone, Copy, Debug)]
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
    let f = match (world_cooridnates, inline_math) {
        (true, true) => GR_TEXT_USE_WC | GR_TEXT_ENABLE_INLINE_MATH,
        (true, false) => GR_TEXT_USE_WC,
        (false, true) => GR_TEXT_ENABLE_INLINE_MATH,
        (false, false) => 0,
    };
    unsafe { gr_textx(x, y, p, f) }
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
