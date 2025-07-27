use crate::*;

/// Implements the `BroadcastTrait` for any type that also implements `Clone` and `Debug`.
/// This blanket implementation allows any clonable and debuggable type to be used in the broadcast system.
impl<T: Clone + Debug> BroadcastTrait for T {}

/// Provides a default implementation for `Broadcast` instances.
///
/// The default broadcast channel is initialized with a predefined sender capacity.
impl<T: BroadcastTrait> Default for Broadcast<T> {
    /// Creates a new `Broadcast` instance with default settings.
    ///
    /// # Returns
    ///
    /// A `Broadcast` instance with a sender initialized to `DEFAULT_BROADCAST_SENDER_CAPACITY`
    /// and a capacity of 0.
    fn default() -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(DEFAULT_BROADCAST_SENDER_CAPACITY);
        Broadcast {
            capacity: 0,
            sender,
        }
    }
}

/// Implements core functionalities for the `Broadcast` struct.
impl<T: BroadcastTrait> Broadcast<T> {
    /// Creates a new `Broadcast` instance with a specified capacity.
    ///
    /// # Arguments
    ///
    /// - `capacity` - The maximum number of messages that can be buffered in the broadcast channel.
    ///
    /// # Returns
    ///
    /// A new `Broadcast` instance configured with the given capacity.
    pub fn new(capacity: Capacity) -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(capacity);
        let mut broadcast: Broadcast<T> = Broadcast::default();
        broadcast.sender = sender;
        broadcast.capacity = capacity;
        broadcast
    }

    /// Retrieves the current number of active receivers subscribed to this broadcast channel.
    ///
    /// # Returns
    ///
    /// The total count of active receivers.
    pub fn receiver_count(&self) -> ReceiverCount {
        self.sender.receiver_count()
    }

    /// Subscribes a new receiver to the broadcast channel.
    ///
    /// # Returns
    ///
    /// A `BroadcastReceiver` that can be used to receive messages sent through this broadcast channel.
    pub fn subscribe(&self) -> BroadcastReceiver<T> {
        self.sender.subscribe()
    }

    /// Sends a message to all active receivers subscribed to this broadcast channel.
    ///
    /// # Arguments
    ///
    /// - `data` - The message to be broadcasted.
    ///
    /// # Returns
    ///
    /// A `BroadcastSendResult` indicating the number of receivers the message was sent to,
    /// or a `BroadcastSendError` if the send operation failed.
    pub fn send(&self, data: T) -> BroadcastSendResult<T> {
        self.sender.send(data)
    }
}
