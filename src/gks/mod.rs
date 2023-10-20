use ::core::fmt;

pub mod ctx;
pub mod out;
pub mod ws;

pub use ctx::Gks;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GksError;
type Result<T> = ::core::result::Result<T, GksError>;

impl fmt::Display for GksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("error in GKS")
    }
}
