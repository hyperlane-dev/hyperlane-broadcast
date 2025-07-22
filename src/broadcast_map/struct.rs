use crate::*;

#[derive(Debug, Clone)]
pub struct BroadcastMap<T: BroadcastTrait>(pub(super) DashMapStringBroadcast<T>);
