use crate::*;

#[derive(Debug, Clone)]
pub struct Broadcast<T: BroadcastTrait> {
    pub(super) capacity: Capacity,
    pub(super) sender: BroadcastSender<T>,
}
