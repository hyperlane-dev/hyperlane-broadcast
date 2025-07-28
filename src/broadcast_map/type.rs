use crate::*;

/// Represents an error that occurs when attempting to send a message via a broadcast channel within a map.
pub type BroadcastMapSendError<T> = SendError<T>;
/// Represents the result of a broadcast map send operation, indicating either success with an optional receiver count or an error.
pub type BroadcastMapSendResult<T> = Result<Option<ReceiverCount>, BroadcastMapSendError<T>>;
/// Represents a receiver endpoint for a broadcast channel within a map, allowing consumption of broadcasted messages.
pub type BroadcastMapReceiver<T> = Receiver<T>;
/// Represents an optional broadcast channel.
pub type OptionBroadcast<T> = Option<Broadcast<T>>;
/// Represents an optional receiver endpoint for a broadcast channel within a map.
pub type OptionBroadcastMapReceiver<T> = Option<BroadcastMapReceiver<T>>;
/// Represents a sender endpoint for a broadcast channel within a map, used to dispatch messages.
pub type BroadcastMapSender<T> = Sender<T>;
/// Represents an optional sender endpoint for a broadcast channel within a map.
pub type OptionBroadcastMapSender<T> = Option<BroadcastMapSender<T>>;
/// Represents an optional count of active receivers.
pub type OptionReceiverCount = Option<ReceiverCount>;
/// A concurrent, thread-safe map where keys are strings and values are broadcast channels.
pub type DashMapStringBroadcast<T> = DashMap<String, Broadcast<T>, BuildHasherDefault<XxHash3_64>>;
