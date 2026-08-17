use loom::future::block_on;
use loom::thread;

#[test]
#[cfg(not(tarpaulin))]
fn closing_tx() {
    loom::model(|| {
        let (tx, mut rx) = bmrng::channel::<u32, u32>(16);

        thread::spawn(move || {
            let res = block_on(tx.send(4));
            assert!(res.is_ok());
            drop(tx);
        });

        let v = block_on(rx.recv());
        assert!(v.is_ok());

        let v = block_on(rx.recv());
        assert!(v.is_err());
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn closing_tx_res() {
    loom::model(|| {
        let (tx, mut rx) = bmrng::channel::<u32, u32>(16);

        thread::spawn(move || {
            let res = block_on(tx.send(5));
            let repl = block_on(res.unwrap().recv());
            assert_eq!(repl, Ok(10));
            drop(tx);
        });

        let v = block_on(rx.recv());
        let (req, responder) = v.unwrap();
        let v = responder.respond(req * 2);
        assert!(v.is_ok());

        let v = block_on(rx.recv());
        assert!(v.is_err());
    })
}
