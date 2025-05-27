use crate::*;

#[derive(Debug, Clone, Setter, Getter)]
pub struct Broadcast<T: BroadcastTrait> {
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) capacity: usize,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) sender: BroadcastSender<T>,
}
