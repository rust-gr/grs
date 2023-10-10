extern "C" {
    pub static mut gks_errno: ::core::ffi::c_int;
}
pub fn FIX_COLORIND(c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c < MAX_COLOR {
        c
    } else {
        MAX_COLOR - 1
    }
}
