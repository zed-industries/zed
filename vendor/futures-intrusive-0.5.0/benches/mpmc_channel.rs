use criterion::{
    criterion_group, criterion_main, Criterion, ParameterizedBenchmark,
};
use futures::{
    executor::block_on, future::join_all, join, sink::SinkExt,
    stream::StreamExt, FutureExt,
};
use futures_intrusive::channel::{
    shared::channel, shared::unbuffered_channel, LocalChannel,
};
use std::time::Duration;

/// Elements to transfer per producer
const ELEMS_TO_SEND: usize = 1000;
/// Buffer size for buffered channels
const CHANNEL_BUFFER_SIZE: usize = 20;

/// Benchmark for Crossbeam channels
fn crossbeam_channel_variable_tx(producers: usize) {
    let elems_per_producer = ELEMS_TO_SEND / producers;
    let (tx, rx) = crossbeam::channel::bounded(CHANNEL_BUFFER_SIZE);

    for _i in 0..producers {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for _i in 0..elems_per_producer {
                tx.send(4).unwrap();
            }
        });
    }

    drop(tx);

    loop {
        let res = rx.recv();
        if res.is_err() {
            break;
        }
    }
}

/// variable producers, single consumer
fn futchan_bounded_variable_tx(producers: usize) {
    use futures::channel::mpsc::channel;
    let elems_per_producer = ELEMS_TO_SEND / producers;
    let (tx, mut rx) = channel(CHANNEL_BUFFER_SIZE);

    for _i in 0..producers {
        let mut tx = tx.clone();
        std::thread::spawn(move || {
            block_on(async {
                for _i in 0..elems_per_producer {
                    tx.send(4).await.unwrap();
                }
            });
        });
    }

    drop(tx);

    block_on(async {
        loop {
            let res = rx.next().await;
            if res.is_none() {
                break;
            }
        }
    });
}

/// variable producers, single consumer
fn tokiochan_bounded_variable_tx(producers: usize) {
    let elems_per_producer = ELEMS_TO_SEND / producers;
    let (tx, mut rx) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);

    for _i in 0..producers {
        let tx = tx.clone();
        std::thread::spawn(move || {
            block_on(async {
                for _i in 0..elems_per_producer {
                    tx.send(4).await.unwrap();
                }
            });
        });
    }

    drop(tx);

    block_on(async {
        loop {
            let res = rx.recv().await;
            if res.is_none() {
                break;
            }
        }
    });
}

macro_rules! intrusive_channel_variable_tx {
    ($producers: expr, $channel_constructor: expr) => {
        let elems_per_producer = ELEMS_TO_SEND / $producers;
        let (tx, rx) = $channel_constructor;

        for _i in 0..$producers {
            let tx = tx.clone();

            std::thread::spawn(move || {
                block_on(async {
                    for _i in 0..elems_per_producer {
                        let r = tx.send(4).await;
                        assert!(r.is_ok());
                    }
                });
            });
        }

        drop(tx);

        block_on(async {
            loop {
                let res = rx.receive().await;
                if res.is_none() {
                    break;
                }
            }
        });
    };
}

/// variable producers, single consumer
fn intrusivechan_bounded_variable_tx(producers: usize) {
    intrusive_channel_variable_tx!(
        producers,
        channel::<i32>(CHANNEL_BUFFER_SIZE)
    );
}

/// variable producers, single consumer
fn intrusivechan_unbuffered_variable_tx(producers: usize) {
    intrusive_channel_variable_tx!(producers, unbuffered_channel::<i32>());
}

/// variable producers, single consumer
fn futchan_bounded_variable_tx_single_thread(producers: usize) {
    let elems_per_producer = ELEMS_TO_SEND / producers;

    block_on(async {
        let (tx, mut rx) = futures::channel::mpsc::channel(CHANNEL_BUFFER_SIZE);
        let produce_done = join_all((0..producers).into_iter().map(|_| {
            let mut tx = tx.clone();
            async move {
                for _i in 0..elems_per_producer {
                    tx.send(4).await.unwrap();
                }
            }
            .boxed()
        }));

        drop(tx);

        let consume_done = async {
            loop {
                let res = rx.next().await;
                if res.is_none() {
                    break;
                }
            }
        };

        join!(produce_done, consume_done);
    });
}

/// variable producers, single consumer
fn tokiochan_bounded_variable_tx_single_thread(producers: usize) {
    let elems_per_producer = ELEMS_TO_SEND / producers;

    block_on(async {
        let (tx, mut rx) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);
        let produce_done = join_all((0..producers).into_iter().map(|_| {
            let tx = tx.clone();
            async move {
                for _i in 0..elems_per_producer {
                    tx.send(4).await.unwrap();
                }
            }
            .boxed()
        }));

        drop(tx);

        let consume_done = async {
            loop {
                let res = rx.recv().await;
                if res.is_none() {
                    break;
                }
            }
        };

        join!(produce_done, consume_done);
    });
}

macro_rules! intrusive_channel_variable_tx_single_thread {
    ($producers: expr, $channel_constructor: expr) => {
        let elems_per_producer = ELEMS_TO_SEND / $producers;

        block_on(async {
            let (tx, rx) = $channel_constructor;
            let produce_done =
                join_all((0..$producers).into_iter().map(|_| {
                    let tx = tx.clone();
                    Box::pin(async move {
                        for _i in 0..elems_per_producer {
                            let r = tx.send(4).await;
                            assert!(r.is_ok());
                        }
                    })
                }));

            drop(tx);

            let consume_done = async {
                loop {
                    let res = rx.receive().await;
                    if res.is_none() {
                        break;
                    }
                }
            };

            join!(produce_done, consume_done);
        });
    };
}

/// variable producers, single consumer
fn intrusivechan_bounded_variable_tx_single_thread(producers: usize) {
    intrusive_channel_variable_tx_single_thread!(
        producers,
        channel::<i32>(CHANNEL_BUFFER_SIZE)
    );
}

/// variable producers, single consumer
fn intrusivechan_unbuffered_variable_tx_single_thread(producers: usize) {
    intrusive_channel_variable_tx_single_thread!(
        producers,
        unbuffered_channel::<i32>()
    );
}

/// variable producers, single consumer
fn intrusive_local_chan_bounded_variable_tx_single_thread(producers: usize) {
    let elems_per_producer = ELEMS_TO_SEND / producers;

    block_on(async {
        let rx = LocalChannel::<i32, [i32; CHANNEL_BUFFER_SIZE]>::new();
        let produce_done = join_all((0..producers).into_iter().map(|_| {
            Box::pin(async {
                for _i in 0..elems_per_producer {
                    let r = rx.send(4).await;
                    assert!(r.is_ok());
                }
            })
        }));

        let consume_done = async {
            let mut count = 0;
            let needed = elems_per_producer * producers;
            loop {
                let _ = rx.receive().await.unwrap();
                // The channel doesn't automatically get closed when producers are
                // gone since producer and consumer are the same object type.
                // Therefore we need to count receives.
                count += 1;
                if count == needed {
                    break;
                }
            }
        };

        join!(produce_done, consume_done);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // Producer and consumer are running on the same thread
    c.bench(
        "Channels (Single Threaded)",
        ParameterizedBenchmark::new(
            "intrusive local channel with producers",
            |b, &&producers| {
                b.iter(|| {
                    intrusive_local_chan_bounded_variable_tx_single_thread(
                        producers,
                    )
                })
            },
            &[5, 20, 100],
        )
        .with_function("intrusive channel with producers", |b, &&producers| {
            b.iter(|| {
                intrusivechan_bounded_variable_tx_single_thread(producers)
            })
        })
        .with_function(
            "intrusive unbuffered channel with producers",
            |b, &&producers| {
                b.iter(|| {
                    intrusivechan_unbuffered_variable_tx_single_thread(
                        producers,
                    )
                })
            },
        )
        .with_function(
            "futures::channel::mpsc with producers",
            |b, &&producers| {
                b.iter(|| futchan_bounded_variable_tx_single_thread(producers))
            },
        )
        .with_function(
            "tokio::sync::mpsc with producers",
            |b, &&producers| {
                b.iter(|| {
                    tokiochan_bounded_variable_tx_single_thread(producers)
                })
            },
        ),
    );

    // Producer and consume run on a different thread
    c.bench(
        "Channels (Thread per producer)",
        ParameterizedBenchmark::new(
            "crossbeam channel with producers",
            |b, &&producers| {
                b.iter(|| crossbeam_channel_variable_tx(producers))
            },
            &[5, 20, 100],
        )
        .with_function("intrusive channel with producers", |b, &&producers| {
            b.iter(|| intrusivechan_bounded_variable_tx(producers))
        })
        .with_function(
            "intrusive unbuffered channel with producers",
            |b, &&producers| {
                b.iter(|| intrusivechan_unbuffered_variable_tx(producers))
            },
        )
        .with_function(
            "futures::channel::mpsc with producers",
            |b, &&producers| b.iter(|| futchan_bounded_variable_tx(producers)),
        )
        .with_function(
            "tokio::sync::mpsc with producers",
            |b, &&producers| {
                b.iter(|| tokiochan_bounded_variable_tx(producers))
            },
        ),
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).nresamples(50);
    targets = criterion_benchmark
}
criterion_main!(benches);
