#![warn(rust_2018_idioms)]
#![cfg(all(tokio_unstable, feature = "time", feature = "rt-multi-thread"))]

use tokio::runtime::Runtime;
use tokio::time::*;

fn rt_combinations() -> Vec<Runtime> {
    let mut rts = vec![];

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

    #[cfg(tokio_unstable)]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_alt_timer()
            .enable_all()
            .build()
            .unwrap();
        rts.push(rt);

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_alt_timer()
            .enable_all()
            .build()
            .unwrap();
        rts.push(rt);
    }

    rts
}

#[test]
fn sleep() {
    const N: u32 = 512;

    for rt in rt_combinations() {
        rt.block_on(async {
            let mut jhs = vec![];

            // sleep outside of the worker threads
            let now = Instant::now();
            tokio::time::sleep(Duration::from_millis(10)).await;
            assert!(now.elapsed() >= Duration::from_millis(10));

            for _ in 0..N {
                let jh = tokio::spawn(async move {
                    // sleep inside of the worker threads
                    let now = Instant::now();
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    assert!(now.elapsed() >= Duration::from_millis(10));
                });
                jhs.push(jh);
            }

            for jh in jhs {
                jh.await.unwrap();
            }
        });
    }
}

#[test]
fn timeout() {
    const N: u32 = 512;

    for rt in rt_combinations() {
        rt.block_on(async {
            let mut jhs = vec![];

            // timeout outside of the worker threads
            let now = Instant::now();
            tokio::time::timeout(Duration::from_millis(10), std::future::pending::<()>())
                .await
                .expect_err("timeout should occur");
            assert!(now.elapsed() >= Duration::from_millis(10));

            for _ in 0..N {
                let jh = tokio::spawn(async move {
                    let now = Instant::now();
                    // timeout inside of the worker threads
                    tokio::time::timeout(Duration::from_millis(10), std::future::pending::<()>())
                        .await
                        .expect_err("timeout should occur");
                    assert!(now.elapsed() >= Duration::from_millis(10));
                });
                jhs.push(jh);
            }

            for jh in jhs {
                jh.await.unwrap();
            }
        });
    }
}
