#![cfg(feature = "invocation")]

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Barrier,
    },
    thread::spawn,
    time::Duration,
};

use jni::{objects::AutoLocal, sys::jint, Executor};

use rusty_fork::rusty_fork_test;

mod util;
use util::{jvm, AtomicIntegerProxy};

#[test]
fn single_thread() {
    let executor = Executor::new(jvm().clone());
    test_single_thread(executor);
}

#[test]
fn serialized_threads() {
    let executor = Executor::new(jvm().clone());
    test_serialized_threads(executor);
}

#[test]
fn concurrent_threads() {
    let executor = Executor::new(jvm().clone());
    const THREAD_NUM: usize = 8;
    test_concurrent_threads(executor, THREAD_NUM)
}

fn test_single_thread(executor: Executor) {
    let mut atomic = AtomicIntegerProxy::new(executor, 0).unwrap();
    assert_eq!(0, atomic.get().unwrap());
    assert_eq!(1, atomic.increment_and_get().unwrap());
    assert_eq!(3, atomic.add_and_get(2).unwrap());
    assert_eq!(3, atomic.get().unwrap());
}

fn test_serialized_threads(executor: Executor) {
    let mut atomic = AtomicIntegerProxy::new(executor, 0).unwrap();
    assert_eq!(0, atomic.get().unwrap());
    let jh = spawn(move || {
        assert_eq!(1, atomic.increment_and_get().unwrap());
        assert_eq!(3, atomic.add_and_get(2).unwrap());
        atomic
    });
    let mut atomic = jh.join().unwrap();
    assert_eq!(3, atomic.get().unwrap());
}

fn test_concurrent_threads(executor: Executor, thread_num: usize) {
    const ITERS_PER_THREAD: usize = 10_000;

    let mut atomic = AtomicIntegerProxy::new(executor, 0).unwrap();
    let barrier = Arc::new(Barrier::new(thread_num));
    let mut threads = Vec::new();

    for _ in 0..thread_num {
        let barrier = Arc::clone(&barrier);
        let mut atomic = atomic.clone();
        let jh = spawn(move || {
            barrier.wait();
            for _ in 0..ITERS_PER_THREAD {
                atomic.increment_and_get().unwrap();
            }
        });
        threads.push(jh);
    }
    for jh in threads {
        jh.join().unwrap();
    }
    let expected = (ITERS_PER_THREAD * thread_num) as jint;
    assert_eq!(expected, atomic.get().unwrap());
}

// We need to test `JavaVM::destroy()` in a separate process otherwise it will break
// all the other tests
rusty_fork_test! {
#[test]
fn test_destroy() {
    const THREAD_NUM: usize = 2;
    const DAEMON_THREAD_NUM: usize = 2;
    static MATH_CLASS: &str = "java/lang/Math";

    // We don't test this using an `Executor` because we don't want to
    // attach all the threads as daemon threads.

    let jvm = jvm().clone();

    let atomic = Arc::new(AtomicUsize::new(0));

    let attach_barrier = Arc::new(Barrier::new(THREAD_NUM + DAEMON_THREAD_NUM + 1));
    let daemons_detached_barrier = Arc::new(Barrier::new(DAEMON_THREAD_NUM + 1));
    let mut threads = Vec::new();

    for _ in 0..THREAD_NUM {
        let attach_barrier = Arc::clone(&attach_barrier);
        let jvm = jvm.clone();
        let atomic = atomic.clone();
        let jh = spawn(move || {
            let mut env = jvm.attach_current_thread().unwrap();
            println!("java thread attach");
            attach_barrier.wait();
            println!("java thread run");
            std::thread::sleep(Duration::from_millis(250));

            println!("use before destroy...");
            // Make some token JNI call
            let _class = AutoLocal::new(env.find_class(MATH_CLASS).unwrap(), &env);

            atomic.fetch_add(1, Ordering::SeqCst);

            println!("java thread finished");
        });
        threads.push(jh);
    }

    for _ in 0..DAEMON_THREAD_NUM {
        let attach_barrier = Arc::clone(&attach_barrier);
        let daemons_detached_barrier = Arc::clone(&daemons_detached_barrier);
        let jvm = jvm.clone();
        let atomic = atomic.clone();
        let jh = spawn(move || {
            // We have to be _very_ careful to ensure we have finished accessing the
            // JavaVM before it gets destroyed, including dropping the AutoLocal
            // for the `MATH_CLASS`
            {
                let mut env = jvm.attach_current_thread_as_daemon().unwrap();
                println!("daemon thread attach");
                attach_barrier.wait();
                println!("daemon thread run");

                println!("daemon JVM use before destroy...");

                let _class = AutoLocal::new(env.find_class(MATH_CLASS).unwrap(), &env);
            }

            // For it to be safe to call `JavaVM::destroy()` we need to ensure that
            // daemon threads are detached from the JavaVM ahead of time because
            // `JavaVM::destroy()` does not synchronize and wait for them to exit
            // which means we would effectively trigger a use-after-free when daemon
            // threads exit and they try to automatically detach from the `JavaVM`
            //
            // # Safety
            // We won't be accessing any (invalid) `JNIEnv` once we have detached this
            // thread
            unsafe {
                jvm.detach_current_thread();
            }

            daemons_detached_barrier.wait();

            for _ in 0..10 {
                std::thread::sleep(Duration::from_millis(100));
                println!("daemon thread running");
            }

            atomic.fetch_add(1, Ordering::SeqCst);

            println!("daemon thread finished");
        });
        threads.push(jh);
    }

    // At this point we at least know that all threads have been attached
    // to the JVM
    println!("MAIN: waiting for threads attached barrier");
    attach_barrier.wait();

    // Before we try and destroy the JavaVM we need to be sure that the daemon
    // threads have finished using the VM since `jvm.destroy()` won't wait
    // for daemon threads to exit.
    println!("MAIN: waiting for daemon threads detached barrier");
    daemons_detached_barrier.wait();

    // # Safety
    //
    // We drop the `jvm` variable immediately after `destroy()` returns to avoid
    // any use-after-free.
    unsafe {
        println!("MAIN: calling DestroyJavaVM()...");
        jvm.destroy().unwrap();
        drop(jvm);
        println!("MAIN: jvm destroyed");
    }

    println!("MAIN: joining (waiting for) all threads");
    let mut joined = 0;
    for jh in threads {
        jh.join().unwrap();
        joined += 1;
        println!(
            "joined {joined} threads, atomic = {}",
            atomic.load(Ordering::SeqCst)
        );
    }

    assert_eq!(
        atomic.load(Ordering::SeqCst),
        THREAD_NUM + DAEMON_THREAD_NUM
    );
}

}
