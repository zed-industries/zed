#![cfg(all(target_os = "windows", target_arch = "x86_64"))]

use {
    common::start_child_and_return,
    minidump::{
        CrashReason, Minidump, MinidumpBreakpadInfo, MinidumpMemoryList, MinidumpSystemInfo,
        MinidumpThreadList,
    },
    minidump_writer::minidump_writer::MinidumpWriter,
};

mod common;

const EXCEPTION_ILLEGAL_INSTRUCTION: i32 = -1073741795;
const STATUS_INVALID_PARAMETER: i32 = -1073741811;
#[link(name = "kernel32")]
extern "system" {
    fn GetCurrentThreadId() -> u32;
}

fn get_crash_reason<'a, T: std::ops::Deref<Target = [u8]> + 'a>(
    md: &Minidump<'a, T>,
) -> CrashReason {
    let exc: minidump::MinidumpException<'_> =
        md.get_stream().expect("unable to find exception stream");

    exc.get_crash_reason(
        minidump::system_info::Os::Windows,
        minidump::system_info::Cpu::X86_64,
    )
}

/// Ensures that we can write minidumps for the current process, even if this is
/// not necessarily the primary intended use case of out-of-process dumping
#[test]
fn dump_current_process() {
    let mut tmpfile = tempfile::Builder::new()
        .prefix("windows_current_process")
        .tempfile()
        .unwrap();

    MinidumpWriter::dump_local_context(
        Some(STATUS_INVALID_PARAMETER),
        None,
        None,
        tmpfile.as_file_mut(),
    )
    .expect("failed to write minidump");

    let md = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");

    let _: MinidumpThreadList = md.get_stream().expect("Couldn't find MinidumpThreadList");
    let _: MinidumpMemoryList = md.get_stream().expect("Couldn't find MinidumpMemoryList");
    let _: MinidumpSystemInfo = md.get_stream().expect("Couldn't find MinidumpSystemInfo");

    let crash_reason = get_crash_reason(&md);

    assert_eq!(
        crash_reason,
        CrashReason::from_windows_error(STATUS_INVALID_PARAMETER as u32)
    );

    // SAFETY: syscall
    let thread_id = unsafe { GetCurrentThreadId() };

    let bp_info: MinidumpBreakpadInfo =
        md.get_stream().expect("Couldn't find MinidumpBreakpadInfo");

    assert_eq!(bp_info.dump_thread_id.unwrap(), thread_id);
    assert_eq!(bp_info.requesting_thread_id.unwrap(), thread_id);
}

#[test]
fn dump_specific_thread() {
    let mut tmpfile = tempfile::Builder::new()
        .prefix("windows_current_process")
        .tempfile()
        .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    let jh = std::thread::spawn(move || {
        // SAFETY: syscall
        let thread_id = unsafe { GetCurrentThreadId() };
        while tx.send(thread_id).is_ok() {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let crashing_thread_id = rx.recv().unwrap();

    MinidumpWriter::dump_local_context(
        Some(STATUS_INVALID_PARAMETER),
        Some(crashing_thread_id),
        None,
        tmpfile.as_file_mut(),
    )
    .expect("failed to write minidump");

    drop(rx);
    jh.join().unwrap();

    let md = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");

    let _: MinidumpThreadList = md.get_stream().expect("Couldn't find MinidumpThreadList");
    let _: MinidumpMemoryList = md.get_stream().expect("Couldn't find MinidumpMemoryList");
    let _: MinidumpSystemInfo = md.get_stream().expect("Couldn't find MinidumpSystemInfo");

    let crash_reason = get_crash_reason(&md);

    assert_eq!(
        crash_reason,
        CrashReason::from_windows_error(STATUS_INVALID_PARAMETER as u32)
    );

    // SAFETY: syscall
    let requesting_thread_id = unsafe { GetCurrentThreadId() };

    let bp_info: MinidumpBreakpadInfo =
        md.get_stream().expect("Couldn't find MinidumpBreakpadInfo");

    assert_eq!(bp_info.dump_thread_id.unwrap(), crashing_thread_id);
    assert_eq!(bp_info.requesting_thread_id.unwrap(), requesting_thread_id);
}

/// Ensures that we can write minidumps for an external process. Unfortunately
/// this requires us to know the actual pointer in the client process for the
/// exception, as the `MiniDumpWriteDump` syscall directly reads points from
/// the process memory, so we communicate that back from the client process
/// via stdout
#[test]
fn dump_external_process() {
    use std::io::BufRead;

    let mut child = start_child_and_return(&[&format!("{:x}", EXCEPTION_ILLEGAL_INSTRUCTION)]);

    let (process_id, exception_pointers, thread_id, exception_code) = {
        let mut f = std::io::BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
        let mut buf = String::new();
        f.read_line(&mut buf).expect("failed to read stdout");
        assert!(!buf.is_empty());

        let mut biter = buf.trim().split(' ');

        let process_id: u32 = biter.next().unwrap().parse().unwrap();
        let exception_pointers: usize = biter.next().unwrap().parse().unwrap();
        let thread_id: u32 = biter.next().unwrap().parse().unwrap();
        let exception_code = u32::from_str_radix(biter.next().unwrap(), 16).unwrap();

        (process_id, exception_pointers, thread_id, exception_code)
    };

    let exception_code = exception_code as i32;
    assert_eq!(exception_code, EXCEPTION_ILLEGAL_INSTRUCTION);

    let crash_context = crash_context::CrashContext {
        exception_pointers: exception_pointers as _,
        process_id,
        thread_id,
        exception_code,
    };

    let mut tmpfile = tempfile::Builder::new()
        .prefix("windows_external_process")
        .tempfile()
        .unwrap();

    // SAFETY: We keep the process we are dumping alive until the minidump is written
    // and the test process keep the pointers it sent us alive until it is killed
    MinidumpWriter::dump_crash_context(&crash_context, None, tmpfile.as_file_mut())
        .expect("failed to write minidump");

    child.kill().expect("failed to kill child");

    let md = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");

    let _: MinidumpThreadList = md.get_stream().expect("Couldn't find MinidumpThreadList");
    let _: MinidumpMemoryList = md.get_stream().expect("Couldn't find MinidumpMemoryList");
    let _: MinidumpSystemInfo = md.get_stream().expect("Couldn't find MinidumpSystemInfo");

    let crash_reason = get_crash_reason(&md);

    assert_eq!(
        crash_reason,
        CrashReason::from_windows_code(EXCEPTION_ILLEGAL_INSTRUCTION as u32)
    );
}
