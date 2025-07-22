use crate::*;

impl<T: Clone + Debug> BroadcastMapTrait for T {}

impl<T: BroadcastMapTrait> Default for BroadcastMap<T> {
    fn default() -> Self {
        Self(DashMap::new())
    }
}

impl<T: BroadcastMapTrait> BroadcastMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self) -> &DashMapStringBroadcast<T> {
        &self.0
    }

    pub fn insert<K>(&self, key: K, capacity: Capacity) -> OptionBroadcast<T>
    where
        K: ToString,
    {
        let key_string: String = key.to_string();
        let broadcast: Broadcast<T> = Broadcast::new(capacity);
        self.get().insert(key_string, broadcast)
    }

    pub fn receiver_count<K>(&self, key: K) -> OptionReceiverCount
    where
        K: ToString,
    {
        self.get()
            .get(&key.to_string())
            .map(|receiver| receiver.receiver_count())
    }

    pub fn subscribe<K>(&self, key: K) -> OptionBroadcastMapReceiver<T>
    where
        K: ToString,
    {
        self.get()
            .get(&key.to_string())
            .map(|receiver| receiver.subscribe())
    }

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
