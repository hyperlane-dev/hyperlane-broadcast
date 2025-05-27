use crate::*;

impl<T: Clone + Debug> BroadcastMapTrait for T {}

impl<T: BroadcastMapTrait> Default for BroadcastMap<T> {
    fn default() -> Self {
        BroadcastMap {
            capacity: DashMap::new(),
            sender: DashMap::new(),
        }
    }
}

impl<T: BroadcastMapTrait> BroadcastMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K>(&self, key: K, capacity: usize) -> Self
    where
        K: ToString,
    {
        let key_string: String = key.to_string();
        let sender: BroadcastSender<T> = BroadcastSender::new(capacity);
        let broadcast: BroadcastMap<T> = BroadcastMap::default();
        broadcast.get_sender().insert(key_string.clone(), sender);
        broadcast.get_capacity().insert(key_string, capacity);
        broadcast
    }

    pub fn receiver_count(&self, key: &str) -> OptionReceiverCount {
        self.get_sender()
            .get(key)
            .map(|receiver| receiver.receiver_count())
    }

    pub fn subscribe(&self, key: &str) -> OptionBroadcastMapReceiver<T> {
        self.get_sender()
            .get(key)
            .map(|receiver| receiver.subscribe())
    }

    pub fn subscribe_unwrap_or_insert(&self, key: &str) -> BroadcastMapReceiver<T> {
        match self.get_sender().get(key) {
            Some(sender) => sender.subscribe(),
            None => {
                let new_sender: Sender<T> = BroadcastSender::new(DEFAULT_BROADCAST_SENDER_CAPACITY);
                self.get_sender()
                    .insert(key.to_string(), new_sender.clone());
                self.get_capacity()
                    .insert(key.to_string(), DEFAULT_BROADCAST_SENDER_CAPACITY);
                new_sender.subscribe()
            }
        }
    }

    pub fn send(&self, key: &str, data: T) -> BroadcastMapSendResult<T> {
        match self.get_sender().get(key) {
            Some(sender) => sender.send(data).map(|result| Some(result)),
            None => Ok(None),
        }
    }
}
