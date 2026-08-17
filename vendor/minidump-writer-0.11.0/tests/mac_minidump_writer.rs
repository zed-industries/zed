#![cfg(target_os = "macos")]

use {
    common::start_child_and_return,
    minidump::{
        CrashReason, Minidump, MinidumpBreakpadInfo, MinidumpMemoryList, MinidumpMiscInfo,
        MinidumpModuleList, MinidumpSystemInfo, MinidumpThreadList,
    },
    minidump_writer::minidump_writer::MinidumpWriter,
};

mod common;

fn get_crash_reason<'a, T: std::ops::Deref<Target = [u8]> + 'a>(
    md: &Minidump<'a, T>,
) -> CrashReason {
    let exc: minidump::MinidumpException<'_> =
        md.get_stream().expect("unable to find exception stream");

    exc.get_crash_reason(
        minidump::system_info::Os::MacOs,
        if cfg!(target_arch = "x86_64") {
            minidump::system_info::Cpu::X86_64
        } else if cfg!(target_arch = "aarch64") {
            minidump::system_info::Cpu::Arm64
        } else {
            unimplemented!()
        },
    )
}

struct Captured<'md> {
    #[allow(dead_code)]
    task: u32,
    #[allow(dead_code)]
    thread: u32,
    minidump: Minidump<'md, memmap2::Mmap>,
}

fn capture_minidump(name: &str, exception_kind: u32) -> Captured<'_> {
    // Create a mach port server to retrieve the crash details from the child
    let mut server = crash_context::ipc::Server::create(&std::ffi::CString::new(name).unwrap())
        .expect("failed to create mach port service");

    let mut child = start_child_and_return(&[name, &exception_kind.to_string()]);

    // Wait for the child to spinup and report a crash context to us
    let mut rcc = server
        .try_recv_crash_context(Some(std::time::Duration::from_secs(5)))
        .expect("failed to receive context")
        .expect("receive timed out");

    let mut tmpfile = tempfile::Builder::new().prefix(name).tempfile().unwrap();

    let task = rcc.crash_context.task;
    let thread = rcc.crash_context.thread;

    let mut dumper = MinidumpWriter::with_crash_context(rcc.crash_context);

    dumper
        .dump(tmpfile.as_file_mut())
        .expect("failed to write minidump");

    // Signal the child that we've received and processed the crash context
    rcc.acker
        .send_ack(1, Some(std::time::Duration::from_secs(2)))
        .expect("failed to send ack");

    child.kill().expect("failed to kill child");

    let minidump = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");

    Captured {
        task,
        thread,
        minidump,
    }
}

#[test]
fn dump_external_process() {
    if std::env::var("CI").is_ok() {
        println!("test disabled, consistently times out because of potato runners");
        return;
    }

    let approximate_proc_start_time = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let md = capture_minidump(
        "dump_external_process",
        mach2::exception_types::EXC_BREAKPOINT,
    )
    .minidump;

    let crash_reason = get_crash_reason(&md);

    assert!(matches!(
        crash_reason,
        CrashReason::MacGeneral(minidump_common::errors::ExceptionCodeMac::EXC_BREAKPOINT, _)
    ));

    let _: MinidumpModuleList = md.get_stream().expect("Couldn't find MinidumpModuleList");
    let _: MinidumpThreadList = md.get_stream().expect("Couldn't find MinidumpThreadList");
    let _: MinidumpMemoryList = md.get_stream().expect("Couldn't find MinidumpMemoryList");
    let _: MinidumpSystemInfo = md.get_stream().expect("Couldn't find MinidumpSystemInfo");
    let _: MinidumpBreakpadInfo = md.get_stream().expect("Couldn't find MinidumpBreakpadInfo");

    let misc_info: MinidumpMiscInfo = md.get_stream().expect("Couldn't find MinidumpMiscInfo");

    if let minidump::RawMiscInfo::MiscInfo2(mi) = &misc_info.raw {
        // Unfortunately the minidump format only has 32-bit precision for the
        // process start time
        let process_create_time = mi.process_create_time as u64;

        assert!(
            process_create_time >= approximate_proc_start_time
                && process_create_time <= approximate_proc_start_time + 2
        );

        // I've tried busy looping to spend CPU time to get this up, but
        // MACH_TASK_BASIC_INFO which should give terminated thread times only ever
        // reports 0, and TASK_THREAD_TIMES_INFO which should show active thread
        // times I've only been able to get upt to a few thousand microseconds
        // even when busy looping for well over a second, and those get truncated
        // to whole seconds. And it seems that crashpad doesn't have tests around
        // this, though that's hard to say given how tedious it is finding stuff
        // in that bloated codebase
        // assert!(mi.process_user_time > 0);
        // assert!(mi.process_kernel_time > 0);

        // These aren't currently available on aarch64, or if they are, they
        // are not via the same sysctlbyname mechanism. Would be nice if Apple
        // documented...anything
        if cfg!(target_arch = "x86_64") {
            assert!(mi.processor_max_mhz > 0);
            assert!(mi.processor_current_mhz > 0);
        }
    } else {
        panic!("unexpected misc info type {:?}", misc_info);
    }
}

// /// Validates we can actually walk the stack for each thread in the minidump,
// /// this is using minidump-processor, which (currently) depends on breakpad
// /// symbols, however https://github.com/mozilla/dump_syms is not available as
// /// a library https://github.com/mozilla/dump_syms/issues/253, so we just require
// /// that it already be installed, hence the ignore
// #[test]
// fn stackwalks() {
//     if std::env::var("CI").is_ok() {
//         println!("test disabled, consistently times out because of potato runners");
//         return;
//     }

//     println!("generating minidump...");
//     let md = capture_minidump("stackwalks", mach2::exception_types::EXC_BREAKPOINT);

//     // Generate the breakpad symbols
//     println!("generating symbols...");
//     dump_syms::dumper::single_file(
//         &dump_syms::dumper::Config {
//             output: dump_syms::dumper::Output::Store(".test-symbols".into()),
//             symbol_server: None,
//             debug_id: None,
//             code_id: None,
//             arch: if cfg!(target_arch = "aarch64") {
//                 "arm64"
//             } else if cfg!(target_arch = "x86_64") {
//                 "x86_64"
//             } else {
//                 panic!("invalid MacOS target architecture")
//             },
//             num_jobs: 2, // default this
//             check_cfi: false,
//             emit_inlines: false,
//             mapping_var: None,
//             mapping_src: None,
//             mapping_dest: None,
//             mapping_file: None,
//         },
//         "target/debug/test",
//     )
//     .expect("failed to dump symbols");

//     let provider = minidump_unwind::Symbolizer::new(minidump_unwind::simple_symbol_supplier(vec![
//         ".test-symbols".into(),
//     ]));

//     let state = futures::executor::block_on(async {
//         minidump_processor::process_minidump(&md.minidump, &provider).await
//     })
//     .unwrap();

//     //state.print(&mut std::io::stdout()).map_err(|_| ()).unwrap();

//     // We expect at least 2 threads, one of which is the fake crashing thread
//     let fake_crash_thread = state
//         .threads
//         .iter()
//         .find(|cs| cs.thread_id == md.thread)
//         .expect("failed to find crash thread");

//     assert_eq!(
//         fake_crash_thread.thread_name.as_deref(),
//         Some("test-thread")
//     );

//     assert!(
//         fake_crash_thread.frames.iter().any(|sf| {
//             sf.function_name
//                 .as_ref()
//                 .map_or(false, |fname| fname.ends_with("wait_until_killed"))
//         }),
//         "unable to locate expected function"
//     );

//     let mod_list: MinidumpModuleList = md
//         .minidump
//         .get_stream()
//         .expect("Couldn't find MinidumpModuleList");

//     // Ensure we found dyld
//     assert!(mod_list
//         .iter()
//         .any(|module| &module.name == "/usr/lib/dyld"));
// }
