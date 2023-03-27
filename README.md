# Sockit 🧦

[<img alt="github" src="https://img.shields.io/badge/github-wcygan/sockit-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcygan/sockit)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sockit.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/sockit)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-sockit-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/sockit)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcygan/sockit/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/wcygan/sockit/actions?query=branch%3Amain)

A high-level UDP Socket that allows for writing and reading (de)serializable values

# Usage

Add this to your Cargo.toml:

```toml
[dependencies]
sockit = "0.2.0"
```

You can create a Socket by binding it to an address like so:

```rust
#[tokio::main]
async fn main() {
  let socket = sockit::UdpSocket::bind("127.0.0.1:0").await?;
}
```

You can use the Socket to send and receive serializable objects:

```rust
use sockit::UdpSocket;
use serde::{Serialize, Deserialize};

/// A (de)serializable type shared between client and server
#[derive(Serialize, Deserialize)]
struct Message {
  id: u32,
  data: String,
}

/// Code running client side
async fn client_side(mut client_socket: UdpSocket) {
  let message = Message {
    id: 1,
    data: "Hello, world!".to_string(),
  };

  client_socket.write::<Message>(&message).await.unwrap();
}

/// Code running server side
async fn server_side(mut server_socket: UdpSocket) {
  let message: Message = server_socket.read::<Message>().await.unwrap().unwrap();
}
```

