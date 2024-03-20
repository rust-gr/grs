use gr_sys::gr::{GR_TEXT_ENABLE_INLINE_MATH, GR_TEXT_USE_WC};
use std::ffi::c_int;

pub(super) fn textx_opts(world_cooridnates: bool, inline_math: bool) -> c_int {
    match (world_cooridnates, inline_math) {
        (true, true) => GR_TEXT_USE_WC | GR_TEXT_ENABLE_INLINE_MATH,
        (true, false) => GR_TEXT_USE_WC,
        (false, true) => GR_TEXT_ENABLE_INLINE_MATH,
        (false, false) => 0,
    }
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
