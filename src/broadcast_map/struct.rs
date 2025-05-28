use crate::*;

#[derive(Debug, Clone, Setter, Getter)]
pub struct BroadcastMap<T: BroadcastTrait> {
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) broadcast: DashMap<String, Broadcast<T>>,
}
