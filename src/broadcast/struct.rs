use crate::*;

/// Represents a broadcast mechanism for sending messages to multiple receivers.
///
/// This struct encapsulates the core components required for broadcasting,
/// including the capacity of the broadcast channel and the sender responsible
/// for dispatching messages.
#[derive(Debug, Clone)]
pub struct Broadcast<T: BroadcastTrait> {
    /// The maximum number of messages that can be buffered in the broadcast channel.
    pub(super) capacity: Capacity,
    /// The sender component responsible for distributing messages to all connected receivers.
    pub(super) sender: BroadcastSender<T>,
}
