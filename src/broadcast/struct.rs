use crate::*;

/// Represents a broadcast mechanism for sending messages to multiple receivers.
///
/// This struct encapsulates the core components required for broadcasting,
/// including the capacity of the broadcast channel and the sender responsible
/// for dispatching messages.
#[derive(Debug, Clone)]
pub struct Broadcast<T: BroadcastTrait>(pub(super) BroadcastSender<T>);
