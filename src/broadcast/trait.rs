use crate::*;

/// Defines the essential traits required for types that can be broadcast.
///
/// Any type implementing `BroadcastTrait` must also implement `Clone` and `Debug`,
/// enabling efficient duplication and debugging within the broadcast system.
pub trait BroadcastTrait: Clone + Debug {}
