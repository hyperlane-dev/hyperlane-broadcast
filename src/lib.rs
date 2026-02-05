//! hyperlane-broadcast
//!
//! hyperlane-broadcast is a lightweight
//! and ergonomic wrapper over Tokio’s broadcast channel designed
//! for easy-to-use publish-subscribe messaging in async Rust applications.
//! It simplifies the native Tokio broadcast API by providing a straightforward
//! interface for broadcasting messages to multiple subscribers with minimal boilerplate.

mod broadcast;
mod broadcast_map;

pub use {broadcast::*, broadcast_map::*};

#[cfg(test)]
use std::time::Duration;
use std::{fmt::Debug, hash::BuildHasherDefault};

#[cfg(test)]
use tokio::{
    sync::broadcast::error::RecvError,
    time::{error::Elapsed, timeout},
};
use {
    dashmap::{mapref::one::Ref, *},
    tokio::sync::broadcast::{
        error::SendError,
        {Receiver, Sender},
    },
    twox_hash::XxHash3_64,
};
