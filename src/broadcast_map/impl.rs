use crate::*;

impl<T: Clone + Debug> BroadcastMapTrait for T {}

impl<T: BroadcastMapTrait> Default for BroadcastMap<T> {
    fn default() -> Self {
        BroadcastMap {
            broadcast: DashMap::new(),
        }
    }
}

impl<T: BroadcastMapTrait> BroadcastMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K>(&self, key: K, capacity: usize) -> OptionBroadcast<T>
    where
        K: ToString,
    {
        let key_string: String = key.to_string();
        let broadcast: Broadcast<T> = Broadcast::new(capacity);
        self.broadcast.insert(key_string, broadcast)
    }

    pub fn receiver_count(&self, key: &str) -> OptionReceiverCount {
        self.broadcast
            .get(key)
            .map(|receiver| receiver.receiver_count())
    }

    pub fn subscribe(&self, key: &str) -> OptionBroadcastMapReceiver<T> {
        self.broadcast.get(key).map(|receiver| receiver.subscribe())
    }

    pub fn subscribe_unwrap_or_insert(&self, key: &str) -> BroadcastMapReceiver<T> {
        match self.broadcast.get(key) {
            Some(sender) => sender.subscribe(),
            None => {
                self.insert(key, DEFAULT_BROADCAST_SENDER_CAPACITY);
                self.subscribe_unwrap_or_insert(key)
            }
        }
    }

    pub fn send(&self, key: &str, data: T) -> BroadcastMapSendResult<T> {
        match self.broadcast.get(key) {
            Some(sender) => sender.send(data).map(|result| Some(result)),
            None => Ok(None),
        }
    }
}
