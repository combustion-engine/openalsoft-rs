extern crate openalsoft_sys as als;
extern crate nalgebra;
#[macro_use]
extern crate lazy_static;

pub mod al;
pub use al::*;

pub use al::{ALDeviceArc, ALContextArc, ALListenerArc};