use grs::gks::{ws::GksRegenerationFlag, Gks};
use std::io::{stdin, Read};

fn run() -> Option<()> {
    let wkid = 1.try_into().ok()?;
    let x = &[0., 1.];
    let mut gks = Gks::open(0)?;
    gks.open_ws(wkid, None, None)?;
    let mut gks = gks.activate(wkid).ok()?;
    gks.polyline(x.len(), x, x).ok()?;
    let mut gks = gks.deactivate_all();
    gks.ws(wkid)?.update(GksRegenerationFlag::Perform);
    stdin().bytes().next();
    Some(())
}

fn main() {
    run().unwrap();
}
