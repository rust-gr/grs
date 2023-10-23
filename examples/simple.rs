use std::io::{stdin, Read};
use grs::gks::{self, ws::GksRegenerationFlag};

fn main() {
    gks::scope(0, |gks| {
        let mut ws = gks.open_ws(1, None, None).unwrap();
        ws.active_scope(|ws| {
            let x = &[0., 1.];
            ws.polyline(x.len(), x, x)
                .expect("`x.len()` cannot be greater than number of elements in `x`");
        });
        ws.update(GksRegenerationFlag::Perform);
        stdin().bytes().next();
    });
}
