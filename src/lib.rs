//! hyperlane-broadcast
//!
//! hyperlane-broadcast is a lightweight
//! and ergonomic wrapper over Tokio’s broadcast channel designed
//! for easy-to-use publish-subscribe messaging in async Rust applications.
//! It simplifies the native Tokio broadcast API by providing a straightforward
//! interface for broadcasting messages to multiple subscribers with minimal boilerplate.

pub(crate) mod broadcast;
pub(crate) mod broadcast_map;
pub(crate) mod cfg;

pub use broadcast::{r#const::*, r#struct::*, r#trait::*, r#type::*};
pub use broadcast_map::{r#struct::*, r#trait::*, r#type::*};

pub(crate) use std::{fmt::Debug, hash::BuildHasherDefault};

pub(crate) use dashmap::*;
pub(crate) use tokio::sync::broadcast::{
    error::SendError,
    {Receiver, Sender},
};
pub(crate) use twox_hash::XxHash3_64;
