use super::util::impl_each;
use super::{ActiveGks, Gks, SegmentGks};
use crate::ffi::gks::{
    gks_cellarray, gks_fillarea, gks_ft_gdp, gks_gdp, gks_polyline, gks_polymarker,
    gks_set_pline_color_index, gks_set_pline_linetype, gks_set_pline_linewidth,
    gks_set_pmark_color_index, gks_set_pmark_size, gks_set_pmark_type, gks_set_resize_behaviour,
    gks_set_text_color_index, gks_set_text_expfac, gks_set_text_fontprec, gks_set_text_height,
    gks_set_text_spacing, gks_set_transparency, gks_set_viewport, gks_set_window, gks_text,
    GKS_K_GDP_DRAW_LINES, GKS_K_GDP_DRAW_MARKERS, GKS_K_GDP_DRAW_PATH, GKS_K_GDP_DRAW_TRIANGLES,
    GKS_K_GDP_FILL_POLYGONS, GKS_K_LINETYPE_DASHED, GKS_K_LINETYPE_DASHED_DOTTED,
    GKS_K_LINETYPE_DASH_2_DOT, GKS_K_LINETYPE_DASH_3_DOT, GKS_K_LINETYPE_DOTTED,
    GKS_K_LINETYPE_DOUBLE_DOT, GKS_K_LINETYPE_LONG_DASH, GKS_K_LINETYPE_LONG_SHORT_DASH,
    GKS_K_LINETYPE_SOLID, GKS_K_LINETYPE_SPACED_DASH, GKS_K_LINETYPE_SPACED_DOT,
    GKS_K_LINETYPE_TRIPLE_DOT, GKS_K_MARKERTYPE_ASTERISK, GKS_K_MARKERTYPE_BOWTIE,
    GKS_K_MARKERTYPE_CIRCLE, GKS_K_MARKERTYPE_DIAGONAL_CROSS, GKS_K_MARKERTYPE_DIAMOND,
    GKS_K_MARKERTYPE_DOT, GKS_K_MARKERTYPE_HEPTAGON, GKS_K_MARKERTYPE_HEXAGON,
    GKS_K_MARKERTYPE_HLINE, GKS_K_MARKERTYPE_HOLLOW_PLUS, GKS_K_MARKERTYPE_HOURGLASS,
    GKS_K_MARKERTYPE_OCTAGON, GKS_K_MARKERTYPE_OMARK, GKS_K_MARKERTYPE_PENTAGON,
    GKS_K_MARKERTYPE_PLUS, GKS_K_MARKERTYPE_SOLID_BOWTIE, GKS_K_MARKERTYPE_SOLID_CIRCLE,
    GKS_K_MARKERTYPE_SOLID_DIAMOND, GKS_K_MARKERTYPE_SOLID_HGLASS, GKS_K_MARKERTYPE_SOLID_PLUS,
    GKS_K_MARKERTYPE_SOLID_SQUARE, GKS_K_MARKERTYPE_SOLID_STAR, GKS_K_MARKERTYPE_SOLID_TRI_DOWN,
    GKS_K_MARKERTYPE_SOLID_TRI_LEFT, GKS_K_MARKERTYPE_SOLID_TRI_RIGHT,
    GKS_K_MARKERTYPE_SOLID_TRI_UP, GKS_K_MARKERTYPE_SQUARE, GKS_K_MARKERTYPE_STAR,
    GKS_K_MARKERTYPE_STAR_4, GKS_K_MARKERTYPE_STAR_5, GKS_K_MARKERTYPE_STAR_6,
    GKS_K_MARKERTYPE_STAR_7, GKS_K_MARKERTYPE_STAR_8, GKS_K_MARKERTYPE_TRIANGLE_DOWN,
    GKS_K_MARKERTYPE_TRIANGLE_UP, GKS_K_MARKERTYPE_TRI_UP_DOWN, GKS_K_MARKERTYPE_VLINE,
};
use crate::ffi::gkscore::MAX_COLOR;
use crate::util::f64range::F64Range;
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
    pub(crate) data: *mut c_int,
    pub(crate) dimensions: (c_int, c_int),
    pub(crate) slice: PhantomData<&'a [c_int]>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GksLinetype {
    Solid = GKS_K_LINETYPE_SOLID as _,
    Dashed = GKS_K_LINETYPE_DASHED as _,
    Dotted = GKS_K_LINETYPE_DOTTED as _,
    DashedDotted = GKS_K_LINETYPE_DASHED_DOTTED as _,
    Dash2Dot = GKS_K_LINETYPE_DASH_2_DOT as _,
    Dash3Dot = GKS_K_LINETYPE_DASH_3_DOT as _,
    LongDash = GKS_K_LINETYPE_LONG_DASH as _,
    LongShortDash = GKS_K_LINETYPE_LONG_SHORT_DASH as _,
    SpacedDash = GKS_K_LINETYPE_SPACED_DASH as _,
    SpacedDot = GKS_K_LINETYPE_SPACED_DOT as _,
    DoubleDot = GKS_K_LINETYPE_DOUBLE_DOT as _,
    TripleDot = GKS_K_LINETYPE_TRIPLE_DOT as _,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GksMarkertype {
    Dot = GKS_K_MARKERTYPE_DOT as _,
    Plus = GKS_K_MARKERTYPE_PLUS as _,
    Asterisk = GKS_K_MARKERTYPE_ASTERISK as _,
    Circle = GKS_K_MARKERTYPE_CIRCLE as _,
    DiagonalCross = GKS_K_MARKERTYPE_DIAGONAL_CROSS as _,
    SolidCircle = GKS_K_MARKERTYPE_SOLID_CIRCLE as _,
    TriangleUp = GKS_K_MARKERTYPE_TRIANGLE_UP as _,
    SolidTriUp = GKS_K_MARKERTYPE_SOLID_TRI_UP as _,
    TriangleDown = GKS_K_MARKERTYPE_TRIANGLE_DOWN as _,
    SolidTriDown = GKS_K_MARKERTYPE_SOLID_TRI_DOWN as _,
    Square = GKS_K_MARKERTYPE_SQUARE as _,
    SolidSquare = GKS_K_MARKERTYPE_SOLID_SQUARE as _,
    Bowtie = GKS_K_MARKERTYPE_BOWTIE as _,
    SolidBowtie = GKS_K_MARKERTYPE_SOLID_BOWTIE as _,
    Hourglass = GKS_K_MARKERTYPE_HOURGLASS as _,
    SolidHglass = GKS_K_MARKERTYPE_SOLID_HGLASS as _,
    Diamond = GKS_K_MARKERTYPE_DIAMOND as _,
    SolidDiamond = GKS_K_MARKERTYPE_SOLID_DIAMOND as _,
    Star = GKS_K_MARKERTYPE_STAR as _,
    SolidStar = GKS_K_MARKERTYPE_SOLID_STAR as _,
    TriUpDown = GKS_K_MARKERTYPE_TRI_UP_DOWN as _,
    SolidTriRight = GKS_K_MARKERTYPE_SOLID_TRI_RIGHT as _,
    SolidTriLeft = GKS_K_MARKERTYPE_SOLID_TRI_LEFT as _,
    HollowPlus = GKS_K_MARKERTYPE_HOLLOW_PLUS as _,
    SolidPlus = GKS_K_MARKERTYPE_SOLID_PLUS as _,
    Pentagon = GKS_K_MARKERTYPE_PENTAGON as _,
    Hexagon = GKS_K_MARKERTYPE_HEXAGON as _,
    Heptagon = GKS_K_MARKERTYPE_HEPTAGON as _,
    Octagon = GKS_K_MARKERTYPE_OCTAGON as _,
    Star4 = GKS_K_MARKERTYPE_STAR_4 as _,
    Star5 = GKS_K_MARKERTYPE_STAR_5 as _,
    Star6 = GKS_K_MARKERTYPE_STAR_6 as _,
    Star7 = GKS_K_MARKERTYPE_STAR_7 as _,
    Star8 = GKS_K_MARKERTYPE_STAR_8 as _,
    Vline = GKS_K_MARKERTYPE_VLINE as _,
    Hline = GKS_K_MARKERTYPE_HLINE as _,
    Omark = GKS_K_MARKERTYPE_OMARK as _,
}

fn check_that(cond: bool) -> Result<()> {
    match cond {
        true => Ok(()),
        false => Err(Default::default()),
    }
}

impl_each! {(Gks, ActiveGks, SegmentGks) {
    pub fn set_transparency(alpha: f64) {
        unsafe { gks_set_transparency(alpha) }
    }

    pub fn set_window(&mut self, tnr: impl Into<c_int>, x: F64Range, y: F64Range) {
        let tnr = tnr.into();
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_window(tnr, xmin, xmax, ymin, ymax) }
    }

    pub fn set_viewport(&mut self, tnr: impl Into<c_int>, x: F64Range, y: F64Range) {
        let tnr = tnr.into();
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_viewport(tnr, xmin, xmax, ymin, ymax) }
    }

    pub fn set_resize_behavior(&mut self, resize: bool) {
        let flag = resize.into();
        unsafe { gks_set_resize_behaviour(flag) }
    }

    pub fn set_polyline_type(&mut self, ltype: impl Into<c_int>) {
        let ltype = ltype.into();
        unsafe { gks_set_pline_linetype(ltype) }
    }

    pub fn set_polyline_width(&mut self, width: f64) {
        unsafe { gks_set_pline_linewidth(width) }
    }

    pub fn set_polyline_color_index(&mut self, coli: impl Into<c_int>) {
        let coli = coli.into();
        unsafe { gks_set_pline_color_index(coli) }
    }

    pub fn set_polymark_type(&mut self, mtype: impl Into<c_int>) {
        let mtype = mtype.into();
        unsafe { gks_set_pmark_type(mtype) }
    }

    pub fn set_polymark_size(&mut self, size: f64) {
        unsafe { gks_set_pmark_size(size) }
    }

    pub fn set_polymark_color_index(&mut self, coli: impl Into<c_int>) {
        let coli = coli.into();
        unsafe { gks_set_pmark_color_index(coli) }
    }

    pub fn set_text_fontprec(&mut self, font: impl Into<c_int>, prec: impl Into<c_int>) {
        let font = font.into();
        let prec = prec.into();
        unsafe { gks_set_text_fontprec(font, prec) }
    }

    pub fn set_text_expansion_factor(&mut self, expfac: f64) {
        unsafe { gks_set_text_expfac(expfac) }
    }

    pub fn set_text_spacing(&mut self, spacing: f64) {
        unsafe { gks_set_text_spacing(spacing) }
    }

    pub fn set_text_color_index(&mut self, coli: impl Into<c_int>) {
        let coli = coli.into();
        unsafe { gks_set_text_color_index(coli) }
    }

    pub fn set_text_height(&mut self, height: f64) {
        unsafe { gks_set_text_height(height) }
    }
}}

impl_each! {(ActiveGks, SegmentGks) {
    #[allow(clippy::unit_arg)]
    pub fn polyline(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        Ok(unsafe { gks_polyline(n, x, y) })
    }

    #[allow(clippy::unit_arg)]
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

    #[allow(clippy::unit_arg)]
    pub fn fillarea(&mut self, n: usize, x: &[f64], y: &[f64]) -> Result<()> {
        check_that(n <= x.len() && n <= y.len())?;
        let n = n.try_into()?;
        let x = x.as_ptr().cast_mut();
        let y = y.as_ptr().cast_mut();
        Ok(unsafe { gks_fillarea(n, x, y) })
    }

    #[allow(clippy::unit_arg)]
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
}}

macro_rules! impl_gdp {
    ($f:ident, $cf:ident) => {
        #[allow(clippy::unit_arg)]
        pub fn $f(
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
            Ok(unsafe { $cf(n, x, y, prim, ldr, dr) })
        }
    };
}

impl_each! {(ActiveGks, SegmentGks) {
    impl_gdp! { gdp, gks_gdp }
    impl_gdp! { ft_gdp, gks_ft_gdp }
}}

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

    pub unsafe fn new_unchecked(
        data: *const c_int,
        columns: impl Into<c_int>,
        rows: impl Into<c_int>,
    ) -> Self {
        Self {
            data: data.cast_mut(),
            dimensions: (columns.into(), rows.into()),
            slice: PhantomData,
        }
    }
}

impl fmt::Display for GksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("error in GKS")
    }
}

impl std::error::Error for GksError {}

impl From<TryFromIntError> for GksError {
    fn from(_value: TryFromIntError) -> Self {
        GksError
    }
}

impl From<GksLinetype> for c_int {
    fn from(value: GksLinetype) -> Self {
        value as Self
    }
}

impl From<GksMarkertype> for c_int {
    fn from(value: GksMarkertype) -> Self {
        value as Self
    }
}
