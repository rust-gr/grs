use super::bindings::gks::{
    gks_activate_ws, gks_close_ws, gks_deactivate_ws, gks_open_ws, gks_update_ws,
    GKS_K_CONID_DEFAULT,
};
use super::bindings::gkscore::gks_errno;
use super::Gks;
use ::core::ffi::c_int;
use ::core::num::NonZeroI32;
use ::core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct GksWs(NonZeroI32);
pub struct GksUnactiveWs(GksWs);
pub struct GksActiveWs<'a>(&'a mut GksUnactiveWs);

#[derive(Debug)]
pub enum GksOpenWsError {
    InvalidState,
    InvalidId,
    InvalidType,
    WorkstationAlreadyOpen,
    IndependentSegmentStorageAlreadyOpen,
    OpenFailed,
    Unknown,
}

#[derive(Debug)]
pub enum GksRegenerationFlag {
    Postpone,
    Perform,
    WritePage,
}

impl Gks {
    pub fn open_ws(
        &self,
        wkid: c_int,
        conid: Option<&str>,
        wtype: c_int,
    ) -> Result<GksUnactiveWs, GksOpenWsError> {
        let errno = unsafe {
            gks_open_ws(
                wkid,
                conid.map_or(GKS_K_CONID_DEFAULT, |s| s.as_ptr() as *mut i8),
                wtype,
            );
            gks_errno
        };
        if errno == 0 {
            let wkid = unsafe { NonZeroI32::new_unchecked(wkid) };
            return Ok(GksUnactiveWs(GksWs(wkid)));
        }
        unsafe { gks_errno = 0 }
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
    pub fn wkid(&self) -> NonZeroI32 {
        self.0
    }

    pub fn update(&mut self, regfl: GksRegenerationFlag) {
        let regfl = match regfl {
            GksRegenerationFlag::Postpone => 0,
            GksRegenerationFlag::Perform => 1,
            GksRegenerationFlag::WritePage => 0,
        };
        unsafe { gks_update_ws(self.0.into(), regfl) }
    }
}

impl Drop for GksWs {
    fn drop(&mut self) {
        let wkid = self.0.into();
        unsafe { gks_close_ws(wkid) }
    }
}

impl GksUnactiveWs {
    pub fn activate(&mut self) -> GksActiveWs {
        let wkid = self.0 .0.into();
        unsafe { gks_activate_ws(wkid) }
        GksActiveWs(self)
    }

    pub fn active_scope<R>(&mut self, f: impl FnOnce(GksActiveWs) -> R) -> R {
        f(self.activate())
    }
}

impl Deref for GksUnactiveWs {
    type Target = GksWs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GksUnactiveWs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> GksActiveWs<'a> {
    pub fn deactivate(self) -> &'a mut GksUnactiveWs {
        let p = self.0 as *mut GksUnactiveWs;
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
