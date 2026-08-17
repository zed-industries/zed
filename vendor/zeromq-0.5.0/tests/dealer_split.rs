#[cfg(test)]
mod test {

    use zeromq::__async_rt as async_rt;
    use zeromq::prelude::*;
    use zeromq::ZmqMessage;

    use std::error::Error;
    use std::time::Duration;

    fn assert_send<T: Send>() {}

    #[test]
    fn split_halves_are_send() {
        assert_send::<zeromq::DealerSendHalf>();
        assert_send::<zeromq::DealerRecvHalf>();
    }

    #[async_rt::test]
    async fn test_dealer_split_concurrent_send_recv() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        // Use DEALER-to-DEALER (compatible pair, no envelope framing needed)
        let mut server = zeromq::DealerSocket::new();
        let endpoint = server.bind("tcp://localhost:0").await?;

        let mut client = zeromq::DealerSocket::new();
        client.connect(endpoint.to_string().as_str()).await?;

        // Give connection time to establish
        async_rt::task::sleep(Duration::from_millis(100)).await;

        let (mut send_half, mut recv_half) = client.split();

        let num_messages: u32 = 5;

        // Server echo loop
        let server_task = async_rt::task::spawn(async move {
            for _ in 0..num_messages {
                let msg = server.recv().await.unwrap();
                server.send(msg).await.unwrap();
            }
        });

        // Sender in its own task
        let send_task = async_rt::task::spawn(async move {
            for i in 0..num_messages {
                let msg = ZmqMessage::from(format!("msg-{}", i));
                send_half.send(msg).await.unwrap();
            }
        });

        // Receiver in its own task
        let recv_task = async_rt::task::spawn(async move {
            for _ in 0..num_messages {
                let _reply = recv_half.recv().await.unwrap();
            }
        });

        let timeout = Duration::from_secs(5);
        async_rt::task::timeout(timeout, async {
            send_task.await.unwrap();
            recv_task.await.unwrap();
            server_task.await.unwrap();
        })
        .await
        .expect("test timed out");

        Ok(())
    }
}
