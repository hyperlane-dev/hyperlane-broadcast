use crate::*;

impl<T: Clone + Debug> BroadcastMapTrait for T {}

impl<'a, T: BroadcastMapTrait> Default for BroadcastMap<'a, T> {
    fn default() -> Self {
        BroadcastMap {
            capacity: DashMap::new(),
            sender: DashMap::new(),
        }
    }
}

impl<'a, T: BroadcastMapTrait> BroadcastMap<'a, T> {
    pub fn new(key: &'a str, capacity: usize) -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(capacity);
        let broadcast: BroadcastMap<T> = BroadcastMap::default();
        let sender_map: &DashMap<&str, Sender<T>> = broadcast.get_sender();
        let capacity_map: &DashMap<&str, usize> = broadcast.get_capacity();
        sender_map.insert(key, sender);
        capacity_map.insert(key, capacity);
        broadcast
    }

    pub fn receiver_count(&self, key: &'a str) -> OptionReceiverCount {
        self.get_sender()
            .get(key)
            .map(|receiver| receiver.receiver_count())
    }

    pub fn subscribe(&self, key: &'a str) -> OptionBroadcastMapReceiver<T> {
        self.get_sender()
            .get(key)
            .map(|receiver| receiver.subscribe())
    }

    pub fn send(&self, key: &'a str, data: T) -> BroadcastMapSendResult<T> {
        match self.get_sender().get(key) {
            Some(sender) => sender.send(data).map(|result| Some(result)),
            None => Ok(None),
        }
    }
}
