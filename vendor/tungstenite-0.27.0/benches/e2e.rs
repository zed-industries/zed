//! Benchmarks for end to end performance including real `Read` & `Write` impls.
use bytes::Bytes;
use criterion::{BatchSize, Criterion, Throughput};
use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::SmallRng,
    SeedableRng,
};
use std::net::TcpListener;
use tungstenite::{accept_hdr_with_config, protocol::WebSocketConfig, Message};

/// Binary message meaning "stop".
const B_STOP: Bytes = Bytes::from_static(b"stop");

fn benchmark(c: &mut Criterion) {
    /// Benchmark that starts a simple server and client then sends (writes+flush) a
    /// single text message client->server and reads a single response text message
    /// server->client. Both message will be of the given `msg_len` size.
    fn send_and_recv(msg_len: usize, b: &mut criterion::Bencher<'_>) {
        let socket = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = socket.local_addr().unwrap().port();
        let conf = WebSocketConfig::default().max_message_size(None).max_frame_size(None);

        let server_thread = std::thread::spawn(move || {
            // single thread / single client server
            let (stream, _) = socket.accept().unwrap();
            let mut websocket =
                accept_hdr_with_config(stream, |_: &_, res| Ok(res), Some(conf)).unwrap();
            loop {
                let uppercase_txt = match websocket.read().unwrap() {
                    Message::Text(msg) => msg.to_ascii_uppercase(),
                    Message::Binary(msg) if msg == B_STOP => return,
                    msg => panic!("Unexpected msg: {msg:?}"),
                };
                websocket.send(Message::text(uppercase_txt)).unwrap();
            }
        });

        let (mut client, _) = tungstenite::client::connect_with_config(
            format!("ws://127.0.0.1:{port}"),
            Some(conf),
            3,
        )
        .unwrap();
        let mut rng = SmallRng::seed_from_u64(123);

        b.iter_batched(
            || {
                let msg = Alphanumeric.sample_string(&mut rng, msg_len);
                let expected_response = msg.to_ascii_uppercase();
                (msg, expected_response)
            },
            |(txt, expected_response)| {
                client.send(Message::text(txt)).unwrap();
                let response = client.read().unwrap();
                match response {
                    Message::Text(v) => assert_eq!(v, expected_response),
                    msg => panic!("Unexpected response msg: {msg:?}"),
                };
            },
            BatchSize::PerIteration,
        );

        // cleanup
        client.send(Message::binary(B_STOP)).unwrap();
        server_thread.join().unwrap();
    }

    // bench sending & receiving various sizes 512B to 1GiB.
    for len in (0..8).map(|n| 512 * 8_usize.pow(n)) {
        let mut group = c.benchmark_group("send+recv");
        group
            .throughput(Throughput::Bytes(len as u64 * 2)) // *2 as we send and then recv it
            .bench_function(HumanLen(len).to_string(), |b| send_and_recv(len, b));
    }
}

struct HumanLen(usize);

impl std::fmt::Display for HumanLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            n if n < 1024 => write!(f, "{n} B"),
            n if n < 1024 * 1024 => write!(f, "{} KiB", n / 1024),
            n if n < 1024 * 1024 * 1024 => write!(f, "{} MiB", n / (1024 * 1024)),
            n => write!(f, "{} GiB", n / (1024 * 1024 * 1024)),
        }
    }
}

criterion::criterion_group!(read_benches, benchmark);
criterion::criterion_main!(read_benches);
