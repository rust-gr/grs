#![allow(warnings)]

pub mod gks {
    include!(concat!(env!("OUT_DIR"), "/gks.rs"));
}

pub mod gkscore {
    include!(concat!(env!("OUT_DIR"), "/gkscore.rs"));
}

pub mod gr {
    include!(concat!(env!("OUT_DIR"), "/gr.rs"));
}

pub mod strlib {
    include!(concat!(env!("OUT_DIR"), "/strlib.rs"));
}
