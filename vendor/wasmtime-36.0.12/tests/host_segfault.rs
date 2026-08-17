// To handle out-of-bounds reads and writes we use segfaults right now. We only
// want to catch a subset of segfaults, however, rather than all segfaults
// happening everywhere. The purpose of this test is to ensure that we *don't*
// catch segfaults if it happens in a random place in the code, but we instead
// bail out of our segfault handler early.
//
// This is sort of hard to test for but the general idea here is that we confirm
// that execution made it to our `segfault` function by printing something, and
// then we also make sure that stderr is empty to confirm that no weird panics
// happened or anything like that.

use libtest_mimic::{Arguments, Trial};
use std::env;
use std::future::Future;
use std::io::{self, Write};
use std::pin::Pin;
use std::process::{Command, ExitStatus};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasmtime::*;

const VAR_NAME: &str = "__TEST_TO_RUN";
const CONFIRM: &str = "well at least we ran up to the crash";

fn segfault() -> ! {
    unsafe {
        println!("{CONFIRM}");
        io::stdout().flush().unwrap();
        *(0x4 as *mut i32) = 3;
        unreachable!()
    }
}

fn allocate_stack_space() -> ! {
    let _a = [0u8; 1024];

    for _ in 0..100000 {
        allocate_stack_space();
    }

    unreachable!()
}

fn overrun_the_stack() -> ! {
    println!("{CONFIRM}");
    io::stdout().flush().unwrap();
    allocate_stack_space();
}

fn run_future<F: Future>(future: F) -> F::Output {
    let mut f = Pin::from(Box::new(future));
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => {}
        }
    }
}

fn dummy_waker() -> Waker {
    return unsafe { Waker::from_raw(clone(5 as *const _)) };

    unsafe fn clone(ptr: *const ()) -> RawWaker {
        assert_eq!(ptr as usize, 5);
        const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        RawWaker::new(ptr, &VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        assert_eq!(ptr as usize, 5);
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        assert_eq!(ptr as usize, 5);
    }

    unsafe fn drop(ptr: *const ()) {
        assert_eq!(ptr as usize, 5);
    }
}

#[derive(PartialEq, Copy, Clone)]
enum StackOverflow {
    No,
    Host,
    HostAsyncStack,
    Wasm,
}

fn main() {
    if cfg!(miri) || cfg!(asan) {
        return;
    }
    // Skip this tests if it looks like we're in a cross-compiled situation and
    // we're emulating this test for a different platform. In that scenario
    // emulators (like QEMU) tend to not report signals the same way and such.
    if wasmtime_test_util::cargo_test_runner().is_some() {
        return;
    }

    // Disable core dumps on Unix to avoid spamming the system core dump utility
    // and having this test take longer.
    #[cfg(unix)]
    unsafe {
        let zero = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        let rc = libc::setrlimit(libc::RLIMIT_CORE, &zero);
        assert_eq!(
            rc,
            0,
            "failed to disable core dumps: {}",
            std::io::Error::last_os_error()
        );
    }

    let tests: &[(&str, fn(), StackOverflow)] = &[
        ("normal segfault", || segfault(), StackOverflow::No),
        (
            "make instance then segfault",
            || {
                let engine = Engine::default();
                let mut store = Store::new(&engine, ());
                let module = Module::new(&engine, "(module)").unwrap();
                let _instance = Instance::new(&mut store, &module, &[]).unwrap();
                segfault();
            },
            StackOverflow::No,
        ),
        (
            "make instance then overrun the stack",
            || {
                let engine = Engine::default();
                let mut store = Store::new(&engine, ());
                let module = Module::new(&engine, "(module)").unwrap();
                let _instance = Instance::new(&mut store, &module, &[]).unwrap();
                overrun_the_stack();
            },
            StackOverflow::Host,
        ),
        (
            "segfault in a host function",
            || {
                let engine = Engine::default();
                let mut store = Store::new(&engine, ());
                let module = Module::new(&engine, r#"(import "" "" (func)) (start 0)"#).unwrap();
                let segfault = Func::wrap(&mut store, || -> () { segfault() });
                Instance::new(&mut store, &module, &[segfault.into()]).unwrap();
                unreachable!();
            },
            StackOverflow::No,
        ),
        (
            "hit async stack guard page",
            || {
                let mut config = Config::default();
                config.async_support(true);
                let engine = Engine::new(&config).unwrap();
                let mut store = Store::new(&engine, ());
                let f = Func::wrap_async(&mut store, |_, _: ()| {
                    Box::new(async {
                        if true {
                            overrun_the_stack();
                        }
                    })
                });
                run_future(f.call_async(&mut store, &[], &mut [])).unwrap();
                unreachable!();
            },
            StackOverflow::HostAsyncStack,
        ),
        (
            "overrun 8k with misconfigured host",
            || overrun_with_big_module(8 << 10),
            StackOverflow::Wasm,
        ),
        (
            "overrun 32k with misconfigured host",
            || overrun_with_big_module(32 << 10),
            StackOverflow::Wasm,
        ),
        #[cfg(not(any(target_arch = "riscv64")))]
        // Due to `InstanceAllocationStrategy::pooling()` trying to alloc more than 6000G memory space.
        // https://gitlab.com/qemu-project/qemu/-/issues/1214
        // https://gitlab.com/qemu-project/qemu/-/issues/290
        (
            "hit async stack guard page with pooling allocator",
            || {
                let mut config = Config::default();
                config.async_support(true);
                let mut cfg = PoolingAllocationConfig::default();
                cfg.total_memories(1);
                cfg.max_memory_size(1 << 16);
                cfg.total_tables(1);
                cfg.table_elements(10);
                cfg.total_stacks(1);
                config.allocation_strategy(cfg);
                let engine = Engine::new(&config).unwrap();
                let mut store = Store::new(&engine, ());
                let f = Func::wrap_async(&mut store, |_, _: ()| {
                    Box::new(async {
                        if true {
                            overrun_the_stack();
                        }
                    })
                });
                run_future(f.call_async(&mut store, &[], &mut [])).unwrap();
                unreachable!();
            },
            StackOverflow::HostAsyncStack,
        ),
    ];
    match env::var(VAR_NAME) {
        Ok(s) => {
            let test = tests
                .iter()
                .find(|p| p.0 == s)
                .expect("failed to find test")
                .1;
            test();
        }
        Err(_) => {
            let mut trials = Vec::new();
            for (name, _test, stack_overflow) in tests {
                trials.push(Trial::test(name.to_string(), || {
                    run_test(name, *stack_overflow);
                    Ok(())
                }));
            }
            libtest_mimic::run(&Arguments::from_args(), trials).exit()
        }
    }
}

fn run_test(name: &str, stack_overflow: StackOverflow) {
    let me = env::current_exe().unwrap();
    let mut cmd = Command::new(me);
    cmd.env(VAR_NAME, name);
    let output = cmd.output().expect("failed to spawn subprocess");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut desc = format!("got status: {}", output.status);

    if !stdout.trim().is_empty() {
        desc.push_str("\nstdout: ----\n");
        desc.push_str("    ");
        desc.push_str(&stdout.replace("\n", "\n    "));
    }

    if !stderr.trim().is_empty() {
        desc.push_str("\nstderr: ----\n");
        desc.push_str("    ");
        desc.push_str(&stderr.replace("\n", "\n    "));
    }

    match stack_overflow {
        // If the host stack overflows then the result should always indicate a
        // stack overflow. If the guest stack overflows then that won't actually
        // trigger an overflow when Cranelift doesn't have native support
        // because Pulley is used in that case.
        StackOverflow::Host | StackOverflow::Wasm => {
            let native_stack_overflow = is_stack_overflow(&output.status, &stderr);
            let expect_native_overflow =
                stack_overflow == StackOverflow::Host || cranelift_native::builder().is_ok();

            if native_stack_overflow == expect_native_overflow {
                assert!(
                    stdout.trim().ends_with(CONFIRM),
                    "failed to find confirmation in test `{name}`\n{desc}"
                );
            } else {
                panic!("\n\nexpected a stack overflow on `{name}`\n{desc}\n\n");
            }
        }

        // When overflowing an async stack platforms that don't have signal
        // handlers aren't able to print a message about stack overflow, so a
        // normal segfault is expected. Otherwise though if a wasmtime-based
        // signal handler is installed a message and SIGABRT is expected.
        StackOverflow::HostAsyncStack => {
            let native_stack_overflow = if cfg!(has_native_signals) {
                is_stack_overflow(&output.status, &stderr)
            } else {
                is_segfault(&output.status)
            };
            assert!(
                native_stack_overflow,
                "expected a native stack overflow for `{name}`:\n{desc}"
            );
            assert!(
                stdout.trim().ends_with(CONFIRM),
                "failed to find confirmation in test `{name}`\n{desc}"
            );
        }

        StackOverflow::No => {
            if is_segfault(&output.status) {
                assert!(
                    stdout.trim().ends_with(CONFIRM) && stderr.is_empty(),
                    "failed to find confirmation in test `{name}`\n{desc}"
                );
            } else {
                panic!("\n\nexpected a segfault on `{name}`\n{desc}\n\n");
            }
        }
    }
}

#[cfg(unix)]
fn is_segfault(status: &ExitStatus) -> bool {
    use std::os::unix::prelude::*;

    match status.signal() {
        Some(libc::SIGSEGV) => true,
        _ => false,
    }
}

#[cfg(unix)]
fn is_stack_overflow(status: &ExitStatus, stderr: &str) -> bool {
    use std::os::unix::prelude::*;

    // The exit status should always be SIGABRT, not SIGSEGV. Something, be it
    // the standard library or Wasmtime, should catch the original SIGSEGV or
    // SIGBUS and abort instead.
    match status.signal() {
        Some(libc::SIGABRT) => {}
        _ => return false,
    }

    // A helpful message should additionally be printed at all times.
    stderr.contains("has overflowed its stack")
}

#[cfg(windows)]
fn is_segfault(status: &ExitStatus) -> bool {
    match status.code().map(|s| s as u32) {
        Some(0xc0000005) => true,
        _ => false,
    }
}

#[cfg(windows)]
fn is_stack_overflow(status: &ExitStatus, _stderr: &str) -> bool {
    match status.code().map(|s| s as u32) {
        Some(0xc00000fd) => true,
        _ => false,
    }
}

fn overrun_with_big_module(approx_stack: usize) {
    // Each call to `$get` produces ten 8-byte values which need to be saved
    // onto the stack, so divide `approx_stack` by 80 to get
    // a rough number of calls to consume `approx_stack` stack.
    let n = approx_stack / 10 / 8;

    let mut s = String::new();
    s.push_str("(module\n");
    s.push_str("(func $big_stack\n");
    for _ in 0..n {
        s.push_str("call $get\n");
    }
    for _ in 0..n {
        s.push_str("call $take\n");
    }
    s.push_str(")\n");
    s.push_str("(func $get (result i64 i64 i64 i64 i64 i64 i64 i64 i64 i64) call $big_stack unreachable)\n");
    s.push_str("(func $take (param i64 i64 i64 i64 i64 i64 i64 i64 i64 i64) unreachable)\n");
    s.push_str("(func (export \"\") call $big_stack)\n");
    s.push_str(")\n");

    // Give 100MB of stack to wasm, representing a misconfigured host. Run the
    // actual module on a 2MB stack in a child thread to guarantee that the
    // module here will overrun the stack. This should deterministically hit the
    // guard page.
    let mut config = Config::default();
    config.max_wasm_stack(100 << 20).async_stack_size(100 << 20);
    let engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, &s).unwrap();
    let mut store = Store::new(&engine, ());
    let i = Instance::new(&mut store, &module, &[]).unwrap();
    let f = i.get_typed_func::<(), ()>(&mut store, "").unwrap();
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(move || {
            println!("{CONFIRM}");
            f.call(&mut store, ()).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
    unreachable!();
}
