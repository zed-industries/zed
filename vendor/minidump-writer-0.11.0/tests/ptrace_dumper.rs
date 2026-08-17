//! All of these tests are specific to ptrace
#![cfg(any(target_os = "linux", target_os = "android"))]

use {
    common::*,
    error_graph::ErrorList,
    minidump_writer::minidump_writer::MinidumpWriterConfig,
    nix::{
        sys::mman::{mmap, MapFlags, ProtFlags},
        sys::signal::Signal,
    },
    std::{
        convert::TryInto,
        io::{BufRead, BufReader},
        mem::size_of,
        os::unix::process::ExitStatusExt,
    },
};

mod common;

/// These tests generally aren't consistent in resource-deprived environments like CI runners and
/// android emulators.
macro_rules! disabled_on_ci_and_android {
    () => {
        if std::env::var("CI").is_ok() || cfg!(target_os = "android") {
            println!("disabled on CI and android, but otherwise works locally");
            return;
        }
    };
}

macro_rules! assert_no_soft_errors(($n: ident, $e: expr) => {{
    let mut $n = ErrorList::default();
    let __result = $e;
    assert!($n.is_empty(), "{:?}", $n);
    __result
}});

#[test]
fn test_setup() {
    spawn_child("setup", &[]);
}

#[test]
fn test_thread_list_from_child() {
    // Child spawns and looks in the parent (== this process) for its own thread-ID

    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    // // We also spawn another thread that we send a SIGHUP to to ensure that the
    // // ptracedumper correctly handles it
    let _thread = std::thread::Builder::new()
        .name("sighup-thread".into())
        .spawn(move || {
            tx.send(unsafe { libc::pthread_self() as usize }).unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        })
        .unwrap();

    let thread_id = rx.recv().unwrap();

    // Unfortunately we need to set a signal handler to ignore the SIGHUP we send
    // to the thread, as otherwise the default test harness fails
    unsafe {
        let mut act: libc::sigaction = std::mem::zeroed();
        if libc::sigemptyset(&mut act.sa_mask) != 0 {
            eprintln!(
                "unable to clear action mask: {:?}",
                std::io::Error::last_os_error()
            );
            return;
        }

        unsafe extern "C" fn on_sig(
            _sig: libc::c_int,
            _info: *mut libc::siginfo_t,
            _uc: *mut libc::c_void,
        ) {
        }

        act.sa_flags = libc::SA_SIGINFO;
        act.sa_sigaction = on_sig as usize;

        // Register the action with the signal handler
        if libc::sigaction(libc::SIGHUP, &act, std::ptr::null_mut()) != 0 {
            eprintln!(
                "unable to register signal handler: {:?}",
                std::io::Error::last_os_error()
            );
        }
    }

    unsafe {
        libc::pthread_kill(thread_id as _, libc::SIGHUP);
    }

    spawn_child("thread_list", &[]);
}

#[test]
fn test_thread_list_from_parent() {
    let num_of_threads = 5;
    let mut child = start_child_and_wait_for_threads(num_of_threads);
    let pid = child.id() as i32;

    let dumper = assert_no_soft_errors!(
        soft_errors,
        MinidumpWriterConfig::new(pid, pid).build_for_testing(&mut soft_errors)
    )
    .expect("Couldn't init dumper");

    assert_eq!(dumper.threads.len(), num_of_threads);

    // let mut matching_threads = 0;
    for (idx, curr_thread) in dumper.threads.iter().enumerate() {
        println!("curr_thread: {curr_thread:?}");
        let info = dumper
            .get_thread_info_by_index(idx)
            .expect("Could not get thread info by index");
        let (_valid_stack_ptr, stack_len) = dumper
            .get_stack_info(info.stack_pointer)
            .expect("Could not get stack_pointer");

        assert!(stack_len > 0);

        // TODO: I currently know of no way to write the thread_id into the registers using Rust,
        //       so this check is deactivated for now, because it always fails
        /*
        // In the helper program, we stored a pointer to the thread id in a
        // specific register. Check that we can recover its value.
        #[cfg(target_arch = "x86_64")]
        let process_tid_location = info.regs.rcx;
        #[cfg(target_arch = "x86")]
        let process_tid_location = info.regs.ecx;
        #[cfg(target_arch = "arm")]
        let process_tid_location = info.regs.uregs[3];
        #[cfg(target_arch = "aarch64")]
        let process_tid_location = info.regs.regs[3];
        #[cfg(target_arch = "mips")]
        let process_tid_location = info.mcontext.gregs[1];

        let thread_id_data = PtraceDumper::copy_from_process(
            *curr_thread,
            process_tid_location as *mut libc::c_void,
            4,
        )
        .expect("Could not copy from process");
        let found_thread_id = i32::from_ne_bytes(
            thread_id_data
                .as_slice()
                .try_into()
                .expect("couldn't parse i32 from read data"),
        );
        matching_threads += if *curr_thread == found_thread_id {
            1
        } else {
            0
        }; */
    }
    drop(dumper);
    child.kill().expect("Failed to kill process");

    // Reap child
    let waitres = child.wait().expect("Failed to wait for child");
    let status = waitres.signal().expect("Child did not die due to signal");
    assert_eq!(waitres.code(), None);
    assert_eq!(status, Signal::SIGKILL as i32);

    // We clean up the child process before checking the final result
    // TODO: I currently know of no way to write the thread_id into the registers using Rust,
    //       so this check is deactivated for now, because it always fails
    // assert_eq!(matching_threads, num_of_threads);
}

// #[cfg(not(any(target_arch = "mips", target_arch = "arm-eabi"))]
#[cfg(not(target_arch = "mips"))]
#[test]
// Ensure that the linux-gate VDSO is included in the mapping list.
fn test_mappings_include_linux_gate() {
    spawn_child("mappings_include_linux_gate", &[]);
}

#[test]
fn test_linux_gate_mapping_id() {
    disabled_on_ci_and_android!();
    spawn_child("linux_gate_mapping_id", &[]);
}

#[test]
fn test_merged_mappings() {
    let page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE).unwrap();
    let page_size = std::num::NonZeroUsize::new(page_size.unwrap() as usize).unwrap();
    let map_size = std::num::NonZeroUsize::new(3 * page_size.get()).unwrap();

    let path: String = if let Ok(p) = std::env::var("TEST_HELPER") {
        p
    } else {
        std::env!("CARGO_BIN_EXE_test").into()
    };
    let file = std::fs::File::open(&path).unwrap();

    // mmap two segments out of the helper binary, one
    // enclosed in the other, but with different protections.
    let mapped_mem = unsafe {
        mmap(
            None,
            map_size,
            ProtFlags::PROT_READ,
            MapFlags::MAP_SHARED,
            &file,
            0,
        )
        .unwrap()
    };

    let mapped = mapped_mem.as_ptr() as usize;

    // Carve a page out of the first mapping with different permissions.
    let _inside_mapping = unsafe {
        mmap(
            std::num::NonZeroUsize::new(mapped + 2 * page_size.get()),
            page_size,
            ProtFlags::PROT_NONE,
            MapFlags::MAP_SHARED | MapFlags::MAP_FIXED,
            &file,
            // Map a different offset just to
            // better test real-world conditions.
            page_size.get().try_into().unwrap(), // try_into() in order to work for 32 and 64 bit
        )
    };

    spawn_child(
        "merged_mappings",
        &[&path, &format!("{mapped}"), &format!("{map_size}")],
    );
}

#[test]
// Ensure that the linux-gate VDSO is included in the mapping list.
fn test_file_id() {
    disabled_on_ci_and_android!();
    spawn_child("file_id", &[]);
}

#[test]
fn test_find_mapping() {
    spawn_child(
        "find_mappings",
        &[
            &format!("{}", libc::printf as *const () as usize),
            &format!("{}", String::new as *const () as usize),
        ],
    );
}

#[test]
fn test_copy_from_process_self() {
    disabled_on_ci_and_android!();

    let stack_var: libc::c_long = 0x11223344;
    let heap_var: Box<libc::c_long> = Box::new(0x55667788);
    spawn_child(
        "copy_from_process",
        &[
            &format!("{}", &stack_var as *const libc::c_long as usize),
            &format!("{}", heap_var.as_ref() as *const libc::c_long as usize),
        ],
    );
}

#[test]
fn test_sanitize_stack_copy() {
    let num_of_threads = 1;
    let mut child = start_child_and_return(&["spawn_alloc_wait"]);
    let pid = child.id() as i32;

    let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
    let mut buf = String::new();
    let _ = f
        .read_line(&mut buf)
        .expect("Couldn't read address provided by child");
    let mut output = buf.split_whitespace();
    let heap_addr = usize::from_str_radix(output.next().unwrap().trim_start_matches("0x"), 16)
        .expect("unable to parse mmap_addr");

    let dumper = assert_no_soft_errors!(
        soft_errors,
        MinidumpWriterConfig::new(pid, pid).build_for_testing(&mut soft_errors)
    )
    .expect("Couldn't init dumper");
    assert_eq!(dumper.threads.len(), num_of_threads);

    let thread_info = dumper
        .get_thread_info_by_index(0)
        .expect("Couldn't find thread_info");

    let defaced;
    #[cfg(target_pointer_width = "64")]
    {
        defaced = 0x0defaced0defacedusize.to_ne_bytes()
    }
    #[cfg(target_pointer_width = "32")]
    {
        defaced = 0x0defacedusize.to_ne_bytes()
    };

    let mut simulated_stack = vec![0xffu8; 2 * size_of::<usize>()];
    // Pointers into the stack shouldn't be sanitized.
    simulated_stack[size_of::<usize>()..].copy_from_slice(&thread_info.stack_pointer.to_ne_bytes());

    dumper
        .sanitize_stack_copy(
            &mut simulated_stack,
            thread_info.stack_pointer,
            size_of::<usize>(),
        )
        .expect("Could not sanitize stack");

    assert!(simulated_stack[size_of::<usize>()..] != defaced);
    // Memory prior to the stack pointer should be cleared.
    assert_eq!(
        &simulated_stack[0..size_of::<usize>()],
        vec![0u8; size_of::<usize>()].as_slice()
    );

    // Small integers should not be sanitized.
    for ii in -4096..=4096isize {
        simulated_stack = vec![0u8; 2 * size_of::<usize>()];
        simulated_stack[0..size_of::<usize>()].copy_from_slice(&(ii as usize).to_ne_bytes());
        dumper
            .sanitize_stack_copy(&mut simulated_stack, thread_info.stack_pointer, 0)
            .expect("Failed to sanitize with small integers");
        assert!(simulated_stack[size_of::<usize>()..] != defaced);
    }

    // The instruction pointer definitely should point into an executable mapping.
    let instr_ptr = thread_info.get_instruction_pointer();
    let mapping_info = dumper
        .find_mapping_no_bias(instr_ptr)
        .expect("Failed to find mapping info");
    assert!(mapping_info.is_executable());

    // Pointers to code shouldn't be sanitized.
    simulated_stack = vec![0u8; 2 * size_of::<usize>()];
    simulated_stack[size_of::<usize>()..].copy_from_slice(&instr_ptr.to_ne_bytes());
    dumper
        .sanitize_stack_copy(&mut simulated_stack, thread_info.stack_pointer, 0)
        .expect("Failed to sanitize with instr_ptr");
    assert!(simulated_stack[0..size_of::<usize>()] != defaced);
    assert!(simulated_stack[size_of::<usize>()..] != defaced);

    // String fragments should be sanitized.
    let junk = "abcdefghijklmnop".as_bytes();
    simulated_stack.copy_from_slice(&junk[0..2 * size_of::<usize>()]);
    dumper
        .sanitize_stack_copy(&mut simulated_stack, thread_info.stack_pointer, 0)
        .expect("Failed to sanitize with junk");
    assert_eq!(simulated_stack[0..size_of::<usize>()], defaced);
    assert_eq!(simulated_stack[size_of::<usize>()..], defaced);

    // Heap pointers should be sanititzed.

    // NOTE: The original test used the heap-address below, but here thread_info.regs.rcx
    //       is the instruction pointer, and thus in direct conflict with the "instruction pointer"
    //       testcase.
    //       Instead we just allocate something on the heap in the child and pass that address to this test.
    // #[cfg(target_arch = "x86_64")]
    // let heap_addr = thread_info.regs.rcx as usize;
    // #[cfg(target_arch = "x86")]
    // let heap_addr = thread_info.regs.ecx as usize;
    // #[cfg(target_arch = "arm")]
    // let heap_addr = thread_info.regs.uregs[3] as usize;
    // #[cfg(target_arch = "aarch64")]
    // let heap_addr = thread_info.regs.regs[3] as usize;
    // #[cfg(target_arch = "mips")]
    // let heap_addr = thread_info.mcontext.gregs[1] as usize;

    simulated_stack = vec![0u8; 2 * size_of::<usize>()];

    simulated_stack[0..size_of::<usize>()].copy_from_slice(&heap_addr.to_ne_bytes());
    dumper
        .sanitize_stack_copy(&mut simulated_stack, thread_info.stack_pointer, 0)
        .expect("Failed to sanitize with heap addr");

    assert_eq!(simulated_stack[0..size_of::<usize>()], defaced);

    drop(dumper);

    child.kill().expect("Failed to kill process");

    // Reap child
    let waitres = child.wait().expect("Failed to wait for child");
    let status = waitres.signal().expect("Child did not die due to signal");
    assert_eq!(waitres.code(), None);
    assert_eq!(status, Signal::SIGKILL as i32);
}
