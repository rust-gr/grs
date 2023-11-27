pub mod ctx;
pub mod out;
pub mod ws;
pub mod seg;
mod util;

#[derive(Debug)]
pub struct Gks(());

#[derive(Debug)]
pub struct ActiveGks(Gks);

#[derive(Debug)]
pub struct SegmentGks(ActiveGks);
