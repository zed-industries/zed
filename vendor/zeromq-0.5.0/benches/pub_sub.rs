use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

use zeromq::{prelude::*, PubSocket, SubSocket};

type BenchGroup<'a> = criterion::BenchmarkGroup<'a, criterion::measurement::WallTime>;

/// Binds n pubs and connects n subs to them
async fn setup(
    m_pubs: u8,
    n_subs: u8,
    endpoint: &str,
) -> (Vec<zeromq::PubSocket>, Vec<zeromq::SubSocket>) {
    let mut pubs = Vec::new();
    let mut subs: Vec<_> = (0..n_subs).map(|_| SubSocket::new()).collect();

    for _ in 0..m_pubs {
        let mut p = PubSocket::new();
        let bind_endpoint = p.bind(endpoint).await.unwrap();
        println!("Bind endpoint: {}", bind_endpoint);
        for s in subs.iter_mut() {
            s.connect(bind_endpoint.clone()).await.unwrap();
        }
        pubs.push(p);
    }

    for s in subs.iter_mut() {
        s.subscribe("")
            .await
            .expect("Could not set subscription filter");
    }

    (pubs, subs)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rt = Runtime::new().unwrap();

    const M_PUBS: u8 = 2;
    const N_SUBS: u8 = 16;

    let mut group = c.benchmark_group(format!("{}-{} Pub Sub", M_PUBS, N_SUBS));

    bench(&mut group, "TCP", "tcp://localhost:0", &mut rt);
    bench(&mut group, "IPC", "ipc://asdf.sock", &mut rt);

    fn bench<'a>(group: &mut BenchGroup<'a>, bench_name: &str, endpoint: &str, rt: &mut Runtime) {
        let (pubs, subs) = rt.block_on(setup(M_PUBS, N_SUBS, endpoint));
        assert_eq!(pubs.len(), usize::from(M_PUBS));
        assert_eq!(subs.len(), usize::from(N_SUBS));
        // Wrap in option to allow `f` to temporarily assume ownership
        let (mut pubs, mut subs) = (Some(pubs), Some(subs));

        group.bench_function(bench_name, |b| {
            b.iter(|| rt.block_on(iter_fn(&mut pubs, &mut subs)))
        });
    }

    async fn iter_fn(pubs: &mut Option<Vec<PubSocket>>, subs: &mut Option<Vec<SubSocket>>) {
        let owned_pubs = pubs.take().unwrap();
        let owned_subs = subs.take().unwrap();
        const MSGS_PER_PUB: u16 = 100;

        async fn send_msgs(mut pubs: Vec<PubSocket>) -> Vec<PubSocket> {
            for i in 0..MSGS_PER_PUB {
                for (j, p) in pubs.iter_mut().enumerate() {
                    p.send(format!("Sending {}th message from publisher {}", i, j).into())
                        .expect("Could not send message");
                }
            }
            println!("Sent all messages");
            pubs
        }
        async fn recv_msgs(mut subs: Vec<SubSocket>) -> Vec<SubSocket> {
            for i in 0..(u32::from(MSGS_PER_PUB) * u32::from(M_PUBS)) {
                for s in subs.iter_mut() {
                    println!("Attempting recv");
                    // TODO: This hangs. Why?
                    let msg = s.recv().await.expect("Could not recv");
                    black_box(msg);
                }
                println!("Received {}", i);
            }
            subs
        }
        println!("Starting send/recv loops");
        let owned_pubs = tokio::spawn(send_msgs(owned_pubs)).await.unwrap();
        tokio::time::delay_for(std::time::Duration::from_millis(1000)).await;
        let owned_subs = tokio::spawn(recv_msgs(owned_subs)).await.unwrap();
        // let (owned_pubs, owned_subs) = tokio::try_join!(pubs_handle,
        // subs_handle).unwrap();
        pubs.replace(owned_pubs);
        subs.replace(owned_subs);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
