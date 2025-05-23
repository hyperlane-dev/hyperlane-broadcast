use crate::*;

pub type BroadcastMapSendError<T> = SendError<T>;
pub type BroadcastMapSendResult<T> = Result<Option<ReceiverCount>, BroadcastMapSendError<T>>;
pub type BroadcastMapReceiver<T> = Receiver<T>;
pub type OptionBroadcastMapReceiver<T> = Option<BroadcastMapReceiver<T>>;
pub type BroadcastMapSender<T> = Sender<T>;
pub type OptionBroadcastMapSender<T> = Option<BroadcastMapSender<T>>;
pub type OptionReceiverCount = Option<ReceiverCount>;
