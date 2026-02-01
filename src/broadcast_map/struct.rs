use crate::*;

/// Represents a concurrent, thread-safe map of broadcast channels, keyed by string.
///
/// This struct provides a way to manage multiple broadcast channels, each identified by a unique string,
/// allowing for dynamic creation, retrieval, and management of broadcast streams.
#[derive(Clone, Debug)]
pub struct BroadcastMap<T: BroadcastTrait>(pub(super) DashMapStringBroadcast<T>);
