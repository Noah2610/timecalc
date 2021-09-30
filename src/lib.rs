extern crate regex;

pub mod calc;
pub mod time;

mod parse;
mod time_result;

pub use time::Time;
pub use time_result::{TimeError, TimeResult};
