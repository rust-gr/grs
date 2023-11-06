use super::Gks;
use crate::ffi::gks::{
    gks_emergency_close, gks_open_gks, gks_set_resize_behaviour, gks_set_viewport, gks_set_window,
};
use crate::util::f64range::F64Range;
use ::core::ffi::c_int;
use ::core::mem;
use ::core::ops::DerefMut;
use ::std::sync::{Mutex, MutexGuard};

fn lock() -> MutexGuard<'static, bool> {
    static GKS_MUTEX: Mutex<bool> = Mutex::new(false);
    let result = GKS_MUTEX.lock();
    unsafe { result.unwrap_unchecked() } // poison impossible
}

impl Gks {
    pub fn open(errfil: c_int) -> Option<Self> {
        match mem::replace(lock().deref_mut(), true) {
            true => None,
            false => {
                unsafe { gks_open_gks(errfil) }
                Some(Self(()))
            }
        }
    }

    pub unsafe fn assume_open() -> Option<Self> {
        match mem::replace(lock().deref_mut(), true) {
            true => None,
            false => Some(Self(())),
        }
    }

    pub fn set_window(&mut self, tnr: c_int, x: F64Range, y: F64Range) {
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_window(tnr, xmin, xmax, ymin, ymax) }
    }

    pub fn set_viewport(&mut self, tnr: c_int, x: F64Range, y: F64Range) {
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_viewport(tnr, xmin, xmax, ymin, ymax) }
    }

    pub fn set_resize_behavior(&mut self, resize: bool) {
        let flag = resize.into();
        unsafe { gks_set_resize_behaviour(flag) }
    }
}

impl Drop for Gks {
    fn drop(&mut self) {
        unsafe { gks_emergency_close() }
        *lock() = false;
    }
}
