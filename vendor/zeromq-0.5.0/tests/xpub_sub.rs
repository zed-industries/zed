#[cfg(test)]
mod test {
    use zeromq::prelude::*;
    use zeromq::ZmqMessage;
    use zeromq::__async_rt as async_rt;

    use std::time::Duration;

    #[async_rt::test]
    async fn test_xpub_basic_pubsub() {
        pretty_env_logger::try_init().ok();

        let mut xpub_socket = zeromq::XPubSocket::new();
        let bound_to = xpub_socket
            .bind("tcp://127.0.0.1:0")
            .await
            .expect("Failed to bind");

        let bound_addr = bound_to.to_string();

        // Spawn SUB socket
        let sub_handle = async_rt::task::spawn(async move {
            let mut sub_socket = zeromq::SubSocket::new();
            sub_socket
                .connect(&bound_addr)
                .await
                .expect("Failed to connect");

            sub_socket.subscribe("").await.expect("Failed to subscribe");

            // Wait a bit for subscription to propagate
            async_rt::task::sleep(Duration::from_millis(200)).await;

            // Receive 5 messages
            let mut received = Vec::new();
            for _ in 0..5 {
                let msg = sub_socket.recv().await.expect("Failed to receive");
                let data = String::from_utf8(msg.get(0).unwrap().to_vec()).unwrap();
                received.push(data);
            }
            received
        });

        // XPUB receives subscription message
        let sub_msg = async_rt::task::timeout(Duration::from_secs(2), xpub_socket.recv())
            .await
            .expect("Timeout waiting for subscription")
            .expect("Failed to receive subscription");

        let data = sub_msg.get(0).unwrap();
        assert_eq!(data[0], 1); // Subscribe byte
        assert_eq!(&data[1..], b""); // Empty subscription (subscribe to all)

        // Give time for subscription to be fully processed
        async_rt::task::sleep(Duration::from_millis(100)).await;

        // Send messages
        for i in 0..5 {
            let msg = ZmqMessage::from(format!("message-{}", i));
            xpub_socket.send(msg).await.expect("Failed to send");
        }

        // Wait for SUB to receive all messages
        let received = sub_handle.await.expect("SUB task failed");
        assert_eq!(received.len(), 5);
        for (i, msg) in received.iter().enumerate() {
            assert_eq!(msg, &format!("message-{}", i));
        }
    }

    #[async_rt::test]
    async fn test_xpub_receives_unsubscribe() {
        pretty_env_logger::try_init().ok();

        let mut xpub_socket = zeromq::XPubSocket::new();
        let bound_to = xpub_socket
            .bind("tcp://127.0.0.1:0")
            .await
            .expect("Failed to bind");

        let bound_addr = bound_to.to_string();
        let handle = async_rt::task::spawn(async move {
            let mut sub_socket = zeromq::SubSocket::new();
            sub_socket
                .connect(&bound_addr)
                .await
                .expect("Failed to connect");

            // Subscribe
            sub_socket
                .subscribe("test")
                .await
                .expect("Failed to subscribe");
            async_rt::task::sleep(Duration::from_millis(100)).await;

            // Unsubscribe
            sub_socket
                .unsubscribe("test")
                .await
                .expect("Failed to unsubscribe");
            async_rt::task::sleep(Duration::from_millis(100)).await;
        });

        // Receive subscribe message
        let sub_msg = async_rt::task::timeout(Duration::from_secs(2), xpub_socket.recv())
            .await
            .expect("Timeout")
            .expect("Failed to receive");

        let data = sub_msg.get(0).unwrap();
        assert_eq!(data[0], 1); // Subscribe byte
        assert_eq!(&data[1..], b"test");

        // Receive unsubscribe message
        let unsub_msg = async_rt::task::timeout(Duration::from_secs(2), xpub_socket.recv())
            .await
            .expect("Timeout")
            .expect("Failed to receive");

        let data = unsub_msg.get(0).unwrap();
        assert_eq!(data[0], 0); // Unsubscribe byte
        assert_eq!(&data[1..], b"test");

        handle.await.expect("Task failed");
    }

    #[async_rt::test]
    async fn test_xpub_filtered_subscriptions() {
        pretty_env_logger::try_init().ok();

        let mut xpub_socket = zeromq::XPubSocket::new();
        let bound_to = xpub_socket
            .bind("tcp://127.0.0.1:0")
            .await
            .expect("Failed to bind");

        let bound_addr = bound_to.to_string();

        // Spawn SUB socket that subscribes to "topic1"
        let sub_handle = async_rt::task::spawn(async move {
            let mut sub_socket = zeromq::SubSocket::new();
            sub_socket
                .connect(&bound_addr)
                .await
                .expect("Failed to connect");

            sub_socket
                .subscribe("topic1")
                .await
                .expect("Failed to subscribe");

            async_rt::task::sleep(Duration::from_millis(200)).await;

            // Should only receive messages starting with "topic1"
            let msg = sub_socket.recv().await.expect("Failed to receive");
            String::from_utf8(msg.get(0).unwrap().to_vec()).unwrap()
        });

        // Receive subscription
        let _sub_msg = async_rt::task::timeout(Duration::from_secs(2), xpub_socket.recv())
            .await
            .expect("Timeout")
            .expect("Failed to receive subscription");

        async_rt::task::sleep(Duration::from_millis(100)).await;

        // Send messages with different topics
        xpub_socket
            .send(ZmqMessage::from("topic2-message"))
            .await
            .expect("Failed to send");
        xpub_socket
            .send(ZmqMessage::from("topic1-message"))
            .await
            .expect("Failed to send");
        xpub_socket
            .send(ZmqMessage::from("topic3-message"))
            .await
            .expect("Failed to send");

        // SUB should only receive "topic1-message"
        let received = sub_handle.await.expect("SUB task failed");
        assert_eq!(received, "topic1-message");
    }
}
