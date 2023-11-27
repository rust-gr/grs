pub mod ctx;
mod macros;
pub mod out;
pub mod ws;

#[derive(Debug)]
pub struct Gks(());

#[derive(Debug)]
pub struct ActiveGks(Gks);

#[derive(Debug)]
pub struct SegmentGks(ActiveGks);
