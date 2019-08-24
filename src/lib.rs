//! A rust redlock implementation for distributed, highly-available redis locks.

mod errors;
mod redlock;
mod scripts;
mod util;

pub use crate::errors::{RedlockResult, RedlockError};
pub use crate::redlock::{Config, Lock, Redlock};
