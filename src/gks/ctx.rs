use super::Gks;
use crate::ffi::gks::{gks_emergency_close, gks_open_gks};
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
                Some(Self)
            }
        }
    }

    pub unsafe fn assume_open() -> Option<Self> {
        match mem::replace(lock().deref_mut(), true) {
            true => None,
            false => Some(Self),
        }
    }
}

impl Drop for Gks {
    fn drop(&mut self) {
        unsafe { gks_emergency_close() }
        *lock() = false;
    }
}
