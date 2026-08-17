mod compliance;
use compliance::{get_monitor_event, setup_monitor};

use zeromq::__async_rt as async_rt;
use zeromq::prelude::*;

use std::time::Duration;

fn setup_their_pub(bind_endpoint: &str) -> (zmq2::Socket, String, zmq2::Socket) {
    let ctx = zmq2::Context::new();
    let their_pub = ctx.socket(zmq2::PUB).expect("Couldn't make pub socket");
    their_pub.set_ipv6(true).expect("Failed to enable IPV6"); // IPV6 off by default
    their_pub.bind(bind_endpoint).expect("Failed to bind");

    let resolved_bind = their_pub.get_last_endpoint().unwrap().unwrap();

    let their_monitor = setup_monitor(&ctx, &their_pub, "inproc://their-monitor");

    (their_pub, resolved_bind, their_monitor)
}

async fn setup_our_subs(bind_endpoint: &str, n_subs: u8) -> Vec<zeromq::SubSocket> {
    let mut our_subs = Vec::new();
    for _ in 0..n_subs {
        let mut our_sub = zeromq::SubSocket::new();
        our_sub
            .connect(bind_endpoint)
            .await
            .expect("Failed to connect");
        our_sub.subscribe("").await.unwrap();
        our_subs.push(our_sub);
    }
    our_subs
}

fn run_their_pub(
    their_pub: zmq2::Socket,
    num_to_send: u32,
) -> std::thread::JoinHandle<zmq2::Socket> {
    assert_eq!(their_pub.get_socket_type().unwrap(), zmq2::PUB);
    std::thread::spawn(move || {
        for i in 0..num_to_send {
            their_pub
                .send(&format!("Their message: {}", i), 0)
                .expect("Failed to send");
        }
        println!("Finished pub task");
        their_pub
    })
}

async fn run_our_subs(our_subs: Vec<zeromq::SubSocket>, num_to_recv: u32) {
    let join_handles = our_subs.into_iter().map(|mut sub| {
        async_rt::task::spawn(async move {
            for i in 0..num_to_recv {
                let msg = sub.recv().await.expect("Failed to recv");
                let msg_string = String::from_utf8(msg.get(0).unwrap().to_vec()).unwrap();
                assert_eq!(msg_string, format!("Their message: {}", i));
            }
        })
    });
    for h in join_handles {
        h.await.expect("Subscriber task panicked!");
    }
    println!("Finished sub task");
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_rt::test]
    async fn test_their_pub_our_sub() {
        const N_SUBS: u8 = 16;

        async fn do_test(their_endpoint: &str) {
            let (their_pub, bind_endpoint, their_monitor) = setup_their_pub(their_endpoint);
            println!("Their pub was bound to {}", bind_endpoint);

            let our_subs = setup_our_subs(&bind_endpoint, N_SUBS).await;
            for _ in 0..N_SUBS {
                assert_eq!(
                    zmq2::SocketEvent::ACCEPTED,
                    get_monitor_event(&their_monitor).0
                );
                assert_eq!(
                    zmq2::SocketEvent::HANDSHAKE_SUCCEEDED,
                    get_monitor_event(&their_monitor).0
                );
            }
            // This is necessary to avoid slow joiner problem
            async_rt::task::sleep(Duration::from_millis(100)).await;
            println!("Setup done");

            const NUM_MSGS: u32 = 64;

            let their_join_handle = run_their_pub(their_pub, NUM_MSGS);
            run_our_subs(our_subs, NUM_MSGS).await;
            let their_pub = their_join_handle
                .join()
                .expect("Their pub terminated with an error!");

            for _ in 0..N_SUBS {
                assert_eq!(
                    get_monitor_event(&their_monitor).0,
                    zmq2::SocketEvent::DISCONNECTED
                );
            }

            drop(their_pub);
            assert_eq!(
                get_monitor_event(&their_monitor).0,
                zmq2::SocketEvent::CLOSED
            );
        }

        let endpoints = vec![
            "tcp://127.0.0.1:0",
            "tcp://[::1]:0",
            "ipc://asdf.sock",
            "ipc://anothersocket-asdf",
        ];
        for e in endpoints {
            println!("Testing with endpoint {}", e);
            do_test(e).await;

            // Unfortunately not all libzmq versions actually delete the ipc file. See
            // https://github.com/zeromq/libzmq/issues/3387
            // So we will delete it ourselves.
            if let Some(path) = e.strip_prefix("ipc://") {
                std::fs::remove_file(path).expect("Failed to remove ipc file");
            }
        }
    }
}
