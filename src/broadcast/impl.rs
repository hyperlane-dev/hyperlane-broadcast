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
    /// - `Broadcast<T>` - A broadcast instance with default sender capacity.
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
    /// - `Capacity` - The maximum number of messages that can be buffered.
    ///
    /// # Returns
    ///
    /// - `Broadcast<T>` - A new broadcast instance.
    #[inline]
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
    /// - `ReceiverCount` - The total count of active receivers.
    #[inline]
    pub fn receiver_count(&self) -> ReceiverCount {
        self.sender.receiver_count()
    }

    /// Subscribes a new receiver to the broadcast channel.
    ///
    /// # Returns
    ///
    /// - `BroadcastReceiver<T>` - A new receiver instance.
    #[inline]
    pub fn subscribe(&self) -> BroadcastReceiver<T> {
        self.sender.subscribe()
    }

    /// Sends a message to all active receivers subscribed to this broadcast channel.
    ///
    /// # Arguments
    ///
    /// - `T` - The message to be broadcasted.
    ///
    /// # Returns
    ///
    /// - `BroadcastSendResult<T>` - Result indicating send status.
    #[inline]
    pub fn send(&self, data: T) -> BroadcastSendResult<T> {
        self.sender.send(data)
    }
}
