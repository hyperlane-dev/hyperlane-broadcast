use crate::*;

pub type ReceiverCount = usize;
pub type BroadcastSendError<T> = SendError<T>;
pub type BroadcastSendResult<T> = Result<ReceiverCount, BroadcastSendError<T>>;
pub type BroadcastReceiver<T> = Receiver<T>;
pub type BroadcastSender<T> = Sender<T>;
