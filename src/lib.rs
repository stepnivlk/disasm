#[macro_use]
extern crate serde_derive;

pub mod dasm;

pub use dasm::InstrSet;
pub use dasm::CpuDesc;
pub use dasm::Buffer;
