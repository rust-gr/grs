use super::util::{impl_each, query_state, GksState};
use super::{ActiveGks, Gks, SegmentGks};
use core::ffi::c_int;
use gr_sys::gks::{
    gks_assoc_seg_with_ws, gks_close_seg, gks_copy_seg_to_ws, gks_create_seg, gks_delete_seg,
    gks_redraw_seg_on_ws, gks_set_seg_xform,
};

impl ActiveGks {
    pub fn create_segment(self, segn: impl Into<c_int>) -> Result<SegmentGks, ActiveGks> {
        let segn = segn.into();
        unsafe { gks_create_seg(segn) }
        match query_state() {
            GksState::SegmentOpen => Ok(SegmentGks(self)),
            GksState::Active => Err(self),
            _ => unreachable!(),
        }
    }
}

impl SegmentGks {
    pub fn close_segment(self) -> ActiveGks {
        unsafe { gks_close_seg() }
        self.0
    }
}

impl_each! {(ActiveGks, SegmentGks) {
    pub fn delete_segment(&mut self, segn: impl Into<c_int>) {
        let segn = segn.into();
        unsafe { gks_delete_seg(segn) }
    }

    pub fn assoc_seg_with_ws(
        &mut self,
        wkid: impl Into<c_int>,
        segn: impl Into<c_int>
    ) {
        let wkid = wkid.into();
        let segn = segn.into();
        unsafe { gks_assoc_seg_with_ws(wkid, segn) }
    }

    pub fn copy_seg_to_ws(
        &mut self,
        wkid: impl Into<c_int>,
        segn: impl Into<c_int>
    ) {
        let wkid = wkid.into();
        let segn = segn.into();
        unsafe { gks_copy_seg_to_ws(wkid, segn) }
    }
}}

impl_each! {(ActiveGks, Gks, SegmentGks) {
    pub fn set_seg_xform(&mut self, segn: impl Into<c_int>, mat: &[f64]) {
        if mat.len() >= 6 {
            let segn = segn.into();
            let mat = mat.as_ptr().cast_mut().cast();
            unsafe { gks_set_seg_xform(segn, mat) }
        }
    }

    pub fn redraw_seg_on_ws(&mut self, wkid: impl Into<c_int>) {
        let wkid = wkid.into();
        unsafe { gks_redraw_seg_on_ws(wkid) }
    }
}}
