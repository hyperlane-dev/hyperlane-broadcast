use crate::*;

/// Represents an error that occurs when attempting to send a message via a broadcast channel within a map.
pub type BroadcastMapSendError<T> = SendError<T>;

/// Represents a receiver endpoint for a broadcast channel within a map, allowing consumption of broadcasted messages.
pub type BroadcastMapReceiver<T> = Receiver<T>;

/// Represents a sender endpoint for a broadcast channel within a map, used to dispatch messages.
pub type BroadcastMapSender<T> = Sender<T>;

/// A concurrent, thread-safe map where keys are strings and values are broadcast channels.
pub type DashMapStringBroadcast<T> = DashMap<String, Broadcast<T>, BuildHasherDefault<XxHash3_64>>;
