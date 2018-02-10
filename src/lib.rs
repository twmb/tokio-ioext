//! This library provides functions that are similar but not equal to `tokio-io` functions.
//!
//! These functions can be more specialized or provide more control for performing asynchronous io.

extern crate futures;
#[macro_use]
extern crate tokio_io;

mod try_read_full;
mod read_lim;

pub use try_read_full::*;
pub use read_lim::*;
