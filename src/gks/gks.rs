use ::core::ops::Deref;
use ::std::sync::{Mutex, MutexGuard, TryLockError};
use super::bindings::gks::{gks_open_gks, gks_close_gks};

static GKS_MUTEX: Mutex<bool> = Mutex::new(false);

pub struct Gks;
pub struct GksGuard(MutexGuard<'static, bool>);
pub enum GksTryLockError {
    Closed,
    WouldBlock,
}

pub fn scope<R>(errfill: ::core::ffi::c_int, f: impl FnOnce(Gks) -> R) -> R {
    struct DropGuard(MutexGuard<'static, bool>);
    impl Drop for DropGuard {
        fn drop(&mut self) {
            if !*self.0 {
                unsafe { gks_close_gks() }
            }
        }
    }
    let guard = DropGuard(GKS_MUTEX.lock().unwrap_or_else(|e| e.into_inner()));
    if !*guard.0 {
        unsafe { gks_open_gks(errfill) }
    }
    f(Gks)
}

impl Gks {
    pub unsafe fn assume_context() -> Gks {
        Gks
    }

    pub fn open(errfill: ::core::ffi::c_int) -> Option<GksGuard> {
        let mut guard = GKS_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        match *guard {
            true => None,
            false => {
                unsafe { gks_open_gks(errfill) }
                *guard = true;
                Some(GksGuard::new(guard))
            },
        }
    }

    pub fn try_open(errfill: ::core::ffi::c_int) -> Option<GksGuard> {
        let mut guard = match GKS_MUTEX.try_lock() {
            Ok(guard) if *guard => return None,
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(err)) => err.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        unsafe { gks_open_gks(errfill) }
        *guard = true;
        Some(GksGuard::new(guard))
    }

    pub fn lock() -> Option<GksGuard> {
        let guard = GKS_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        match *guard {
            true => Some(GksGuard::new(guard)),
            false => None
        }
    }

    pub fn try_lock() -> Result<GksGuard, GksTryLockError> {
        match GKS_MUTEX.try_lock() {
            Ok(guard) if *guard => Ok(GksGuard::new(guard)),
            Ok(_) => Err(GksTryLockError::Closed),
            Err(TryLockError::Poisoned(err)) => Ok(GksGuard::new(err.into_inner())),
            Err(TryLockError::WouldBlock) => Err(GksTryLockError::WouldBlock),
        }
    }

    pub fn opened(errfill: ::core::ffi::c_int) -> GksGuard {
        let mut guard = GKS_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        if !*guard {
            unsafe { gks_open_gks(errfill) }
            *guard = true;
        }
        GksGuard::new(guard)
    }

    pub fn try_opened(errfill: ::core::ffi::c_int) -> Option<GksGuard> {
        let mut guard = match GKS_MUTEX.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(err)) => err.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        if !*guard {
            unsafe { gks_open_gks(errfill) }
            *guard = true;
        }
        Some(GksGuard::new(guard))
    }
}

impl GksGuard {
    fn new(guard: MutexGuard<'static, bool>) -> Self {
        Self(guard)
    }

    pub fn close(mut self) {
        unsafe { gks_close_gks() }
        *self.0 = false;
    }
}

impl Deref for GksGuard {
    type Target = Gks;

    fn deref(&self) -> &Self::Target {
        &Gks
    }
}
