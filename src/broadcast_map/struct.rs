use crate::*;

#[derive(Debug, Clone, Setter, Getter)]
pub struct BroadcastMap<T: BroadcastTrait> {
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) capacity: DashMap<String, usize>,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) sender: DashMap<String, BroadcastSender<T>>,
}
