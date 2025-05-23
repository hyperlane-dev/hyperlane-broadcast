#[tokio::test]
pub async fn test_broadcast() {
    use crate::*;

    let broadcast: Broadcast<usize> = Broadcast::new(10);
    let mut rec1: BroadcastReceiver<usize> = broadcast.subscribe();
    let mut rec2: BroadcastReceiver<usize> = broadcast.subscribe();
    broadcast.send(20).unwrap();
    assert_eq!(rec1.recv().await, Ok(20));
    assert_eq!(rec2.recv().await, Ok(20));
}

#[tokio::test]
pub async fn test_broadcast_map() {
    use crate::*;

    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new("a", 10);
    let mut rec1: BroadcastMapReceiver<usize> = broadcast_map.subscribe("a").unwrap();
    let mut rec2: BroadcastMapReceiver<usize> = broadcast_map.subscribe("a").unwrap();
    broadcast_map.send("a", 20).unwrap();
    assert_eq!(rec1.recv().await, Ok(20));
    assert_eq!(rec2.recv().await, Ok(20));
}
