use super::Gks;
use crate::ffi::gks::{
    gks_activate_ws, gks_clear_ws, gks_close_ws, gks_configure_ws, gks_deactivate_ws, gks_message,
    gks_open_ws, gks_set_ws_viewport, gks_set_ws_window, gks_update_ws, GKS_K_CONID_DEFAULT,
    GKS_K_PERFORM_FLAG, GKS_K_POSTPONE_FLAG, GKS_K_WRITE_PAGE_FLAG, GKS_K_WSTYPE_DEFAULT,
};
use crate::ffi::gkscore::gks_errno;
use crate::util::f64range::F64Range;
use ::core::ffi::{c_int, CStr};
use ::core::num::NonZeroI32;
use ::core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct GksWs(NonZeroI32);
pub struct GksInactiveWs(GksWs);
pub struct GksActiveWs<'a>(&'a mut GksInactiveWs);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GksOpenWsError {
    InvalidState,
    InvalidId,
    InvalidType,
    WorkstationAlreadyOpen,
    IndependentSegmentStorageAlreadyOpen,
    OpenFailed,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GksRegenerationFlag {
    Postpone = GKS_K_POSTPONE_FLAG as isize,
    Perform = GKS_K_PERFORM_FLAG as isize,
    WritePage = GKS_K_WRITE_PAGE_FLAG as isize,
}

impl Gks {
    pub fn open_ws(
        &self,
        wkid: c_int,
        conid: Option<&CStr>,
        wtype: Option<NonZeroI32>,
    ) -> Result<GksInactiveWs, GksOpenWsError> {
        let errno = unsafe {
            gks_open_ws(
                wkid,
                conid.map_or(GKS_K_CONID_DEFAULT, |s| s.as_ptr().cast_mut()),
                wtype.map_or(GKS_K_WSTYPE_DEFAULT, Into::into),
            );
            gks_errno
        };
        if errno == 0 {
            let wkid = unsafe { NonZeroI32::new_unchecked(wkid) };
            return Ok(GksInactiveWs(GksWs(wkid)));
        }
        unsafe {
            gks_errno = 0;
        }
        Err(match errno {
            8 => GksOpenWsError::InvalidState,
            20 => GksOpenWsError::InvalidId,
            22 => GksOpenWsError::InvalidType,
            24 => GksOpenWsError::WorkstationAlreadyOpen,
            28 => GksOpenWsError::IndependentSegmentStorageAlreadyOpen,
            901 => GksOpenWsError::OpenFailed,
            _ => GksOpenWsError::Unknown,
        })
    }
}

impl GksWs {
    pub fn id(&self) -> NonZeroI32 {
        self.0
    }

    pub fn update(&mut self, regfl: GksRegenerationFlag) {
        let wkid = self.0.into();
        unsafe { gks_update_ws(wkid, regfl as c_int) }
    }

    pub fn configure(&mut self) {
        let wkid = self.0.into();
        unsafe { gks_configure_ws(wkid) }
    }

    pub fn clear(&mut self, cofl: c_int) {
        let wkid = self.0.into();
        unsafe { gks_clear_ws(wkid, cofl) }
    }

    pub fn set_window(&mut self, x: F64Range, y: F64Range) -> bool {
        let valid = 0f64 <= x.min() && x.max() <= 1f64 && 0f64 <= y.min() && y.max() <= 1f64;
        if valid {
            let wkid = self.0.into();
            unsafe { gks_set_ws_window(wkid, x.min(), x.max(), y.min(), y.max()) }
        }
        valid
    }

    pub fn set_viewport(&mut self, x: F64Range, y: F64Range) {
        let wkid = self.0.into();
        unsafe { gks_set_ws_viewport(wkid, x.min(), x.max(), y.min(), y.max()) }
    }

    pub fn message(&mut self, s: impl AsRef<CStr>) {
        let wkid = self.0.into();
        let p = s.as_ref().as_ptr().cast_mut();
        unsafe { gks_message(wkid, p) }
    }
}

impl Drop for GksWs {
    fn drop(&mut self) {
        let wkid = self.0.into();
        unsafe { gks_close_ws(wkid) }
    }
}

impl GksInactiveWs {
    pub fn activate(&mut self) -> GksActiveWs {
        let wkid = self.0 .0.into();
        unsafe { gks_activate_ws(wkid) }
        GksActiveWs(self)
    }

    pub fn active_scope<R>(&mut self, f: impl FnOnce(GksActiveWs) -> R) -> R {
        f(self.activate())
    }
}

impl Deref for GksInactiveWs {
    type Target = GksWs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GksInactiveWs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> GksActiveWs<'a> {
    pub fn deactivate(self) -> &'a mut GksInactiveWs {
        let p: *mut GksInactiveWs = self.0;
        drop(self); // needed to avoid 'technically-UB'
        unsafe { p.as_mut().unwrap_unchecked() }
    }
}

impl Deref for GksActiveWs<'_> {
    type Target = GksWs;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for GksActiveWs<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for GksActiveWs<'_> {
    fn drop(&mut self) {
        let wkid = self.0 .0 .0.into();
        unsafe { gks_deactivate_ws(wkid) }
    }
}
