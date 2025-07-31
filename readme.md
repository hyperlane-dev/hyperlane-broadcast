<center>

## hyperlane-broadcast

[![](https://img.shields.io/crates/v/hyperlane-broadcast.svg)](https://crates.io/crates/hyperlane-broadcast)
[![](https://img.shields.io/crates/d/hyperlane-broadcast.svg)](https://img.shields.io/crates/d/hyperlane-broadcast.svg)
[![](https://docs.rs/hyperlane-broadcast/badge.svg)](https://docs.rs/hyperlane-broadcast)
[![](https://github.com/hyperlane-utils/hyperlane-broadcast/workflows/Rust/badge.svg)](https://github.com/hyperlane-utils/hyperlane-broadcast/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane_broadcast.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-broadcast/)

[Api Docs](https://docs.rs/hyperlane-broadcast/latest/hyperlane_broadcast/)

> hyperlane-broadcast is a lightweight and ergonomic wrapper over Tokio’s broadcast channel designed for easy-to-use publish-subscribe messaging in async Rust applications. It simplifies the native Tokio broadcast API by providing a straightforward interface for broadcasting messages to multiple subscribers with minimal boilerplate.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-broadcast
```

## Use

```rust
use hyperlane_broadcast::*;

let broadcast: Broadcast<usize> = Broadcast::new(10);
let mut rec1: BroadcastReceiver<usize> = broadcast.subscribe();
let mut rec2: BroadcastReceiver<usize> = broadcast.subscribe();
broadcast.send(20).unwrap();
assert_eq!(rec1.recv().await, Ok(20));
assert_eq!(rec2.recv().await, Ok(20));

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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
