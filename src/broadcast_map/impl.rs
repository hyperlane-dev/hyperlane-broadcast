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
    /// - `BroadcastMap<T>` - An empty broadcast map.
    fn default() -> Self {
        Self(DashMap::with_hasher(BuildHasherDefault::default()))
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
    /// - `BroadcastMap<T>` - An empty broadcast map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves an immutable reference to the underlying `DashMapStringBroadcast`.
    ///
    /// This private helper method provides direct access to the internal map.
    ///
    /// # Returns
    ///
    /// - `&DashMapStringBroadcast<T>` - Reference to the internal map.
    fn get(&self) -> &DashMapStringBroadcast<T> {
        &self.0
    }

    /// Inserts a new broadcast channel into the map with a specified key and capacity.
    ///
    /// If a broadcast channel with the given key already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Key convertible to `str`.
    /// - `capacity` - Maximum number of buffered messages.
    ///
    /// # Returns
    ///
    /// - `Option<Broadcast<T>>` - Previous broadcast channel if replaced.
    pub fn insert<K>(&self, key: K, capacity: Capacity) -> OptionBroadcast<T>
    where
        K: AsRef<str>,
    {
        let broadcast: Broadcast<T> = Broadcast::new(capacity);
        self.get().insert(key.as_ref().to_owned(), broadcast)
    }

    /// Retrieves the number of active receivers for the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Key convertible to `str`.
    ///
    /// # Returns
    ///
    /// - `Option<ReceiverCount>` - Number of receivers if channel exists.
    pub fn receiver_count<K>(&self, key: K) -> OptionReceiverCount
    where
        K: AsRef<str>,
    {
        self.get()
            .get(key.as_ref())
            .map(|receiver| receiver.receiver_count())
    }

    /// Subscribes a new receiver to the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Key convertible to `str`.
    ///
    /// # Returns
    ///
    /// - `Option<BroadcastReceiver<T>>` - New receiver if channel exists.
    pub fn subscribe<K>(&self, key: K) -> OptionBroadcastMapReceiver<T>
    where
        K: AsRef<str>,
    {
        self.get()
            .get(key.as_ref())
            .map(|receiver| receiver.subscribe())
    }

    /// Subscribes a new receiver to the broadcast channel associated with the given key.
    /// If the channel does not exist, it will be created with the specified capacity before subscribing.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Key convertible to `str`.
    /// - `capacity` - Capacity for new channel if needed.
    ///
    /// # Returns
    ///
    /// - `BroadcastReceiver<T>` - New receiver for the channel.
    pub fn subscribe_or_insert<K>(&self, key: K, capacity: Capacity) -> BroadcastMapReceiver<T>
    where
        K: AsRef<str>,
    {
        let key_ref: &str = key.as_ref();
        match self.get().get(key_ref) {
            Some(sender) => sender.subscribe(),
            None => {
                self.insert(key_ref, capacity);
                self.subscribe_or_insert(key_ref, capacity)
            }
        }
    }

    /// Sends a message to the broadcast channel associated with the given key.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Key convertible to `str`.
    /// - `data` - Message to broadcast.
    ///
    /// # Returns
    ///
    /// - `Result<Option<ReceiverCount>, SendError<T>>` - Send result with receiver count or error.
    pub fn send<K: AsRef<str>>(&self, key: K, data: T) -> BroadcastMapSendResult<T>
    where
        K: AsRef<str>,
    {
        match self.get().get(key.as_ref()) {
            Some(sender) => sender.send(data).map(|result| Some(result)),
            None => Ok(None),
        }
    }
}
