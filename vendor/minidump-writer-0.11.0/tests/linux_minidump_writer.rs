#![cfg(any(target_os = "linux", target_os = "android"))]
#![allow(unused_imports, unused_variables)]

use {
    common::*,
    minidump::*,
    minidump_common::format::{GUID, MINIDUMP_STREAM_TYPE::*},
    minidump_writer::{
        app_memory::AppMemory,
        crash_context::CrashContext,
        maps_reader::{MappingEntry, MappingInfo, SystemMappingInfo},
        minidump_writer::{errors::WriterError, MinidumpWriter, MinidumpWriterConfig},
        module_reader::{BuildId, ReadFromModule},
        Pid,
    },
    nix::{errno::Errno, sys::signal::Signal},
    procfs_core::process::MMPermissions,
    serde_json::json,
    std::{
        collections::HashSet,
        io::{BufRead, BufReader},
        os::unix::process::ExitStatusExt,
        process::{Command, Stdio},
    },
};

mod common;

#[derive(Debug, PartialEq)]
enum Context {
    With,
    Without,
}

impl Context {
    pub fn minidump_writer(&self, pid: Pid) -> MinidumpWriterConfig {
        let mut mw = MinidumpWriterConfig::new(pid, pid);
        #[cfg(not(target_arch = "mips"))]
        if self == &Context::With {
            let crash_context = get_crash_context(pid);
            mw.set_crash_context(crash_context);
        }
        mw
    }
}

#[cfg(not(target_arch = "mips"))]
fn get_ucontext() -> Result<crash_context::ucontext_t> {
    let mut context = std::mem::MaybeUninit::uninit();
    unsafe {
        let res = crash_context::crash_context_getcontext(context.as_mut_ptr());
        Errno::result(res)?;

        Ok(context.assume_init())
    }
}

#[cfg(not(target_arch = "mips"))]
fn get_crash_context(tid: Pid) -> CrashContext {
    let siginfo: libc::signalfd_siginfo = unsafe { std::mem::zeroed() };
    let context = get_ucontext().expect("Failed to get ucontext");
    #[cfg(not(target_arch = "arm"))]
    let float_state = unsafe { std::mem::zeroed() };
    CrashContext {
        inner: crash_context::CrashContext {
            siginfo,
            pid: std::process::id() as _,
            tid,
            context,
            #[cfg(not(target_arch = "arm"))]
            float_state,
        },
    }
}

macro_rules! contextual_test {
    ( $(#[$attr:meta])? fn $name:ident ($ctx:ident : Context) $body:block ) => {
        mod $name {
            use super::*;

            fn test($ctx: Context) $body

            #[test]
            $(#[$attr])?
            fn without_context() {
                test(Context::Without)
            }

            #[cfg(not(target_arch = "mips"))]
            #[test]
            $(#[$attr])?
            fn with_context() {
                test(Context::With)
            }
        }
    }
}

contextual_test! {
    fn write_dump(context: Context) {
        let num_of_threads = 3;
        let mut child = start_child_and_wait_for_threads(num_of_threads);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("write_dump")
            .tempfile()
            .unwrap();

        let tmp = context.minidump_writer(pid);
        let in_memory_buffer = tmp.write(&mut tmpfile).expect("Could not write minidump");
        child.kill().expect("Failed to kill process");

        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        let meta = std::fs::metadata(tmpfile.path()).expect("Couldn't get metadata for tempfile");
        assert!(meta.len() > 0);

        let mem_slice = std::fs::read(tmpfile.path()).expect("Failed to minidump");
        assert_eq!(mem_slice.len(), in_memory_buffer.len());
        assert_eq!(mem_slice, in_memory_buffer);
    }
}

contextual_test! {
    #[ignore]
    fn write_and_read_dump_from_parent(context: Context) {
        let mut child = start_child_and_return(&["spawn_mmap_wait"]);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("write_and_read_dump")
            .tempfile()
            .unwrap();

        let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
        let mut buf = String::new();
        let _ = f
            .read_line(&mut buf)
            .expect("Couldn't read address provided by child");
        let mut output = buf.split_whitespace();
        let mmap_addr = output
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse mmap_addr");
        let memory_size = output
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse memory_size");
        // Add information about the mapped memory.
        let mapping = MappingInfo {
            start_address: mmap_addr,
            size: memory_size,
            offset: 0,
            permissions: MMPermissions::READ | MMPermissions::WRITE,
            name: Some("a fake mapping".into()),
            system_mapping_info: SystemMappingInfo {
                start_address: mmap_addr,
                end_address: mmap_addr + memory_size,
            },
        };

        let identifier = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF,
        ];
        let entry = MappingEntry {
            mapping,
            identifier,
        };

        let mut tmp = context.minidump_writer(pid);

        tmp.set_user_mapping_list(vec![entry]);
        tmp
            .write(&mut tmpfile)
            .expect("Could not write minidump");

        child.kill().expect("Failed to kill process");
        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        let module_list: MinidumpModuleList = dump
            .get_stream()
            .expect("Couldn't find stream MinidumpModuleList");
        let module = module_list
            .module_at_address(mmap_addr as u64)
            .expect("Couldn't find user mapping module");
        assert_eq!(module.base_address(), mmap_addr as u64);
        assert_eq!(module.size(), memory_size as u64);
        assert_eq!(module.code_file(), "a fake mapping");
        assert_eq!(
            module.debug_identifier(),
            Some("33221100554477668899AABBCCDDEEFF0".parse().unwrap())
        );

        let _: MinidumpException = dump.get_stream().expect("Couldn't find MinidumpException");
        let _: MinidumpThreadList = dump.get_stream().expect("Couldn't find MinidumpThreadList");
        let _: MinidumpMemoryList = dump.get_stream().expect("Couldn't find MinidumpMemoryList");
        let _: MinidumpSystemInfo = dump.get_stream().expect("Couldn't find MinidumpSystemInfo");
        let _ = dump
            .get_raw_stream(LinuxCpuInfo as u32)
            .expect("Couldn't find LinuxCpuInfo");
        let _ = dump
            .get_raw_stream(LinuxProcStatus as u32)
            .expect("Couldn't find LinuxProcStatus");
        let _ = dump
            .get_raw_stream(LinuxCmdLine as u32)
            .expect("Couldn't find LinuxCmdLine");
        let _ = dump
            .get_raw_stream(LinuxEnviron as u32)
            .expect("Couldn't find LinuxEnviron");
        let _ = dump
            .get_raw_stream(LinuxAuxv as u32)
            .expect("Couldn't find LinuxAuxv");
        let _ = dump
            .get_raw_stream(LinuxMaps as u32)
            .expect("Couldn't find LinuxMaps");
        let _ = dump
            .get_raw_stream(LinuxDsoDebug as u32)
            .expect("Couldn't find LinuxDsoDebug");
        let _ = dump
            .get_raw_stream(MozLinuxLimits as u32)
            .expect("Couldn't find MozLinuxLimits");
    }
}

contextual_test! {
    fn write_with_additional_memory(context: Context) {
        let mut child = start_child_and_return(&["spawn_alloc_wait"]);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("additional_memory")
            .tempfile()
            .unwrap();

        let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
        let mut buf = String::new();
        let _ = f
            .read_line(&mut buf)
            .expect("Couldn't read address provided by child");
        let mut output = buf.split_whitespace();
        let memory_addr = usize::from_str_radix(output.next().unwrap().trim_start_matches("0x"), 16)
            .expect("unable to parse mmap_addr");
        let memory_size = output
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse memory_size");

        let app_memory = AppMemory {
            ptr: memory_addr,
            length: memory_size,
        };

        let mut tmp = context.minidump_writer(pid);

        tmp.set_app_memory(vec![app_memory]);
        tmp
            .write(&mut tmpfile)
            .expect("Could not write minidump");

        child.kill().expect("Failed to kill process");
        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Read dump file and check its contents
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");

        let section: MinidumpMemoryList = dump.get_stream().expect("Couldn't find MinidumpMemoryList");
        let region = section
            .memory_at_address(memory_addr as u64)
            .expect("Couldn't find memory region");

        assert_eq!(region.base_address, memory_addr as u64);
        assert_eq!(region.size, memory_size as u64);

        let mut values = Vec::<u8>::with_capacity(memory_size);
        for idx in 0..memory_size {
            values.push((idx % 255) as u8);
        }

        // Verify memory contents.
        assert_eq!(region.bytes, values);
    }
}

contextual_test! {
    fn skip_if_requested(context: Context) {
        let expected_errors = vec![
            json!({
                "InitErrors": ["PrincipalMappingNotReferenced"]
            }),
        ];

        let num_of_threads = 1;
        let mut child = start_child_and_wait_for_threads(num_of_threads);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("skip_if_requested")
            .tempfile()
            .unwrap();

        let mut tmp = context.minidump_writer(pid);

        let pr_mapping_addr;
        #[cfg(target_pointer_width = "64")]
        {
            pr_mapping_addr = 0x0102030405060708;
        }
        #[cfg(target_pointer_width = "32")]
        {
            pr_mapping_addr = 0x010203040;
        };
        tmp
            .skip_stacks_if_mapping_unreferenced()
            .set_principal_mapping_address(pr_mapping_addr);
        let res = tmp
            .write(&mut tmpfile);
        child.kill().expect("Failed to kill process");

        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Ensure the MozSoftErrors stream contains the expected errors
        let dump = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");
        assert_soft_errors_in_minidump(&dump, &expected_errors);
    }
}

contextual_test! {
    fn sanitized_stacks(context: Context) {
        if context == Context::With {
            // FIXME the context's stack pointer very often doesn't lie in mapped memory, resulting
            // in the stack memory having 0 size (so no slice will match `defaced` in the
            // assertion).
            return;
        }
        let num_of_threads = 1;
        let mut child = start_child_and_wait_for_threads(num_of_threads);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("sanitized_stacks")
            .tempfile()
            .unwrap();

        let mut tmp = context.minidump_writer(pid);
        tmp.sanitize_stack();
        tmp
            .write(&mut tmpfile)
            .expect("Faild to dump minidump");
        child.kill().expect("Failed to kill process");

        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Read dump file and check its contents
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        let dump_array = std::fs::read(tmpfile.path()).expect("Failed to read minidump as vec");
        let thread_list: MinidumpThreadList =
            dump.get_stream().expect("Couldn't find MinidumpThreadList");

        let defaced;
        #[cfg(target_pointer_width = "64")]
        {
            defaced = 0x0defaced0defacedusize.to_ne_bytes();
        }
        #[cfg(target_pointer_width = "32")]
        {
            defaced = 0x0defacedusize.to_ne_bytes()
        };

        for thread in thread_list.threads {
            let mem = thread.raw.stack.memory;
            let start = mem.rva as usize;
            let end = (mem.rva + mem.data_size) as usize;
            let slice = &dump_array.as_slice()[start..end];
            assert!(slice.windows(defaced.len()).any(|window| window == defaced));
        }
    }
}

contextual_test! {
    fn write_early_abort(context: Context) {
        let mut child = start_child_and_return(&["spawn_alloc_wait"]);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("additional_memory")
            .tempfile()
            .unwrap();

        let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
        let mut buf = String::new();
        let _ = f
            .read_line(&mut buf)
            .expect("Couldn't read address provided by child");
        let mut output = buf.split_whitespace();
        // We do not read the actual memory_address, but use NULL, which
        // should create an error during dumping and lead to a truncated minidump.
        let _ = usize::from_str_radix(output.next().unwrap().trim_start_matches("0x"), 16)
            .expect("unable to parse mmap_addr");
        let memory_addr = 0;
        let memory_size = output
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse memory_size");

        let app_memory = AppMemory {
            ptr: memory_addr,
            length: memory_size,
        };

        let mut tmp = context.minidump_writer(pid);
        tmp.set_app_memory(vec![app_memory]);

        // This should fail, because during the dump an error is detected (try_from fails)
        match tmp.write(&mut tmpfile) {
            Err(WriterError::SectionAppMemoryError(_)) => (),
            _ => panic!("Wrong kind of error returned"),
        }

        child.kill().expect("Failed to kill process");
        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Read dump file and check its contents. There should be a truncated minidump available
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        // Should be there
        let _: MinidumpThreadList = dump.get_stream().expect("Couldn't find MinidumpThreadList");
        let _: MinidumpModuleList = dump.get_stream().expect("Couldn't find MinidumpThreadList");

        // Should be missing:
        assert!(dump.get_stream::<MinidumpMemoryList>().is_err());
    }
}

contextual_test! {
    fn named_threads(context: Context) {
        let num_of_threads = 5;
        let mut child = start_child_and_wait_for_named_threads(num_of_threads);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("named_threads")
            .tempfile()
            .unwrap();

        let tmp = context.minidump_writer(pid);
        let _ = tmp.write(&mut tmpfile).expect("Could not write minidump");
        child.kill().expect("Failed to kill process");

        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Read dump file and check its contents. There should be a truncated minidump available
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");

        let threads: MinidumpThreadList = dump.get_stream().expect("Couldn't find MinidumpThreadList");

        let thread_names: MinidumpThreadNames = dump
            .get_stream()
            .expect("Couldn't find MinidumpThreadNames");

        let thread_ids: Vec<_> = threads.threads.iter().map(|t| t.raw.thread_id).collect();
        let names: HashSet<_> = thread_ids
            .iter()
            .map(|id| thread_names.get_name(*id).unwrap_or_default())
            .map(|cow| cow.into_owned())
            .collect();
        let mut expected = HashSet::new();
        expected.insert("test".to_string());
        for id in 1..num_of_threads {
            expected.insert(format!("thread_{id}"));
        }
        assert_eq!(expected, names);
    }
}

contextual_test! {
    fn file_descriptors(context: Context) {
        let num_of_files = 5;
        let mut child = start_child_and_wait_for_create_files(num_of_files);
        let pid = child.id() as i32;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("testfiles")
            .tempfile()
            .unwrap();

        let tmp = context.minidump_writer(pid);
        let _ = tmp.write(&mut tmpfile).expect("Could not write minidump");
        child.kill().expect("Failed to kill process");

        // Reap child
        let waitres = child.wait().expect("Failed to wait for child");
        let status = waitres.signal().expect("Child did not die due to signal");
        assert_eq!(waitres.code(), None);
        assert_eq!(status, Signal::SIGKILL as i32);

        // Read dump file and check its contents. There should be a truncated minidump available
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        let fds: MinidumpHandleDataStream = dump.get_stream().expect("Couldn't find MinidumpHandleDataStream");
        // We check that we create num_of_files plus stdin, stdout and stderr
        for i in 0..3 {
            let descriptor = fds.handles.get(i).expect("Descriptor should be present");
            let fd = *descriptor.raw.handle().expect("Handle should be populated");
            assert_eq!(fd, i as u64);
        }

        let non_std_files = &fds.handles[3..];

        // We need to handle the android case where additional pipes might be opened and
        // interspersed with the test_files (emulator? adb?) so that CI doesn't sporadically fail
        for i in 0..num_of_files {
            if !non_std_files.iter().any(|descriptor| {
                let Some(name) = &descriptor.object_name else { return false; };
                let Some(file_name) = name.rsplit_once('/').map(|(_, fname)| fname) else { return false; };
                if !file_name.starts_with("test_file") {
                    return false;
                }

                file_name.ends_with(&i.to_string())
            }) {
                panic!("unable to locate expected file `test_file{i}` in file handle stream");
            }
        }
    }
}

#[test]
fn minidump_size_limit() {
    let num_of_threads = 40;
    let mut child = start_child_and_wait_for_threads(num_of_threads);
    let pid = child.id() as i32;

    let mut total_normal_stack_size = 0;
    let normal_file_size;
    // First, write a minidump with no size limit.
    {
        let mut tmpfile = tempfile::Builder::new()
            .prefix("write_dump_unlimited")
            .tempfile()
            .unwrap();

        MinidumpWriterConfig::new(pid, pid)
            .write(&mut tmpfile)
            .expect("Could not write minidump");

        let meta = std::fs::metadata(tmpfile.path()).expect("Couldn't get metadata for tempfile");
        assert!(meta.len() > 0);

        normal_file_size = meta.len();

        // Read dump file and check its contents
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        let thread_list: MinidumpThreadList =
            dump.get_stream().expect("Couldn't find MinidumpThreadList");
        for thread in thread_list.threads {
            assert!(thread.raw.thread_id > 0);
            assert!(thread.raw.stack.memory.data_size > 0);
            total_normal_stack_size += thread.raw.stack.memory.data_size;
        }
    }

    // Second, write a minidump with a size limit big enough to not trigger
    // anything.
    {
        // Set size limit arbitrarily 2MiB larger than the normal file size -- such
        // that the limiting code will not kick in.
        let minidump_size_limit = normal_file_size + 2 * 1024 * 1024;

        let mut tmpfile = tempfile::Builder::new()
            .prefix("write_dump_pseudolimited")
            .tempfile()
            .unwrap();

        let mut tmp = MinidumpWriterConfig::new(pid, pid);
        tmp.set_minidump_size_limit(minidump_size_limit);
        tmp.write(&mut tmpfile).expect("Could not write minidump");

        let meta = std::fs::metadata(tmpfile.path()).expect("Couldn't get metadata for tempfile");

        // Make sure limiting wasn't actually triggered.  NOTE: If you fail this,
        // first make sure that "minidump_size_limit" above is indeed set to a
        // large enough value -- the limit-checking code in minidump_writer.rs
        // does just a rough estimate.
        // TODO: Fix this properly
        //assert_eq!(meta.len(), normal_file_size);
        let min = std::cmp::min(meta.len(), normal_file_size);
        let max = std::cmp::max(meta.len(), normal_file_size);

        // Setting a stack limit limits the size of non-main stacks even before
        // the limit is reached. This will cause slight variations in size
        // between a limited and an unlimited minidump.
        assert!(max - min < 1024, "max = {max:} min = {min:}");
    }

    // Third, write a minidump with a size limit small enough to be triggered.
    {
        // Set size limit to some arbitrary amount, such that the limiting code
        // will kick in.  The equation used to set this value was determined by
        // simply reversing the size-limit logic a little bit in order to pick a
        // size we know will trigger it.

        // Copyied from sections/thread_list_stream.rs
        const LIMIT_AVERAGE_THREAD_STACK_LENGTH: u64 = 8 * 1024;
        let mut minidump_size_limit = LIMIT_AVERAGE_THREAD_STACK_LENGTH * 40;

        // If, in reality, each of the threads' stack is *smaller* than
        // kLimitAverageThreadStackLength, the normal file size could very well be
        // smaller than the arbitrary limit that was just set.  In that case,
        // either of these numbers should trigger the size-limiting code, but we
        // might as well pick the smallest.
        if normal_file_size < minidump_size_limit {
            minidump_size_limit = normal_file_size;
        }

        let mut tmpfile = tempfile::Builder::new()
            .prefix("write_dump_limited")
            .tempfile()
            .unwrap();

        let mut tmp = MinidumpWriterConfig::new(pid, pid);
        tmp.set_minidump_size_limit(minidump_size_limit);
        tmp.write(&mut tmpfile).expect("Could not write minidump");

        let meta = std::fs::metadata(tmpfile.path()).expect("Couldn't get metadata for tempfile");
        assert!(meta.len() > 0);
        // Make sure the file size is at least smaller than the original.  If this
        // fails because it's the same size, then the size-limit logic didn't kick
        // in like it was supposed to.
        assert!(meta.len() < normal_file_size);

        let mut total_limit_stack_size = 0;
        // Read dump file and check its contents
        let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
        let thread_list: MinidumpThreadList =
            dump.get_stream().expect("Couldn't find MinidumpThreadList");
        for thread in thread_list.threads {
            assert!(thread.raw.thread_id > 0);
            assert!(thread.raw.stack.memory.data_size > 0);
            total_limit_stack_size += thread.raw.stack.memory.data_size;
        }

        // Make sure stack size shrunk by at least 1KB per extra thread.
        // Note: The 1KB is arbitrary, and assumes that the thread stacks are big
        // enough to shrink by that much.  For example, if each thread stack was
        // originally only 2KB, the current size-limit logic wouldn't actually
        // shrink them because that's the size to which it tries to shrink.  If
        // you fail this part of the test due to something like that, the test
        // logic should probably be improved to account for your situation.

        // Copyied from sections/thread_list_stream.rs
        const LIMIT_BASE_THREAD_COUNT: usize = 20;
        const MIN_PER_EXTRA_THREAD_STACK_REDUCTION: usize = 1024;
        let min_expected_reduction =
            (40 - LIMIT_BASE_THREAD_COUNT) * MIN_PER_EXTRA_THREAD_STACK_REDUCTION;
        assert!(total_limit_stack_size < total_normal_stack_size - min_expected_reduction as u32);
    }

    child.kill().expect("Failed to kill process");

    // Reap child
    let waitres = child.wait().expect("Failed to wait for child");
    let status = waitres.signal().expect("Child did not die due to signal");
    assert_eq!(waitres.code(), None);
    assert_eq!(status, Signal::SIGKILL as i32);
}

#[test]
fn with_deleted_binary() {
    let num_of_threads = 1;
    let binary_copy_dir = tempfile::Builder::new()
        .prefix("deleted_binary")
        .tempdir()
        .unwrap();
    let binary_copy = binary_copy_dir.as_ref().join("binary_copy");

    let path: String = if let Ok(p) = std::env::var("TEST_HELPER") {
        p
    } else {
        std::env!("CARGO_BIN_EXE_test").into()
    };

    std::fs::copy(path, &binary_copy).expect("Failed to copy binary");
    let mem_slice = std::fs::read(&binary_copy).expect("Failed to read binary");

    let mut child = Command::new(&binary_copy)
        .env("RUST_BACKTRACE", "1")
        .arg("spawn_and_wait")
        .arg(num_of_threads.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    wait_for_threads(&mut child, num_of_threads);

    let pid = child.id() as i32;

    let BuildId(mut build_id) =
        BuildId::read_from_module(mem_slice.as_slice().into()).expect("Failed to get build_id");

    std::fs::remove_file(&binary_copy).expect("Failed to remove binary");

    let mut tmpfile = tempfile::Builder::new()
        .prefix("deleted_binary")
        .tempfile()
        .unwrap();

    MinidumpWriterConfig::new(pid, pid)
        .write(&mut tmpfile)
        .expect("Could not write minidump");

    child.kill().expect("Failed to kill process");

    // Reap child
    let waitres = child.wait().expect("Failed to wait for child");
    let status = waitres.signal().expect("Child did not die due to signal");
    assert_eq!(waitres.code(), None);
    assert_eq!(status, Signal::SIGKILL as i32);

    // Begin checks on dump
    let meta = std::fs::metadata(tmpfile.path()).expect("Couldn't get metadata for tempfile");
    assert!(meta.len() > 0);

    let dump = Minidump::read_path(tmpfile.path()).expect("Failed to read minidump");
    let module_list: MinidumpModuleList = dump
        .get_stream()
        .expect("Couldn't find stream MinidumpModuleList");
    let main_module = module_list
        .main_module()
        .expect("Could not get main module");
    //assert_eq!(main_module.code_file(), binary_copy.to_string_lossy());

    let did = main_module
        .debug_identifier()
        .expect("expected value debug id");
    {
        let uuid = did.uuid();
        let uuid = uuid.as_bytes();

        // Swap bytes in the original to match the expected uuid
        if cfg!(target_endian = "little") {
            build_id[..4].reverse();
            build_id[4..6].reverse();
            build_id[6..8].reverse();
        }

        // The build_id from the binary can be as little as 8 bytes, eg LLD uses
        // xxhash to calculate the build_id by default from 10+
        build_id.resize(16, 0);

        assert_eq!(uuid.as_slice(), &build_id);
    }

    // The 'age'/appendix, always 0 on non-windows targets
    assert_eq!(did.appendix(), 0);
}

#[test]
fn memory_info_list_stream() {
    let mut child = start_child_and_wait_for_threads(1);
    let pid = child.id() as i32;

    let mut tmpfile = tempfile::Builder::new()
        .prefix("memory_info_list_stream")
        .tempfile()
        .unwrap();

    // Write a minidump
    MinidumpWriterConfig::new(pid, pid)
        .write(&mut tmpfile)
        .expect("cound not write minidump");
    child.kill().expect("Failed to kill process");
    child.wait().expect("Failed to wait on killed process");

    // Ensure the minidump has a MemoryInfoListStream present and has at least one entry.
    let dump = Minidump::read_path(tmpfile.path()).expect("failed to read minidump");
    let list: MinidumpMemoryInfoList = dump.get_stream().expect("no memory info list");
    assert!(list.iter().count() > 1);
}
