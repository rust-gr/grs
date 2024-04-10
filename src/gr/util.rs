use gr_sys::gr::textx_option_t_GR_TEXT_ENABLE_INLINE_MATH as GR_TEXT_ENABLE_INLINE_MATH;
use gr_sys::gr::textx_option_t_GR_TEXT_USE_WC as GR_TEXT_USE_WC;
use std::ffi::c_int;

pub(super) fn textx_opts(world_cooridnates: bool, inline_math: bool) -> c_int {
    let r = match (world_cooridnates, inline_math) {
        (true, true) => GR_TEXT_USE_WC | GR_TEXT_ENABLE_INLINE_MATH,
        (true, false) => GR_TEXT_USE_WC,
        (false, true) => GR_TEXT_ENABLE_INLINE_MATH,
        (false, false) => 0,
    };
    r as _
}

macro_rules! impl_primitive_function {
    ($name:ident($($n:ident: $t:ty),*) $(-> $rt:ty)?) => {
        pub fn $name($($n: impl Into<$t>),*) $(-> $rt)? {
            $(let $n = $n.into();)*
            unsafe { paste!([<gr_$name>])($($n),*) }
        }
    };
}

pub(super) use impl_primitive_function;
