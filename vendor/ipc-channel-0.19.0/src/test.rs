// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
use crate::ipc::IpcReceiver;
use crate::ipc::{self, IpcReceiverSet, IpcSender, IpcSharedMemory};
use crate::router::{RouterProxy, ROUTER};
use crossbeam_channel::{self, Sender};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
use std::env;
use std::iter;
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios",)))]
use std::process::{self, Command, Stdio};
#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "android",
    target_os = "ios",
    target_os = "windows",
)))]
use std::ptr;
use std::rc::Rc;
use std::thread;

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "android",
    target_os = "ios",
    target_os = "windows"
)))]
use crate::ipc::IpcOneShotServer;

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "android",
    target_os = "ios",
    target_os = "windows",
)))]
use std::io::Error;
use std::time::{Duration, Instant};

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
// I'm not actually sure invoking this is indeed unsafe -- but better safe than sorry...
pub unsafe fn fork<F: FnOnce()>(child_func: F) -> libc::pid_t {
    match libc::fork() {
        -1 => panic!("Fork failed: {}", Error::last_os_error()),
        0 => {
            child_func();
            libc::exit(0);
        },
        pid => pid,
    }
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
pub trait Wait {
    fn wait(self);
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
impl Wait for libc::pid_t {
    fn wait(self) {
        unsafe {
            libc::waitpid(self, ptr::null_mut(), 0);
        }
    }
}

// Helper to get a channel_name argument passed in; used for the
// cross-process spawn server tests.
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
pub fn get_channel_name_arg(which: &str) -> Option<String> {
    for arg in env::args() {
        let arg_str = &*format!("channel_name-{}:", which);
        if let Some(arg) = arg.strip_prefix(arg_str) {
            return Some(arg.to_owned());
        }
    }
    None
}

// Helper to get a channel_name argument passed in; used for the
// cross-process spawn server tests.
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios",)))]
pub fn spawn_server(test_name: &str, server_args: &[(&str, &str)]) -> process::Child {
    Command::new(env::current_exe().unwrap())
        .arg(test_name)
        .args(
            server_args
                .iter()
                .map(|(name, val)| format!("channel_name-{}:{}", name, val)),
        )
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute server process")
}

type Person = (String, u32);

#[test]
fn simple() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person.clone()).unwrap();
    let received_person = rx.recv().unwrap();
    assert_eq!(person, received_person);
    drop(tx);
    match rx.recv().unwrap_err() {
        ipc::IpcError::Disconnected => (),
        e => panic!("expected disconnected error, got {:?}", e),
    }
}

#[test]
fn embedded_senders() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (sub_tx, sub_rx) = ipc::channel().unwrap();
    let person_and_sender = (person.clone(), sub_tx);
    let (super_tx, super_rx) = ipc::channel().unwrap();
    super_tx.send(person_and_sender).unwrap();
    let received_person_and_sender = super_rx.recv().unwrap();
    assert_eq!(received_person_and_sender.0, person);
    received_person_and_sender.1.send(person.clone()).unwrap();
    let received_person = sub_rx.recv().unwrap();
    assert_eq!(received_person, person);
}

#[test]
fn embedded_receivers() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (sub_tx, sub_rx) = ipc::channel().unwrap();
    let person_and_receiver = (person.clone(), sub_rx);
    let (super_tx, super_rx) = ipc::channel().unwrap();
    super_tx.send(person_and_receiver).unwrap();
    let received_person_and_receiver = super_rx.recv().unwrap();
    assert_eq!(received_person_and_receiver.0, person);
    sub_tx.send(person.clone()).unwrap();
    let received_person = received_person_and_receiver.1.recv().unwrap();
    assert_eq!(received_person, person);
}

#[test]
fn select() {
    let (tx0, rx0) = ipc::channel().unwrap();
    let (tx1, rx1) = ipc::channel().unwrap();
    let mut rx_set = IpcReceiverSet::new().unwrap();
    let rx0_id = rx_set.add(rx0).unwrap();
    let rx1_id = rx_set.add(rx1).unwrap();

    let person = ("Patrick Walton".to_owned(), 29);
    tx0.send(person.clone()).unwrap();
    let (received_id, received_data) = rx_set
        .select()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    let received_person: Person = received_data.to().unwrap();
    assert_eq!(received_id, rx0_id);
    assert_eq!(received_person, person);

    tx1.send(person.clone()).unwrap();
    let (received_id, received_data) = rx_set
        .select()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    let received_person: Person = received_data.to().unwrap();
    assert_eq!(received_id, rx1_id);
    assert_eq!(received_person, person);

    tx0.send(person.clone()).unwrap();
    tx1.send(person.clone()).unwrap();
    let (mut received0, mut received1) = (false, false);
    while !received0 || !received1 {
        for result in rx_set.select().unwrap().into_iter() {
            let (received_id, received_data) = result.unwrap();
            let received_person: Person = received_data.to().unwrap();
            assert_eq!(received_person, person);
            assert!(received_id == rx0_id || received_id == rx1_id);
            if received_id == rx0_id {
                assert!(!received0);
                received0 = true;
            } else if received_id == rx1_id {
                assert!(!received1);
                received1 = true;
            }
        }
    }
}

#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
#[test]
fn cross_process_embedded_senders_spawn() {
    let person = ("Patrick Walton".to_owned(), 29);

    let server0_name = get_channel_name_arg("server0");
    let server2_name = get_channel_name_arg("server2");
    if let (Some(server0_name), Some(server2_name)) = (server0_name, server2_name) {
        let (tx1, rx1): (IpcSender<Person>, IpcReceiver<Person>) = ipc::channel().unwrap();
        let tx0 = IpcSender::connect(server0_name).unwrap();
        tx0.send(tx1).unwrap();
        rx1.recv().unwrap();
        let tx2: IpcSender<Person> = IpcSender::connect(server2_name).unwrap();
        tx2.send(person.clone()).unwrap();

        unsafe {
            libc::exit(0);
        }
    }
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
#[test]
fn cross_process_embedded_senders_fork() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (server0, server0_name) = IpcOneShotServer::new().unwrap();
    let (server2, server2_name) = IpcOneShotServer::new().unwrap();
    let child_pid = unsafe {
        fork(|| {
            let (tx1, rx1): (IpcSender<Person>, IpcReceiver<Person>) = ipc::channel().unwrap();
            let tx0 = IpcSender::connect(server0_name).unwrap();
            tx0.send(tx1).unwrap();
            rx1.recv().unwrap();
            let tx2: IpcSender<Person> = IpcSender::connect(server2_name).unwrap();
            tx2.send(person.clone()).unwrap();
        })
    };
    let (_, tx1): (_, IpcSender<Person>) = server0.accept().unwrap();
    tx1.send(person.clone()).unwrap();
    let (_, received_person): (_, Person) = server2.accept().unwrap();
    child_pid.wait();
    assert_eq!(received_person, person);
}

#[test]
fn router_simple_global() {
    // Note: All ROUTER operation need to run in a single test,
    // since the state of the router will carry across tests.

    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person.clone()).unwrap();

    let (callback_fired_sender, callback_fired_receiver) = crossbeam_channel::unbounded::<Person>();
    #[allow(deprecated)]
    ROUTER.add_route(
        rx.to_opaque(),
        Box::new(move |person| {
            callback_fired_sender.send(person.to().unwrap()).unwrap();
        }),
    );
    let received_person = callback_fired_receiver.recv().unwrap();
    assert_eq!(received_person, person);

    // Try the same, with a strongly typed route
    let message: usize = 42;
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(message.clone()).unwrap();

    let (callback_fired_sender, callback_fired_receiver) = crossbeam_channel::unbounded::<usize>();
    ROUTER.add_typed_route(
        rx,
        Box::new(move |message| {
            callback_fired_sender.send(message.unwrap()).unwrap();
        }),
    );
    let received_message = callback_fired_receiver.recv().unwrap();
    assert_eq!(received_message, message);

    // Now shutdown the router.
    ROUTER.shutdown();

    // Use router after shutdown.
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person.clone()).unwrap();

    let (callback_fired_sender, callback_fired_receiver) = crossbeam_channel::unbounded::<Person>();
    ROUTER.add_typed_route(
        rx,
        Box::new(move |person| {
            callback_fired_sender.send(person.unwrap()).unwrap();
        }),
    );

    // The sender should have been dropped.
    let received_person = callback_fired_receiver.recv();
    assert!(received_person.is_err());

    // Shutdown the router, again(should be a no-op).
    ROUTER.shutdown();
}

#[test]
fn router_routing_to_new_crossbeam_receiver() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person.clone()).unwrap();

    let router = RouterProxy::new();
    let crossbeam_receiver = router.route_ipc_receiver_to_new_crossbeam_receiver(rx);
    let received_person = crossbeam_receiver.recv().unwrap();
    assert_eq!(received_person, person);
}

#[test]
fn router_multiplexing() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx0, rx0) = ipc::channel().unwrap();
    tx0.send(person.clone()).unwrap();
    let (tx1, rx1) = ipc::channel().unwrap();
    tx1.send(person.clone()).unwrap();

    let router = RouterProxy::new();
    let crossbeam_rx_0 = router.route_ipc_receiver_to_new_crossbeam_receiver(rx0);
    let crossbeam_rx_1 = router.route_ipc_receiver_to_new_crossbeam_receiver(rx1);
    let received_person_0 = crossbeam_rx_0.recv().unwrap();
    let received_person_1 = crossbeam_rx_1.recv().unwrap();
    assert_eq!(received_person_0, person);
    assert_eq!(received_person_1, person);
}

#[test]
fn router_multithreaded_multiplexing() {
    let person = ("Patrick Walton".to_owned(), 29);

    let person_for_thread = person.clone();
    let (tx0, rx0) = ipc::channel().unwrap();
    thread::spawn(move || tx0.send(person_for_thread).unwrap());
    let person_for_thread = person.clone();
    let (tx1, rx1) = ipc::channel().unwrap();
    thread::spawn(move || tx1.send(person_for_thread).unwrap());

    let router = RouterProxy::new();
    let crossbeam_rx_0 = router.route_ipc_receiver_to_new_crossbeam_receiver(rx0);
    let crossbeam_rx_1 = router.route_ipc_receiver_to_new_crossbeam_receiver(rx1);
    let received_person_0 = crossbeam_rx_0.recv().unwrap();
    let received_person_1 = crossbeam_rx_1.recv().unwrap();
    assert_eq!(received_person_0, person);
    assert_eq!(received_person_1, person);
}

#[test]
fn router_drops_callbacks_on_sender_shutdown() {
    struct Dropper {
        sender: Sender<i32>,
    }

    impl Drop for Dropper {
        fn drop(&mut self) {
            self.sender.send(42).unwrap();
        }
    }

    let (tx0, rx0) = ipc::channel::<()>().unwrap();
    let (drop_tx, drop_rx) = crossbeam_channel::unbounded();
    let dropper = Dropper { sender: drop_tx };

    let router = RouterProxy::new();
    router.add_typed_route(
        rx0,
        Box::new(move |_| {
            let _ = &dropper;
        }),
    );
    drop(tx0);
    assert_eq!(drop_rx.recv(), Ok(42));
}

#[test]
fn router_drops_callbacks_on_cloned_sender_shutdown() {
    struct Dropper {
        sender: Sender<i32>,
    }

    impl Drop for Dropper {
        fn drop(&mut self) {
            self.sender.send(42).unwrap()
        }
    }

    let (tx0, rx0) = ipc::channel::<()>().unwrap();
    let (drop_tx, drop_rx) = crossbeam_channel::unbounded();
    let dropper = Dropper { sender: drop_tx };

    let router = RouterProxy::new();
    router.add_typed_route(
        rx0,
        Box::new(move |_| {
            let _ = &dropper;
        }),
    );
    let txs = vec![tx0.clone(), tx0.clone(), tx0.clone()];
    drop(txs);
    drop(tx0);
    assert_eq!(drop_rx.recv(), Ok(42));
}

#[test]
fn router_big_data() {
    let person = ("Patrick Walton".to_owned(), 29);
    let people: Vec<_> = iter::repeat(person).take(64 * 1024).collect();
    let (tx, rx) = ipc::channel().unwrap();
    let people_for_subthread = people.clone();
    let thread = thread::spawn(move || {
        tx.send(people_for_subthread).unwrap();
    });

    let (callback_fired_sender, callback_fired_receiver) =
        crossbeam_channel::unbounded::<Vec<Person>>();
    let router = RouterProxy::new();
    router.add_typed_route(
        rx,
        Box::new(move |people| callback_fired_sender.send(people.unwrap()).unwrap()),
    );
    let received_people = callback_fired_receiver.recv().unwrap();
    assert_eq!(received_people, people);
    thread.join().unwrap();
}

#[test]
fn shared_memory() {
    let person = ("Patrick Walton".to_owned(), 29);
    let person_and_shared_memory = (person, IpcSharedMemory::from_byte(0xba, 1024 * 1024));
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person_and_shared_memory.clone()).unwrap();
    let received_person_and_shared_memory = rx.recv().unwrap();
    assert_eq!(
        received_person_and_shared_memory.0,
        person_and_shared_memory.0
    );
    assert!(person_and_shared_memory.1.iter().all(|byte| *byte == 0xba));
    assert!(received_person_and_shared_memory
        .1
        .iter()
        .all(|byte| *byte == 0xba));
}

#[test]
fn shared_memory_slice() {
    let (tx, rx) = ipc::channel().unwrap();
    // test byte of size 0
    let shared_memory = IpcSharedMemory::from_byte(42, 0);
    tx.send(shared_memory.clone()).unwrap();
    let received_shared_memory = rx.recv().unwrap();
    assert_eq!(*received_shared_memory, *shared_memory);
    // test empty slice
    let shared_memory = IpcSharedMemory::from_bytes(&[]);
    tx.send(shared_memory.clone()).unwrap();
    let received_shared_memory = rx.recv().unwrap();
    assert_eq!(*received_shared_memory, *shared_memory);
    // test non-empty slice
    let shared_memory = IpcSharedMemory::from_bytes(&[4, 2, 42]);
    tx.send(shared_memory.clone()).unwrap();
    let received_shared_memory = rx.recv().unwrap();
    assert_eq!(*received_shared_memory, *shared_memory);
}

#[test]
fn shared_memory_object_equality() {
    let person = ("Patrick Walton".to_owned(), 29);
    let person_and_shared_memory = (person, IpcSharedMemory::from_byte(0xba, 1024 * 1024));
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person_and_shared_memory.clone()).unwrap();
    let received_person_and_shared_memory = rx.recv().unwrap();
    assert_eq!(received_person_and_shared_memory, person_and_shared_memory);
}

#[test]
fn opaque_sender() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    let opaque_tx = tx.to_opaque();
    let tx: IpcSender<Person> = opaque_tx.to();
    tx.send(person.clone()).unwrap();
    let received_person = rx.recv().unwrap();
    assert_eq!(person, received_person);
}

#[test]
fn embedded_opaque_senders() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (sub_tx, sub_rx) = ipc::channel::<Person>().unwrap();
    let person_and_sender = (person.clone(), sub_tx.to_opaque());
    let (super_tx, super_rx) = ipc::channel().unwrap();
    super_tx.send(person_and_sender).unwrap();
    let received_person_and_sender = super_rx.recv().unwrap();
    assert_eq!(received_person_and_sender.0, person);
    received_person_and_sender
        .1
        .to::<Person>()
        .send(person.clone())
        .unwrap();
    let received_person = sub_rx.recv().unwrap();
    assert_eq!(received_person, person);
}

#[test]
fn try_recv() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    match rx.try_recv() {
        Err(ipc::TryRecvError::Empty) => (),
        v => panic!("Expected empty channel err: {:?}", v),
    }
    tx.send(person.clone()).unwrap();
    let received_person = rx.try_recv().unwrap();
    assert_eq!(person, received_person);
    match rx.try_recv() {
        Err(ipc::TryRecvError::Empty) => (),
        v => panic!("Expected empty channel err: {:?}", v),
    }
    drop(tx);
    match rx.try_recv() {
        Err(ipc::TryRecvError::IpcError(ipc::IpcError::Disconnected)) => (),
        v => panic!("Expected disconnected err: {:?}", v),
    }
}

#[test]
fn try_recv_timeout() {
    let person = ("Jacob Kiesel".to_owned(), 25);
    let (tx, rx) = ipc::channel().unwrap();
    let timeout = Duration::from_millis(1000);
    let start_recv = Instant::now();
    match rx.try_recv_timeout(timeout) {
        Err(ipc::TryRecvError::Empty) => {
            assert!(start_recv.elapsed() >= Duration::from_millis(500))
        },
        v => panic!("Expected empty channel err: {:?}", v),
    }
    tx.send(person.clone()).unwrap();
    let start_recv = Instant::now();
    let received_person = rx.try_recv_timeout(timeout).unwrap();
    assert!(start_recv.elapsed() < timeout);
    assert_eq!(person, received_person);
    let start_recv = Instant::now();
    match rx.try_recv_timeout(timeout) {
        Err(ipc::TryRecvError::Empty) => {
            assert!(start_recv.elapsed() >= Duration::from_millis(500))
        },
        v => panic!("Expected empty channel err: {:?}", v),
    }
    drop(tx);
    match rx.try_recv_timeout(timeout) {
        Err(ipc::TryRecvError::IpcError(ipc::IpcError::Disconnected)) => (),
        v => panic!("Expected disconnected err: {:?}", v),
    }
}

#[test]
fn multiple_paths_to_a_sender() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (sub_tx, sub_rx) = ipc::channel().unwrap();
    let person_and_sender = Rc::new((person.clone(), sub_tx));
    let send_data = vec![
        person_and_sender.clone(),
        person_and_sender.clone(),
        person_and_sender.clone(),
    ];
    let (super_tx, super_rx) = ipc::channel().unwrap();
    super_tx.send(send_data).unwrap();
    let received_data = super_rx.recv().unwrap();
    assert_eq!(received_data[0].0, person);
    assert_eq!(received_data[1].0, person);
    assert_eq!(received_data[2].0, person);
    received_data[0].1.send(person.clone()).unwrap();
    let received_person = sub_rx.recv().unwrap();
    assert_eq!(received_person, person);
    received_data[1].1.send(person.clone()).unwrap();
    let received_person = sub_rx.recv().unwrap();
    assert_eq!(received_person, person);
}

#[test]
fn bytes() {
    // N.B. We're using an odd number of bytes here to expose alignment issues.
    let bytes = [1, 2, 3, 4, 5, 6, 7];
    let (tx, rx) = ipc::bytes_channel().unwrap();
    tx.send(&bytes[..]).unwrap();
    let received_bytes = rx.recv().unwrap();
    assert_eq!(&bytes, &received_bytes[..]);
}

#[test]
fn embedded_bytes_receivers() {
    let (sub_tx, sub_rx) = ipc::bytes_channel().unwrap();
    let (super_tx, super_rx) = ipc::channel().unwrap();
    super_tx.send(sub_tx).unwrap();
    let sub_tx = super_rx.recv().unwrap();
    let bytes = [1, 2, 3, 4, 5, 6, 7];
    sub_tx.send(&bytes[..]).unwrap();
    let received_bytes = sub_rx.recv().unwrap();
    assert_eq!(&bytes, &received_bytes[..]);
}

#[test]
fn test_so_linger() {
    let (sender, receiver) = ipc::channel().unwrap();
    sender.send(42).unwrap();
    drop(sender);
    let val = match receiver.recv() {
        Ok(val) => val,
        Err(e) => {
            panic!("err: `{:?}`", e);
        },
    };
    assert_eq!(val, 42);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HasWeirdSerializer(Option<String>);

thread_local! { static WEIRD_CHANNEL: RefCell<Option<IpcSender<HasWeirdSerializer>>> = const { RefCell::new(None) } }

impl Serialize for HasWeirdSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.is_some() {
            WEIRD_CHANNEL.with(|chan| {
                chan.borrow()
                    .as_ref()
                    .unwrap()
                    .send(HasWeirdSerializer(None))
                    .unwrap();
            });
        }
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HasWeirdSerializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HasWeirdSerializer(Deserialize::deserialize(deserializer)?))
    }
}

#[test]
fn test_reentrant() {
    let null = HasWeirdSerializer(None);
    let hello = HasWeirdSerializer(Some(String::from("hello")));
    let (sender, receiver) = ipc::channel().unwrap();
    WEIRD_CHANNEL.with(|chan| {
        *chan.borrow_mut() = Some(sender.clone());
    });
    sender.send(hello.clone()).unwrap();
    assert_eq!(null, receiver.recv().unwrap());
    assert_eq!(hello, receiver.recv().unwrap());
    sender.send(null.clone()).unwrap();
    assert_eq!(null, receiver.recv().unwrap());
}

#[test]
fn transfer_closed_sender() {
    let (main_tx, main_rx) = ipc::channel().unwrap();
    let (transfer_tx, _) = ipc::channel::<()>().unwrap();
    assert!(main_tx.send(transfer_tx).is_ok());
    let _transferred_tx = main_rx.recv().unwrap();
}

#[cfg(feature = "async")]
#[test]
fn test_receiver_stream() {
    use futures::task::Context;
    use futures::task::Poll;
    use futures::Stream;
    use std::pin::Pin;
    let (tx, rx) = ipc::channel().unwrap();
    let (waker, count) = futures_test::task::new_count_waker();
    let mut ctx = Context::from_waker(&waker);
    let mut stream = rx.to_stream();

    assert_eq!(count, 0);
    match Pin::new(&mut stream).poll_next(&mut ctx) {
        Poll::Pending => (),
        _ => panic!("Stream shouldn't have data"),
    };
    assert_eq!(count, 0);
    tx.send(5).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));
    assert_eq!(count, 1);
    match Pin::new(&mut stream).poll_next(&mut ctx) {
        Poll::Ready(Some(Ok(5))) => (),
        _ => panic!("Stream should have 5"),
    };
}
