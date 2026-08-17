use jobserver::Client;
use std::sync::atomic::*;
use std::sync::*;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

#[test]
fn helper_smoke() {
    let client = t!(Client::new(1));
    drop(client.clone().into_helper_thread(|_| ()).unwrap());
    drop(client.clone().into_helper_thread(|_| ()).unwrap());
    drop(client.clone().into_helper_thread(|_| ()).unwrap());
    drop(client.clone().into_helper_thread(|_| ()).unwrap());
    drop(client.clone().into_helper_thread(|_| ()).unwrap());
    drop(client.into_helper_thread(|_| ()).unwrap());
}

#[test]
fn acquire() {
    let (tx, rx) = mpsc::channel();
    let client = t!(Client::new(1));
    let helper = client
        .into_helper_thread(move |a| drop(tx.send(a)))
        .unwrap();
    assert!(rx.try_recv().is_err());
    helper.request_token();
    rx.recv().unwrap().unwrap();
    helper.request_token();
    rx.recv().unwrap().unwrap();

    helper.request_token();
    helper.request_token();
    rx.recv().unwrap().unwrap();
    rx.recv().unwrap().unwrap();

    helper.request_token();
    helper.request_token();
    drop(helper);
}

#[test]
fn prompt_shutdown() {
    for _ in 0..100 {
        let client = jobserver::Client::new(4).unwrap();
        let count = Arc::new(AtomicU32::new(0));
        let count2 = count.clone();
        let tokens = Arc::new(Mutex::new(Vec::new()));
        let helper = client
            .into_helper_thread(move |token| {
                tokens.lock().unwrap().push(token);
                count2.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        // Request more tokens than what are available.
        for _ in 0..5 {
            helper.request_token();
        }
        // Wait for at least some of the requests to finish.
        while count.load(Ordering::SeqCst) < 3 {
            std::thread::yield_now();
        }
        // Drop helper
        let t = std::time::Instant::now();
        drop(helper);
        let d = t.elapsed();
        assert!(d.as_secs_f64() < 0.5);
    }
}
