use gr_sys::gks::{gks_inq_operating_state, GKS_K_GKCL, GKS_K_SGOP, GKS_K_WSAC, GKS_K_WSOP};

pub(super) enum GksState {
    Closed = GKS_K_GKCL as _,
    Open = GKS_K_WSOP as _,
    Active = GKS_K_WSAC as _,
    SegmentOpen = GKS_K_SGOP as _,
}

pub(super) fn query_state() -> GksState {
    let mut state = 0;
    unsafe { gks_inq_operating_state(&mut state) }
    match state {
        GKS_K_GKCL => GksState::Closed,
        GKS_K_WSOP => GksState::Open,
        GKS_K_WSAC => GksState::Active,
        GKS_K_SGOP => GksState::SegmentOpen,
        _ => unreachable!(),
    }
}

macro_rules! impl_each {
    (($($types:ident),+) $body:tt) => {
        $(
            impl $types $body
        )+
    };
}

pub(super) use impl_each;
