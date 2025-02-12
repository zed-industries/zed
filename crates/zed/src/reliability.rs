use backtrace::{self, Backtrace};
use release_channel::{AppCommitSha, ReleaseChannel};
use std::{ffi::c_void, sync::atomic::Ordering};
use std::{panic, sync::atomic::AtomicU32, thread};

static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn init_panic_hook(app_commit_sha: Option<AppCommitSha>) {
    panic::set_hook(Box::new(move |info| {
        let prior_panic_count = PANIC_COUNT.fetch_add(1, Ordering::SeqCst);
        if prior_panic_count > 0 {
            // Give the panic-ing thread time to write the panic file
            loop {
                std::thread::yield_now();
            }
        }

        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Box<Any>".to_string());

        if *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
            let location = info.location().unwrap();
            let backtrace = Backtrace::new();
            eprintln!(
                "Thread {:?} panicked with {:?} at {}:{}:{}\n{}{:?}",
                thread_name,
                payload,
                location.file(),
                location.line(),
                location.column(),
                match app_commit_sha.as_ref() {
                    Some(commit_sha) => format!(
                        "https://github.com/zed-industries/zed/blob/{}/src/{}#L{} \
                        (may not be uploaded, line may be incorrect if files modified)\n",
                        commit_sha.0,
                        location.file(),
                        location.line()
                    ),
                    None => "".to_string(),
                },
                backtrace,
            );
            std::process::exit(-1);
        }
        let main_module_base_address = get_main_module_base_address();

        let backtrace = Backtrace::new();
        let mut symbols = backtrace
            .frames()
            .iter()
            .flat_map(|frame| {
                let base = frame
                    .module_base_address()
                    .unwrap_or(main_module_base_address);
                frame.symbols().iter().map(move |symbol| {
                    format!(
                        "{}+{}",
                        symbol
                            .name()
                            .as_ref()
                            .map_or("<unknown>".to_owned(), <_>::to_string),
                        (frame.ip() as isize).saturating_sub(base as isize)
                    )
                })
            })
            .collect::<Vec<_>>();

        // Strip out leading stack frames for rust panic-handling.
        if let Some(ix) = symbols
            .iter()
            .position(|name| name == "rust_begin_unwind" || name == "_rust_begin_unwind")
        {
            symbols.drain(0..=ix);
        }

        std::process::abort();
    }));
}

#[cfg(not(target_os = "windows"))]
fn get_main_module_base_address() -> *mut c_void {
    let mut dl_info = libc::Dl_info {
        dli_fname: std::ptr::null(),
        dli_fbase: std::ptr::null_mut(),
        dli_sname: std::ptr::null(),
        dli_saddr: std::ptr::null_mut(),
    };
    unsafe {
        libc::dladdr(get_main_module_base_address as _, &mut dl_info);
    }
    dl_info.dli_fbase
}

#[cfg(target_os = "windows")]
fn get_main_module_base_address() -> *mut c_void {
    std::ptr::null_mut()
}
