use grs::gks::{GksRegenerationFlag, Gks};
use std::io::{stdin, Read};

fn main() {
    let wkid = 1;
    let x = &[0., 1.];
    let mut gks = Gks::open(0).unwrap();
    gks.open_ws(wkid, None, None).unwrap();
    let mut gks = gks.activate(wkid).unwrap();
    gks.polyline(x.len(), x, x).unwrap();
    let mut gks = gks.deactivate_all();
    gks.ws(wkid)
        .expect("workstation should still exist")
        .update(GksRegenerationFlag::Perform);
    stdin().bytes().next();
}
