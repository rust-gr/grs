use super::{ActiveGks, Gks};
use crate::ffi::gks::{
    gks_activate_ws, gks_clear_ws, gks_close_ws, gks_configure_ws, gks_deactivate_ws,
    gks_inq_active_ws, gks_inq_operating_state, gks_inq_ws_conntype, gks_message, gks_open_ws,
    gks_set_ws_viewport, gks_set_ws_window, gks_update_ws, GKS_K_CONID_DEFAULT, GKS_K_ERROR,
    GKS_K_GKCL, GKS_K_NO_ERROR, GKS_K_PERFORM_FLAG, GKS_K_POSTPONE_FLAG, GKS_K_SGOP,
    GKS_K_WRITE_PAGE_FLAG, GKS_K_WSAC, GKS_K_WSOP, GKS_K_WSTYPE_DEFAULT,
};
use crate::util::f64range::F64Range;
use ::core::ffi::{c_int, CStr};
use ::core::marker::PhantomData;
use ::core::num::NonZeroI32;
use ::core::ops::{Deref, DerefMut};
use ::core::ptr::addr_of_mut;

struct GksWsData {
    #[allow(dead_code)]
    conid: c_int,
    #[allow(dead_code)]
    wtype: c_int,
}

enum GksState {
    Closed = GKS_K_GKCL as isize,
    Open = GKS_K_WSOP as isize,
    Active = GKS_K_WSAC as isize,
    SegmentOpen = GKS_K_SGOP as isize,
}

#[derive(Debug)]
pub enum MaybeActive {
    Active(ActiveGks),
    Inactive(Gks),
}

#[derive(Debug)]
pub struct GksWs<'a> {
    inner: ActiveGksWs<'a>,
}

#[derive(Debug)]
pub struct ActiveGksWs<'a> {
    gks: PhantomData<&'a mut Gks>,
    id: NonZeroI32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GksRegenerationFlag {
    Postpone = GKS_K_POSTPONE_FLAG as isize,
    Perform = GKS_K_PERFORM_FLAG as isize,
    WritePage = GKS_K_WRITE_PAGE_FLAG as isize,
}

fn query_state() -> GksState {
    let mut state = 0;
    let stateptr = addr_of_mut!(state);
    unsafe { gks_inq_operating_state(stateptr) }
    match state {
        GKS_K_GKCL => GksState::Closed,
        GKS_K_WSOP => GksState::Open,
        GKS_K_WSAC => GksState::Active,
        GKS_K_SGOP => GksState::SegmentOpen,
        _ => unreachable!(),
    }
}

fn query_ws(id: NonZeroI32) -> Option<GksWsData> {
    let id = id.into();
    let mut errind = 0;
    let mut conid = 0;
    let mut wtype = 0;
    let errptr = addr_of_mut!(errind);
    let conptr = addr_of_mut!(conid);
    let typeptr = addr_of_mut!(wtype);
    unsafe { gks_inq_ws_conntype(id, errptr, conptr, typeptr) }
    match errind {
        GKS_K_ERROR => None,
        GKS_K_NO_ERROR => Some(GksWsData { conid, wtype }),
        _ => unreachable!(),
    }
}

impl Gks {
    fn new_ws(&mut self, id: NonZeroI32) -> GksWs {
        GksWs {
            inner: ActiveGksWs {
                gks: Default::default(),
                id,
            },
        }
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
        let None = query_ws(wkid) else {
            return None;
        };
        unsafe { gks_open_ws(id, conid, wtype) }
        self.ws(wkid)
    }

    pub fn ws(&mut self, wkid: NonZeroI32) -> Option<GksWs> {
        query_ws(wkid).map(|_| self.new_ws(wkid))
    }

    pub fn activate(self, wkid: NonZeroI32) -> Result<ActiveGks, Gks> {
        let id = wkid.into();
        unsafe { gks_activate_ws(id) }
        match query_state() {
            GksState::Active => Ok(ActiveGks(self)),
            _ => Err(self),
        }
    }
}

impl ActiveGks {
    pub fn activate(&mut self, wkid: NonZeroI32) {
        let id = wkid.into();
        unsafe { gks_activate_ws(id) }
    }

    pub fn deactivate(self, wkid: NonZeroI32) -> MaybeActive {
        let id = wkid.into();
        unsafe { gks_deactivate_ws(id) }
        match query_state() {
            GksState::Open => MaybeActive::Inactive(self.0),
            _ => MaybeActive::Active(self),
        }
    }

    pub fn deactivate_all(self) -> Gks {
        let mut errind = 0;
        let mut remaining = 0;
        let mut wkid = 0;
        let errptr = addr_of_mut!(errind);
        let remptr = addr_of_mut!(remaining);
        let idptr = addr_of_mut!(wkid);
        loop {
            unsafe { gks_inq_active_ws(1, errptr, remptr, idptr) }
            if remaining == 0 {
                break self.0;
            }
            unsafe { gks_deactivate_ws(wkid) }
        }
    }
}

impl Deref for ActiveGks {
    type Target = Gks;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ActiveGks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GksWs<'_> {
    pub fn close(self) {
        let id = self.id.into();
        unsafe { gks_close_ws(id) }
    }
}

impl<'a> Deref for GksWs<'a> {
    type Target = ActiveGksWs<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for GksWs<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ActiveGksWs<'_> {
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
