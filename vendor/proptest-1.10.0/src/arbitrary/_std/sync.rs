//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::sync`.

use std::fmt;
use std::sync::mpsc::*;
use std::sync::*;
use std::thread;
use std::time::Duration;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

// OnceState can not escape Once::call_once_force.
// PoisonError depends implicitly on the lifetime on MutexGuard, etc.
// This transitively applies to TryLockError.

// Not doing Weak because .upgrade() would always return None.

#[cfg(not(feature = "unstable"))]
wrap_ctor!(Mutex);
#[cfg(feature = "unstable")]
wrap_from!(Mutex);

#[cfg(not(feature = "unstable"))]
wrap_ctor!(RwLock);
#[cfg(feature = "unstable")]
wrap_from!(RwLock);

arbitrary!(Barrier, SMapped<u16, Self>;  // usize would be extreme!
    static_map(any::<u16>(), |n| Barrier::new(n as usize))
);

arbitrary!(BarrierWaitResult,
    TupleUnion<(WA<LazyJustFn<Self>>, WA<LazyJustFn<Self>>)>;
    prop_oneof![LazyJust::new(bwr_true), LazyJust::new(bwr_false)]
);

lazy_just!(
    Condvar, Default::default;
    Once, Once::new
);

arbitrary!(WaitTimeoutResult, TupleUnion<(WA<Just<Self>>, WA<Just<Self>>)>;
    prop_oneof![Just(wtr_true()), Just(wtr_false())]
);

fn bwr_true() -> BarrierWaitResult {
    Barrier::new(1).wait()
}

fn bwr_false() -> BarrierWaitResult {
    let barrier = Arc::new(Barrier::new(2));
    let b2 = barrier.clone();
    let jh = thread::spawn(move || b2.wait());
    let bwr1 = barrier.wait();
    let bwr2 = jh.join().unwrap();
    if bwr1.is_leader() {
        bwr2
    } else {
        bwr1
    }
}

fn wtr_false() -> WaitTimeoutResult {
    let cvar = Arc::new(Condvar::new());
    let cvar2 = cvar.clone();
    thread::spawn(move || {
        cvar2.notify_one();
    });
    let lock = Mutex::new(());
    let wt = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(1));
    let (_unused, wtr) = wt.unwrap();
    wtr
}

fn wtr_true() -> WaitTimeoutResult {
    let cvar = Condvar::new();
    let lock = Mutex::new(());
    let wt = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(0));
    let (_unused, wtr) = wt.unwrap();
    wtr
}

arbitrary!(RecvError; RecvError);

arbitrary!([T: Arbitrary] SendError<T>, SMapped<T, Self>, T::Parameters;
    args => static_map(any_with::<T>(args), SendError)
);

arbitrary!(RecvTimeoutError, TupleUnion<(WA<Just<Self>>, WA<Just<Self>>)>;
    prop_oneof![
        Just(RecvTimeoutError::Disconnected),
        Just(RecvTimeoutError::Timeout)
    ]
);

arbitrary!(TryRecvError, TupleUnion<(WA<Just<Self>>, WA<Just<Self>>)>;
    prop_oneof![
        Just(TryRecvError::Disconnected),
        Just(TryRecvError::Empty)
    ]
);

arbitrary!(
    [P: Clone + Default, T: Arbitrary<Parameters = P>] TrySendError<T>,
    TupleUnion<(WA<SMapped<T, Self>>, WA<SMapped<T, Self>>)>, P;
    args => prop_oneof![
        static_map(any_with::<T>(args.clone()), TrySendError::Disconnected),
        static_map(any_with::<T>(args), TrySendError::Full),
    ]
);

// If only half of a pair is generated then you will get a hang-up.
// Thus the only meaningful impls are in pairs.
arbitrary!([A] (Sender<A>, Receiver<A>), LazyJustFn<Self>;
    LazyJust::new(channel)
);

arbitrary!([A: fmt::Debug] (Sender<A>, IntoIter<A>), LazyJustFn<Self>;
    LazyJust::new(|| {
        let (rx, tx) = channel();
        (rx, tx.into_iter())
    })
);

arbitrary!([A] (SyncSender<A>, Receiver<A>), SMapped<u16, Self>;
    static_map(any::<u16>(), |size| sync_channel(size as usize))
);

arbitrary!([A: fmt::Debug] (SyncSender<A>, IntoIter<A>), SMapped<u16, Self>;
    static_map(any::<u16>(), |size| {
        let (rx, tx) = sync_channel(size as usize);
        (rx, tx.into_iter())
    })
);

#[cfg(test)]
mod test {
    no_panic_test!(
        mutex => Mutex<u8>,
        rw_lock => RwLock<u8>,
        barrier => Barrier,
        barrier_wait_result => BarrierWaitResult,
        condvar => Condvar,
        once => Once,
        wait_timeout_result => WaitTimeoutResult,
        recv_error => RecvError,
        send_error => SendError<u8>,
        recv_timeout_error => RecvTimeoutError,
        try_recv_error => TryRecvError,
        try_send_error => TrySendError<u8>,
        rx_tx => (Sender<u8>, Receiver<u8>),
        rx_txiter => (Sender<u8>, IntoIter<u8>),
        syncrx_tx => (SyncSender<u8>, Receiver<u8>),
        syncrx_txiter => (SyncSender<u8>, IntoIter<u8>)
    );
}
