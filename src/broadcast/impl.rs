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
    pub fn new(capacity: usize) -> Self {
        let sender: BroadcastSender<T> = BroadcastSender::new(capacity);
        let mut broadcast: Broadcast<T> = Broadcast::default();
        broadcast.set_sender(sender);
        broadcast.set_capacity(capacity);
        broadcast
    }

    pub fn receiver_count(&self) -> usize {
        self.get_sender().receiver_count()
    }

    pub fn subscribe(&self) -> BroadcastReceiver<T> {
        self.get_sender().subscribe()
    }

    pub fn send(&self, data: T) -> BroadcastSendResult<T> {
        self.get_sender().send(data)
    }
}
