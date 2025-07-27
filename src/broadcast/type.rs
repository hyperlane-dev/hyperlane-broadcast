use crate::*;

/// Represents the number of active receivers subscribed to a broadcast channel.
pub type ReceiverCount = usize;
/// Represents an error that occurs when attempting to send a message via broadcast.
pub type BroadcastSendError<T> = SendError<T>;
/// Represents the result of a broadcast send operation, indicating either success with the number of receivers or an error.
pub type BroadcastSendResult<T> = Result<ReceiverCount, BroadcastSendError<T>>;
/// Represents a receiver endpoint for a broadcast channel, allowing consumption of broadcasted messages.
pub type BroadcastReceiver<T> = Receiver<T>;
/// Represents a sender endpoint for a broadcast channel, used to dispatch messages to all subscribed receivers.
pub type BroadcastSender<T> = Sender<T>;
/// Represents the maximum capacity or buffer size of a broadcast channel.
pub type Capacity = usize;
