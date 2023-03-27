extern crate sockit;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use sockit::{UdpSocket, UdpSocketError};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestMessage {
        id: u32,
        name: String,
        payload: Vec<u8>,
    }

    async fn setup() -> (UdpSocket, UdpSocket) {
        let a = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let b = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        (a, b)
    }

    #[tokio::test]
    async fn write_and_read_message() -> Result<(), UdpSocketError> {
        let (mut a, mut b) = setup().await;

        let message = TestMessage {
            id: 123,
            name: "Test Message".to_string(),
            payload: vec![1, 2, 3, 4, 5],
        };

        // Make sure the size of the type fits into a UDP Datagram
        assert!(std::mem::size_of::<TestMessage>() < 512);

        a.write(&message, b.local_addr()?).await.unwrap();
        let (parsed_message, from) = b.read::<TestMessage>().await?;

        assert_eq!(from, a.local_addr()?);
        assert_eq!(message, parsed_message);
        Ok(())
    }
}
