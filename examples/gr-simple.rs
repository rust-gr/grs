use grs::gr;
use std::io::{stdin, Read};

fn main() {
    let x = &[0., 1.];
    gr::polyline(2, x, x).unwrap();
    stdin().bytes().next();
}
