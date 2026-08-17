mod compliance;
use compliance::{get_monitor_event, setup_monitor};

#[cfg(test)]
mod test {

    use super::*;

    use std::convert::TryInto;
    use zeromq::__async_rt as async_rt;
    use zeromq::prelude::*;
    use zeromq::ZmqMessage;

    // Returns (socket, bound_endpoint, monitor)
    fn setup_their_rep(bind_endpoint: &str) -> (zmq2::Socket, String, zmq2::Socket) {
        let ctx = zmq2::Context::new();
        let their_rep = ctx.socket(zmq2::REP).expect("Couldn't make rep socket");
        their_rep.bind(bind_endpoint).expect("Failed to bind");

        let resolved_bind = their_rep.get_last_endpoint().unwrap().unwrap();

        let their_monitor = setup_monitor(&ctx, &their_rep, "inproc://their-monitor");

        (their_rep, resolved_bind, their_monitor)
    }

    async fn setup_our_req(bind_endpoint: &str) -> zeromq::ReqSocket {
        let mut our_req = zeromq::ReqSocket::new();
        our_req
            .connect(bind_endpoint)
            .await
            .expect("Failed to connect");
        our_req
    }

    fn run_their_rep(
        their_rep: zmq2::Socket,
        num_req: u32,
    ) -> std::thread::JoinHandle<zmq2::Socket> {
        assert_eq!(their_rep.get_socket_type().unwrap(), zmq2::REP);
        std::thread::spawn(move || {
            for i in 0..num_req {
                let request = their_rep.recv_msg(0).expect("Failed to recv");
                assert_eq!(request.as_str().unwrap(), format!("Request: {}", i));
                their_rep
                    .send(&format!("Reply: {}", i), 0)
                    .expect("Failed to send");
            }
            println!("Finished pub task");
            their_rep
        })
    }

    async fn run_our_req(our_req: &mut zeromq::ReqSocket, num_req: u32) {
        for i in 0..num_req {
            let ms: String = format!("Request: {}", i);
            let message = ZmqMessage::from(ms);
            our_req.send(message).await.expect("Failed to send");
            let reply = our_req.recv().await.expect("Failed to recv");

            let reply_payload: String = reply.try_into().unwrap();
            println!("Received reply: {}", &reply_payload);
            assert_eq!(reply_payload, format!("Reply: {}", i));
        }
    }

    #[async_rt::test]
    async fn test_their_rep_our_req() {
        let (their_rep, bind_endpoint, their_monitor) = setup_their_rep("tcp://127.0.0.1:0");
        println!("Their rep was bound to {}", bind_endpoint);

        let mut our_req = setup_our_req(&bind_endpoint).await;
        assert_eq!(
            zmq2::SocketEvent::ACCEPTED,
            get_monitor_event(&their_monitor).0
        );
        assert_eq!(
            zmq2::SocketEvent::HANDSHAKE_SUCCEEDED,
            get_monitor_event(&their_monitor).0
        );
        println!("Setup done");

        const NUM_MSGS: u32 = 64;

        let their_join_handle = run_their_rep(their_rep, NUM_MSGS);
        run_our_req(&mut our_req, NUM_MSGS).await;
        let _their_rep = their_join_handle
            .join()
            .expect("Their pub terminated with an error!");
        assert_eq!(our_req.close().await.len(), 0);
        // TODO: check that socket disconnected via monitor when we implement that
        // functionality
    }
}
