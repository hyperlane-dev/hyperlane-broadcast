use crate::*;

/// Implements the `BroadcastMapTrait` for any type that also implements `Clone` and `Debug`.
/// This blanket implementation allows any clonable and debuggable type to be used as a value in the broadcast map system.
impl<T: Clone + Debug> BroadcastMapTrait for T {}

/// Provides a default implementation for `BroadcastMap` instances.
///
/// The default broadcast map is initialized as an empty `DashMap`.
impl<T: BroadcastMapTrait> Default for BroadcastMap<T> {
    /// Creates a new, empty `BroadcastMap` instance.
    ///
    /// # Returns
    ///
    /// An empty `BroadcastMap`.
    fn default() -> Self {
        Self(DashMap::new())
    }
}

/// Implements core functionalities for the `BroadcastMap` struct.
impl<T: BroadcastMapTrait> BroadcastMap<T> {
    /// Creates a new, empty `BroadcastMap` instance.
    ///
    /// This is a convenience constructor that simply calls `default()`.
    ///
    /// # Returns
    ///
    /// An empty `BroadcastMap`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves an immutable reference to the underlying `DashMapStringBroadcast`.
    ///
    /// This private helper method provides direct access to the internal map.
    ///
    /// # Returns
    ///
    /// A reference to the `DashMapStringBroadcast` containing the broadcast channels.
    fn get(&self) -> &DashMapStringBroadcast<T> {
        &self.0
    }

    /// Inserts a new broadcast channel into the map with a specified key and capacity.
    ///
    /// If a broadcast channel with the given key already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// - `key` - The key (convertible to `String`) to associate with the new broadcast channel.
    /// - `capacity` - The maximum number of messages that can be buffered in the new broadcast channel.
    ///
    /// # Returns
    ///
    /// An `Option` containing the old `Broadcast` channel if one was replaced, otherwise `None`.
    pub fn insert<K>(&self, key: K, capacity: Capacity) -> OptionBroadcast<T>
    where
        K: ToString,
    {
        let key_string: String = key.to_string();
        let broadcast: Broadcast<T> = Broadcast::new(capacity);
        self.get().insert(key_string, broadcast)
    }

    /// Retrieves the number of active receivers for the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `key` - The key (convertible to `String`) of the broadcast channel.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `ReceiverCount` if the broadcast channel exists, otherwise `None`.
    pub fn receiver_count<K>(&self, key: K) -> OptionReceiverCount
    where
        K: ToString,
    {
        self.get()
            .get(&key.to_string())
            .map(|receiver| receiver.receiver_count())
    }

    /// Subscribes a new receiver to the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `key` - The key (convertible to `String`) of the broadcast channel.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `BroadcastMapReceiver` if the broadcast channel exists, otherwise `None`.
    pub fn subscribe<K>(&self, key: K) -> OptionBroadcastMapReceiver<T>
    where
        K: ToString,
    {
        self.get()
            .get(&key.to_string())
            .map(|receiver| receiver.subscribe())
    }

    /// Subscribes a new receiver to the broadcast channel associated with the given key.
    /// If the channel does not exist, it will be created with the specified capacity before subscribing.
    ///
    /// # Arguments
    ///
    /// - `key` - The key (convertible to `String`) of the broadcast channel.
    /// - `capacity` - The capacity to use if a new broadcast channel needs to be created.
    ///
    /// # Returns
    ///
    /// A `BroadcastMapReceiver` for the specified broadcast channel.
    pub fn subscribe_or_insert<K>(&self, key: K, capacity: Capacity) -> BroadcastMapReceiver<T>
    where
        K: ToString,
    {
        let key_string: String = key.to_string();
        match self.get().get(&key_string) {
            Some(sender) => sender.subscribe(),
            None => {
                self.insert(key, capacity);
                self.subscribe_or_insert(key_string, capacity)
            }
        }
    }

    /// Sends a message to the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `key` - The key (convertible to `String`) of the broadcast channel.
    /// - `data` - The message to be broadcasted.
    ///
    /// # Returns
    ///
    /// A `BroadcastMapSendResult` indicating the number of receivers the message was sent to (if the channel exists),
    /// or an error if the send operation failed. If the channel does not exist, `Ok(None)` is returned.
    pub fn send<K: ToString>(&self, key: K, data: T) -> BroadcastMapSendResult<T>
    where
        K: ToString,
    {
        match self.get().get(&key.to_string()) {
            Some(sender) => sender.send(data).map(|result| Some(result)),
            None => Ok(None),
        }
    }
}
