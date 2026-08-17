use std::sync::{Arc, Condvar, Mutex};

use ntest::timeout;
use test_log::test;

use zbus::{
    blocking::{self, MessageIterator},
    message::Message,
    names::UniqueName,
};

#[test]
#[timeout(15000)]
fn issue_122() {
    let conn = blocking::Connection::session().unwrap();
    let stream = MessageIterator::from(&conn);

    #[allow(clippy::mutex_atomic)]
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let child = std::thread::spawn(move || {
        {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        }

        for m in stream {
            let msg = m.unwrap();
            let hdr = msg.header();

            if hdr.member().map(|m| m.as_str()) == Some("ZBusIssue122") {
                break;
            }
        }
    });

    // Wait for the receiving thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    // Still give it some milliseconds to ensure it's already blocking on receive_message call
    // when we send a message.
    std::thread::sleep(std::time::Duration::from_millis(100));

    let destination = conn.unique_name().map(UniqueName::<'_>::from).unwrap();
    let msg = Message::method_call("/does/not/matter", "ZBusIssue122")
        .unwrap()
        .destination(destination)
        .unwrap()
        .build(&())
        .unwrap();
    conn.send(&msg).unwrap();

    child.join().unwrap();
}
