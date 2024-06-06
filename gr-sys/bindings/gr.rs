/* automatically generated by rust-bindgen 0.68.1 */

pub const GR_MAX_CONTEXT: ::core::ffi::c_int = 8192;
pub const GR_DEFAULT_MATH_FONT: ::core::ffi::c_int = 232;
pub const projection_type_t_GR_PROJECTION_DEFAULT: projection_type_t = 0;
pub const projection_type_t_GR_PROJECTION_ORTHOGRAPHIC: projection_type_t = 1;
pub const projection_type_t_GR_PROJECTION_PERSPECTIVE: projection_type_t = 2;
pub type projection_type_t = ::core::ffi::c_uint;
pub const volume_border_calculation_t_GR_VOLUME_WITHOUT_BORDER: volume_border_calculation_t = 0;
pub const volume_border_calculation_t_GR_VOLUME_WITH_BORDER: volume_border_calculation_t = 1;
pub type volume_border_calculation_t = ::core::ffi::c_uint;
pub const volume_rendering_model_t_GR_VOLUME_EMISSION: volume_rendering_model_t = 0;
pub const volume_rendering_model_t_GR_VOLUME_ABSORPTION: volume_rendering_model_t = 1;
pub const volume_rendering_model_t_GR_VOLUME_MIP: volume_rendering_model_t = 2;
pub type volume_rendering_model_t = ::core::ffi::c_uint;
pub const textx_option_t_GR_TEXT_USE_WC: textx_option_t = 1;
pub const textx_option_t_GR_TEXT_ENABLE_INLINE_MATH: textx_option_t = 2;
pub type textx_option_t = ::core::ffi::c_uint;
pub const f2pass_option_t_GR_2PASS_CLEANUP: f2pass_option_t = 1;
pub const f2pass_option_t_GR_2PASS_RENDER: f2pass_option_t = 2;
pub type f2pass_option_t = ::core::ffi::c_uint;
pub const scale_option_t_GR_OPTION_X_LOG: scale_option_t = 1;
pub const scale_option_t_GR_OPTION_Y_LOG: scale_option_t = 2;
pub const scale_option_t_GR_OPTION_Z_LOG: scale_option_t = 4;
pub const scale_option_t_GR_OPTION_FLIP_X: scale_option_t = 8;
pub const scale_option_t_GR_OPTION_FLIP_Y: scale_option_t = 16;
pub const scale_option_t_GR_OPTION_FLIP_Z: scale_option_t = 32;
pub const scale_option_t_GR_OPTION_X_LOG2: scale_option_t = 64;
pub const scale_option_t_GR_OPTION_Y_LOG2: scale_option_t = 128;
pub const scale_option_t_GR_OPTION_Z_LOG2: scale_option_t = 256;
pub const scale_option_t_GR_OPTION_X_LN: scale_option_t = 512;
pub const scale_option_t_GR_OPTION_Y_LN: scale_option_t = 1024;
pub const scale_option_t_GR_OPTION_Z_LN: scale_option_t = 2048;
pub type scale_option_t = ::core::ffi::c_uint;
pub const linespec_t_GR_SPEC_LINE: linespec_t = 1;
pub const linespec_t_GR_SPEC_MARKER: linespec_t = 2;
pub const linespec_t_GR_SPEC_COLOR: linespec_t = 4;
pub type linespec_t = ::core::ffi::c_uint;
pub const surface_option_t_GR_OPTION_LINES: surface_option_t = 0;
pub const surface_option_t_GR_OPTION_MESH: surface_option_t = 1;
pub const surface_option_t_GR_OPTION_FILLED_MESH: surface_option_t = 2;
pub const surface_option_t_GR_OPTION_Z_SHADED_MESH: surface_option_t = 3;
pub const surface_option_t_GR_OPTION_COLORED_MESH: surface_option_t = 4;
pub const surface_option_t_GR_OPTION_CELL_ARRAY: surface_option_t = 5;
pub const surface_option_t_GR_OPTION_SHADED_MESH: surface_option_t = 6;
pub const surface_option_t_GR_OPTION_3D_MESH: surface_option_t = 7;
pub type surface_option_t = ::core::ffi::c_uint;
pub const color_model_t_GR_MODEL_RGB: color_model_t = 0;
pub const color_model_t_GR_MODEL_HSV: color_model_t = 1;
pub type color_model_t = ::core::ffi::c_uint;
pub const path_code_t_GR_STOP: path_code_t = 0;
pub const path_code_t_GR_MOVETO: path_code_t = 1;
pub const path_code_t_GR_LINETO: path_code_t = 2;
pub const path_code_t_GR_CURVE3: path_code_t = 3;
pub const path_code_t_GR_CURVE4: path_code_t = 4;
pub const path_code_t_GR_CLOSEPOLY: path_code_t = 79;
pub type path_code_t = ::core::ffi::c_uint;
pub const xform_types_t_GR_XFORM_BOOLEAN: xform_types_t = 0;
pub const xform_types_t_GR_XFORM_LINEAR: xform_types_t = 1;
pub const xform_types_t_GR_XFORM_LOG: xform_types_t = 2;
pub const xform_types_t_GR_XFORM_LOGLOG: xform_types_t = 3;
pub const xform_types_t_GR_XFORM_CUBIC: xform_types_t = 4;
pub const xform_types_t_GR_XFORM_EQUALIZED: xform_types_t = 5;
pub type xform_types_t = ::core::ffi::c_uint;
pub const interp2_method_t_GR_INTERP2_NEAREST: interp2_method_t = 0;
pub const interp2_method_t_GR_INTERP2_LINEAR: interp2_method_t = 1;
pub const interp2_method_t_GR_INTERP2_SPLINE: interp2_method_t = 2;
pub const interp2_method_t_GR_INTERP2_CUBIC: interp2_method_t = 3;
pub type interp2_method_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vertex_t {
    pub x: f64,
    pub y: f64,
}
#[doc = " Three-dimensional coordinate"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct point3d_t {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[doc = " Data point for `gr_volume_nogrid`"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct data_point3d_t {
    #[doc = "< Coordinates of data point"]
    pub pt: point3d_t,
    #[doc = "< Intensity of data point"]
    pub data: f64,
}
#[doc = " Provides optional extra data for `gr_volume_interp_gauss`"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gauss_t {
    #[doc = "< Square root of determinant of covariance matrix"]
    pub sqrt_det: f64,
    #[doc = "< \\f$\\Sigma^{-\\frac{1}{2}}\\f$ encoded as three column vectors"]
    pub gauss_sig_1: point3d_t,
    #[doc = "< \\f$\\Sigma^{-\\frac{1}{2}}\\f$ encoded as three column vectors"]
    pub gauss_sig_2: point3d_t,
    #[doc = "< \\f$\\Sigma^{-\\frac{1}{2}}\\f$ encoded as three column vectors"]
    pub gauss_sig_3: point3d_t,
}
#[doc = " Provides optional extra data for `gr_volume_interp_tri_linear`"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tri_linear_t {
    #[doc = "< Reciproke of interpolation kernel extent in x-direction"]
    pub grid_x_re: f64,
    #[doc = "< Reciproke of interpolation kernel extent in y-direction"]
    pub grid_y_re: f64,
    #[doc = "< Reciproke of interpolation kernel extent in z-direction"]
    pub grid_z_re: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cpubasedvolume_2pass_priv {
    _unused: [u8; 0],
}
pub type cpubasedvolume_2pass_priv_t = cpubasedvolume_2pass_priv;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cpubasedvolume_2pass_t {
    pub dmin: f64,
    pub dmax: f64,
    pub action: ::core::ffi::c_int,
    pub priv_: *mut cpubasedvolume_2pass_priv_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hexbin_2pass_priv {
    _unused: [u8; 0],
}
pub type hexbin_2pass_priv_t = hexbin_2pass_priv;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hexbin_2pass_t {
    pub nc: ::core::ffi::c_int,
    pub cntmax: ::core::ffi::c_int,
    pub action: ::core::ffi::c_int,
    pub priv_: *mut hexbin_2pass_priv_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct format_reference_t {
    pub scientific: ::core::ffi::c_int,
    pub decimal_digits: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tick_t {
    pub value: f64,
    pub is_major: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tick_label_t {
    pub tick: f64,
    pub label: *mut ::core::ffi::c_char,
    pub width: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct axis_t {
    pub min: f64,
    pub max: f64,
    pub tick: f64,
    pub org: f64,
    pub major_count: ::core::ffi::c_int,
    pub num_ticks: ::core::ffi::c_int,
    pub ticks: *mut tick_t,
    pub num_tick_labels: ::core::ffi::c_int,
    pub tick_labels: *mut tick_label_t,
    pub tick_size: f64,
}
extern "C" {
    pub fn gr_initgr();
}
extern "C" {
    pub fn gr_debug() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_opengks();
}
extern "C" {
    pub fn gr_closegks();
}
extern "C" {
    pub fn gr_inqdspsize(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_openws(
        arg1: ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_closews(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_activatews(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_deactivatews(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_configurews();
}
extern "C" {
    pub fn gr_clearws();
}
extern "C" {
    pub fn gr_updatews();
}
extern "C" {
    pub fn gr_polyline(arg1: ::core::ffi::c_int, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_polymarker(arg1: ::core::ffi::c_int, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_text(arg1: f64, arg2: f64, arg3: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gr_textx(arg1: f64, arg2: f64, arg3: *mut ::core::ffi::c_char, arg4: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqtext(
        arg1: f64,
        arg2: f64,
        arg3: *mut ::core::ffi::c_char,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_inqtextx(
        arg1: f64,
        arg2: f64,
        arg3: *mut ::core::ffi::c_char,
        arg4: ::core::ffi::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_fillarea(arg1: ::core::ffi::c_int, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_cellarray(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: ::core::ffi::c_int,
        arg10: ::core::ffi::c_int,
        arg11: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_nonuniformcellarray(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: ::core::ffi::c_int,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_polarcellarray(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: ::core::ffi::c_int,
        arg10: ::core::ffi::c_int,
        arg11: ::core::ffi::c_int,
        arg12: ::core::ffi::c_int,
        arg13: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_nonuniformpolarcellarray(
        arg1: f64,
        arg2: f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: ::core::ffi::c_int,
        arg10: ::core::ffi::c_int,
        arg11: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_gdp(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
        arg6: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_spline(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_gridit(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
    );
}
extern "C" {
    pub fn gr_setlinetype(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqlinetype(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setlinewidth(arg1: f64);
}
extern "C" {
    pub fn gr_inqlinewidth(arg1: *mut f64);
}
extern "C" {
    pub fn gr_setlinecolorind(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqlinecolorind(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setmarkertype(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqmarkertype(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setmarkersize(arg1: f64);
}
extern "C" {
    pub fn gr_inqmarkersize(arg1: *mut f64);
}
extern "C" {
    pub fn gr_setmarkercolorind(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqmarkercolorind(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_settextfontprec(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setcharexpan(arg1: f64);
}
extern "C" {
    pub fn gr_setcharspace(arg1: f64);
}
extern "C" {
    pub fn gr_settextcolorind(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqtextcolorind(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setcharheight(arg1: f64);
}
extern "C" {
    pub fn gr_setwscharheight(chh: f64, height: f64);
}
extern "C" {
    pub fn gr_inqcharheight(arg1: *mut f64);
}
extern "C" {
    pub fn gr_setcharup(arg1: f64, arg2: f64);
}
extern "C" {
    pub fn gr_settextpath(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_settextalign(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setfillintstyle(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqfillintstyle(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setfillstyle(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqfillstyle(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setfillcolorind(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqfillcolorind(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setresizebehaviour(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqresizebehaviour(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setcolorrep(arg1: ::core::ffi::c_int, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_setwindow(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_inqwindow(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
}
extern "C" {
    pub fn gr_setviewport(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_inqviewport(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
}
extern "C" {
    pub fn gr_selntran(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setclip(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setwswindow(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_setwsviewport(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_createseg(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_copysegws(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_redrawsegws();
}
extern "C" {
    pub fn gr_setsegtran(
        arg1: ::core::ffi::c_int,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
        arg8: f64,
    );
}
extern "C" {
    pub fn gr_closeseg();
}
extern "C" {
    pub fn gr_samplelocator(arg1: *mut f64, arg2: *mut f64, arg3: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_emergencyclosegks();
}
extern "C" {
    pub fn gr_updategks();
}
extern "C" {
    pub fn gr_setspace(
        arg1: f64,
        arg2: f64,
        arg3: ::core::ffi::c_int,
        arg4: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_inqspace(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_setscale(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_inqscale(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_textext(arg1: f64, arg2: f64, arg3: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_inqtextext(
        arg1: f64,
        arg2: f64,
        arg3: *mut ::core::ffi::c_char,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_setscientificformat(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_axes(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: f64,
    );
}
extern "C" {
    pub fn gr_axeslbl(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: f64,
        arg8: ::core::option::Option<
            unsafe extern "C" fn(arg1: f64, arg2: f64, arg3: *const ::core::ffi::c_char, arg4: f64),
        >,
        arg9: ::core::option::Option<
            unsafe extern "C" fn(arg1: f64, arg2: f64, arg3: *const ::core::ffi::c_char, arg4: f64),
        >,
    );
}
extern "C" {
    pub fn gr_axis(arg1: ::core::ffi::c_char, arg2: *mut axis_t);
}
extern "C" {
    pub fn gr_drawaxis(arg1: ::core::ffi::c_char, arg2: *mut axis_t);
}
extern "C" {
    pub fn gr_freeaxis(arg1: *mut axis_t);
}
extern "C" {
    pub fn gr_grid(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_grid3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_verrorbars(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_herrorbars(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_polyline3d(arg1: ::core::ffi::c_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
}
extern "C" {
    pub fn gr_polymarker3d(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
    );
}
extern "C" {
    pub fn gr_axes3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: ::core::ffi::c_int,
        arg10: f64,
    );
}
extern "C" {
    pub fn gr_titles3d(
        arg1: *mut ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
        arg3: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gr_settitles3d(
        arg1: *mut ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
        arg3: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gr_surface(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_contour(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_contourf(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_tricontour(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: ::core::ffi::c_int,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_hexbin(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_hexbin_2pass(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: *const hexbin_2pass_t,
    ) -> *const hexbin_2pass_t;
}
extern "C" {
    pub fn gr_setcolormap(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqcolormap(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setcolormapfromrgb(
        n: ::core::ffi::c_int,
        r: *mut f64,
        g: *mut f64,
        b: *mut f64,
        x: *mut f64,
    );
}
extern "C" {
    pub fn gr_inqcolormapinds(arg1: *mut ::core::ffi::c_int, arg2: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_colorbar();
}
extern "C" {
    pub fn gr_inqcolor(arg1: ::core::ffi::c_int, arg2: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqcolorfromrgb(arg1: f64, arg2: f64, arg3: f64) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_hsvtorgb(h: f64, s: f64, v: f64, r: *mut f64, g: *mut f64, b: *mut f64);
}
extern "C" {
    pub fn gr_tick(arg1: f64, arg2: f64) -> f64;
}
extern "C" {
    pub fn gr_validaterange(arg1: f64, arg2: f64) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_adjustlimits(arg1: *mut f64, arg2: *mut f64);
}
extern "C" {
    pub fn gr_adjustrange(arg1: *mut f64, arg2: *mut f64);
}
extern "C" {
    pub fn gr_beginprint(arg1: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gr_beginprintext(
        arg1: *mut ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
        arg3: *mut ::core::ffi::c_char,
        arg4: *mut ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gr_endprint();
}
extern "C" {
    pub fn gr_ndctowc(arg1: *mut f64, arg2: *mut f64);
}
extern "C" {
    pub fn gr_wctondc(arg1: *mut f64, arg2: *mut f64);
}
extern "C" {
    pub fn gr_wc3towc(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_drawrect(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_fillrect(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_drawarc(arg1: f64, arg2: f64, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
}
extern "C" {
    pub fn gr_fillarc(arg1: f64, arg2: f64, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
}
extern "C" {
    pub fn gr_drawpath(
        arg1: ::core::ffi::c_int,
        arg2: *mut vertex_t,
        arg3: *mut ::core::ffi::c_uchar,
        arg4: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_setarrowstyle(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setarrowsize(arg1: f64);
}
extern "C" {
    pub fn gr_drawarrow(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_readimage(
        arg1: *mut ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_drawimage(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
        arg7: *mut ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_importgraphics(arg1: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_setshadow(arg1: f64, arg2: f64, arg3: f64);
}
extern "C" {
    pub fn gr_settransparency(arg1: f64);
}
extern "C" {
    pub fn gr_inqtransparency(arg1: *mut f64);
}
extern "C" {
    pub fn gr_setcoordxform(arg1: *mut [f64; 2usize]);
}
extern "C" {
    pub fn gr_begingraphics(arg1: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gr_endgraphics();
}
extern "C" {
    pub fn gr_getgraphics() -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn gr_drawgraphics(arg1: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_startlistener() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_mathtex(arg1: f64, arg2: f64, arg3: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn gr_inqmathtex(
        arg1: f64,
        arg2: f64,
        arg3: *mut ::core::ffi::c_char,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_mathtex3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: *mut ::core::ffi::c_char,
        arg5: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_inqmathtex3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: *mut ::core::ffi::c_char,
        arg5: ::core::ffi::c_int,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
    );
}
extern "C" {
    pub fn gr_beginselection(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_endselection();
}
extern "C" {
    pub fn gr_setbboxcallback(
        arg1: ::core::ffi::c_int,
        arg2: ::core::option::Option<
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
    pub fn gr_cancelbboxcallback();
}
extern "C" {
    pub fn gr_moveselection(arg1: f64, arg2: f64);
}
extern "C" {
    pub fn gr_resizeselection(arg1: ::core::ffi::c_int, arg2: f64, arg3: f64);
}
extern "C" {
    pub fn gr_inqbbox(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
}
extern "C" {
    pub fn gr_precision() -> f64;
}
extern "C" {
    pub fn gr_text_maxsize() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_setregenflags(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqregenflags() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_savestate();
}
extern "C" {
    pub fn gr_restorestate();
}
extern "C" {
    pub fn gr_savecontext(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_selectcontext(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_destroycontext(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_unselectcontext();
}
extern "C" {
    pub fn gr_uselinespec(arg1: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_delaunay(
        arg1: ::core::ffi::c_int,
        arg2: *const f64,
        arg3: *const f64,
        arg4: *mut ::core::ffi::c_int,
        arg5: *mut *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_reducepoints(
        arg1: ::core::ffi::c_int,
        arg2: *const f64,
        arg3: *const f64,
        arg4: ::core::ffi::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_trisurface(arg1: ::core::ffi::c_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
}
extern "C" {
    pub fn gr_gradient(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
    );
}
extern "C" {
    pub fn gr_quiver(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_interp2(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        x: *const f64,
        y: *const f64,
        z: *const f64,
        nxq: ::core::ffi::c_int,
        nyq: ::core::ffi::c_int,
        xq: *const f64,
        yq: *const f64,
        zq: *mut f64,
        method: interp2_method_t,
        extrapval: f64,
    );
}
extern "C" {
    pub fn gr_version() -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn gr_shade(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
        arg6: *mut f64,
        arg7: ::core::ffi::c_int,
        arg8: ::core::ffi::c_int,
        arg9: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_shadepoints(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_shadelines(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
        arg6: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_panzoom(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
    );
}
extern "C" {
    pub fn gr_findboundary(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: f64,
        arg5: ::core::option::Option<unsafe extern "C" fn(arg1: f64, arg2: f64) -> f64>,
        arg6: ::core::ffi::c_int,
        arg7: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn gr_setresamplemethod(arg1: ::core::ffi::c_uint);
}
extern "C" {
    pub fn gr_inqresamplemethod(arg1: *mut ::core::ffi::c_uint);
}
extern "C" {
    pub fn gr_path(
        arg1: ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn gr_setborderwidth(arg1: f64);
}
extern "C" {
    pub fn gr_inqborderwidth(arg1: *mut f64);
}
extern "C" {
    pub fn gr_setbordercolorind(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqbordercolorind(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_selectclipxform(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqclipxform(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setprojectiontype(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqprojectiontype(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setperspectiveprojection(arg1: f64, arg2: f64, arg3: f64);
}
extern "C" {
    pub fn gr_inqperspectiveprojection(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_settransformationparameters(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
        arg8: f64,
        arg9: f64,
    );
}
extern "C" {
    pub fn gr_inqtransformationparameters(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
    );
}
extern "C" {
    pub fn gr_setorthographicprojection(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
    );
}
extern "C" {
    pub fn gr_inqorthographicprojection(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_camerainteraction(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_setwindow3d(arg1: f64, arg2: f64, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
}
extern "C" {
    pub fn gr_inqwindow3d(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_setscalefactors3d(arg1: f64, arg2: f64, arg3: f64);
}
extern "C" {
    pub fn gr_inqscalefactors3d(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64);
}
extern "C" {
    pub fn gr_setspace3d(arg1: f64, arg2: f64, arg3: f64, arg4: f64);
}
extern "C" {
    pub fn gr_inqspace3d(
        arg1: *mut ::core::ffi::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
    );
}
extern "C" {
    pub fn gr_text3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: *mut ::core::ffi::c_char,
        axis: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_inqtext3d(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: *mut ::core::ffi::c_char,
        axis: ::core::ffi::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    pub fn gr_settextencoding(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqtextencoding(arg1: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_loadfont(arg1: *mut ::core::ffi::c_char, arg2: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setcallback(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        >,
    );
}
extern "C" {
    pub fn gr_setthreadnumber(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setpicturesizeforvolume(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setvolumebordercalculation(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setapproximativecalculation(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqvolumeflags(
        arg1: *mut ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut ::core::ffi::c_int,
        arg5: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn gr_cpubasedvolume(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: *mut f64,
        arg5: ::core::ffi::c_int,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
    );
}
extern "C" {
    pub fn gr_cpubasedvolume_2pass(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: *mut f64,
        arg5: ::core::ffi::c_int,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
        arg10: *const cpubasedvolume_2pass_t,
    ) -> *const cpubasedvolume_2pass_t;
}
extern "C" {
    pub fn gr_inqvpsize(
        arg1: *mut ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_int,
        arg3: *mut f64,
    );
}
extern "C" {
    pub fn gr_polygonmesh3d(
        arg1: ::core::ffi::c_int,
        arg2: *const f64,
        arg3: *const f64,
        arg4: *const f64,
        arg5: ::core::ffi::c_int,
        arg6: *const ::core::ffi::c_int,
        arg7: *const ::core::ffi::c_int,
    );
}
pub type kernel_f = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *const data_point3d_t,
        arg2: *const ::core::ffi::c_void,
        arg3: *const point3d_t,
        arg4: *const point3d_t,
    ) -> f64,
>;
pub type radius_f = ::core::option::Option<
    unsafe extern "C" fn(arg1: *const data_point3d_t, arg2: *const ::core::ffi::c_void) -> f64,
>;
extern "C" {
    pub fn gr_volume_nogrid(
        arg1: ::core::ffi::c_ulong,
        arg2: *const data_point3d_t,
        arg3: *const ::core::ffi::c_void,
        arg4: ::core::ffi::c_int,
        arg5: kernel_f,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: f64,
        arg9: radius_f,
    );
}
extern "C" {
    pub fn gr_volume_interp_tri_linear_init(arg1: f64, arg2: f64, arg3: f64);
}
extern "C" {
    pub fn gr_volume_interp_gauss_init(arg1: f64, arg2: *mut f64);
}
extern "C" {
    pub fn gr_volume_interp_tri_linear(
        arg1: *const data_point3d_t,
        arg2: *const ::core::ffi::c_void,
        arg3: *const point3d_t,
        arg4: *const point3d_t,
    ) -> f64;
}
extern "C" {
    pub fn gr_volume_interp_gauss(
        arg1: *const data_point3d_t,
        arg2: *const ::core::ffi::c_void,
        arg3: *const point3d_t,
        arg4: *const point3d_t,
    ) -> f64;
}
extern "C" {
    pub fn gr_setmathfont(font: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqmathfont(font: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_setclipregion(region: ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_inqclipregion(region: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn gr_settextoffset(xoff: f64, yoff: f64);
}
extern "C" {
    pub fn gr_ftoa(
        string: *mut ::core::ffi::c_char,
        value: f64,
        reference: *mut format_reference_t,
    ) -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn gr_getformat(
        result: *mut format_reference_t,
        origin: f64,
        min: f64,
        max: f64,
        tick_width: f64,
        major: ::core::ffi::c_int,
    );
}
