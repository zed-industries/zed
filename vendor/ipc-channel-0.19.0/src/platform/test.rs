// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ipc::IpcMessage;
use crate::platform::OsIpcSharedMemory;
use crate::platform::{self, OsIpcChannel, OsIpcReceiverSet};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::platform::{OsIpcOneShotServer, OsIpcSender};
#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
use crate::test::{fork, Wait};
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
use libc;
#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
use libc::{kill, SIGCONT, SIGSTOP};

// Helper to get a channel_name argument passed in; used for the
// cross-process spawn server tests.
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
use crate::test::{get_channel_name_arg, spawn_server};

#[test]
fn simple() {
    let (tx, rx) = platform::channel().unwrap();
    let data: &[u8] = b"1234567";
    tx.send(data, Vec::new(), Vec::new()).unwrap();
    let ipc_message = rx.recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn sender_transfer() {
    let (super_tx, super_rx) = platform::channel().unwrap();
    let (sub_tx, sub_rx) = platform::channel().unwrap();
    let data: &[u8] = b"foo";
    super_tx
        .send(data, vec![OsIpcChannel::Sender(sub_tx)], vec![])
        .unwrap();
    let mut ipc_message = super_rx.recv().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_tx = ipc_message.os_ipc_channels.pop().unwrap().to_sender();
    sub_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message = sub_rx.recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn receiver_transfer() {
    let (super_tx, super_rx) = platform::channel().unwrap();
    let (sub_tx, sub_rx) = platform::channel().unwrap();
    let data: &[u8] = b"foo";
    super_tx
        .send(data, vec![OsIpcChannel::Receiver(sub_rx)], vec![])
        .unwrap();
    let mut ipc_message = super_rx.recv().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_rx = ipc_message.os_ipc_channels.pop().unwrap().to_receiver();
    sub_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message = sub_rx.recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn multisender_transfer() {
    let (super_tx, super_rx) = platform::channel().unwrap();
    let (sub0_tx, sub0_rx) = platform::channel().unwrap();
    let (sub1_tx, sub1_rx) = platform::channel().unwrap();
    let data: &[u8] = b"asdfasdf";
    super_tx
        .send(
            data,
            vec![OsIpcChannel::Sender(sub0_tx), OsIpcChannel::Sender(sub1_tx)],
            vec![],
        )
        .unwrap();
    let mut ipc_message1 = super_rx.recv().unwrap();
    assert_eq!(ipc_message1.os_ipc_channels.len(), 2);

    let sub0_tx = ipc_message1.os_ipc_channels.remove(0).to_sender();
    sub0_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message2 = sub0_rx.recv().unwrap();
    assert_eq!(ipc_message2, IpcMessage::from_data(data.to_vec()));

    let sub1_tx = ipc_message1.os_ipc_channels.remove(0).to_sender();
    sub1_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message3 = sub1_rx.recv().unwrap();
    assert_eq!(ipc_message3, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn medium_data() {
    let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    let (tx, rx) = platform::channel().unwrap();
    tx.send(data, vec![], vec![]).unwrap();
    let ipc_message = rx.recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn medium_data_with_sender_transfer() {
    let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    let (super_tx, super_rx) = platform::channel().unwrap();
    let (sub_tx, sub_rx) = platform::channel().unwrap();
    super_tx
        .send(data, vec![OsIpcChannel::Sender(sub_tx)], vec![])
        .unwrap();
    let mut ipc_message = super_rx.recv().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_tx = ipc_message.os_ipc_channels.pop().unwrap().to_sender();
    sub_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message = sub_rx.recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

fn check_big_data(size: u32) {
    let (tx, rx) = platform::channel().unwrap();
    let thread = thread::spawn(move || {
        let data: Vec<u8> = (0..size).map(|i| (i % 251) as u8).collect();
        let data: &[u8] = &data[..];
        tx.send(data, vec![], vec![]).unwrap();
    });
    let ipc_message = rx.recv().unwrap();
    let data: Vec<u8> = (0..size).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    assert_eq!(ipc_message.data.len(), data.len());
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    thread.join().unwrap();
}

#[test]
fn big_data() {
    check_big_data(1024 * 1024);
}

#[test]
fn huge_data() {
    check_big_data(1024 * 1024 * 50);
    check_big_data(1024 * 1024 * 46);
    check_big_data(1024 * 1024 * 49);
    check_big_data(1024 * 1024 * 47);
    check_big_data(1024 * 1024 * 48);
}

#[test]
fn big_data_with_sender_transfer() {
    let (super_tx, super_rx) = platform::channel().unwrap();
    let (sub_tx, sub_rx) = platform::channel().unwrap();
    let thread = thread::spawn(move || {
        let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
        let data: &[u8] = &data[..];
        super_tx
            .send(data, vec![OsIpcChannel::Sender(sub_tx)], vec![])
            .unwrap();
    });
    let mut ipc_message = super_rx.recv().unwrap();
    let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    assert_eq!(ipc_message.data.len(), data.len());
    assert_eq!(&ipc_message.data[..], data);
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    assert_eq!(ipc_message.os_ipc_shared_memory_regions.len(), 0);

    let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    let sub_tx = ipc_message.os_ipc_channels[0].to_sender();
    sub_tx.send(data, vec![], vec![]).unwrap();
    let ipc_message = sub_rx.recv().unwrap();
    assert_eq!(ipc_message.data.len(), data.len());
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    thread.join().unwrap();
}

fn with_n_fds(n: usize, size: usize) {
    let (sender_fds, receivers): (Vec<_>, Vec<_>) = (0..n)
        .map(|_| platform::channel().unwrap())
        .map(|(tx, rx)| (OsIpcChannel::Sender(tx), rx))
        .unzip();
    let (super_tx, super_rx) = platform::channel().unwrap();

    let data: Vec<u8> = (0..size).map(|i| (i % 251) as u8).collect();
    super_tx.send(&data[..], sender_fds, vec![]).unwrap();
    let ipc_message = super_rx.recv().unwrap();

    assert_eq!(ipc_message.data.len(), data.len());
    assert_eq!(&ipc_message.data[..], &data[..]);
    assert_eq!(ipc_message.os_ipc_channels.len(), receivers.len());
    assert_eq!(ipc_message.os_ipc_shared_memory_regions.len(), 0);

    let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
    for (mut sender_fd, sub_rx) in ipc_message
        .os_ipc_channels
        .into_iter()
        .zip(receivers.into_iter())
    {
        let sub_tx = sender_fd.to_sender();
        sub_tx.send(&data, vec![], vec![]).unwrap();
        let ipc_message = sub_rx.recv().unwrap();
        assert_eq!(ipc_message.data.len(), data.len());
        assert_eq!(ipc_message, IpcMessage::from_data(data.clone()));
    }
}

// These tests only apply to platforms that need fragmentation.
#[cfg(all(
    not(feature = "force-inprocess"),
    any(target_os = "linux", target_os = "freebsd", target_os = "windows")
))]
mod fragment_tests {
    use super::with_n_fds;
    use crate::platform;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FRAGMENT_SIZE: usize = platform::OsIpcSender::get_max_fragment_size();
    }

    #[test]
    fn full_packet() {
        with_n_fds(0, *FRAGMENT_SIZE);
    }

    #[test]
    fn full_packet_with_1_fds() {
        with_n_fds(1, *FRAGMENT_SIZE);
    }
    #[test]
    fn full_packet_with_2_fds() {
        with_n_fds(2, *FRAGMENT_SIZE);
    }
    #[test]
    fn full_packet_with_3_fds() {
        with_n_fds(3, *FRAGMENT_SIZE);
    }
    #[test]
    fn full_packet_with_4_fds() {
        with_n_fds(4, *FRAGMENT_SIZE);
    }
    #[test]
    fn full_packet_with_5_fds() {
        with_n_fds(5, *FRAGMENT_SIZE);
    }
    #[test]
    fn full_packet_with_6_fds() {
        with_n_fds(6, *FRAGMENT_SIZE);
    }

    // MAX_FDS_IN_CMSG is currently 64.
    #[test]
    fn full_packet_with_64_fds() {
        with_n_fds(64, *FRAGMENT_SIZE);
    }

    #[test]
    fn overfull_packet() {
        with_n_fds(0, *FRAGMENT_SIZE + 1);
    }

    // In fragmented transfers, one FD is used up for the dedicated channel.
    #[test]
    fn overfull_packet_with_63_fds() {
        with_n_fds(63, *FRAGMENT_SIZE + 1);
    }
}

#[test]
fn empty() {
    with_n_fds(0, 0);
}

#[test]
// This currently fails with the `macos` implementation.
//
// It could be argued that though that this is not really a bug,
// since sending a channel with the official high-level `ipc-channel` API
// will always add some data during serialisation.
// (Namely the index of the corresponding entry within the `channels` vector.)
#[cfg_attr(all(target_os = "macos", not(feature = "force-inprocess")), ignore)]
fn fd_only() {
    with_n_fds(1, 0);
}

macro_rules! create_big_data_with_n_fds {
    ($name:ident, $n:expr) => {
        #[test]
        fn $name() {
            let (sender_fds, receivers): (Vec<_>, Vec<_>) = (0..$n)
                .map(|_| platform::channel().unwrap())
                .map(|(tx, rx)| (OsIpcChannel::Sender(tx), rx))
                .unzip();
            let (super_tx, super_rx) = platform::channel().unwrap();
            let thread = thread::spawn(move || {
                let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
                let data: &[u8] = &data[..];
                super_tx.send(data, sender_fds, vec![]).unwrap();
            });
            let ipc_message = super_rx.recv().unwrap();
            thread.join().unwrap();

            let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
            let data: &[u8] = &data[..];
            assert_eq!(ipc_message.data.len(), data.len());
            assert_eq!(&ipc_message.data[..], &data[..]);
            assert_eq!(ipc_message.os_ipc_channels.len(), receivers.len());
            assert_eq!(ipc_message.os_ipc_shared_memory_regions.len(), 0);

            let data: Vec<u8> = (0..65536).map(|i| (i % 251) as u8).collect();
            let data: &[u8] = &data[..];
            for (mut sender_fd, sub_rx) in ipc_message
                .os_ipc_channels
                .into_iter()
                .zip(receivers.into_iter())
            {
                let sub_tx = sender_fd.to_sender();
                sub_tx.send(data, vec![], vec![]).unwrap();
                let ipc_message = sub_rx.recv().unwrap();
                assert_eq!(ipc_message.data.len(), data.len());
                assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
            }
        }
    };
}

create_big_data_with_n_fds!(big_data_with_0_fds, 0);
create_big_data_with_n_fds!(big_data_with_1_fds, 1);
create_big_data_with_n_fds!(big_data_with_2_fds, 2);
create_big_data_with_n_fds!(big_data_with_3_fds, 3);
create_big_data_with_n_fds!(big_data_with_4_fds, 4);
create_big_data_with_n_fds!(big_data_with_5_fds, 5);
create_big_data_with_n_fds!(big_data_with_6_fds, 6);

#[test]
fn concurrent_senders() {
    let num_senders = 3;

    let (tx, rx) = platform::channel().unwrap();

    let threads: Vec<_> = (0..num_senders)
        .map(|i| {
            let tx = tx.clone();
            thread::spawn(move || {
                let data: Vec<u8> = (0..1024 * 1024).map(|j| (j % 13) as u8 | i << 4).collect();
                let data: &[u8] = &data[..];
                tx.send(data, vec![], vec![]).unwrap();
            })
        })
        .collect();

    let mut received_vals: Vec<u8> = vec![];
    for _ in 0..num_senders {
        let ipc_message = rx.recv().unwrap();
        let val = ipc_message.data[0] >> 4;
        received_vals.push(val);
        let data: Vec<u8> = (0..1024 * 1024)
            .map(|j| (j % 13) as u8 | val << 4)
            .collect();
        let data: &[u8] = &data[..];
        assert_eq!(ipc_message.data.len(), data.len());
        assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    }
    assert!(rx.try_recv().is_err()); // There should be no further messages pending.
    received_vals.sort();
    assert_eq!(received_vals, (0..num_senders).collect::<Vec<_>>()); // Got exactly the values we sent.

    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn receiver_set() {
    let (tx0, rx0) = platform::channel().unwrap();
    let (tx1, rx1) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx0_id = rx_set.add(rx0).unwrap();
    let rx1_id = rx_set.add(rx1).unwrap();

    let data: &[u8] = b"1234567";
    tx0.send(data, vec![], vec![]).unwrap();
    let (received_id, ipc_message) = rx_set
        .select()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(received_id, rx0_id);
    assert_eq!(ipc_message.data, data);

    tx1.send(data, vec![], vec![]).unwrap();
    let (received_id, ipc_message) = rx_set
        .select()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(received_id, rx1_id);
    assert_eq!(ipc_message.data, data);

    tx0.send(data, vec![], vec![]).unwrap();
    tx1.send(data, vec![], vec![]).unwrap();
    let (mut received0, mut received1) = (false, false);
    while !received0 || !received1 {
        for result in rx_set.select().unwrap().into_iter() {
            let (received_id, ipc_message) = result.unwrap();
            assert_eq!(ipc_message.data, data);
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

#[test]
#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
fn receiver_set_eintr() {
    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let child_pid = unsafe {
        fork(|| {
            let (tx0, rx0) = platform::channel().unwrap();
            let mut rx_set = OsIpcReceiverSet::new().unwrap();
            let rx_id = rx_set.add(rx0).unwrap();
            // Let the parent know we're ready
            let tx1 = OsIpcSender::connect(name).unwrap();
            tx1.send(b" Ready! ", vec![OsIpcChannel::Sender(tx0)], vec![])
                .unwrap();
            // Send the result of the select back to the parent
            let result = rx_set.select().unwrap();
            let mut result_iter = result.into_iter();
            let (id, ipc_message) = result_iter.next().unwrap().unwrap();
            assert_eq!(rx_id, id);
            assert_eq!(b"Test".as_ref(), &*ipc_message.data);
            assert!(result_iter.next().is_none());
            tx1.send(b"Success!", vec![], vec![]).unwrap();
        })
    };
    // Wait until the child is ready
    let (server, mut ipc_message) = server.accept().unwrap();
    assert!(ipc_message.data == b" Ready! ");
    let tx1 = ipc_message.os_ipc_channels.first_mut().unwrap().to_sender();
    unsafe {
        kill(child_pid, SIGSTOP);
        thread::sleep(Duration::from_millis(42));
        kill(child_pid, SIGCONT);
    }
    // The interrupt shouldn't affect the following send
    tx1.send(b"Test", vec![], vec![]).unwrap();
    let ipc_message = server.recv().unwrap();
    assert!(ipc_message.data == b"Success!");
    child_pid.wait();
}

#[test]
fn receiver_set_empty() {
    let (tx, rx) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx_id = rx_set.add(rx).unwrap();

    let data: &[u8] = b"";
    tx.send(data, vec![], vec![]).unwrap();
    let (received_id, ipc_message) = rx_set
        .select()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(received_id, rx_id);
    assert!(ipc_message.data.is_empty());
}

#[test]
fn receiver_set_close_before_adding() {
    let (_, rx) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx_id = rx_set.add(rx).unwrap();

    match rx_set.select().unwrap().into_iter().next().unwrap() {
        platform::OsIpcSelectionResult::ChannelClosed(received_id) => {
            assert_eq!(received_id, rx_id);
        },
        _ => {
            panic!("Unexpected result!");
        },
    }
}

#[test]
fn receiver_set_close_after_adding() {
    let (tx, rx) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx_id = rx_set.add(rx).unwrap();

    drop(tx);
    match rx_set.select().unwrap().into_iter().next().unwrap() {
        platform::OsIpcSelectionResult::ChannelClosed(received_id) => {
            assert_eq!(received_id, rx_id);
        },
        _ => {
            panic!("Unexpected result!");
        },
    }
}

#[test]
fn receiver_set_medium_data() {
    let (tx0, rx0) = platform::channel().unwrap();
    let (tx1, rx1) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx0_id = rx_set.add(rx0).unwrap();
    let rx1_id = rx_set.add(rx1).unwrap();

    let data0: Vec<u8> = (0..65536).map(|offset| (offset % 127) as u8).collect();
    let data1: Vec<u8> = (0..65536)
        .map(|offset| (offset % 127) as u8 | 0x80)
        .collect();

    tx0.send(&data0, vec![], vec![]).unwrap();
    tx1.send(&data1, vec![], vec![]).unwrap();
    let (mut received0, mut received1) = (false, false);
    while !received0 || !received1 {
        for result in rx_set.select().unwrap().into_iter() {
            let (received_id, mut ipc_message) = result.unwrap();
            ipc_message.data.truncate(65536);
            assert!(received_id == rx0_id || received_id == rx1_id);
            if received_id == rx0_id {
                assert_eq!(ipc_message.data, data0);
                assert!(!received0);
                received0 = true;
            } else if received_id == rx1_id {
                assert_eq!(ipc_message.data, data1);
                assert!(!received1);
                received1 = true;
            }
        }
    }
}

#[test]
fn receiver_set_big_data() {
    let (tx0, rx0) = platform::channel().unwrap();
    let (tx1, rx1) = platform::channel().unwrap();
    let mut rx_set = OsIpcReceiverSet::new().unwrap();
    let rx0_id = rx_set.add(rx0).unwrap();
    let rx1_id = rx_set.add(rx1).unwrap();

    let data0: Vec<u8> = (0..1024 * 1024)
        .map(|offset| (offset % 127) as u8)
        .collect();
    let data1: Vec<u8> = (0..1024 * 1024)
        .map(|offset| (offset % 127) as u8 | 0x80)
        .collect();
    let (reference_data0, reference_data1) = (data0.clone(), data1.clone());

    let thread0 = thread::spawn(move || {
        tx0.send(&data0, vec![], vec![]).unwrap();
        tx0 // Don't close just yet -- the receiver-side test code below doesn't expect that...
    });
    let thread1 = thread::spawn(move || {
        tx1.send(&data1, vec![], vec![]).unwrap();
        tx1
    });

    let (mut received0, mut received1) = (false, false);
    while !received0 || !received1 {
        for result in rx_set.select().unwrap().into_iter() {
            let (received_id, mut ipc_message) = result.unwrap();
            ipc_message.data.truncate(1024 * 1024);
            assert!(received_id == rx0_id || received_id == rx1_id);
            if received_id == rx0_id {
                assert_eq!(ipc_message.data.len(), reference_data0.len());
                assert_eq!(ipc_message.data, reference_data0);
                assert!(!received0);
                received0 = true;
            } else if received_id == rx1_id {
                assert_eq!(ipc_message.data.len(), reference_data1.len());
                assert_eq!(ipc_message.data, reference_data1);
                assert!(!received1);
                received1 = true;
            }
        }
    }

    thread0.join().unwrap();
    thread1.join().unwrap();
}

#[test]
fn receiver_set_concurrent() {
    let num_channels = 5;
    let messages_per_channel = 100;
    let max_msg_size = 16381;

    let mut rx_set = OsIpcReceiverSet::new().unwrap();

    let (threads, mut receiver_records): (Vec<_>, HashMap<_, _>) = (0..num_channels)
        .map(|chan_index| {
            let (tx, rx) = platform::channel().unwrap();
            let rx_id = rx_set.add(rx).unwrap();

            let data: Vec<u8> = (0..max_msg_size)
                .map(|offset| (offset % 13) as u8 | (chan_index as u8) << 4)
                .collect();
            let reference_data = data.clone();

            let thread = thread::spawn(move || {
                for msg_index in 0..messages_per_channel {
                    let msg_size = (msg_index * 99991 + chan_index * 90001) % max_msg_size;
                    // The `macos` back-end won't receive exact size unless it's a multiple of 4...
                    // (See https://github.com/servo/ipc-channel/pull/79 etc. )
                    let msg_size = msg_size & !3;
                    tx.send(&data[0..msg_size], vec![], vec![]).unwrap();
                }
            });
            (thread, (rx_id, (reference_data, chan_index, 0usize)))
        })
        .unzip();

    while !receiver_records.is_empty() {
        for result in rx_set.select().unwrap().into_iter() {
            match result {
                platform::OsIpcSelectionResult::DataReceived(rx_id, ipc_message) => {
                    let &mut (ref reference_data, chan_index, ref mut msg_index) =
                        receiver_records.get_mut(&rx_id).unwrap();
                    let msg_size = (*msg_index * 99991 + chan_index * 90001) % max_msg_size;
                    let msg_size = msg_size & !3;
                    assert_eq!(ipc_message.data.len(), msg_size);
                    assert_eq!(&ipc_message.data[..], &reference_data[..msg_size]);
                    *msg_index += 1;
                },
                platform::OsIpcSelectionResult::ChannelClosed(rx_id) => {
                    let (_, _, msg_index) = receiver_records.remove(&rx_id).unwrap();
                    assert_eq!(msg_index, messages_per_channel);
                },
            }
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn server_accept_first() {
    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let data: &[u8] = b"1234567";

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        let tx = OsIpcSender::connect(name).unwrap();
        tx.send(data, vec![], vec![]).unwrap();
    });

    let (_, ipc_message) = server.accept().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn server_connect_first() {
    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let data: &[u8] = b"1234567";

    thread::spawn(move || {
        let tx = OsIpcSender::connect(name).unwrap();
        tx.send(data, vec![], vec![]).unwrap();
    });

    thread::sleep(Duration::from_millis(30));
    let (_, mut ipc_message) = server.accept().unwrap();
    ipc_message.data.truncate(7);
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
#[test]
fn cross_process_spawn() {
    let data: &[u8] = b"1234567";

    let channel_name = get_channel_name_arg("server");
    if let Some(channel_name) = channel_name {
        let tx = OsIpcSender::connect(channel_name).unwrap();
        tx.send(data, vec![], vec![]).unwrap();

        unsafe {
            libc::exit(0);
        }
    }

    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let mut child_pid = spawn_server("cross_process_spawn", &[("server", &*name)]);

    let (_, ipc_message) = server.accept().unwrap();
    child_pid.wait().expect("failed to wait on child");
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
#[test]
fn cross_process_fork() {
    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let data: &[u8] = b"1234567";

    let child_pid = unsafe {
        fork(|| {
            let tx = OsIpcSender::connect(name).unwrap();
            tx.send(data, vec![], vec![]).unwrap();
        })
    };

    let (_, ipc_message) = server.accept().unwrap();
    child_pid.wait();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
#[test]
fn cross_process_sender_transfer_spawn() {
    let channel_name = get_channel_name_arg("server");
    if let Some(channel_name) = channel_name {
        let super_tx = OsIpcSender::connect(channel_name).unwrap();
        let (sub_tx, sub_rx) = platform::channel().unwrap();
        let data: &[u8] = b"foo";
        super_tx
            .send(data, vec![OsIpcChannel::Sender(sub_tx)], vec![])
            .unwrap();
        sub_rx.recv().unwrap();
        let data: &[u8] = b"bar";
        super_tx.send(data, vec![], vec![]).unwrap();

        unsafe {
            libc::exit(0);
        }
    }

    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let mut child_pid = spawn_server("cross_process_sender_transfer_spawn", &[("server", &*name)]);

    let (super_rx, mut ipc_message) = server.accept().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_tx = ipc_message.os_ipc_channels[0].to_sender();
    let data: &[u8] = b"baz";
    sub_tx.send(data, vec![], vec![]).unwrap();

    let data: &[u8] = b"bar";
    let ipc_message = super_rx.recv().unwrap();
    child_pid.wait().expect("failed to wait on child");
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
#[test]
fn cross_process_sender_transfer_fork() {
    let (server, name) = OsIpcOneShotServer::new().unwrap();

    let child_pid = unsafe {
        fork(|| {
            let super_tx = OsIpcSender::connect(name).unwrap();
            let (sub_tx, sub_rx) = platform::channel().unwrap();
            let data: &[u8] = b"foo";
            super_tx
                .send(data, vec![OsIpcChannel::Sender(sub_tx)], vec![])
                .unwrap();
            sub_rx.recv().unwrap();
            let data: &[u8] = b"bar";
            super_tx.send(data, vec![], vec![]).unwrap();
        })
    };

    let (super_rx, mut ipc_message) = server.accept().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_tx = ipc_message.os_ipc_channels[0].to_sender();
    let data: &[u8] = b"baz";
    sub_tx.send(data, vec![], vec![]).unwrap();

    let data: &[u8] = b"bar";
    let ipc_message = super_rx.recv().unwrap();
    child_pid.wait();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
}

#[test]
fn no_senders_notification() {
    let (sender, receiver) = platform::channel().unwrap();
    drop(sender);
    let result = receiver.recv();
    assert!(result.is_err());
    assert!(result.unwrap_err().channel_is_closed());
}

/// Checks that a broken pipe notification is returned by `send()`
/// after the receive end was closed.
#[test]
fn no_receiver_notification() {
    let (sender, receiver) = platform::channel().unwrap();
    drop(receiver);
    let data: &[u8] = b"1234567";
    loop {
        if let Err(err) = sender.send(data, vec![], vec![]) {
            // We don't have an actual method for distinguishing a "broken pipe" error --
            // but at least it's not supposed to signal the same condition as closing the sender.
            assert!(!err.channel_is_closed());
            break;
        }
    }
}

/// Checks for broken pipe notification when receiver is closed
/// while there are pending unreceived messages.
///
/// This can result in a different error condition
/// than dropping the receiver before a send is attempted.
/// (Linux reports `ECONNRESET` instead of `EPIPE` in this case.)
#[test]
fn no_receiver_notification_pending() {
    let (sender, receiver) = platform::channel().unwrap();
    let data: &[u8] = b"1234567";

    let result = sender.send(data, vec![], vec![]);
    assert!(result.is_ok());

    drop(receiver);
    loop {
        if let Err(err) = sender.send(data, vec![], vec![]) {
            // We don't have an actual method for distinguishing a "broken pipe" error --
            // but at least it's not supposed to signal the same condition as closing the sender.
            assert!(!err.channel_is_closed());
            break;
        }
    }
}

/// Checks for broken pipe notification when receiver is closed after a delay.
///
/// This might uncover some timing-related issues.
#[test]
fn no_receiver_notification_delayed() {
    let (sender, receiver) = platform::channel().unwrap();

    let thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(42));
        drop(receiver);
    });

    let data: &[u8] = b"1234567";
    loop {
        if let Err(err) = sender.send(data, vec![], vec![]) {
            // We don't have an actual method for distinguishing a "broken pipe" error --
            // but at least it's not supposed to signal the same condition as closing the sender.
            assert!(!err.channel_is_closed());
            break;
        }
    }

    thread.join().unwrap();
}

#[test]
fn shared_memory() {
    let (tx, rx) = platform::channel().unwrap();
    let data: &[u8] = b"1234567";
    let shmem_data = OsIpcSharedMemory::from_byte(0xba, 1024 * 1024);
    tx.send(data, vec![], vec![shmem_data]).unwrap();
    let ipc_message = rx.recv().unwrap();
    assert_eq!(&ipc_message.data, data);
    assert!(&ipc_message.os_ipc_channels.is_empty());
    assert_eq!(
        ipc_message.os_ipc_shared_memory_regions[0].len(),
        1024 * 1024
    );
    assert!(ipc_message.os_ipc_shared_memory_regions[0]
        .iter()
        .all(|byte| *byte == 0xba));
}

#[test]
fn shared_memory_clone() {
    let shmem_data_0 = OsIpcSharedMemory::from_byte(0xba, 1024 * 1024);
    let shmem_data_1 = shmem_data_0.clone();
    assert_eq!(&shmem_data_0[..], &shmem_data_1[..]);
}

#[test]
fn try_recv() {
    let (tx, rx) = platform::channel().unwrap();
    assert!(rx.try_recv().is_err());
    let data: &[u8] = b"1234567";
    tx.send(data, Vec::new(), Vec::new()).unwrap();
    let ipc_message = rx.try_recv().unwrap();
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    assert!(rx.try_recv().is_err());
}

/// Checks that a channel closed notification is returned by `try_recv()`.
///
/// Also checks that the "no data" notification returned by `try_recv()`
/// when no data is pending but before the channel is closed,
/// is distinguishable from the actual "channel closed" notification.
#[test]
fn no_senders_notification_try_recv() {
    let (sender, receiver) = platform::channel().unwrap();
    let result = receiver.try_recv();
    assert!(result.is_err());
    assert!(!result.unwrap_err().channel_is_closed());
    drop(sender);
    loop {
        let result = receiver.try_recv();
        assert!(result.is_err());
        if result.unwrap_err().channel_is_closed() {
            break;
        }
    }
}

/// Checks for channel closed notification when receiver is closed after a delay.
///
/// This might uncover some timing-related issues.
#[test]
fn no_senders_notification_try_recv_delayed() {
    let (sender, receiver) = platform::channel().unwrap();
    let result = receiver.try_recv();
    assert!(result.is_err());
    assert!(!result.unwrap_err().channel_is_closed());

    let thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(42));
        drop(sender);
    });

    loop {
        let result = receiver.try_recv();
        assert!(result.is_err());
        if result.unwrap_err().channel_is_closed() {
            break;
        }
    }

    thread.join().unwrap();
}

#[test]
fn try_recv_large() {
    let (tx, rx) = platform::channel().unwrap();
    assert!(rx.try_recv().is_err());

    let thread = thread::spawn(move || {
        let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
        let data: &[u8] = &data[..];
        tx.send(data, vec![], vec![]).unwrap();
    });

    let mut result;
    while {
        result = rx.try_recv();
        result.is_err()
    } {}
    thread.join().unwrap();
    let ipc_message = result.unwrap();

    let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
    let data: &[u8] = &data[..];
    assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    assert!(rx.try_recv().is_err());
}

#[test]
fn try_recv_large_delayed() {
    // These settings work well on my system when doing cargo test --release.
    // Haven't found a good way to test this with non-release builds...
    let num_senders = 10;
    let thread_delay = 50;
    let msg_size = 512 * 1024;

    let delay = {
        fn delay_iterations(iterations: u64) {
            // Let's hope rustc won't ever be able to optimise this away...
            let mut v = vec![];
            for _ in 0..iterations {
                v.push(0);
                v.pop();
            }
        }

        let iterations_per_ms;
        {
            let (mut iterations, mut time_per_run) = (1u64, Duration::new(0, 0));
            while time_per_run < Duration::new(0, 10_000_000) {
                iterations *= 10;
                let start = Instant::now();
                delay_iterations(iterations);
                time_per_run = start.elapsed();
            }
            // This assumes time_per_run stays below one second.
            // Unless something weird happens, we should be safe within this margin...
            iterations_per_ms = iterations * 1_000_000 / time_per_run.subsec_nanos() as u64;
        }

        Arc::new(move |ms: u64| delay_iterations(ms * iterations_per_ms))
    };

    let (tx, rx) = platform::channel().unwrap();
    assert!(rx.try_recv().is_err());

    let threads: Vec<_> = (0..num_senders)
        .map(|i| {
            let tx = tx.clone();
            let delay = delay.clone();
            thread::spawn(move || {
                let data: Vec<u8> = (0..msg_size).map(|j| (j % 13) as u8 | i << 4).collect();
                let data: &[u8] = &data[..];
                delay(thread_delay);
                tx.send(data, vec![], vec![]).unwrap();
            })
        })
        .collect();

    let mut received_vals: Vec<u8> = vec![];
    for _ in 0..num_senders {
        let mut result;
        while {
            result = rx.try_recv();
            result.is_err()
        } {}
        let ipc_message = result.unwrap();

        let val = ipc_message.data[0] >> 4;
        received_vals.push(val);
        let data: Vec<u8> = (0..msg_size).map(|j| (j % 13) as u8 | val << 4).collect();
        let data: &[u8] = &data[..];
        assert_eq!(ipc_message.data.len(), data.len());
        assert_eq!(ipc_message, IpcMessage::from_data(data.to_vec()));
    }
    assert!(rx.try_recv().is_err()); // There should be no further messages pending.
    received_vals.sort();
    assert_eq!(received_vals, (0..num_senders).collect::<Vec<_>>()); // Got exactly the values we sent.

    for thread in threads {
        thread.join().unwrap();
    }
}

mod sync_test {
    use crate::platform;
    use static_assertions::assert_not_impl_any;

    #[test]
    fn receiver_not_sync() {
        assert_not_impl_any!(platform::OsIpcSender : Sync);
    }
}

// This test panics on Windows, because the other process will panic
// when it detects that it receives handles that are intended for another
// process.  It's marked as ignore/known-fail on Windows for this reason.
//
// TODO -- this fails on OSX as well with a MACH_SEND_INVALID_RIGHT!
// Needs investigation.  It may be a similar underlying issue, just done by
// the kernel instead of explicitly (ports in a message that's already
// buffered are intended for only one process).
#[cfg(not(any(feature = "force-inprocess", target_os = "android", target_os = "ios")))]
#[cfg_attr(any(target_os = "windows"), ignore)]
#[test]
fn cross_process_two_step_transfer_spawn() {
    let cookie: &[u8] = b"cookie";

    let channel_name = get_channel_name_arg("server");
    if let Some(channel_name) = channel_name {
        // connect by name to our other process
        let super_tx = OsIpcSender::connect(channel_name).unwrap();

        // create a channel for real communication between the two processes
        let (sub_tx, sub_rx) = platform::channel().unwrap();

        // send the other process the tx side, so it can send us the channels
        super_tx
            .send(&[], vec![OsIpcChannel::Sender(sub_tx)], vec![])
            .unwrap();

        // get two_rx from the other process
        let mut ipc_message = sub_rx.recv().unwrap();
        assert_eq!(ipc_message.os_ipc_channels.len(), 1);
        let two_rx = ipc_message.os_ipc_channels[0].to_receiver();

        // get one_rx from two_rx's buffer
        let mut ipc_message = two_rx.recv().unwrap();
        assert_eq!(ipc_message.os_ipc_channels.len(), 1);
        let one_rx = ipc_message.os_ipc_channels[0].to_receiver();

        // get a cookie from one_rx
        let ipc_message = one_rx.recv().unwrap();
        assert_eq!(&ipc_message.data[..], cookie);

        // finally, send a cookie back
        super_tx.send(&ipc_message.data, vec![], vec![]).unwrap();

        // terminate
        unsafe {
            libc::exit(0);
        }
    }

    // create channel 1
    let (one_tx, one_rx) = platform::channel().unwrap();
    // put data in channel 1's pipe
    one_tx.send(cookie, vec![], vec![]).unwrap();

    // create channel 2
    let (two_tx, two_rx) = platform::channel().unwrap();
    // put channel 1's rx end in channel 2's pipe
    two_tx
        .send(&[], vec![OsIpcChannel::Receiver(one_rx)], vec![])
        .unwrap();

    // create a one-shot server, and spawn another process
    let (server, name) = OsIpcOneShotServer::new().unwrap();
    let mut child_pid = spawn_server(
        "cross_process_two_step_transfer_spawn",
        &[("server", &*name)],
    );

    // The other process will have sent us a transmit channel in received channels
    let (super_rx, mut ipc_message) = server.accept().unwrap();
    assert_eq!(ipc_message.os_ipc_channels.len(), 1);
    let sub_tx = ipc_message.os_ipc_channels[0].to_sender();

    // Send the outer payload channel, so the server can use it to
    // retrieve the inner payload and the cookie
    sub_tx
        .send(&[], vec![OsIpcChannel::Receiver(two_rx)], vec![])
        .unwrap();

    // Then we wait for the cookie to make its way back to us
    let ipc_message = super_rx.recv().unwrap();
    let child_exit_code = child_pid.wait().expect("failed to wait on child");
    assert!(child_exit_code.success());
    assert_eq!(ipc_message, IpcMessage::from_data(cookie.to_vec()));
}
