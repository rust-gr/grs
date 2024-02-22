use core::ffi::c_int;
use core::mem::MaybeUninit;
use gr_sys::gr::gr_inqlinetype;
use gr_sys::gr::gr_setlinetype;
use paste::paste;

macro_rules! impl_primitve_set_inq {
    ($name:ident, $type:ty) => {
        paste! {
            pub fn [<set$name>](val: impl Into<$type>) {
                let val = val.into();
                unsafe { [<gr_set$name>](val) };
            }

            pub fn [<inq$name>]() -> $type {
                let mut val = MaybeUninit::uninit();
                unsafe {
                    [<gr_inq$name>](val.as_mut_ptr());
                    val.assume_init()
                }
            }
        }
    };
}

impl_primitve_set_inq! {linetype, c_int}
