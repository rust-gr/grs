use grs::gr;
use std::{
    ffi::{CStr, CString},
    io::{stdin, Read},
};

struct Ticker {
    invocations: usize,
}
impl Ticker {
    const fn new() -> Self {
        Ticker { invocations: 0 }
    }

    fn callback<'a, 'b>(&'a mut self, pos: (f64, f64), _s: &'b CStr, v: f64) {
        self.invocations += 1;
        let s = format!("{v} ({})", self.invocations);
        let s = CString::new(s).unwrap();
        gr::text(pos, s);
    }
}

fn main() {
    let x = &[0., 1.];
    let mut ticker = Ticker::new();
    gr::polyline(2, x, x).unwrap();
    gr::axeslbl(
        (0.1, 0.1),
        (0.0, 0.0),
        (Some(5), Some(10)),
        0.02,
        Some(&mut |p, s, v| ticker.callback(p, s, v)),
        None,
    );
    stdin().bytes().next();
}
