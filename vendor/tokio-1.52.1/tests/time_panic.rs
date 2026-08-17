#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(target_os = "wasi")))] // Wasi doesn't support panic recovery
#![cfg(panic = "unwind")]

use futures::future;
use std::error::Error;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::{self, interval, interval_at, timeout, Instant};

mod support {
    pub mod panic;
}
use support::panic::test_panic;

fn rt_combinations() -> Vec<Runtime> {
    let mut rts = vec![];

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

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
fn pause_panic_caller() -> Result<(), Box<dyn Error>> {
    for rt in rt_combinations() {
        let panic_location_file = test_panic(|| {
            rt.block_on(async {
                time::pause();
                time::pause();
            });
        });

        // The panic location should be in this file
        assert_eq!(&panic_location_file.unwrap(), file!());
    }

    Ok(())
}

#[test]
fn resume_panic_caller() -> Result<(), Box<dyn Error>> {
    let panic_location_file = test_panic(|| {
        let rt = current_thread();

        rt.block_on(async {
            time::resume();
        });
    });

    // The panic location should be in this file
    assert_eq!(&panic_location_file.unwrap(), file!());

    Ok(())
}

#[test]
fn interval_panic_caller() -> Result<(), Box<dyn Error>> {
    let panic_location_file = test_panic(|| {
        let _ = interval(Duration::from_millis(0));
    });

    // The panic location should be in this file
    assert_eq!(&panic_location_file.unwrap(), file!());

    Ok(())
}

#[test]
fn interval_at_panic_caller() -> Result<(), Box<dyn Error>> {
    let panic_location_file = test_panic(|| {
        let _ = interval_at(Instant::now(), Duration::from_millis(0));
    });

    // The panic location should be in this file
    assert_eq!(&panic_location_file.unwrap(), file!());

    Ok(())
}

#[test]
fn timeout_panic_caller() -> Result<(), Box<dyn Error>> {
    let panic_location_file = test_panic(|| {
        // Runtime without `enable_time` so it has no current timer set.
        let rt = Builder::new_current_thread().build().unwrap();
        rt.block_on(async {
            let _timeout = timeout(Duration::from_millis(5), future::pending::<()>());
        });
    });

    // The panic location should be in this file
    assert_eq!(&panic_location_file.unwrap(), file!());

    Ok(())
}

fn current_thread() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
