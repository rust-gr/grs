mod ctx;
mod out;
mod ws;
mod seg;
mod util;

pub use out::*;
pub use ws::*;

#[derive(Debug)]
pub struct Gks(());

#[derive(Debug)]
pub struct ActiveGks(Gks);

#[derive(Debug)]
pub struct SegmentGks(ActiveGks);
