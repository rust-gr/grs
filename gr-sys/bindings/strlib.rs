/* automatically generated by rust-bindgen 0.68.1 */

pub const SCIENTIFIC_FORMAT_OPTION_E: ::core::ffi::c_int = 1;
pub const SCIENTIFIC_FORMAT_OPTION_TEXTEX: ::core::ffi::c_int = 2;
pub const SCIENTIFIC_FORMAT_OPTION_MATHTEX: ::core::ffi::c_int = 3;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct format_reference_t {
    pub scientific: ::core::ffi::c_int,
    pub decimal_digits: ::core::ffi::c_int,
}
extern "C" {
    pub fn str_get_format_reference(
        arg1: *mut format_reference_t,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: ::core::ffi::c_int,
    ) -> *mut format_reference_t;
}
extern "C" {
    pub fn str_remove(
        arg1: *mut ::core::ffi::c_char,
        arg2: ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn str_pad(
        arg1: *mut ::core::ffi::c_char,
        arg2: ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn str_ftoa(
        arg1: *mut ::core::ffi::c_char,
        arg2: f64,
        arg3: *mut format_reference_t,
        arg4: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn str_casecmp(
        arg1: *mut ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn str_utf8_to_unicode(
        utf8_str: *const ::core::ffi::c_uchar,
        length: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
