use crate::*;

#[derive(Debug, Clone, Setter, Getter)]
pub struct BroadcastMap<'a, T: BroadcastTrait> {
    pub(super) capacity: DashMap<&'a str, usize>,
    pub(super) sender: DashMap<&'a str, BroadcastSender<T>>,
}
