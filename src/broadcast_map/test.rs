use crate::*;

#[tokio::test]
pub async fn test_broadcast_map() {
    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new();
    broadcast_map.insert("a", 10);
    let mut rec1: BroadcastMapReceiver<usize> = broadcast_map.subscribe("a").unwrap();
    let mut rec2: BroadcastMapReceiver<usize> = broadcast_map.subscribe("a").unwrap();
    let mut rec3: BroadcastMapReceiver<usize> =
        broadcast_map.subscribe_or_insert("b", DEFAULT_BROADCAST_SENDER_CAPACITY);
    broadcast_map.send("a", 20).unwrap();
    broadcast_map.send("b", 10).unwrap();
    assert_eq!(rec1.recv().await, Ok(20));
    assert_eq!(rec2.recv().await, Ok(20));
    assert_eq!(rec3.recv().await, Ok(10));
}
