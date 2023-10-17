/* automatically generated by rust-bindgen 0.68.1 */

pub const GKS_K_CONID_DEFAULT: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
pub unsafe fn gsetlinecolorind(x: Gint) -> ::core::ffi::c_int {
    gsetlinecolourind(x)
}
pub unsafe fn gsetmarkercolorind(x: Gint) -> ::core::ffi::c_int {
    gsetmarkercolourind(x)
}
pub unsafe fn gsettextcolorind(x: Gint) -> ::core::ffi::c_int {
    gsettextcolourind(x)
}
pub unsafe fn gsetfillcolorind(x: Gint) -> ::core::ffi::c_int {
    gsetfillcolourind(x)
}
pub unsafe fn gsetcolorrep(x: Gint, y: Gint, p: *mut Gcobundl) -> ::core::ffi::c_int {
    gsetcolourrep(x, y, p)
}
pub unsafe fn ginqlinecolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int {
    ginqlinecolourind(a, b)
}
pub unsafe fn ginqtextcolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int {
    ginqtextcolourind(a, b)
}
pub unsafe fn ginqmarkercolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int {
    ginqmarkercolourind(a, b)
}


pub const GKS_K_WSTYPE_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_WSTYPE_MO: ::core::ffi::c_int = 2;
pub const GKS_K_WSTYPE_MI: ::core::ffi::c_int = 3;
pub const GKS_K_WSTYPE_WISS: ::core::ffi::c_int = 5;
pub const GKS_K_ASF_BUNDLED: ::core::ffi::c_int = 0;
pub const GKS_K_ASF_INDIVIDUAL: ::core::ffi::c_int = 1;
pub const GKS_K_CLEAR_CONDITIONALLY: ::core::ffi::c_int = 0;
pub const GKS_K_CLEAR_ALWAYS: ::core::ffi::c_int = 1;
pub const GKS_K_NOCLIP: ::core::ffi::c_int = 0;
pub const GKS_K_CLIP: ::core::ffi::c_int = 1;
pub const GKS_K_COORDINATES_WC: ::core::ffi::c_int = 0;
pub const GKS_K_COORDINATES_NDC: ::core::ffi::c_int = 1;
pub const GKS_K_METERS: ::core::ffi::c_int = 0;
pub const GKS_K_OTHER_UNITS: ::core::ffi::c_int = 1;
pub const GKS_K_INTSTYLE_HOLLOW: ::core::ffi::c_int = 0;
pub const GKS_K_INTSTYLE_SOLID: ::core::ffi::c_int = 1;
pub const GKS_K_INTSTYLE_PATTERN: ::core::ffi::c_int = 2;
pub const GKS_K_INTSTYLE_HATCH: ::core::ffi::c_int = 3;
pub const GKS_K_INTSTYLE_SOLID_WITH_BORDER: ::core::ffi::c_int = 4;
pub const GKS_K_STATUS_NONE: ::core::ffi::c_int = 0;
pub const GKS_K_STATUS_OK: ::core::ffi::c_int = 1;
pub const GKS_K_LEVEL_0A: ::core::ffi::c_int = 0;
pub const GKS_K_LEVEL_0B: ::core::ffi::c_int = 1;
pub const GKS_K_LEVEL_0C: ::core::ffi::c_int = 2;
pub const GKS_K_LEVEL_1A: ::core::ffi::c_int = 3;
pub const GKS_K_LEVEL_1B: ::core::ffi::c_int = 4;
pub const GKS_K_LEVEL_1C: ::core::ffi::c_int = 5;
pub const GKS_K_LEVEL_2A: ::core::ffi::c_int = 6;
pub const GKS_K_LEVEL_2B: ::core::ffi::c_int = 7;
pub const GKS_K_LEVEL_2C: ::core::ffi::c_int = 8;
pub const GKS_K_GKCL: ::core::ffi::c_int = 0;
pub const GKS_K_GKOP: ::core::ffi::c_int = 1;
pub const GKS_K_WSOP: ::core::ffi::c_int = 2;
pub const GKS_K_WSAC: ::core::ffi::c_int = 3;
pub const GKS_K_SGOP: ::core::ffi::c_int = 4;
pub const GKS_K_POSTPONE_FLAG: ::core::ffi::c_int = 0;
pub const GKS_K_PERFORM_FLAG: ::core::ffi::c_int = 1;
pub const GKS_K_WRITE_PAGE_FLAG: ::core::ffi::c_int = 2;
pub const GKS_K_TEXT_HALIGN_NORMAL: ::core::ffi::c_int = 0;
pub const GKS_K_TEXT_HALIGN_LEFT: ::core::ffi::c_int = 1;
pub const GKS_K_TEXT_HALIGN_CENTER: ::core::ffi::c_int = 2;
pub const GKS_K_TEXT_HALIGN_RIGHT: ::core::ffi::c_int = 3;
pub const GKS_K_TEXT_VALIGN_NORMAL: ::core::ffi::c_int = 0;
pub const GKS_K_TEXT_VALIGN_TOP: ::core::ffi::c_int = 1;
pub const GKS_K_TEXT_VALIGN_CAP: ::core::ffi::c_int = 2;
pub const GKS_K_TEXT_VALIGN_HALF: ::core::ffi::c_int = 3;
pub const GKS_K_TEXT_VALIGN_BASE: ::core::ffi::c_int = 4;
pub const GKS_K_TEXT_VALIGN_BOTTOM: ::core::ffi::c_int = 5;
pub const GKS_K_TEXT_PATH_RIGHT: ::core::ffi::c_int = 0;
pub const GKS_K_TEXT_PATH_LEFT: ::core::ffi::c_int = 1;
pub const GKS_K_TEXT_PATH_UP: ::core::ffi::c_int = 2;
pub const GKS_K_TEXT_PATH_DOWN: ::core::ffi::c_int = 3;
pub const GKS_K_TEXT_PRECISION_STRING: ::core::ffi::c_int = 0;
pub const GKS_K_TEXT_PRECISION_CHAR: ::core::ffi::c_int = 1;
pub const GKS_K_TEXT_PRECISION_STROKE: ::core::ffi::c_int = 2;
pub const GKS_K_TEXT_PRECISION_OUTLINE: ::core::ffi::c_int = 3;
pub const GKS_K_TEXT_MAX_SIZE: ::core::ffi::c_int = 500;
pub const GKS_K_WSCAT_OUTPUT: ::core::ffi::c_int = 0;
pub const GKS_K_WSCAT_INPUT: ::core::ffi::c_int = 1;
pub const GKS_K_WSCAT_OUTIN: ::core::ffi::c_int = 2;
pub const GKS_K_WSCAT_WISS: ::core::ffi::c_int = 3;
pub const GKS_K_WSCAT_MO: ::core::ffi::c_int = 4;
pub const GKS_K_WSCAT_MI: ::core::ffi::c_int = 5;
pub const GKS_K_WS_INACTIVE: ::core::ffi::c_int = 0;
pub const GKS_K_WS_ACTIVE: ::core::ffi::c_int = 1;
pub const GKS_K_LINETYPE_SOLID: ::core::ffi::c_int = 1;
pub const GKS_K_LINETYPE_DASHED: ::core::ffi::c_int = 2;
pub const GKS_K_LINETYPE_DOTTED: ::core::ffi::c_int = 3;
pub const GKS_K_LINETYPE_DASHED_DOTTED: ::core::ffi::c_int = 4;
pub const GKS_K_LINETYPE_DASH_2_DOT: ::core::ffi::c_int = -1;
pub const GKS_K_LINETYPE_DASH_3_DOT: ::core::ffi::c_int = -2;
pub const GKS_K_LINETYPE_LONG_DASH: ::core::ffi::c_int = -3;
pub const GKS_K_LINETYPE_LONG_SHORT_DASH: ::core::ffi::c_int = -4;
pub const GKS_K_LINETYPE_SPACED_DASH: ::core::ffi::c_int = -5;
pub const GKS_K_LINETYPE_SPACED_DOT: ::core::ffi::c_int = -6;
pub const GKS_K_LINETYPE_DOUBLE_DOT: ::core::ffi::c_int = -7;
pub const GKS_K_LINETYPE_TRIPLE_DOT: ::core::ffi::c_int = -8;
pub const GKS_K_MARKERTYPE_DOT: ::core::ffi::c_int = 1;
pub const GKS_K_MARKERTYPE_PLUS: ::core::ffi::c_int = 2;
pub const GKS_K_MARKERTYPE_ASTERISK: ::core::ffi::c_int = 3;
pub const GKS_K_MARKERTYPE_CIRCLE: ::core::ffi::c_int = 4;
pub const GKS_K_MARKERTYPE_DIAGONAL_CROSS: ::core::ffi::c_int = 5;
pub const GKS_K_MARKERTYPE_SOLID_CIRCLE: ::core::ffi::c_int = -1;
pub const GKS_K_MARKERTYPE_TRIANGLE_UP: ::core::ffi::c_int = -2;
pub const GKS_K_MARKERTYPE_SOLID_TRI_UP: ::core::ffi::c_int = -3;
pub const GKS_K_MARKERTYPE_TRIANGLE_DOWN: ::core::ffi::c_int = -4;
pub const GKS_K_MARKERTYPE_SOLID_TRI_DOWN: ::core::ffi::c_int = -5;
pub const GKS_K_MARKERTYPE_SQUARE: ::core::ffi::c_int = -6;
pub const GKS_K_MARKERTYPE_SOLID_SQUARE: ::core::ffi::c_int = -7;
pub const GKS_K_MARKERTYPE_BOWTIE: ::core::ffi::c_int = -8;
pub const GKS_K_MARKERTYPE_SOLID_BOWTIE: ::core::ffi::c_int = -9;
pub const GKS_K_MARKERTYPE_HOURGLASS: ::core::ffi::c_int = -10;
pub const GKS_K_MARKERTYPE_SOLID_HGLASS: ::core::ffi::c_int = -11;
pub const GKS_K_MARKERTYPE_DIAMOND: ::core::ffi::c_int = -12;
pub const GKS_K_MARKERTYPE_SOLID_DIAMOND: ::core::ffi::c_int = -13;
pub const GKS_K_MARKERTYPE_STAR: ::core::ffi::c_int = -14;
pub const GKS_K_MARKERTYPE_SOLID_STAR: ::core::ffi::c_int = -15;
pub const GKS_K_MARKERTYPE_TRI_UP_DOWN: ::core::ffi::c_int = -16;
pub const GKS_K_MARKERTYPE_SOLID_TRI_RIGHT: ::core::ffi::c_int = -17;
pub const GKS_K_MARKERTYPE_SOLID_TRI_LEFT: ::core::ffi::c_int = -18;
pub const GKS_K_MARKERTYPE_HOLLOW_PLUS: ::core::ffi::c_int = -19;
pub const GKS_K_MARKERTYPE_SOLID_PLUS: ::core::ffi::c_int = -20;
pub const GKS_K_MARKERTYPE_PENTAGON: ::core::ffi::c_int = -21;
pub const GKS_K_MARKERTYPE_HEXAGON: ::core::ffi::c_int = -22;
pub const GKS_K_MARKERTYPE_HEPTAGON: ::core::ffi::c_int = -23;
pub const GKS_K_MARKERTYPE_OCTAGON: ::core::ffi::c_int = -24;
pub const GKS_K_MARKERTYPE_STAR_4: ::core::ffi::c_int = -25;
pub const GKS_K_MARKERTYPE_STAR_5: ::core::ffi::c_int = -26;
pub const GKS_K_MARKERTYPE_STAR_6: ::core::ffi::c_int = -27;
pub const GKS_K_MARKERTYPE_STAR_7: ::core::ffi::c_int = -28;
pub const GKS_K_MARKERTYPE_STAR_8: ::core::ffi::c_int = -29;
pub const GKS_K_MARKERTYPE_VLINE: ::core::ffi::c_int = -30;
pub const GKS_K_MARKERTYPE_HLINE: ::core::ffi::c_int = -31;
pub const GKS_K_MARKERTYPE_OMARK: ::core::ffi::c_int = -32;
pub const GKS_K_VALUE_SET: ::core::ffi::c_int = 0;
pub const GKS_K_VALUE_REALIZED: ::core::ffi::c_int = 1;
pub const GKS_K_UPSAMPLE_VERTICAL_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_UPSAMPLE_HORIZONTAL_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_DOWNSAMPLE_VERTICAL_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_DOWNSAMPLE_HORIZONTAL_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_UPSAMPLE_VERTICAL_NEAREST: ::core::ffi::c_int = 1;
pub const GKS_K_UPSAMPLE_HORIZONTAL_NEAREST: ::core::ffi::c_int = 256;
pub const GKS_K_DOWNSAMPLE_VERTICAL_NEAREST: ::core::ffi::c_int = 65536;
pub const GKS_K_DOWNSAMPLE_HORIZONTAL_NEAREST: ::core::ffi::c_int = 16777216;
pub const GKS_K_UPSAMPLE_VERTICAL_LINEAR: ::core::ffi::c_int = 2;
pub const GKS_K_UPSAMPLE_HORIZONTAL_LINEAR: ::core::ffi::c_int = 512;
pub const GKS_K_DOWNSAMPLE_VERTICAL_LINEAR: ::core::ffi::c_int = 131072;
pub const GKS_K_DOWNSAMPLE_HORIZONTAL_LINEAR: ::core::ffi::c_int = 33554432;
pub const GKS_K_UPSAMPLE_VERTICAL_LANCZOS: ::core::ffi::c_int = 3;
pub const GKS_K_UPSAMPLE_HORIZONTAL_LANCZOS: ::core::ffi::c_int = 768;
pub const GKS_K_DOWNSAMPLE_VERTICAL_LANCZOS: ::core::ffi::c_int = 196608;
pub const GKS_K_DOWNSAMPLE_HORIZONTAL_LANCZOS: ::core::ffi::c_int = 50331648;
pub const GKS_K_RESAMPLE_DEFAULT: ::core::ffi::c_int = 0;
pub const GKS_K_RESAMPLE_NEAREST: ::core::ffi::c_int = 16843009;
pub const GKS_K_RESAMPLE_LINEAR: ::core::ffi::c_int = 33686018;
pub const GKS_K_RESAMPLE_LANCZOS: ::core::ffi::c_int = 50529027;
pub const GKS_K_GDP_DRAW_PATH: ::core::ffi::c_int = 1;
pub const GKS_K_GDP_DRAW_LINES: ::core::ffi::c_int = 2;
pub const GKS_K_GDP_DRAW_MARKERS: ::core::ffi::c_int = 3;
pub const GKS_K_GDP_DRAW_TRIANGLES: ::core::ffi::c_int = 4;
pub const GKS_K_GDP_FILL_POLYGONS: ::core::ffi::c_int = 5;
pub const GKS_K_NO_RESIZE: ::core::ffi::c_int = 0;
pub const GKS_K_RESIZE: ::core::ffi::c_int = 1;
pub const GKS_K_NO_ERROR: ::core::ffi::c_int = 0;
pub const GKS_K_ERROR: ::core::ffi::c_int = 1;
pub type Gfile = ::core::ffi::c_void;
pub type Gchar = ::core::ffi::c_char;
pub type Gconn = ::core::ffi::c_char;
pub type Gfloat = f64;
pub type Gwstype = ::core::ffi::c_int;
pub type Gint = ::core::ffi::c_int;
pub type Guint = ::core::ffi::c_uint;
pub type Glong = ::core::ffi::c_long;
pub const Gasf_GBUNDLED: Gasf = 0;
pub const Gasf_GINDIVIDUAL: Gasf = 1;
pub type Gasf = ::core::ffi::c_uint;
pub const Gclip_GCLIP: Gclip = 0;
pub const Gclip_GNOCLIP: Gclip = 1;
pub type Gclip = ::core::ffi::c_uint;
pub const Gclrflag_GCONDITIONALLY: Gclrflag = 0;
pub const Gclrflag_GALWAYS: Gclrflag = 1;
pub type Gclrflag = ::core::ffi::c_uint;
pub const Gcsw_GWC: Gcsw = 0;
pub const Gcsw_GNDC: Gcsw = 1;
pub type Gcsw = ::core::ffi::c_uint;
pub const Gdevunits_GDC_METRES: Gdevunits = 0;
pub const Gdevunits_GDC_OTHER: Gdevunits = 1;
pub type Gdevunits = ::core::ffi::c_uint;
pub const Gflinter_GHOLLOW: Gflinter = 0;
pub const Gflinter_GSOLID: Gflinter = 1;
pub const Gflinter_GPATTERN: Gflinter = 2;
pub const Gflinter_GHATCH: Gflinter = 3;
pub type Gflinter = ::core::ffi::c_uint;
pub const Gistat_GOK: Gistat = 0;
pub const Gistat_GNONE: Gistat = 1;
pub type Gistat = ::core::ffi::c_uint;
pub const Glntype_GLN_SOLID: Glntype = 1;
pub const Glntype_GLN_DASHED: Glntype = 2;
pub const Glntype_GLN_DOTTED: Glntype = 3;
pub const Glntype_GLN_DASHDOT: Glntype = 4;
pub const Glntype_GLN_TRIPLE_DOT: Glntype = -8;
pub const Glntype_GLN_DOUBLE_DOT: Glntype = -7;
pub const Glntype_GLN_SPACED_DOT: Glntype = -6;
pub const Glntype_GLN_SPACED_DASH: Glntype = -5;
pub const Glntype_GLN_LONG_SHORT_DASH: Glntype = -4;
pub const Glntype_GLN_LONG_DASH: Glntype = -3;
pub const Glntype_GLN_DASH_3_DOT: Glntype = -2;
pub const Glntype_GLN_DASH_2_DOT: Glntype = -1;
pub type Glntype = ::core::ffi::c_int;
pub const Gmktype_GMK_POINT: Gmktype = 1;
pub const Gmktype_GMK_PLUS: Gmktype = 2;
pub const Gmktype_GMK_STAR: Gmktype = 3;
pub const Gmktype_GMK_O: Gmktype = 4;
pub const Gmktype_GMK_X: Gmktype = 5;
pub const Gmktype_GMK_SOLID_DIAMOND: Gmktype = -13;
pub const Gmktype_GMK_DIAMOND: Gmktype = -12;
pub const Gmktype_GMK_SOLID_HGLASS: Gmktype = -11;
pub const Gmktype_GMK_HOURGLASS: Gmktype = -10;
pub const Gmktype_GMK_SOLID_BOWTIE: Gmktype = -9;
pub const Gmktype_GMK_BOWTIE: Gmktype = -8;
pub const Gmktype_GMK_SOLID_SQUARE: Gmktype = -7;
pub const Gmktype_GMK_SQUARE: Gmktype = -6;
pub const Gmktype_GMK_SOLID_TRI_DOWN: Gmktype = -5;
pub const Gmktype_GMK_TRIANGLE_DOWN: Gmktype = -4;
pub const Gmktype_GMK_SOLID_TRI_UP: Gmktype = -3;
pub const Gmktype_GMK_TRIANGLE_UP: Gmktype = -2;
pub const Gmktype_GMK_SOLID_CIRCLE: Gmktype = -1;
pub type Gmktype = ::core::ffi::c_int;
pub const Gopst_GGKCL: Gopst = 0;
pub const Gopst_GGKOP: Gopst = 1;
pub const Gopst_GWSOP: Gopst = 2;
pub const Gopst_GWSAC: Gopst = 3;
pub const Gopst_GSGOP: Gopst = 4;
pub type Gopst = ::core::ffi::c_uint;
pub const Gregen_GPERFORM: Gregen = 0;
pub const Gregen_GPOSTPONE: Gregen = 1;
pub type Gregen = ::core::ffi::c_uint;
pub const Gtxhor_GAH_NORMAL: Gtxhor = 0;
pub const Gtxhor_GAH_LEFT: Gtxhor = 1;
pub const Gtxhor_GAH_CENTRE: Gtxhor = 2;
pub const Gtxhor_GAH_RIGHT: Gtxhor = 3;
pub type Gtxhor = ::core::ffi::c_uint;
pub const Gtxpath_GTP_RIGHT: Gtxpath = 0;
pub const Gtxpath_GTP_LEFT: Gtxpath = 1;
pub const Gtxpath_GTP_UP: Gtxpath = 2;
pub const Gtxpath_GTP_DOWN: Gtxpath = 3;
pub type Gtxpath = ::core::ffi::c_uint;
pub const Gtxprec_GP_STRING: Gtxprec = 0;
pub const Gtxprec_GP_CHAR: Gtxprec = 1;
pub const Gtxprec_GP_STROKE: Gtxprec = 2;
pub type Gtxprec = ::core::ffi::c_uint;
pub const Gtxver_GAV_NORMAL: Gtxver = 0;
pub const Gtxver_GAV_TOP: Gtxver = 1;
pub const Gtxver_GAV_CAP: Gtxver = 2;
pub const Gtxver_GAV_HALF: Gtxver = 3;
pub const Gtxver_GAV_BASE: Gtxver = 4;
pub const Gtxver_GAV_BOTTOM: Gtxver = 5;
pub type Gtxver = ::core::ffi::c_uint;
pub const Gwscat_GOUTPUT: Gwscat = 0;
pub const Gwscat_GINPUT: Gwscat = 1;
pub const Gwscat_GOUTIN: Gwscat = 2;
pub const Gwscat_GWISS: Gwscat = 3;
pub const Gwscat_GMO: Gwscat = 4;
pub const Gwscat_GMI: Gwscat = 5;
pub type Gwscat = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gipoint {
    pub x: Gint,
    pub y: Gint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gpoint {
    pub x: Gfloat,
    pub y: Gfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Glimit {
    pub xmin: Gfloat,
    pub xmax: Gfloat,
    pub ymin: Gfloat,
    pub ymax: Gfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gtxfp {
    pub font: Gint,
    pub prec: Gtxprec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gtxalign {
    pub hor: Gtxhor,
    pub ver: Gtxver,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Grect {
    pub ul: Gpoint,
    pub lr: Gpoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gidim {
    pub x_dim: Guint,
    pub y_dim: Guint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gasfs {
    pub ln_type: Gasf,
    pub ln_width: Gasf,
    pub ln_colour: Gasf,
    pub mk_type: Gasf,
    pub mk_size: Gasf,
    pub mk_colour: Gasf,
    pub tx_fp: Gasf,
    pub tx_exp: Gasf,
    pub tx_space: Gasf,
    pub tx_colour: Gasf,
    pub fl_inter: Gasf,
    pub fl_style: Gasf,
    pub fl_colour: Gasf,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gcobundl {
    pub red: Gfloat,
    pub green: Gfloat,
    pub blue: Gfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gcliprect {
    pub ind: Gclip,
    pub rec: Glimit,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdspsize {
    pub units: Gdevunits,
    pub device: Gpoint,
    pub raster: Gipoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gextent {
    pub concat: Gpoint,
    pub corner_1: Gpoint,
    pub corner_2: Gpoint,
    pub corner_3: Gpoint,
    pub corner_4: Gpoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gloc {
    pub transform: Gint,
    pub position: Gpoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gqloc {
    pub status: Gistat,
    pub loc: Gloc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gqstring {
    pub status: Gistat,
    pub string: *mut Gchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gscale {
    pub x_scale: Gfloat,
    pub y_scale: Gfloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gtran {
    pub w: Glimit,
    pub v: Glimit,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ggksmit {
    pub type_: Gint,
    pub length: Gint,
}
extern "C" {
    pub fn gks_init_gks();
}
extern "C" {
    pub fn gks_open_gks(errfil: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_close_gks();
}
extern "C" {
    pub fn gks_open_ws(
        wkid: ::core::ffi::c_int,
        conid: *mut ::core::ffi::c_char,
        wtype: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_close_ws(wkid: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_activate_ws(wkid: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_deactivate_ws(wkid: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_configure_ws(wkid: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_clear_ws(wkid: ::core::ffi::c_int, cofl: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_redraw_seg_on_ws(wkid: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_update_ws(wkid: ::core::ffi::c_int, regfl: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_deferral_state(
        wkid: ::core::ffi::c_int,
        defmo: ::core::ffi::c_int,
        regmo: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_escape(
        funid: ::core::ffi::c_int,
        dimidr: ::core::ffi::c_int,
        idr: *mut ::core::ffi::c_int,
        maxodr: ::core::ffi::c_int,
        lenodr: *mut ::core::ffi::c_int,
        odr: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_message(wkid: ::core::ffi::c_int, message: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gks_polyline(n: ::core::ffi::c_int, pxa: *mut f64, pya: *mut f64);
}
extern "C" {
    pub fn gks_polymarker(n: ::core::ffi::c_int, pxa: *mut f64, pya: *mut f64);
}
extern "C" {
    pub fn gks_text(px: f64, py: f64, str_: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gks_fillarea(n: ::core::ffi::c_int, pxa: *mut f64, pya: *mut f64);
}
extern "C" {
    pub fn gks_cellarray(
        qx: f64,
        qy: f64,
        rx: f64,
        ry: f64,
        dimx: ::core::ffi::c_int,
        dimy: ::core::ffi::c_int,
        scol: ::core::ffi::c_int,
        srow: ::core::ffi::c_int,
        ncol: ::core::ffi::c_int,
        nrow: ::core::ffi::c_int,
        colia: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_gdp(
        n: ::core::ffi::c_int,
        px: *mut f64,
        py: *mut f64,
        primid: ::core::ffi::c_int,
        ldr: ::core::ffi::c_int,
        datrec: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_set_pline_index(index: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_pline_linetype(ltype: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_pline_linewidth(lwidth: f64);
}
extern "C" {
    pub fn gks_set_pline_color_index(coli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_pmark_index(index: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_pmark_type(mtype: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_pmark_size(mszsc: f64);
}
extern "C" {
    pub fn gks_set_pmark_color_index(coli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_text_index(index: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_text_fontprec(font: ::core::ffi::c_int, prec: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_text_expfac(chxp: f64);
}
extern "C" {
    pub fn gks_set_text_spacing(chsp: f64);
}
extern "C" {
    pub fn gks_set_text_color_index(coli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_text_height(chh: f64);
}
extern "C" {
    pub fn gks_inq_ws_text_height(chh: f64, height: f64) -> f64;
}
extern "C" {
    pub fn gks_set_text_upvec(chux: f64, chuy: f64);
}
extern "C" {
    pub fn gks_set_text_path(txp: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_text_align(txalh: ::core::ffi::c_int, txalv: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_fill_index(index: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_fill_int_style(ints: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_fill_style_index(styli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_fill_color_index(coli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_asf(flag: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_color_rep(
        wkid: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
        red: f64,
        green: f64,
        blue: f64,
    );
}
extern "C" {
    pub fn gks_set_window(tnr: ::core::ffi::c_int, xmin: f64, xmax: f64, ymin: f64, ymax: f64);
}
extern "C" {
    pub fn gks_set_viewport(tnr: ::core::ffi::c_int, xmin: f64, xmax: f64, ymin: f64, ymax: f64);
}
extern "C" {
    pub fn gks_select_xform(tnr: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_clipping(clsw: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_ws_window(wkid: ::core::ffi::c_int, xmin: f64, xmax: f64, ymin: f64, ymax: f64);
}
extern "C" {
    pub fn gks_set_ws_viewport(
        wkid: ::core::ffi::c_int,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    );
}
extern "C" {
    pub fn gks_create_seg(segn: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_close_seg();
}
extern "C" {
    pub fn gks_delete_seg(segn: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_assoc_seg_with_ws(wkid: ::core::ffi::c_int, segn: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_copy_seg_to_ws(wkid: ::core::ffi::c_int, segn: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_seg_xform(segn: ::core::ffi::c_int, mat: *mut [f64; 2usize]);
}
extern "C" {
    pub fn gks_initialize_locator(
        wkid: ::core::ffi::c_int,
        lcdnr: ::core::ffi::c_int,
        tnr: ::core::ffi::c_int,
        px: f64,
        py: f64,
        pet: ::core::ffi::c_int,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        ldr: ::core::ffi::c_int,
        datrec: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gks_request_locator(
        wkid: ::core::ffi::c_int,
        lcdnr: ::core::ffi::c_int,
        stat: *mut ::core::ffi::c_int,
        tnr: *mut ::core::ffi::c_int,
        px: *mut f64,
        py: *mut f64,
    );
}
extern "C" {
    pub fn gks_request_stroke(
        wkid: ::core::ffi::c_int,
        skdnr: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        stat: *mut ::core::ffi::c_int,
        tnr: *mut ::core::ffi::c_int,
        np: *mut ::core::ffi::c_int,
        pxa: *mut f64,
        pya: *mut f64,
    );
}
extern "C" {
    pub fn gks_request_choice(
        wkid: ::core::ffi::c_int,
        chdnr: ::core::ffi::c_int,
        stat: *mut ::core::ffi::c_int,
        chnr: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_request_string(
        wkid: ::core::ffi::c_int,
        stdnr: ::core::ffi::c_int,
        stat: *mut ::core::ffi::c_int,
        lostr: *mut ::core::ffi::c_int,
        str_: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gks_read_item(
        wkid: ::core::ffi::c_int,
        lenidr: ::core::ffi::c_int,
        maxodr: ::core::ffi::c_int,
        odr: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gks_get_item(
        wkid: ::core::ffi::c_int,
        type_: *mut ::core::ffi::c_int,
        lenodr: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_interpret_item(
        type_: ::core::ffi::c_int,
        lenidr: ::core::ffi::c_int,
        dimidr: ::core::ffi::c_int,
        idr: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gks_eval_xform_matrix(
        fx: f64,
        fy: f64,
        transx: f64,
        transy: f64,
        phi: f64,
        scalex: f64,
        scaley: f64,
        coord: ::core::ffi::c_int,
        tran: *mut [f64; 2usize],
    );
}
extern "C" {
    pub fn gks_inq_operating_state(opsta: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_level(errind: *mut ::core::ffi::c_int, lev: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_wstype(
        n: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        number: *mut ::core::ffi::c_int,
        wtype: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_max_xform(errind: *mut ::core::ffi::c_int, maxtnr: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_open_ws(
        n: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        ol: *mut ::core::ffi::c_int,
        wkid: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_active_ws(
        n: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        ol: *mut ::core::ffi::c_int,
        wkid: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_segn_ws(
        wkid: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        ol: *mut ::core::ffi::c_int,
        segn: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_color_rep(
        wkid: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
        type_: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        red: *mut f64,
        green: *mut f64,
        blue: *mut f64,
    );
}
extern "C" {
    pub fn gks_inq_pline_linetype(errind: *mut ::core::ffi::c_int, ltype: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_pline_linewidth(errind: *mut ::core::ffi::c_int, lwidth: *mut f64);
}
extern "C" {
    pub fn gks_inq_pline_color_index(
        errind: *mut ::core::ffi::c_int,
        coli: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_pmark_type(errind: *mut ::core::ffi::c_int, mtype: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_pmark_size(errind: *mut ::core::ffi::c_int, mszsc: *mut f64);
}
extern "C" {
    pub fn gks_inq_pmark_color_index(
        errind: *mut ::core::ffi::c_int,
        coli: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_text_fontprec(
        errind: *mut ::core::ffi::c_int,
        font: *mut ::core::ffi::c_int,
        prec: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_text_expfac(errind: *mut ::core::ffi::c_int, chxp: *mut f64);
}
extern "C" {
    pub fn gks_inq_text_spacing(errind: *mut ::core::ffi::c_int, chsp: *mut f64);
}
extern "C" {
    pub fn gks_inq_text_color_index(errind: *mut ::core::ffi::c_int, coli: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_text_height(errind: *mut ::core::ffi::c_int, chh: *mut f64);
}
extern "C" {
    pub fn gks_inq_text_upvec(errind: *mut ::core::ffi::c_int, chux: *mut f64, chuy: *mut f64);
}
extern "C" {
    pub fn gks_inq_text_path(errind: *mut ::core::ffi::c_int, txp: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_text_align(
        errind: *mut ::core::ffi::c_int,
        txalh: *mut ::core::ffi::c_int,
        txalv: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_fill_int_style(errind: *mut ::core::ffi::c_int, ints: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_fill_style_index(
        errind: *mut ::core::ffi::c_int,
        styli: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_fill_color_index(errind: *mut ::core::ffi::c_int, coli: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_transparency(errind: *mut ::core::ffi::c_int, alpha: *mut f64);
}
extern "C" {
    pub fn gks_inq_open_segn(errind: *mut ::core::ffi::c_int, segn: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_current_xformno(errind: *mut ::core::ffi::c_int, tnr: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_xform(
        tnr: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        wn: *mut f64,
        vp: *mut f64,
    );
}
extern "C" {
    pub fn gks_inq_clip(
        errind: *mut ::core::ffi::c_int,
        clsw: *mut ::core::ffi::c_int,
        clrt: *mut f64,
    );
}
extern "C" {
    pub fn gks_inq_ws_conntype(
        wkid: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        conid: *mut ::core::ffi::c_int,
        wtype: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_ws_category(
        wkid: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        wscat: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_text_extent(
        wkid: ::core::ffi::c_int,
        px: f64,
        py: f64,
        str_: *mut ::core::ffi::c_char,
        errind: *mut ::core::ffi::c_int,
        cpx: *mut f64,
        cpy: *mut f64,
        tx: *mut f64,
        ty: *mut f64,
    );
}
extern "C" {
    pub fn gks_inq_max_ds_size(
        wtype: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        dcunit: *mut ::core::ffi::c_int,
        rx: *mut f64,
        ry: *mut f64,
        lx: *mut ::core::ffi::c_int,
        ly: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_inq_vp_size(
        wkid: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        device_pixel_ratio: *mut f64,
    );
}
extern "C" {
    pub fn gks_sample_locator(
        wkid: ::core::ffi::c_int,
        errind: *mut ::core::ffi::c_int,
        x: *mut f64,
        y: *mut f64,
        buttons: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_emergency_close();
}
extern "C" {
    pub fn gks_set_text_slant(slant: f64);
}
extern "C" {
    pub fn gks_draw_image(
        x: f64,
        y: f64,
        scalex: f64,
        scaley: f64,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_set_shadow(offsetx: f64, offsety: f64, blur: f64);
}
extern "C" {
    pub fn gks_set_transparency(alpha: f64);
}
extern "C" {
    pub fn gks_set_coord_xform(mat: *mut [f64; 2usize]);
}
extern "C" {
    pub fn gks_begin_selection(index: ::core::ffi::c_int, kind: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_end_selection();
}
extern "C" {
    pub fn gks_move_selection(x: f64, y: f64);
}
extern "C" {
    pub fn gks_resize_selection(kind: ::core::ffi::c_int, x: f64, y: f64);
}
extern "C" {
    pub fn gks_begin_grm_selection(
        index: ::core::ffi::c_int,
        fun: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: ::core::ffi::c_int,
                arg2: f64,
                arg3: f64,
                arg4: f64,
                arg5: f64,
            ),
        >,
    );
}
extern "C" {
    pub fn gks_end_grm_selection();
}
extern "C" {
    pub fn gks_inq_bbox(
        errind: *mut ::core::ffi::c_int,
        xmin: *mut f64,
        xmax: *mut f64,
        ymin: *mut f64,
        ymax: *mut f64,
    );
}
extern "C" {
    pub fn gks_inq_text_slant(errind: *mut ::core::ffi::c_int, slant: *mut f64);
}
extern "C" {
    pub fn gks_precision() -> f64;
}
extern "C" {
    pub fn gks_text_maxsize() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gks_set_border_color_index(coli: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_border_color_index(
        errind: *mut ::core::ffi::c_int,
        coli: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_set_border_width(bwidth: f64);
}
extern "C" {
    pub fn gks_inq_border_width(errind: *mut ::core::ffi::c_int, bwidth: *mut f64);
}
extern "C" {
    pub fn gks_select_clip_xform(tnr: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_clip_xform(errind: *mut ::core::ffi::c_int, tnr: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_set_resample_method(flag: ::core::ffi::c_uint);
}
extern "C" {
    pub fn gks_inq_resample_method(flag: *mut ::core::ffi::c_uint);
}
extern "C" {
    pub fn gks_ft_gdp(
        n: ::core::ffi::c_int,
        px: *mut f64,
        py: *mut f64,
        primid: ::core::ffi::c_int,
        ldr: ::core::ffi::c_int,
        datrec: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gks_state() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn gks_set_resize_behaviour(flag: ::core::ffi::c_int);
}
extern "C" {
    pub fn gks_inq_resize_behaviour(flag: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gopengks(arg1: *mut Gfile, arg2: Glong) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gclosegks() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gopenws(arg1: Gint, arg2: *mut Gconn, arg3: *mut Gwstype) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gclosews(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gactivatews(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gdeactivatews(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gconfigurews(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gclearws(arg1: Gint, arg2: Gclrflag) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gupdatews(arg1: Gint, arg2: Gregen) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gmessage(arg1: Gint, arg2: *mut Gchar) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gpolyline(arg1: Gint, arg2: *mut Gpoint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gpolymarker(arg1: Gint, arg2: *mut Gpoint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gtext(arg1: *mut Gpoint, arg2: *mut Gchar) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gfillarea(arg1: Gint, arg2: *mut Gpoint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gcellarray(arg1: *mut Grect, arg2: *mut Gidim, arg3: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetasf(arg1: *mut Gasfs) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetlineind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetlinetype(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetlinewidth(arg1: Gfloat) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetlinecolourind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetmarkerind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetmarkertype(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetmarkersize(arg1: Gfloat) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetmarkercolourind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsettextind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsettextfontprec(arg1: *mut Gtxfp) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetcharexpan(arg1: Gfloat) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetcharspace(arg1: Gfloat) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsettextcolourind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetcharheight(arg1: Gfloat) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetcharup(arg1: *mut Gpoint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsettextpath(arg1: Gtxpath) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsettextalign(arg1: *mut Gtxalign) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetfillind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetfillintstyle(arg1: Gflinter) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetfillstyle(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetfillcolourind(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetcolourrep(arg1: Gint, arg2: Gint, arg3: *mut Gcobundl) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetwindow(arg1: Gint, arg2: *mut Glimit) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetviewport(arg1: Gint, arg2: *mut Glimit) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gselntran(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetclip(arg1: Gclip) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetwswindow(arg1: Gint, arg2: *mut Glimit) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetwsviewport(arg1: Gint, arg2: *mut Glimit) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn greqloc(arg1: Gint, arg2: Gint, arg3: *mut Gqloc) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn greqstring(arg1: Gint, arg2: Gint, arg3: *mut Gqstring) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gcreateseg(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gcopysegws(arg1: Gint, arg2: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gredrawsegws(arg1: Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gcloseseg() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gevaltran(
        arg1: *mut Gpoint,
        arg2: *mut Gpoint,
        arg3: Gfloat,
        arg4: *mut Gscale,
        arg5: Gcsw,
        arg6: *mut [Gfloat; 3usize],
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gsetsegtran(arg1: Gint, arg2: *mut [Gfloat; 3usize]) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqopst(arg1: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqlevelgks(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqmaxntrannum(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqcharheight(arg1: *mut Gfloat, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqcharup(arg1: *mut Gpoint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqtextpath(arg1: *mut Gtxpath, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqtextalign(arg1: *mut Gtxalign, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqlinetype(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqlinewidth(arg1: *mut Gfloat, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqlinecolourind(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqmarkertype(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqmarkersize(arg1: *mut Gfloat, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqmarkercolourind(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqtextfontprec(arg1: *mut Gtxfp, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqcharexpan(arg1: *mut Gfloat, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqcharspace(arg1: *mut Gfloat, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqtextcolourind(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqfillintstyle(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqfillstyle(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqfillcolourind(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqcurntrannum(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqntran(arg1: Gint, arg2: *mut Gtran, arg3: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqclip(arg1: *mut Gcliprect, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqwscategory(
        arg1: *mut Gwstype,
        arg2: *mut Gint,
        arg3: *mut Gint,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqdisplaysize(
        arg1: *mut Gwstype,
        arg2: *mut Gdspsize,
        arg3: *mut Gint,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqtextextent(
        arg1: Gint,
        arg2: *mut Gpoint,
        arg3: *mut Gchar,
        arg4: *mut Gextent,
        arg5: *mut Gint,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ginqnameopenseg(arg1: *mut Gint, arg2: *mut Gint) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gemergencyclosegks() -> ::core::ffi::c_int;
}
