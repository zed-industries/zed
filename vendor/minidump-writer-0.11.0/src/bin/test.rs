// This binary shouldn't be under /src, but under /tests, but that is
// currently not possible (https://github.com/rust-lang/cargo/issues/4356)

type Error = Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux {
    use {
        super::*,
        error_graph::ErrorList,
        minidump_writer::{
            minidump_writer::{MinidumpWriter, MinidumpWriterConfig},
            module_reader, LINUX_GATE_LIBRARY_NAME,
        },
        nix::{
            sys::mman::{mmap_anonymous, MapFlags, ProtFlags},
            unistd::getppid,
        },
    };

    macro_rules! test {
        ($x:expr, $errmsg:expr) => {
            if !$x {
                return Err($errmsg.into());
            }
        };
    }

    macro_rules! fail_on_soft_error(($n: ident, $e: expr) => {{
        let mut $n = ErrorList::default();
        let __result = $e;
        if !$n.is_empty() {
            return Err($n.into());
        }
        __result
    }});

    fn test_setup() -> Result<()> {
        let ppid = getppid();
        fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid.as_raw(), ppid.as_raw())
                .build_for_testing(&mut soft_errors)?
        );
        Ok(())
    }

    fn test_thread_list() -> Result<()> {
        let ppid = getppid();
        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid.as_raw(), ppid.as_raw())
                .build_for_testing(&mut soft_errors)?
        );
        test!(!dumper.threads.is_empty(), "No threads");
        test!(
            dumper
                .threads
                .iter()
                .filter(|x| x.tid == ppid.as_raw())
                .count()
                == 1,
            "Thread found multiple times"
        );

        test!(
            dumper
                .threads
                .iter()
                .any(|thread| thread.name.as_deref() == Some("sighup-thread")),
            "Failed to locate and/or stop sighup-thread"
        );

        Ok(())
    }

    fn test_copy_from_process(stack_var: usize, heap_var: usize) -> Result<()> {
        use minidump_writer::mem_reader::MemReader;

        let ppid = getppid().as_raw();
        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid, ppid).build_for_testing(&mut soft_errors)?
        );

        // We support 3 different methods of reading memory from another
        // process, ensure they all function and give the same results

        let expected_stack = 0x11223344usize.to_ne_bytes();
        let expected_heap = 0x55667788usize.to_ne_bytes();

        let validate = |reader: &mut MemReader| -> Result<()> {
            let mut val = [0u8; std::mem::size_of::<usize>()];
            let read = reader.read(stack_var, &mut val)?;
            assert_eq!(read, val.len());
            test!(val == expected_stack, "stack var not correct");

            let read = reader.read(heap_var, &mut val)?;
            assert_eq!(read, val.len());
            test!(val == expected_heap, "heap var not correct");

            Ok(())
        };

        // virtual mem
        {
            let mut mr = MemReader::for_virtual_mem(ppid);
            validate(&mut mr)
                .map_err(|err| format!("failed to validate memory for {mr:?}: {err}"))?;
        }

        // file
        {
            let mut mr = MemReader::for_file(ppid)
                .map_err(|err| format!("failed to open `/proc/{ppid}/mem`: {err}"))?;
            validate(&mut mr)
                .map_err(|err| format!("failed to validate memory for {mr:?}: {err}"))?;
        }

        // ptrace
        {
            let mut mr = MemReader::for_ptrace(ppid);
            validate(&mut mr)
                .map_err(|err| format!("failed to validate memory for {mr:?}: {err}"))?;
        }

        let stack_res =
            MinidumpWriter::copy_from_process(ppid, stack_var, std::mem::size_of::<usize>())?;

        test!(stack_res == expected_stack, "stack var not correct");

        let heap_res =
            MinidumpWriter::copy_from_process(ppid, heap_var, std::mem::size_of::<usize>())?;

        test!(heap_res == expected_heap, "heap var not correct");

        drop(dumper);

        Ok(())
    }

    fn test_find_mappings(addr1: usize, addr2: usize) -> Result<()> {
        let ppid = getppid();

        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid.as_raw(), ppid.as_raw())
                .build_for_testing(&mut soft_errors)?
        );
        dumper
            .find_mapping(addr1)
            .ok_or("No mapping for addr1 found")?;

        dumper
            .find_mapping(addr2)
            .ok_or("No mapping for addr2 found")?;

        test!(dumper.find_mapping(0).is_none(), "NULL found");
        Ok(())
    }

    fn test_file_id() -> Result<()> {
        let ppid = getppid().as_raw();
        let exe_link = format!("/proc/{ppid}/exe");
        let exe_name = std::fs::read_link(exe_link)?.into_os_string();

        let mut dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid, ppid).build_for_testing(&mut soft_errors)?
        );

        let mut found_exe = None;
        for (idx, mapping) in dumper.mappings.iter().enumerate() {
            if mapping.name.as_ref().map(|x| x.into()).as_ref() == Some(&exe_name) {
                found_exe = Some(idx);
                break;
            }
        }
        let idx = found_exe.unwrap();
        let module_reader::BuildId(id) = dumper.from_process_memory_for_index(idx)?;

        drop(dumper);

        assert!(!id.is_empty());
        assert!(id.iter().any(|&x| x > 0));
        Ok(())
    }

    fn test_merged_mappings(path: String, mapped_mem: usize, mem_size: usize) -> Result<()> {
        // Now check that PtraceDumper interpreted the mappings properly.
        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(getppid().as_raw(), getppid().as_raw())
                .build_for_testing(&mut soft_errors)?
        );
        let mut mapping_count = 0;
        for map in &dumper.mappings {
            if map
                .name
                .as_ref()
                .is_some_and(|name| name.to_string_lossy().starts_with(&path))
            {
                mapping_count += 1;
                // This mapping should encompass the entire original mapped
                // range.
                assert_eq!(map.start_address, mapped_mem);
                assert_eq!(map.size, mem_size);
                assert_eq!(0, map.offset);
            }
        }
        assert_eq!(1, mapping_count);
        Ok(())
    }

    fn test_linux_gate_mapping_id() -> Result<()> {
        let ppid = getppid().as_raw();
        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid, ppid).build_for_testing(&mut soft_errors)?
        );
        let mut found_linux_gate = false;
        for mapping in dumper.mappings.clone() {
            if mapping.name == Some(LINUX_GATE_LIBRARY_NAME.into()) {
                found_linux_gate = true;

                let module_reader::BuildId(id) =
                    MinidumpWriter::from_process_memory_for_mapping(&mapping, ppid)?;
                test!(!id.is_empty(), "id-vec is empty");
                test!(id.iter().any(|&x| x > 0), "all id elements are 0");
                drop(dumper);
                break;
            }
        }
        test!(found_linux_gate, "found no linux_gate");
        Ok(())
    }

    fn test_mappings_include_linux_gate() -> Result<()> {
        let ppid = getppid().as_raw();
        let dumper = fail_on_soft_error!(
            soft_errors,
            MinidumpWriterConfig::new(ppid, ppid).build_for_testing(&mut soft_errors)?
        );
        let linux_gate_loc = dumper.auxv.get_linux_gate_address().unwrap();
        test!(linux_gate_loc != 0, "linux_gate_loc == 0");
        let mut found_linux_gate = false;
        for mapping in &dumper.mappings {
            if mapping.name == Some(LINUX_GATE_LIBRARY_NAME.into()) {
                found_linux_gate = true;
                test!(
                    usize::try_from(linux_gate_loc)? == mapping.start_address,
                    "linux_gate_loc != start_address"
                );

                // This doesn't work here, as we do not test via "fork()", so the addresses are different
                // let ll = mapping.start_address as *const u8;
                // for idx in 0..header::SELFMAG {
                //     let mag = unsafe { std::ptr::read(ll.offset(idx as isize)) == header::ELFMAG[idx] };
                //     test!(
                //         mag,
                //         format!("ll: {} != ELFMAG: {} at {}", mag, header::ELFMAG[idx], idx)
                //     )?;
                // }
                break;
            }
        }
        test!(found_linux_gate, "found no linux_gate");
        Ok(())
    }

    fn spawn_and_wait(num: usize) -> Result<()> {
        // One less than the requested amount, as the main thread counts as well
        for _ in 1..num {
            std::thread::spawn(|| {
                println!("1");
                loop {
                    std::thread::park();
                }
            });
        }
        println!("1");
        loop {
            std::thread::park();
        }
    }

    fn spawn_name_wait(num: usize) -> Result<()> {
        // One less than the requested amount, as the main thread counts as well
        for id in 1..num {
            std::thread::Builder::new()
                .name(format!("thread_{id}"))
                .spawn(|| {
                    println!("1");
                    loop {
                        std::thread::park();
                    }
                })?;
        }
        println!("1");
        loop {
            std::thread::park();
        }
    }

    fn spawn_mmap_wait() -> Result<()> {
        let page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE).unwrap();
        let memory_size = std::num::NonZeroUsize::new(page_size.unwrap() as usize).unwrap();
        // Get some memory to be mapped by the child-process
        let mapped_mem = unsafe {
            mmap_anonymous(
                None,
                memory_size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_ANON,
            )
            .unwrap()
        };

        println!("{} {}", mapped_mem.as_ptr() as usize, memory_size);
        loop {
            std::thread::park();
        }
    }

    fn spawn_alloc_wait() -> Result<()> {
        let page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE).unwrap();
        let memory_size = page_size.unwrap() as usize;

        let mut values = Vec::<u8>::with_capacity(memory_size);
        for idx in 0..memory_size {
            values.push((idx % 255) as u8);
        }

        println!("{:p} {}", values.as_ptr(), memory_size);
        loop {
            std::thread::park();
        }
    }

    fn create_files_wait(num: usize) -> Result<()> {
        let mut file_array = Vec::<tempfile::NamedTempFile>::with_capacity(num);
        for id in 0..num {
            let file = tempfile::Builder::new()
                .prefix("test_file")
                .suffix::<str>(id.to_string().as_ref())
                .tempfile()
                .unwrap();
            file_array.push(file);
            println!("1");
        }
        println!("1");
        loop {
            std::thread::park();
            // This shouldn't be executed, but we put it here to ensure that
            // all the files within the array are kept open.
            println!("{}", file_array.len());
        }
    }

    pub(super) fn real_main(args: Vec<String>) -> Result<()> {
        match args.len() {
            1 => match args[0].as_ref() {
                "nop" => Ok(()),
                "file_id" => test_file_id(),
                "setup" => test_setup(),
                "thread_list" => test_thread_list(),
                "mappings_include_linux_gate" => test_mappings_include_linux_gate(),
                "linux_gate_mapping_id" => test_linux_gate_mapping_id(),
                "spawn_mmap_wait" => spawn_mmap_wait(),
                "spawn_alloc_wait" => spawn_alloc_wait(),
                _ => Err("Len 1: Unknown test option".into()),
            },
            2 => match args[0].as_ref() {
                "spawn_and_wait" => {
                    let num_of_threads: usize = args[1].parse().unwrap();
                    spawn_and_wait(num_of_threads)
                }
                "spawn_name_wait" => {
                    let num_of_threads: usize = args[1].parse().unwrap();
                    spawn_name_wait(num_of_threads)
                }
                "create_files_wait" => {
                    let num_of_files: usize = args[1].parse().unwrap();
                    create_files_wait(num_of_files)
                }
                _ => Err(format!("Len 2: Unknown test option: {}", args[0]).into()),
            },
            3 => {
                if args[0] == "find_mappings" {
                    let addr1: usize = args[1].parse().unwrap();
                    let addr2: usize = args[2].parse().unwrap();
                    test_find_mappings(addr1, addr2)
                } else if args[0] == "copy_from_process" {
                    let stack_var: usize = args[1].parse().unwrap();
                    let heap_var: usize = args[2].parse().unwrap();
                    test_copy_from_process(stack_var, heap_var)
                } else {
                    Err(format!("Len 3: Unknown test option: {}", args[0]).into())
                }
            }
            4 => {
                if args[0] == "merged_mappings" {
                    let path = &args[1];
                    let mapped_mem: usize = args[2].parse().unwrap();
                    let mem_size: usize = args[3].parse().unwrap();
                    test_merged_mappings(path.to_string(), mapped_mem, mem_size)
                } else {
                    Err(format!("Len 4: Unknown test option: {}", args[0]).into())
                }
            }
            _ => Err("Unknown test option".into()),
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use std::mem;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetCurrentProcessId() -> u32;
        pub fn GetCurrentThreadId() -> u32;
        pub fn GetCurrentThread() -> isize;
        pub fn GetThreadContext(thread: isize, context: *mut crash_context::CONTEXT) -> i32;
    }

    #[inline(never)]
    pub(super) fn real_main(args: Vec<String>) -> Result<()> {
        let exception_code = u32::from_str_radix(&args[0], 16).unwrap();

        // Generate the exception and communicate back where the exception pointers
        // are
        unsafe {
            let mut exception_record: crash_context::EXCEPTION_RECORD = mem::zeroed();
            let mut exception_context = std::mem::MaybeUninit::uninit();

            let pid = GetCurrentProcessId();
            let tid = GetCurrentThreadId();

            GetThreadContext(GetCurrentThread(), exception_context.as_mut_ptr());

            let mut exception_context = exception_context.assume_init();

            let exception_ptrs = crash_context::EXCEPTION_POINTERS {
                ExceptionRecord: &mut exception_record,
                ContextRecord: &mut exception_context,
            };

            exception_record.ExceptionCode = exception_code as _;

            let exc_ptr_addr = &exception_ptrs as *const _ as usize;

            println!("{pid} {exc_ptr_addr} {tid} {exception_code:x}");

            // Wait until we're killed
            loop {
                std::thread::park();
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use super::*;
    use std::time::Duration;

    #[inline(never)]
    pub(super) fn real_main(args: Vec<String>) -> Result<()> {
        let port_name = args.get(0).ok_or("mach port name not specified")?;
        let exception: u32 = args.get(1).ok_or("exception code not specified")?.parse()?;

        let client =
            crash_context::ipc::Client::create(&std::ffi::CString::new(port_name.clone())?)?;

        std::thread::Builder::new()
            .name("test-thread".to_owned())
            .spawn(move || {
                #[inline(never)]
                fn wait_until_killed(client: crash_context::ipc::Client, exception: u32) {
                    // SAFETY: syscalls
                    let cc = unsafe {
                        crash_context::CrashContext {
                            task: mach2::traps::mach_task_self(),
                            thread: mach2::mach_init::mach_thread_self(),
                            handler_thread: mach2::port::MACH_PORT_NULL,
                            exception: Some(crash_context::ExceptionInfo {
                                kind: exception,
                                code: 0,
                                subcode: None,
                            }),
                        }
                    };

                    // Send the crash context to the server and wait for it to
                    // finish dumping, we should be killed shortly afterwards
                    client
                        .send_crash_context(
                            &cc,
                            Some(Duration::from_secs(2)),
                            Some(Duration::from_secs(5)),
                        )
                        .expect("failed to send crash context/receive ack");

                    // Wait until we're killed
                    loop {
                        std::thread::park();
                    }
                }

                wait_until_killed(client, exception)
            })
            .unwrap()
            .join()
            .unwrap();

        Ok(())
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "android"))] {
            linux::real_main(args)
        } else if #[cfg(target_os = "windows")] {
            windows::real_main(args)
        } else if #[cfg(target_os = "macos")] {
            mac::real_main(args)
        } else {
            unimplemented!();
        }
    }
}
