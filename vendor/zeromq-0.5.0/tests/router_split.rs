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
        assert_send::<zeromq::RouterSendHalf>();
        assert_send::<zeromq::RouterRecvHalf>();
    }

    #[async_rt::test]
    async fn test_router_split_concurrent_send_recv() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        // ROUTER binds, DEALER connects
        let mut router = zeromq::RouterSocket::new();
        let endpoint = router.bind("tcp://localhost:0").await?;

        let mut dealer = zeromq::DealerSocket::new();
        dealer.connect(endpoint.to_string().as_str()).await?;

        async_rt::task::sleep(Duration::from_millis(100)).await;

        // Split the router into send/recv halves
        let (mut send_half, mut recv_half) = router.split();

        let num_messages: u32 = 5;

        // Dealer sends messages
        let dealer_task = async_rt::task::spawn(async move {
            for i in 0..num_messages {
                dealer
                    .send(ZmqMessage::from(format!("msg-{}", i)))
                    .await
                    .unwrap();
            }
            // Wait for all replies
            for _ in 0..num_messages {
                let _reply = dealer.recv().await.unwrap();
            }
        });

        // Router recv half receives identity-framed messages
        // Router send half echoes them back using the identity
        let echo_task = async_rt::task::spawn(async move {
            for _ in 0..num_messages {
                // recv gives [identity, ...payload]
                let msg = recv_half.recv().await.unwrap();
                // send expects [identity, ...payload] to route back
                send_half.send(msg).await.unwrap();
            }
        });

        let timeout = Duration::from_secs(5);
        async_rt::task::timeout(timeout, async {
            echo_task.await.unwrap();
            dealer_task.await.unwrap();
        })
        .await
        .expect("test timed out");

        Ok(())
    }
}
