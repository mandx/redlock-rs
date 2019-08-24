use std::time::SystemTimeError;

use redis::RedisError;
use snafu::Snafu;

pub use snafu::ResultExt;

pub type RedlockResult<T> = Result<T, RedlockError>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")] // Default
pub enum RedlockError {
    #[snafu(display("Redis connection error: {:?}", source))]
    RedisConnection {
        source: RedisError,
    },
    #[snafu(display("Redis script error: {:?}", source))]
    RedisScript {
        source: RedisError,
    },
    TimeErr {
        source: SystemTimeError,
    },
    #[snafu(display("Redlock must be initialized with at least one redis server"))]
    NoServer,
    #[snafu(display("Redlock request timeout"))]
    Timeout,
    #[snafu(display("The lock has already expired"))]
    LockExpired,
    #[snafu(display("Unable to lock the resource"))]
    UnableToLock,
    #[snafu(display("Unable to unlock the resource"))]
    UnableToUnlock,
    #[snafu(display("Unable to extend the resource"))]
    UnableToExtend,
}
