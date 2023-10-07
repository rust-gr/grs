use ::core::fmt::{self, Debug, Formatter};
use ::core::ops::Deref;
use ::std::sync::{Mutex, MutexGuard, TryLockError};
use super::bindings;

pub const GKS: GksState = GksState::new();

pub struct Gks;
pub struct GksState(Mutex<bool>);
pub struct GksContext<'a>(MutexGuard<'a, bool>);

pub enum GksTryLockError {
    Closed,
    WouldBlock,
}

impl Gks {
    pub unsafe fn assume_context() -> Self {
        Gks
    }
}

impl GksState {
    const fn new() -> Self {
        Self(Mutex::new(false))
    }

    pub fn open(&self, errfill: ::core::ffi::c_int) -> Option<GksContext> {
        let mut guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        match *guard {
            true => None,
            false => {
                unsafe { bindings::gks::gks_open_gks(errfill) }
                *guard = true;
                Some(GksContext::new(guard))
            },
        }
    }

    pub fn try_open(&self, errfill: ::core::ffi::c_int) -> Option<GksContext> {
        let result = self.0.try_lock();
        let mut guard = match result {
            Ok(guard) if *guard => return None,
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(err)) => err.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        unsafe { bindings::gks::gks_open_gks(errfill) }
        *guard = true;
        Some(GksContext::new(guard))
    }

    pub fn lock(&self) -> Option<GksContext> {
        let guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        match *guard {
            true => Some(GksContext::new(guard)),
            false => None
        }
    }

    pub fn try_lock(&self) -> Result<GksContext, GksTryLockError> {
        match self.0.try_lock() {
            Ok(guard) if *guard => Ok(GksContext::new(guard)),
            Ok(_) => Err(GksTryLockError::Closed),
            Err(TryLockError::Poisoned(err)) => Ok(GksContext::new(err.into_inner())),
            Err(TryLockError::WouldBlock) => Err(GksTryLockError::WouldBlock),
        }
    }

    pub fn lock_and_open(&self, errfill: ::core::ffi::c_int) -> GksContext {
        let mut guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if !*guard {
            unsafe { bindings::gks::gks_open_gks(errfill) }
            *guard = true;
        }
        GksContext::new(guard)
    }

    pub fn try_lock_and_open(&self, errfill: ::core::ffi::c_int) -> Option<GksContext> {
        let mut guard = match self.0.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(err)) => err.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        if !*guard {
            unsafe { bindings::gks::gks_open_gks(errfill) }
            *guard = true;
        }
        Some(GksContext::new(guard))
    }
}

impl Debug for GksState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("GksState")
         .field(
            "initialized",
            match self.try_lock() {
                Ok(_) => &true,
                Err(GksTryLockError::Closed) => &false,
                Err(GksTryLockError::WouldBlock) => &"<locked>",
            }
         )
         .finish()
    }
}

impl Drop for GksState {
    fn drop(&mut self) {
        self.lock().map(GksContext::close);
    }
}

impl<'a> GksContext<'a> {
    fn new(guard: MutexGuard<'a, bool>) -> Self {
        Self(guard)
    }

    pub fn close(mut self) {
        unsafe { bindings::gks::gks_close_gks() }
        *self.0 = false;
    }
}

impl Deref for GksContext<'_> {
    type Target = Gks;

    fn deref(&self) -> &Self::Target {
        &Gks
    }
}
