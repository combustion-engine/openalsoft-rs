#![feature(proc_macro)]
extern crate openalsoft_sys as als;
extern crate nalgebra;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod al;
pub use al::*;

pub use al::{ALDeviceArc, ALContextArc, ALListenerArc};