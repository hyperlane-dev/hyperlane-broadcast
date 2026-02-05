use crate::*;

#[tokio::test]
pub async fn test_broadcast_map() {
    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec2: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec3: BroadcastMapReceiver<usize> =
        broadcast_map.subscribe_or_insert("another_key", DEFAULT_BROADCAST_SENDER_CAPACITY);
    broadcast_map.send("test_key", 20).unwrap();
    broadcast_map.send("another_key", 10).unwrap();
    assert_eq!(rec1.recv().await, Ok(20));
    assert_eq!(rec2.recv().await, Ok(20));
    assert_eq!(rec3.recv().await, Ok(10));
}

#[tokio::test]
pub async fn test_broadcast_map_unsubscribe() {
    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    let removed: Option<Broadcast<usize>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    drop(removed);
    let not_exist: Option<Broadcast<usize>> = broadcast_map.unsubscribe("nonexistent_key");
    assert!(not_exist.is_none());
    assert!(broadcast_map.subscribe("test_key").is_none());
    let send_result: Result<Option<usize>, SendError<usize>> =
        broadcast_map.try_send("test_key", 30);
    assert!(send_result.unwrap().is_none());
    let result: Result<Result<usize, RecvError>, Elapsed> =
        timeout(Duration::from_millis(100), rec1.recv()).await;
    assert!(result.is_ok(), "recv should not timeout after unsubscribe");
    assert_eq!(result.unwrap(), Err(RecvError::Closed));
}

#[tokio::test]
pub async fn test_broadcast_map_unsubscribe_and_reinsert() {
    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    broadcast_map.subscribe("test_key").unwrap();
    let removed: Option<Broadcast<usize>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    broadcast_map.insert("test_key", 10);
    let mut rec2: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    broadcast_map.send("test_key", 100).unwrap();
    assert_eq!(rec2.recv().await, Ok(100));
}

#[tokio::test]
pub async fn test_broadcast_map_unsubscribe_receiver_count() {
    let broadcast_map: BroadcastMap<String> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let _rec1: BroadcastMapReceiver<String> = broadcast_map.subscribe("test_key").unwrap();
    let _rec2: BroadcastMapReceiver<String> = broadcast_map.subscribe("test_key").unwrap();
    assert_eq!(broadcast_map.receiver_count("test_key"), Some(2));
    let removed: Option<Broadcast<String>> = broadcast_map.unsubscribe("test_key");
    assert!(removed.is_some());
    assert_eq!(broadcast_map.receiver_count("test_key"), None);
}

#[tokio::test]
pub async fn test_broadcast_map_send() {
    let broadcast_map: BroadcastMap<usize> = BroadcastMap::new();
    broadcast_map.insert("test_key", 10);
    let mut rec1: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    let mut rec2: BroadcastMapReceiver<usize> = broadcast_map.subscribe("test_key").unwrap();
    let count: Option<ReceiverCount> = broadcast_map.send("test_key", 42);
    assert_eq!(count, Some(2));
    assert_eq!(rec1.recv().await, Ok(42));
    assert_eq!(rec2.recv().await, Ok(42));
    let non_existent: Option<ReceiverCount> = broadcast_map.send("non_existent_key", 100);
    assert_eq!(non_existent, None);
}
