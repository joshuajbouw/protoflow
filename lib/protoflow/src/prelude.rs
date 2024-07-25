// This is free and unencumbered software released into the public domain.

extern crate alloc;

pub use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};

pub use core::{
    fmt,
    marker::PhantomData,
    ops::Range,
    result::Result,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

#[cfg(all(feature = "std", not(feature = "tokio")))]
pub use std::time::Instant;
#[cfg(feature = "tokio")]
pub use tokio::time::Instant;
