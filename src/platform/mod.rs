#[cfg(feature = "cortex-m7")]
mod cortex_m7;

#[cfg(feature = "cortex-m7")]
pub use cortex_m7::*;

pub trait ISpawner {}

pub trait IMessenger {}

pub trait IScheduler {}
