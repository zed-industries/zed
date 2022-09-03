#![cfg(target_os = "macos")]
use super::*;
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

impl From<u32> for LocalProcessStatus {
    fn from(s: u32) -> Self {
        match s {
            1 => Self::Idle,
            2 => Self::Run,
            3 => Self::Sleep,
            4 => Self::Stop,
            5 => Self::Zombie,
            _ => Self::Unknown,
        }
    }
}

impl LocalProcessInfo {
    pub fn current_working_dir(pid: u32) -> Option<PathBuf> {
        let mut pathinfo: libc::proc_vnodepathinfo = unsafe { std::mem::zeroed() };
        let size = std::mem::size_of_val(&pathinfo) as libc::c_int;
        let ret = unsafe {
            libc::proc_pidinfo(
                pid as _,
                libc::PROC_PIDVNODEPATHINFO,
                0,
                &mut pathinfo as *mut _ as *mut _,
                size,
            )
        };
        if ret != size {
            return None;
        }

        // Workaround a workaround for an old rustc version supported by libc;
        // the type of vip_path should just be [c_char; MAXPATHLEN] but it
        // is defined as a horrible nested array by the libc crate:
        // `[[c_char; 32]; 32]`.
        // Urgh.  Let's re-cast it as the correct kind of slice.
        let vip_path = unsafe {
            std::slice::from_raw_parts(
                pathinfo.pvi_cdir.vip_path.as_ptr() as *const u8,
                libc::MAXPATHLEN as usize,
            )
        };
        let nul = vip_path.iter().position(|&c| c == 0)?;
        Some(OsStr::from_bytes(&vip_path[0..nul]).into())
    }

    pub fn executable_path(pid: u32) -> Option<PathBuf> {
        let mut buffer: Vec<u8> = Vec::with_capacity(libc::PROC_PIDPATHINFO_MAXSIZE as _);
        let x = unsafe {
            libc::proc_pidpath(
                pid as _,
                buffer.as_mut_ptr() as *mut _,
                libc::PROC_PIDPATHINFO_MAXSIZE as _,
            )
        };
        if x <= 0 {
            return None;
        }

        unsafe { buffer.set_len(x as usize) };
        Some(OsString::from_vec(buffer).into())
    }

    pub fn with_root_pid(pid: u32) -> Option<Self> {
        /// Enumerate all current process identifiers
        fn all_pids() -> Vec<libc::pid_t> {
            let num_pids = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
            if num_pids < 1 {
                return vec![];
            }

            // Give a bit of padding to avoid looping if processes are spawning
            // rapidly while we're trying to collect this info
            const PADDING: usize = 32;
            let mut pids: Vec<libc::pid_t> = Vec::with_capacity(num_pids as usize + PADDING);
            loop {
                let n = unsafe {
                    libc::proc_listallpids(
                        pids.as_mut_ptr() as *mut _,
                        (pids.capacity() * std::mem::size_of::<libc::pid_t>()) as _,
                    )
                };

                if n < 1 {
                    return vec![];
                }

                let n = n as usize;

                if n > pids.capacity() {
                    pids.reserve(n + PADDING);
                    continue;
                }

                unsafe { pids.set_len(n) };
                return pids;
            }
        }

        /// Obtain info block for a pid.
        /// Note that the process could have gone away since we first
        /// observed the pid and the time we call this, so we must
        /// be able to tolerate this failing.
        fn info_for_pid(pid: libc::pid_t) -> Option<libc::proc_bsdinfo> {
            let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
            let wanted_size = std::mem::size_of::<libc::proc_bsdinfo>() as _;
            let res = unsafe {
                libc::proc_pidinfo(
                    pid,
                    libc::PROC_PIDTBSDINFO,
                    0,
                    &mut info as *mut _ as *mut _,
                    wanted_size,
                )
            };

            if res == wanted_size {
                Some(info)
            } else {
                None
            }
        }

        fn cwd_for_pid(pid: libc::pid_t) -> PathBuf {
            LocalProcessInfo::current_working_dir(pid as _).unwrap_or_else(PathBuf::new)
        }

        fn exe_and_args_for_pid_sysctl(pid: libc::pid_t) -> Option<(PathBuf, Vec<String>)> {
            use libc::c_int;
            let mut size = 64 * 1024;
            let mut buf: Vec<u8> = Vec::with_capacity(size);
            let mut mib = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid as c_int];

            let res = unsafe {
                libc::sysctl(
                    mib.as_mut_ptr(),
                    mib.len() as _,
                    buf.as_mut_ptr() as *mut _,
                    &mut size,
                    std::ptr::null_mut(),
                    0,
                )
            };
            if res == -1 {
                return None;
            }
            if size < (std::mem::size_of::<c_int>() * 2) {
                // Not big enough
                return None;
            }
            unsafe { buf.set_len(size) };

            // The data in our buffer is laid out like this:
            // argc - c_int
            // exe_path - NUL terminated string
            // argv[0] - NUL terminated string
            // argv[1] - NUL terminated string
            // ...
            // argv[n] - NUL terminated string
            // envp[0] - NUL terminated string
            // ...

            let mut ptr = &buf[0..size];

            let argc: c_int = unsafe { std::ptr::read(ptr.as_ptr() as *const c_int) };
            ptr = &ptr[std::mem::size_of::<c_int>()..];

            fn consume_cstr(ptr: &mut &[u8]) -> Option<String> {
                let nul = ptr.iter().position(|&c| c == 0)?;
                let s = String::from_utf8_lossy(&ptr[0..nul]).to_owned().to_string();
                *ptr = ptr.get(nul + 1..)?;
                Some(s)
            }

            let exe_path = consume_cstr(&mut ptr)?.into();

            let mut args = vec![];
            for _ in 0..argc {
                args.push(consume_cstr(&mut ptr)?);
            }

            dbg!(&exe_path);
            dbg!(&args);
            Some((exe_path, args))
        }

        fn exe_for_pid(pid: libc::pid_t) -> PathBuf {
            LocalProcessInfo::executable_path(pid as _).unwrap_or_else(PathBuf::new)
        }

        let procs: Vec<_> = all_pids().into_iter().filter_map(info_for_pid).collect();

        fn build_proc(info: &libc::proc_bsdinfo, procs: &[libc::proc_bsdinfo]) -> LocalProcessInfo {
            let mut children = HashMap::new();

            for kid in procs {
                if kid.pbi_ppid == info.pbi_pid {
                    children.insert(kid.pbi_pid, build_proc(kid, procs));
                }
            }

            let (executable, argv) = exe_and_args_for_pid_sysctl(info.pbi_pid as _)
                .unwrap_or_else(|| (exe_for_pid(info.pbi_pid as _), vec![]));

            let name = unsafe { std::ffi::CStr::from_ptr(info.pbi_comm.as_ptr() as _) };
            let name = name.to_str().unwrap_or("").to_string();

            LocalProcessInfo {
                pid: info.pbi_pid,
                ppid: info.pbi_ppid,
                name,
                executable,
                cwd: cwd_for_pid(info.pbi_pid as _),
                argv,
                start_time: info.pbi_start_tvsec,
                status: LocalProcessStatus::from(info.pbi_status),
                children,
            }
        }

        if let Some(info) = procs.iter().find(|info| info.pbi_pid == pid) {
            Some(build_proc(info, &procs))
        } else {
            None
        }
    }
}
