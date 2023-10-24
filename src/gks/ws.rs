use super::Gks;
use crate::ffi::gks::{
    gks_clear_ws, gks_configure_ws, gks_message, gks_open_ws, gks_set_ws_viewport,
    gks_set_ws_window, gks_update_ws, GKS_K_CONID_DEFAULT, GKS_K_PERFORM_FLAG, GKS_K_POSTPONE_FLAG,
    GKS_K_WRITE_PAGE_FLAG, GKS_K_WSTYPE_DEFAULT,
};
use crate::ffi::gkscore::gks_errno;
use crate::util::f64range::F64Range;
use ::core::ffi::{c_int, CStr};
use ::core::mem;
use ::core::num::NonZeroI32;
use ::core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct GksActiveWs<'a> {
    gks: &'a mut Gks,
    id: NonZeroI32,
}

#[derive(Debug)]
pub struct GksWs<'a> {
    inner: GksActiveWs<'a>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GksRegenerationFlag {
    Postpone = GKS_K_POSTPONE_FLAG as isize,
    Perform = GKS_K_PERFORM_FLAG as isize,
    WritePage = GKS_K_WRITE_PAGE_FLAG as isize,
}

impl Gks {
    fn new_ws(&mut self, id: NonZeroI32) -> GksWs {
        GksWs { inner: GksActiveWs { gks: self, id } }
    }

    pub fn open_ws(
        &mut self,
        wkid: NonZeroI32,
        conid: Option<&CStr>,
        wtype: Option<NonZeroI32>,
    ) -> Option<GksWs> {
        let id = wkid.into();
        let conid = conid.map_or(GKS_K_CONID_DEFAULT, |s| s.as_ptr().cast_mut());
        let wtype = wtype.map_or(GKS_K_WSTYPE_DEFAULT, Into::into);
        let errno = unsafe {
            gks_open_ws(id, conid, wtype);
            mem::replace(&mut gks_errno, 0)
        };
        match errno {
            0 => {
                self.workstations.insert(wkid, false);
                Some(self.new_ws(wkid))
            }
            _ => None,
        }
    }

    pub fn ws(&mut self, wkid: NonZeroI32) -> Option<GksWs> {
        self.workstations.get(&wkid)?;
        Some(self.new_ws(wkid))
    }
}

impl GksActiveWs<'_> {
    pub fn id(&self) -> NonZeroI32 {
        self.id
    }

    pub fn update(&self, regfl: GksRegenerationFlag) {
        let wkid = self.id.into();
        let regfl = regfl as c_int;
        unsafe { gks_update_ws(wkid, regfl) }
    }

    pub fn configure(&self) {
        let wkid = self.id.into();
        unsafe { gks_configure_ws(wkid) }
    }

    pub fn clear(&self, cofl: c_int) {
        let wkid = self.id.into();
        unsafe { gks_clear_ws(wkid, cofl) }
    }

    pub fn set_window(&self, x: F64Range, y: F64Range) {
        let wkid = self.id.into();
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_ws_window(wkid, xmin, xmax, ymin, ymax) }
    }

    pub fn set_viewport(&self, x: F64Range, y: F64Range) {
        let wkid = self.id.into();
        let (xmin, xmax) = x.into();
        let (ymin, ymax) = y.into();
        unsafe { gks_set_ws_viewport(wkid, xmin, xmax, ymin, ymax) }
    }

    pub fn message(&self, s: impl AsRef<CStr>) {
        let wkid = self.id.into();
        let p = s.as_ref().as_ptr().cast_mut();
        unsafe { gks_message(wkid, p) }
    }
}

impl GksWs<'_> {
    pub fn close(self) {
        let ws = self.inner;
        let r = ws.gks.workstations.remove(&ws.id);
        debug_assert!(r == Some(false))
    }
}

impl<'a> Deref for GksWs<'a> {
    type Target = GksActiveWs<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for GksWs<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
