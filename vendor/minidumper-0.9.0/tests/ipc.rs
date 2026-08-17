use std::sync::{Arc, atomic};

/// Tests that the user can send and receive their own messages over IPC
#[test]
fn ipc_messages() {
    let name = minidumper::SocketName::path("ipc_messages");

    let mut server = minidumper::Server::with_name(name).unwrap();

    struct Message {
        kind: u32,
        msg: String,
    }

    struct Server {
        messages: Arc<parking_lot::Mutex<Vec<Message>>>,
    }

    impl minidumper::ServerHandler for Server {
        fn create_minidump_file(
            &self,
        ) -> Result<(std::fs::File, std::path::PathBuf), std::io::Error> {
            panic!("should not be called");
        }

        fn on_minidump_created(
            &self,
            _result: Result<minidumper::MinidumpBinary, minidumper::Error>,
        ) -> minidumper::LoopAction {
            panic!("should not be called");
        }

        fn on_message(&self, kind: u32, buffer: Vec<u8>) {
            self.messages.lock().push(Message {
                kind,
                msg: String::from_utf8(buffer).unwrap(),
            });
        }
    }

    let messages = Arc::new(parking_lot::Mutex::new(Vec::new()));

    let server_handler = Server {
        messages: messages.clone(),
    };

    let shutdown = Arc::new(atomic::AtomicBool::new(false));
    let is_shutdown = shutdown.clone();
    let server_loop =
        std::thread::spawn(move || server.run(Box::new(server_handler), &is_shutdown, None));

    let client = minidumper::Client::with_name(name).unwrap();

    for i in 0..1000 {
        assert!(client.send_message(i, format!("msg #{i}")).is_ok(), "{i}");
    }

    shutdown.store(true, atomic::Ordering::Relaxed);
    server_loop.join().unwrap().unwrap();

    let messages = messages.lock();
    for (i, msg) in (0..1000).zip(messages.iter()) {
        assert_eq!(i, msg.kind);
        assert_eq!(format!("msg #{i}"), msg.msg);
    }
}

/// Tests that the server reaps inactive clients
#[test]
fn inactive_reap() {
    if std::env::var("CI").is_ok() {
        println!("not potato compatible");
        return;
    }

    let name = minidumper::SocketName::path("inactive_reap");

    let mut server = minidumper::Server::with_name(name).unwrap();

    struct Message {
        msg: String,
    }

    struct Server {
        messages: Arc<parking_lot::Mutex<Vec<Message>>>,
    }

    impl minidumper::ServerHandler for Server {
        fn create_minidump_file(
            &self,
        ) -> Result<(std::fs::File, std::path::PathBuf), std::io::Error> {
            panic!("should not be called");
        }

        fn on_minidump_created(
            &self,
            _result: Result<minidumper::MinidumpBinary, minidumper::Error>,
        ) -> minidumper::LoopAction {
            panic!("should not be called");
        }

        fn on_message(&self, _kind: u32, buffer: Vec<u8>) {
            self.messages.lock().push(Message {
                msg: String::from_utf8(buffer).unwrap(),
            });
        }

        fn on_client_disconnected(&self, num_clients: usize) -> minidumper::LoopAction {
            self.messages.lock().push(Message {
                msg: format!("num_clients = {num_clients}"),
            });

            if num_clients == 0 {
                minidumper::LoopAction::Exit
            } else {
                minidumper::LoopAction::Continue
            }
        }
    }

    let messages = Arc::new(parking_lot::Mutex::new(Vec::new()));

    let server_handler = Server {
        messages: messages.clone(),
    };

    let shutdown = Arc::new(atomic::AtomicBool::new(false));
    let server_loop = std::thread::spawn(move || {
        server.run(
            Box::new(server_handler),
            &shutdown,
            Some(std::time::Duration::from_millis(20)),
        )
    });

    let client_one = minidumper::Client::with_name(name).unwrap();
    let client_two = minidumper::Client::with_name(name).unwrap();

    client_one.send_message(1, "msg #1").expect("1");
    client_two.send_message(2, "msg #2").expect("2");

    std::thread::sleep(std::time::Duration::from_millis(12));

    client_one.ping().expect("ping");

    std::thread::sleep(std::time::Duration::from_millis(12));

    client_one.send_message(1, "msg #3").expect("3");

    server_loop.join().unwrap().unwrap();

    let messages = messages.lock();

    assert_eq!(messages.len(), 5);
    assert_eq!(messages[0].msg, "msg #1");
    assert_eq!(messages[1].msg, "msg #2");
    assert_eq!(messages[2].msg, "num_clients = 1");
    assert_eq!(messages[3].msg, "msg #3");
    assert_eq!(messages[4].msg, "num_clients = 0");
}

#[test]
fn ping() {
    if std::env::var("CI").is_ok() {
        println!("not potato compatible");
        return;
    }

    let name = minidumper::SocketName::from(std::path::Path::new("ping"));

    let mut server = minidumper::Server::with_name(name).unwrap();

    struct Server;

    impl minidumper::ServerHandler for Server {
        fn create_minidump_file(
            &self,
        ) -> Result<(std::fs::File, std::path::PathBuf), std::io::Error> {
            panic!("should not be called");
        }

        fn on_minidump_created(
            &self,
            _result: Result<minidumper::MinidumpBinary, minidumper::Error>,
        ) -> minidumper::LoopAction {
            panic!("should not be called");
        }

        fn on_message(&self, _kind: u32, _buffer: Vec<u8>) {
            panic!("should not be called");
        }

        fn on_client_disconnected(&self, num_clients: usize) -> minidumper::LoopAction {
            if num_clients == 0 {
                minidumper::LoopAction::Exit
            } else {
                minidumper::LoopAction::Continue
            }
        }
    }

    let server_handler = Server;

    let shutdown = Arc::new(atomic::AtomicBool::new(false));
    let server_loop = std::thread::spawn(move || {
        server.run(
            Box::new(server_handler),
            &shutdown,
            Some(std::time::Duration::from_millis(20)),
        )
    });

    let client = minidumper::Client::with_name(name).unwrap();

    let start = std::time::Instant::now();
    for i in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let res = client.ping();
        assert!(res.is_ok(), "ping {i} {res:?}");
    }

    server_loop.join().unwrap().unwrap();

    #[allow(clippy::dbg_macro)]
    let elapsed = dbg!(start.elapsed());

    assert!(
        elapsed < std::time::Duration::from_millis(60 + 30 /* account for potato computers */)
            && elapsed > std::time::Duration::from_millis(45)
    );

    assert!(client.ping().is_err(), "server should be gone");
}
