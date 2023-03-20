//! A high-level UDP Socket that allows for writing and reading (de)serializable values
//!
//! # Example
//!
//! ```
//! use sockit::UdpSocket;
//! use serde::{Deserialize, Serialize};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!  // Create UDP Sockets
//!  let (mut a, mut b): (UdpSocket, UdpSocket) = setup().await;
//!
//!  let message = TestMessage {
//!     id: 123,
//!     name: "Test Message".to_string(),
//!     payload: vec![1, 2, 3, 4, 5]
//!  };
//!
//!  // Write a message to b
//!  a.write(&message, b.local_addr()?).await?;
//!
//!  // Read a message from b's socket
//!  let (parsed_message, from) = b.read::<TestMessage>().await?;
//!
//!  // Assert that the message was sent from a and that it matches the original message
//!  assert_eq!(from, a.local_addr()?);
//!  assert_eq!(message, parsed_message);
//!
//!  Ok(())
//! }
//!
//! // ... boilerplate
//!
//! #[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
//! struct TestMessage { id: u32, name: String, payload: Vec<u8> }
//!
//! async fn setup() -> (UdpSocket, UdpSocket) {
//!    let a = UdpSocket::bind("127.0.0.1:0").await.unwrap();
//!    let b = UdpSocket::bind("127.0.0.1:0").await.unwrap();
//!    (a, b)
//! }
//! ```
use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::ToSocketAddrs;

#[derive(Error, Debug)]
pub enum UdpSocketError {
    #[error("`{0}`")]
    IoError(std::io::Error),
    #[error("`{0}`")]
    BincodeError(bincode::Error),
}

/// A high-level UDP Socket that allows for writing and reading (de)serializable values
pub struct UdpSocket {
    buffer: [u8; 512],
    socket: tokio::net::UdpSocket,
}

impl UdpSocket {
    /// Attempt to create a new [`UdpSocket`] by binding it to the provided address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sockit::UdpSocket;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///   Ok(())
    /// }
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, UdpSocketError> {
        let socket = tokio::net::UdpSocket::bind(addr).await?;
        Ok(Self::new(socket))
    }

    /// Create a new UDP socket from an existing [`tokio::net::UdpSocket`]
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sockit::UdpSocket;
    /// use tokio::net::UdpSocket as TokioUdpSocket;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let tokio_socket = TokioUdpSocket::bind("127.0.0.1:0").await?;
    ///   let mut sockit_socket = UdpSocket::new(tokio_socket);
    ///   Ok(())
    /// }
    pub fn new(socket: tokio::net::UdpSocket) -> Self {
        let buffer = [0; 512];
        Self { buffer, socket }
    }

    /// Write a serializable value to the socket
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sockit::UdpSocket;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let mut socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///   socket.write(&"Hello World!", "127.0.0.1:9090".parse()?).await?;
    ///   Ok(())
    /// }
    ///```
    pub async fn write<T: Serialize>(
        &mut self,
        value: &T,
        send_to: SocketAddr,
    ) -> Result<(), UdpSocketError> {
        let buf = serialize(value)?;
        self.socket.send_to(buf.as_slice(), send_to).await?;
        Ok(())
    }

    /// Read a deserializable value from a single datagram on the socket
    ///
    /// This method returns an error when it isn't possible to deserialize the value
    /// from the datagram. This can happen if the value doesn't fit into a single datagram.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sockit::UdpSocket;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let mut socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///   let message = socket.read::<String>().await?;
    ///   Ok(())
    /// }
    ///```
    pub async fn read<T: DeserializeOwned>(&mut self) -> Result<(T, SocketAddr), UdpSocketError> {
        let (_, src) = self.socket.recv_from(&mut self.buffer).await?;
        let value = deserialize::<T>(self.buffer.as_slice())?;
        Ok((value, src))
    }

    /// Get the local address of the socket
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sockit::UdpSocket;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///  let socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///  let addr = socket.local_addr()?;
    ///  Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> Result<SocketAddr, UdpSocketError> {
        Ok(self.socket.local_addr()?)
    }
}

impl From<Box<bincode::ErrorKind>> for UdpSocketError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        UdpSocketError::BincodeError(e)
    }
}

impl From<std::io::Error> for UdpSocketError {
    fn from(e: std::io::Error) -> Self {
        UdpSocketError::IoError(e)
    }
}
