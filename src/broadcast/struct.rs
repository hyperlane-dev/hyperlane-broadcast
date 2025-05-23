use crate::*;

#[derive(Debug, Clone, Setter, Getter)]
pub struct Broadcast<T: BroadcastTrait> {
    pub(super) capacity: usize,
    pub(super) sender: BroadcastSender<T>,
}
