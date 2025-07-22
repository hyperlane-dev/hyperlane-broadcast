use crate::*;

impl<T: Clone + Debug> BroadcastTrait for T {}

impl<T: BroadcastTrait> Default for Broadcast<T> {
    fn default() -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(DEFAULT_BROADCAST_SENDER_CAPACITY);
        Broadcast {
            capacity: 0,
            sender,
        }
    }
}

impl<T: BroadcastTrait> Broadcast<T> {
    pub fn new(capacity: Capacity) -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(capacity);
        let mut broadcast: Broadcast<T> = Broadcast::default();
        broadcast.sender = sender;
        broadcast.capacity = capacity;
        broadcast
    }

    pub fn receiver_count(&self) -> ReceiverCount {
        self.sender.receiver_count()
    }

    pub fn subscribe(&self) -> BroadcastReceiver<T> {
        self.sender.subscribe()
    }

    pub fn send(&self, data: T) -> BroadcastSendResult<T> {
        self.sender.send(data)
    }
}
