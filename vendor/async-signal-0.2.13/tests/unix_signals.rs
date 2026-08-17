#![cfg(unix)]

use async_signal::{Signal, Signals};
use futures_lite::{future, prelude::*};

#[test]
fn endurance() {
    let user_signals = [Signal::Usr1, Signal::Usr2];

    future::block_on(async {
        let mut signals = Signals::new(user_signals).unwrap();

        for &signal in user_signals.iter().cycle().take(100_000) {
            let mut next = signals.next();
            assert!(future::poll_once(&mut next).await.is_none());
            unsafe {
                libc::raise(signal as libc::c_int);
            }
            assert_eq!(
                future::poll_once(&mut next)
                    .await
                    .unwrap()
                    .unwrap()
                    .unwrap(),
                signal
            );
        }
    });
}

#[test]
fn distanced() {
    let mut signals = Signals::new(Some(Signal::Alarm)).unwrap();

    std::thread::spawn(|| {
        future::block_on(async {
            let mut rng = fastrand::Rng::new();

            for _ in 0..1_000 {
                unsafe {
                    libc::raise(Signal::Alarm as libc::c_int);
                }

                if rng.bool() {
                    async_io::Timer::after(std::time::Duration::from_millis(rng.u64(1..5))).await;
                }
            }
        });
    });

    future::block_on(async {
        for _ in 0..1_000 {
            assert_eq!(signals.next().await.unwrap().unwrap(), Signal::Alarm);
        }

        assert!(future::poll_once(signals.next()).await.is_none());
    });
}
