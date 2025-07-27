use crate::*;

/// Defines the essential traits required for types that can be used as values in a `BroadcastMap`.
///
/// Any type implementing `BroadcastMapTrait` must also implement `Clone` and `Debug`,
/// enabling efficient duplication and debugging within the broadcast map system.
pub trait BroadcastMapTrait: Clone + Debug {}
