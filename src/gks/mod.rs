pub mod ctx;
pub mod ws;

use ::core::num::NonZeroI32;
use ::std::collections::HashMap;

#[derive(Debug)]
pub struct Gks {
    workstations: HashMap<NonZeroI32, bool>,
}
