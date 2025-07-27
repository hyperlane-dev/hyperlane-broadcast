//! hyperlane-broadcast
//!
//! hyperlane-broadcast is a lightweight
//! and ergonomic wrapper over Tokio’s broadcast channel designed
//! for easy-to-use publish-subscribe messaging in async Rust applications.
//! It simplifies the native Tokio broadcast API by providing a straightforward
//! interface for broadcasting messages to multiple subscribers with minimal boilerplate.

/// Internal module for core broadcast functionalities.
pub(crate) mod broadcast;
/// Internal module for managing broadcast maps.
pub(crate) mod broadcast_map;
/// Internal module for configuration.
pub(crate) mod cfg;

/// Re-exports constants, structs, traits, and types from the `broadcast` module.
pub use broadcast::{r#const::*, r#struct::*, r#trait::*, r#type::*};
/// Re-exports structs, traits, and types from the `broadcast_map` module.
pub use broadcast_map::{r#struct::*, r#trait::*, r#type::*};

/// Re-exports `dashmap` for concurrent hash map functionalities.
pub(crate) use dashmap::*;
/// Re-exports `Debug` trait for debugging purposes.
pub(crate) use std::fmt::Debug;
/// Re-exports `Receiver` and `Sender` from `tokio::sync::broadcast` for asynchronous message passing.
pub(crate) use tokio::sync::broadcast::{
    error::SendError,
    {Receiver, Sender},
};
