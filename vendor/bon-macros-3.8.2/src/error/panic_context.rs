// The new name is used on newer rust versions
#[rustversion::since(1.81.0)]
use std::panic::PanicHookInfo as StdPanicHookInfo;

// The deprecated name for is used on older rust versions
#[rustversion::before(1.81.0)]
use std::panic::PanicInfo as StdPanicHookInfo;

use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

fn with_global_panic_context<T>(f: impl FnOnce(&mut GlobalPanicContext) -> T) -> T {
    thread_local! {
        /// A lazily initialized global panic context. It aggregates the panics from the
        /// current thread. This is used to capture info about the panic after the
        /// `catch_unwind` call and observe the context of the panic that happened.
        ///
        /// Unfortunately, we can't use a global static variable that would be
        /// accessible by all threads because `std::sync::Mutex::new` became
        /// `const` only in Rust 1.63.0, which is above our MSRV 1.59.0. However,
        /// a thread-local works perfectly fine for our use case because we don't
        /// spawn threads in proc macros.
        static GLOBAL: RefCell<GlobalPanicContext> = const {
            RefCell::new(GlobalPanicContext {
                last_panic: None,
                initialized: false,
            })
        };
    }

    GLOBAL.with(|global| f(&mut global.borrow_mut()))
}

struct GlobalPanicContext {
    last_panic: Option<PanicContext>,
    initialized: bool,
}

/// This struct without any fields exists to make sure that [`PanicListener::register()`]
/// is called first before the code even attempts to get the last panic information.
#[derive(Default)]
pub(super) struct PanicListener {
    /// Required to make sure struct is not constructable via a struct literal
    /// in the code outside of this module.
    _private: (),
}

impl PanicListener {
    pub(super) fn register() -> Self {
        with_global_panic_context(Self::register_with_global)
    }

    fn register_with_global(global: &mut GlobalPanicContext) -> Self {
        if global.initialized {
            return Self { _private: () };
        }

        let prev_panic_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            with_global_panic_context(|global| {
                let panics_count = global.last_panic.as_ref().map(|p| p.0.panics_count);
                let panics_count = panics_count.unwrap_or(0) + 1;

                global.last_panic = Some(PanicContext::from_std(panic_info, panics_count));
            });

            prev_panic_hook(panic_info);
        }));

        global.initialized = true;

        Self { _private: () }
    }

    /// Returns the last panic that happened since the [`PanicListener::register()`] call.
    // `self` is required to make sure this code runs only after we initialized
    // the global panic listener in the `register` method.
    #[allow(clippy::unused_self)]
    pub(super) fn get_last_panic(&self) -> Option<PanicContext> {
        with_global_panic_context(|global| global.last_panic.clone())
    }
}

/// Contains all the necessary bits of information about the occurred panic.
#[derive(Clone)]
pub(super) struct PanicContext(Rc<PanicContextShared>);

struct PanicContextShared {
    #[allow(clippy::incompatible_msrv)]
    backtrace: backtrace::Backtrace,

    location: Option<PanicLocation>,
    thread: String,

    /// Defines the number of panics that happened before this one. Each panic
    /// increments this counter. This is useful to know how many panics happened
    /// before the current one.
    panics_count: usize,
}

impl PanicContext {
    #[allow(clippy::incompatible_msrv)]
    fn from_std(std_panic_info: &StdPanicHookInfo<'_>, panics_count: usize) -> Self {
        let location = std_panic_info.location();
        let current_thread = std::thread::current();
        let thread_ = current_thread
            .name()
            .map(String::from)
            .unwrap_or_else(|| format!("{:?}", current_thread.id()));

        Self(Rc::new(PanicContextShared {
            // This is expected, and we handle the compatibility with
            // conditional compilation via the `rustversion` crate.
            #[allow(clippy::incompatible_msrv)]
            backtrace: backtrace::Backtrace::capture(),
            location: location.map(PanicLocation::from_std),
            thread: thread_,
            panics_count,
        }))
    }
}

impl fmt::Debug for PanicContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for PanicContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PanicContextShared {
            location,
            backtrace,
            thread,
            panics_count,
        } = &*self.0;

        write!(f, "panic occurred")?;

        if let Some(location) = location {
            write!(f, " at {location}")?;
        }

        write!(f, " in thread '{thread}'")?;

        if *panics_count > 1 {
            write!(f, " (total panics observed: {panics_count})")?;
        }

        #[allow(clippy::incompatible_msrv)]
        if backtrace.status() == backtrace::BacktraceStatus::Captured {
            write!(f, "\nbacktrace:\n{backtrace}")?;
        }

        Ok(())
    }
}

/// Extract the message of a panic.
pub(super) fn message_from_panic_payload(payload: &dyn Any) -> Option<String> {
    if let Some(str_slice) = payload.downcast_ref::<&str>() {
        return Some((*str_slice).to_owned());
    }
    if let Some(owned_string) = payload.downcast_ref::<String>() {
        return Some(owned_string.clone());
    }

    None
}

/// Location of the panic call site.
#[derive(Clone)]
struct PanicLocation {
    file: String,
    line: u32,
    col: u32,
}

impl PanicLocation {
    fn from_std(loc: &std::panic::Location<'_>) -> Self {
        Self {
            file: loc.file().to_owned(),
            line: loc.line(),
            col: loc.column(),
        }
    }
}

impl fmt::Display for PanicLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

#[rustversion::since(1.65.0)]
mod backtrace {
    pub(super) use std::backtrace::{Backtrace, BacktraceStatus};
}

#[rustversion::before(1.65.0)]
mod backtrace {
    #[derive(PartialEq)]
    pub(super) enum BacktraceStatus {
        Captured,
    }

    pub(super) struct Backtrace;

    impl Backtrace {
        pub(super) fn capture() -> Self {
            Self
        }
        pub(super) fn status(&self) -> BacktraceStatus {
            BacktraceStatus::Captured
        }
    }

    impl std::fmt::Display for Backtrace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("{update your Rust compiler to >=1.65.0 to see the backtrace}")
        }
    }
}
