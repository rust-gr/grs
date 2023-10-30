use super::ActiveGks;
use crate::ffi::gks::{
    gks_cellarray, gks_fillarea, gks_gdp, gks_polyline, gks_polymarker, gks_text,
    GKS_K_GDP_DRAW_LINES, GKS_K_GDP_DRAW_MARKERS, GKS_K_GDP_DRAW_PATH, GKS_K_GDP_DRAW_TRIANGLES,
    GKS_K_GDP_FILL_POLYGONS,
};
use crate::ffi::gkscore::MAX_COLOR;
use ::core::ffi::{c_int, CStr};
use ::core::fmt;
use ::core::marker::PhantomData;
use ::core::num::{NonZeroUsize, TryFromIntError};

pub const MAX_COLOR_INDEX: usize = MAX_COLOR as _;

pub enum GksPrimitive {
    Path = GKS_K_GDP_DRAW_PATH as _,
    Lines = GKS_K_GDP_DRAW_LINES as _,
    Markers = GKS_K_GDP_DRAW_MARKERS as _,
    Triangles = GKS_K_GDP_DRAW_TRIANGLES as _,
    Polygons = GKS_K_GDP_FILL_POLYGONS as _,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GksError;
type Result<T> = ::core::result::Result<T, GksError>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GksColorIndexArray<'a> {
    data: *mut c_int,
    dimensions: (c_int, c_int),
    slice: PhantomData<&'a [c_int]>,
}

fn check_that(cond: bool) -> Result<()> {
    match cond {
        true => Ok(()),
        false => Err(Default::default()),
    }
}

impl ActiveGks {
    pub fn polyline(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        Ok(unsafe { gks_polyline(n, x, y) })
    }

    pub fn polymarker(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        Ok(unsafe { gks_polymarker(n, x, y) })
    }

    pub fn text(&mut self, (x, y): (f64, f64), s: impl AsRef<CStr>) {
        let p = s.as_ref().as_ptr().cast_mut();
        unsafe { gks_text(x, y, p) }
    }

    pub fn fillarea(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        Ok(unsafe { gks_fillarea(n, x, y) })
    }

    pub fn cellarray(
        &mut self,
        ((px, py), start): ((f64, f64), (usize, usize)),
        ((qx, qy), end): ((f64, f64), (usize, usize)),
        color_array: GksColorIndexArray,
    ) -> Result<()> {
        let sx = (start.0 + 1).try_into()?;
        let sy = (start.1 + 1).try_into()?;
        let nx = (end.0 - start.0).try_into()?;
        let ny = (end.1 - start.1).try_into()?;
        let GksColorIndexArray {
            data,
            dimensions: (dx, dy),
            slice: _,
        } = color_array;
        Ok(unsafe { gks_cellarray(px, py, qx, qy, dx, dy, sx, sy, nx, ny, data) })
    }

    pub fn gdp(
        &mut self,
        n: usize,
        x: &[f64],
        y: &[f64],
        primitive: GksPrimitive,
        data_records: &[c_int],
    ) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        let prim = primitive as c_int;
        let ldr = data_records.len().try_into()?;
        let dr = data_records.as_ptr().cast_mut();
        Ok(unsafe { gks_gdp(n, x, y, prim, ldr, dr) })
    }
}

impl<'a> GksColorIndexArray<'a> {
    pub fn with_stride(data: &'a [c_int], stride: NonZeroUsize) -> Result<Self> {
        let rows = data.len() / stride;
        let stride = stride.get().try_into()?;
        let rows = rows.try_into()?;
        check_that(rows != 0)?;
        Ok(Self {
            data: data.as_ptr().cast_mut(),
            dimensions: (stride, rows),
            slice: PhantomData,
        })
    }

    pub unsafe fn new_unchecked(data: *const c_int, columns: c_int, rows: c_int) -> Self {
        Self {
            data: data.cast_mut(),
            dimensions: (columns, rows),
            slice: PhantomData,
        }
    }
}

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
