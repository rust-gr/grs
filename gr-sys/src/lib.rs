#![allow(warnings)]

pub mod gks {
    include!(concat!(env!("OUT_DIR"), "/gks.rs"));
}

pub mod gkscore {
    include!(concat!(env!("OUT_DIR"), "/gkscore.rs"));
}
