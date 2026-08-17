// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd};
use std::{
    boxed::Box as Box_,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{ffi, translate::*, GStr, GString, LogWriterOutput};

#[derive(Debug)]
pub struct LogHandlerId(u32);

#[doc(hidden)]
impl FromGlib<u32> for LogHandlerId {
    #[inline]
    unsafe fn from_glib(value: u32) -> Self {
        Self(value)
    }
}

#[doc(hidden)]
impl IntoGlib for LogHandlerId {
    type GlibType = u32;

    #[inline]
    fn into_glib(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    #[doc(alias = "G_LOG_LEVEL_ERROR")]
    Error,
    #[doc(alias = "G_LOG_LEVEL_CRITICAL")]
    Critical,
    #[doc(alias = "G_LOG_LEVEL_WARNING")]
    Warning,
    #[doc(alias = "G_LOG_LEVEL_MESSAGE")]
    Message,
    #[doc(alias = "G_LOG_LEVEL_INFO")]
    Info,
    #[doc(alias = "G_LOG_LEVEL_DEBUG")]
    Debug,
}

#[doc(hidden)]
impl IntoGlib for LogLevel {
    type GlibType = u32;

    #[inline]
    fn into_glib(self) -> u32 {
        match self {
            Self::Error => ffi::G_LOG_LEVEL_ERROR,
            Self::Critical => ffi::G_LOG_LEVEL_CRITICAL,
            Self::Warning => ffi::G_LOG_LEVEL_WARNING,
            Self::Message => ffi::G_LOG_LEVEL_MESSAGE,
            Self::Info => ffi::G_LOG_LEVEL_INFO,
            Self::Debug => ffi::G_LOG_LEVEL_DEBUG,
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for LogLevel {
    #[inline]
    unsafe fn from_glib(value: u32) -> Self {
        if value & ffi::G_LOG_LEVEL_ERROR != 0 {
            Self::Error
        } else if value & ffi::G_LOG_LEVEL_CRITICAL != 0 {
            Self::Critical
        } else if value & ffi::G_LOG_LEVEL_WARNING != 0 {
            Self::Warning
        } else if value & ffi::G_LOG_LEVEL_MESSAGE != 0 {
            Self::Message
        } else if value & ffi::G_LOG_LEVEL_INFO != 0 {
            Self::Info
        } else if value & ffi::G_LOG_LEVEL_DEBUG != 0 {
            Self::Debug
        } else {
            panic!("Unknown log level: {value}")
        }
    }
}

impl LogLevel {
    #[doc(hidden)]
    pub fn priority(&self) -> &'static str {
        match self {
            Self::Error => "3",
            Self::Critical => "4",
            Self::Warning => "4",
            Self::Message => "5",
            Self::Info => "6",
            Self::Debug => "7",
        }
    }
}

bitflags::bitflags! {
    #[doc(alias = "GLogLevelFlags")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LogLevels: u32 {
        #[doc(alias = "G_LOG_LEVEL_ERROR")]
        const LEVEL_ERROR = ffi::G_LOG_LEVEL_ERROR;
        #[doc(alias = "G_LOG_LEVEL_CRITICAL")]
        const LEVEL_CRITICAL = ffi::G_LOG_LEVEL_CRITICAL;
        #[doc(alias = "G_LOG_LEVEL_WARNING")]
        const LEVEL_WARNING = ffi::G_LOG_LEVEL_WARNING;
        #[doc(alias = "G_LOG_LEVEL_MESSAGE")]
        const LEVEL_MESSAGE = ffi::G_LOG_LEVEL_MESSAGE;
        #[doc(alias = "G_LOG_LEVEL_INFO")]
        const LEVEL_INFO = ffi::G_LOG_LEVEL_INFO;
        #[doc(alias = "G_LOG_LEVEL_DEBUG")]
        const LEVEL_DEBUG = ffi::G_LOG_LEVEL_DEBUG;
    }
}

#[doc(hidden)]
impl IntoGlib for LogLevels {
    type GlibType = ffi::GLogLevelFlags;

    #[inline]
    fn into_glib(self) -> ffi::GLogLevelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GLogLevelFlags> for LogLevels {
    #[inline]
    unsafe fn from_glib(value: ffi::GLogLevelFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

fn to_log_flags(fatal: bool, recursion: bool) -> u32 {
    (if fatal { ffi::G_LOG_FLAG_FATAL } else { 0 })
        | if recursion {
            ffi::G_LOG_FLAG_RECURSION
        } else {
            0
        }
}

#[doc(alias = "g_log_set_handler_full")]
pub fn log_set_handler<P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static>(
    log_domain: Option<&str>,
    log_levels: LogLevels,
    fatal: bool,
    recursion: bool,
    log_func: P,
) -> LogHandlerId {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: ffi::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: ffi::gpointer,
    ) {
        let log_domain: Borrowed<Option<GString>> = from_glib_borrow(log_domain);
        let message: Borrowed<GString> = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(
            (*log_domain).as_ref().map(|s| s.as_str()),
            from_glib(log_level),
            message.as_str(),
        );
    }
    let log_func = Some(log_func_func::<P> as _);
    unsafe extern "C" fn destroy_func<
        P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static,
    >(
        data: ffi::gpointer,
    ) {
        let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(destroy_func::<P> as _);
    let super_callback0: Box_<P> = log_func_data;
    unsafe {
        from_glib(ffi::g_log_set_handler_full(
            log_domain.to_glib_none().0,
            log_levels.into_glib() | to_log_flags(fatal, recursion),
            log_func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call4,
        ))
    }
}

#[doc(alias = "g_log_remove_handler")]
pub fn log_remove_handler(log_domain: Option<&str>, handler_id: LogHandlerId) {
    unsafe {
        ffi::g_log_remove_handler(log_domain.to_glib_none().0, handler_id.into_glib());
    }
}

#[doc(alias = "g_log_set_always_fatal")]
pub fn log_set_always_fatal(fatal_levels: LogLevels) -> LogLevels {
    unsafe { from_glib(ffi::g_log_set_always_fatal(fatal_levels.into_glib())) }
}

#[doc(alias = "g_log_set_fatal_mask")]
pub fn log_set_fatal_mask(log_domain: Option<&str>, fatal_levels: LogLevels) -> LogLevels {
    unsafe {
        from_glib(ffi::g_log_set_fatal_mask(
            log_domain.to_glib_none().0,
            fatal_levels.into_glib(),
        ))
    }
}

type PrintCallback = dyn Fn(&str) + Send + Sync + 'static;

fn print_handler() -> &'static Mutex<Option<Arc<PrintCallback>>> {
    static MUTEX: OnceLock<Mutex<Option<Arc<PrintCallback>>>> = OnceLock::new();
    MUTEX.get_or_init(|| Mutex::new(None))
}

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`unset_print_handler`] function.
#[doc(alias = "g_set_print_handler")]
pub fn set_print_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        if let Some(callback) = print_handler()
            .lock()
            .expect("Failed to lock PRINT_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let string: Borrowed<GString> = from_glib_borrow(string);
            (*callback)(string.as_str())
        }
    }
    *print_handler()
        .lock()
        .expect("Failed to lock PRINT_HANDLER to change callback") = Some(Arc::new(func));
    unsafe { ffi::g_set_print_handler(Some(func_func as _)) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`set_print_handler`] function.
pub fn unset_print_handler() {
    *print_handler()
        .lock()
        .expect("Failed to lock PRINT_HANDLER to remove callback") = None;
    unsafe { ffi::g_set_print_handler(None) };
}

fn printerr_handler() -> &'static Mutex<Option<Arc<PrintCallback>>> {
    static MUTEX: OnceLock<Mutex<Option<Arc<PrintCallback>>>> = OnceLock::new();
    MUTEX.get_or_init(|| Mutex::new(None))
}

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`unset_printerr_handler`] function.
#[doc(alias = "g_set_printerr_handler")]
pub fn set_printerr_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        if let Some(callback) = printerr_handler()
            .lock()
            .expect("Failed to lock PRINTERR_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let string: Borrowed<GString> = from_glib_borrow(string);
            (*callback)(string.as_str())
        }
    }
    *printerr_handler()
        .lock()
        .expect("Failed to lock PRINTERR_HANDLER to change callback") = Some(Arc::new(func));
    unsafe { ffi::g_set_printerr_handler(Some(func_func as _)) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`set_printerr_handler`] function.
pub fn unset_printerr_handler() {
    *printerr_handler()
        .lock()
        .expect("Failed to lock PRINTERR_HANDLER to remove callback") = None;
    unsafe { ffi::g_set_printerr_handler(None) };
}

type LogCallback = dyn Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static;

fn default_handler() -> &'static Mutex<Option<Arc<LogCallback>>> {
    static MUTEX: OnceLock<Mutex<Option<Arc<LogCallback>>>> = OnceLock::new();
    MUTEX.get_or_init(|| Mutex::new(None))
}

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`log_unset_default_handler`] function.
#[doc(alias = "g_log_set_default_handler")]
pub fn log_set_default_handler<P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static>(
    log_func: P,
) {
    unsafe extern "C" fn func_func(
        log_domain: *const libc::c_char,
        log_levels: ffi::GLogLevelFlags,
        message: *const libc::c_char,
        _user_data: ffi::gpointer,
    ) {
        if let Some(callback) = default_handler()
            .lock()
            .expect("Failed to lock DEFAULT_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let log_domain: Borrowed<Option<GString>> = from_glib_borrow(log_domain);
            let message: Borrowed<GString> = from_glib_borrow(message);
            (*callback)(
                (*log_domain).as_ref().map(|s| s.as_str()),
                from_glib(log_levels),
                message.as_str(),
            );
        }
    }
    *default_handler()
        .lock()
        .expect("Failed to lock DEFAULT_HANDLER to change callback") = Some(Arc::new(log_func));
    unsafe { ffi::g_log_set_default_handler(Some(func_func as _), std::ptr::null_mut()) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`log_set_default_handler`] function.
#[doc(alias = "g_log_set_default_handler")]
pub fn log_unset_default_handler() {
    *default_handler()
        .lock()
        .expect("Failed to lock DEFAULT_HANDLER to remove callback") = None;
    unsafe {
        ffi::g_log_set_default_handler(Some(ffi::g_log_default_handler), std::ptr::null_mut())
    };
}

#[doc(alias = "g_log_default_handler")]
pub fn log_default_handler(log_domain: Option<&str>, log_level: LogLevel, message: Option<&str>) {
    unsafe {
        ffi::g_log_default_handler(
            log_domain.to_glib_none().0,
            log_level.into_glib(),
            message.to_glib_none().0,
            std::ptr::null_mut(),
        )
    }
}

// rustdoc-stripper-ignore-next
/// Structure representing a single field in a structured log entry.
///
/// See [`g_log_structured`][gls] for details. Log fields may contain UTF-8 strings, binary with
/// embedded nul bytes, or arbitrary pointers.
///
/// [gls]: https://docs.gtk.org/glib/func.log_structured.html
#[repr(transparent)]
#[derive(Debug)]
#[doc(alias = "GLogField")]
pub struct LogField<'a>(ffi::GLogField, std::marker::PhantomData<&'a GStr>);

impl<'a> LogField<'a> {
    // rustdoc-stripper-ignore-next
    /// Creates a field from a borrowed key and value.
    pub fn new(key: &'a GStr, value: &[u8]) -> Self {
        let (value, length) = if value.is_empty() {
            // Use an empty C string to represent empty data, since length: 0 is reserved for user
            // data fields.
            (&[0u8] as &[u8], -1isize)
        } else {
            (value, value.len().try_into().unwrap())
        };
        Self(
            ffi::GLogField {
                key: key.as_ptr(),
                value: value.as_ptr() as *const _,
                length,
            },
            Default::default(),
        )
    }
    // rustdoc-stripper-ignore-next
    /// Creates a field with an empty value and `data` as a user data key. Fields created with this
    /// function are ignored by the default log writer. These fields are used to pass custom data
    /// into a writer function set with [`log_set_writer_func`], where it can be retrieved using
    /// [`Self::user_data`].
    ///
    /// The passed `usize` can be used by the log writer as a key into a static data structure.
    /// Thread locals are preferred since the log writer function will run in the same thread that
    /// invoked [`log_structured_array`].
    pub fn new_user_data(key: &'a GStr, data: usize) -> Self {
        Self(
            ffi::GLogField {
                key: key.as_ptr(),
                value: data as *const _,
                length: 0,
            },
            Default::default(),
        )
    }
    // rustdoc-stripper-ignore-next
    /// Retrieves the field key.
    pub fn key(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.0.key as *const _) }
            .to_str()
            .unwrap()
    }
    // rustdoc-stripper-ignore-next
    /// Retrieves a byte array of the field value. Returns `None` if the field was created with
    /// [`Self::new_user_data`].
    pub fn value_bytes(&self) -> Option<&[u8]> {
        match self.0.length {
            0 => None,
            n if n < 0 => {
                Some(unsafe { std::ffi::CStr::from_ptr(self.0.value as *const _) }.to_bytes())
            }
            _ => Some(unsafe {
                std::slice::from_raw_parts(self.0.value as *const u8, self.0.length as usize)
            }),
        }
    }
    // rustdoc-stripper-ignore-next
    /// Retrieves a string of the field value, or `None` if the string is not valid UTF-8. Also
    /// returns `None` if the field was created with [`Self::new_user_data`].
    pub fn value_str(&self) -> Option<&str> {
        std::str::from_utf8(self.value_bytes()?).ok()
    }
    // rustdoc-stripper-ignore-next
    /// Retrieves the the user data value from a field created with [`Self::new_user_data`].
    /// Returns `None` if the field was created with [`Self::new`].
    pub fn user_data(&self) -> Option<usize> {
        (self.0.length == 0).then_some(self.0.value as usize)
    }
}

type WriterCallback = dyn Fn(LogLevel, &[LogField<'_>]) -> LogWriterOutput + Send + Sync + 'static;

static WRITER_FUNC: OnceLock<Box<WriterCallback>> = OnceLock::new();

#[doc(alias = "g_log_set_writer_func")]
pub fn log_set_writer_func<
    P: Fn(LogLevel, &[LogField<'_>]) -> LogWriterOutput + Send + Sync + 'static,
>(
    writer_func: P,
) {
    if WRITER_FUNC.set(Box::new(writer_func)).is_err() {
        panic!("Writer func can only be set once");
    }
    unsafe extern "C" fn writer_trampoline(
        log_level: ffi::GLogLevelFlags,
        fields: *const ffi::GLogField,
        n_fields: libc::size_t,
        _user_data: ffi::gpointer,
    ) -> ffi::GLogWriterOutput {
        let writer_func = WRITER_FUNC.get().unwrap();
        let fields = std::slice::from_raw_parts(fields as *const LogField<'_>, n_fields);
        writer_func(from_glib(log_level), fields).into_glib()
    }
    unsafe {
        ffi::g_log_set_writer_func(Some(writer_trampoline), std::ptr::null_mut(), None);
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! g_log_inner {
    ($log_domain:expr, $log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        let mut w = $crate::GStringBuilder::default();

        // Can't really happen but better safe than sorry
        if !std::fmt::Write::write_fmt(&mut w, std::format_args!($format, $($arg),*)).is_err() {
            unsafe {
                $crate::ffi::g_log(
                    $crate::translate::ToGlibPtr::to_glib_none(&$log_domain).0,
                    <$crate::LogLevel as $crate::translate::IntoGlib>::into_glib(
                        $log_level
                    ),
                    b"%s\0".as_ptr() as *const _,
                    $crate::translate::ToGlibPtr::<*const std::os::raw::c_char>::to_glib_none(
                        &w.into_string()
                    ).0,
                );
            }
        }
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// Example:
///
/// ```no_run
/// use glib::{LogLevel, g_log};
///
/// g_log!("test", LogLevel::Debug, "test");
/// g_log!("test", LogLevel::Message, "test");
/// // trailing commas work as well:
/// g_log!("test", LogLevel::Message, "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_log!("test", LogLevel::Error, "test: {}", x);
/// g_log!("test", LogLevel::Critical, "test: {}", x);
/// g_log!("test", LogLevel::Warning, "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_log!("test", LogLevel::Warning, "test: {} {}", x, "a",);
/// ```
///
/// To be noted that the log domain is optional:
///
/// ```no_run
/// use glib::{LogLevel, g_log};
///
/// // As you can see: no log domain:
/// g_log!(LogLevel::Message, "test");
/// // For the rest, it's just like when you have the log domain:
/// // trailing commas:
/// g_log!(LogLevel::Message, "test",);
///
/// // formatting:
/// let x = 12;
/// g_log!(LogLevel::Warning, "test: {} {}", x, "a");
/// g_log!(LogLevel::Warning, "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_log {
    ($log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        $crate::g_log_inner!(None::<&str>, $log_level, $format, $($arg),*);
    }};
    ($log_domain:expr, $log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        let log_domain = <Option<&str> as std::convert::From<_>>::from($log_domain);
        $crate::g_log_inner!(log_domain, $log_level, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Error`].
///
/// Example:
///
/// ```no_run
/// use glib::g_error;
///
/// g_error!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Error, "test");
///
/// // trailing commas work as well:
/// g_error!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_error!("test", "test: {}", x);
/// g_error!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_error!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_error {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Error, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Error, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Critical`].
///
/// Example:
///
/// ```no_run
/// use glib::g_critical;
///
/// g_critical!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Critical, "test");
///
/// // trailing commas work as well:
/// g_critical!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_critical!("test", "test: {}", x);
/// g_critical!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_critical!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_critical {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Critical, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Critical, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Warning`].
///
/// Example:
///
/// ```no_run
/// use glib::g_warning;
///
/// g_warning!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Warning, "test");
///
/// // trailing commas work as well:
/// g_warning!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_warning!("test", "test: {}", x);
/// g_warning!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_warning!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_warning {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Warning, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Warning, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Message`].
///
/// Example:
///
/// ```no_run
/// use glib::g_message;
///
/// g_message!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Message, "test");
///
/// // trailing commas work as well:
/// g_message!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_message!("test", "test: {}", x);
/// g_message!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_message!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_message {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Message, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Message, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Info`].
///
/// Example:
///
/// ```no_run
/// use glib::g_info;
///
/// g_info!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Info, "test");
///
/// // trailing commas work as well:
/// g_info!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_info!("test", "test: {}", x);
/// g_info!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_info!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_info {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Info, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Info, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://docs.gtk.org/glib/func.log.html
///
/// It is the same as calling the [`g_log!`](crate::g_log!) macro with [`LogLevel::Debug`].
///
/// Example:
///
/// ```no_run
/// use glib::g_debug;
///
/// g_debug!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Debug, "test");
///
/// // trailing commas work as well:
/// g_debug!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_debug!("test", "test: {}", x);
/// g_debug!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_debug!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_debug {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Debug, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Debug, $format);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! g_print_inner {
    ($func:ident, $format:expr $(, $arg:expr)* $(,)?) => {{
        let mut w = $crate::GStringBuilder::default();

        // Can't really happen but better safe than sorry
        if !std::fmt::Write::write_fmt(&mut w, std::format_args!($format, $($arg),*)).is_err() {
            unsafe {
                $crate::ffi::$func(
                    b"%s\0".as_ptr() as *const _,
                    $crate::translate::ToGlibPtr::<*const std::os::raw::c_char>::to_glib_none(
                        &w.into_string()
                    ).0,
                );
            }
        }
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to print messages. It uses [g_print].
///
/// [g_print]: https://docs.gtk.org/glib/func.print.html
///
/// Example:
///
/// ```no_run
/// use glib::g_print;
///
/// g_print!("test");
/// // trailing commas work as well:
/// g_print!("test",);
///
/// let x = 12;
/// g_print!("test: {}", x);
/// g_print!("test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_print!("test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_print {
    ($format:expr $(,$arg:expr)* $(,)?) => {{
        $crate::g_print_inner!(g_print, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to print error messages. It uses [g_printerr].
///
/// [g_printerr]: https://docs.gtk.org/glib/func.printerr.html
///
/// Example:
///
/// ```no_run
/// use glib::g_printerr;
///
/// g_printerr!("test");
/// // trailing commas work as well:
/// g_printerr!("test",);
///
/// let x = 12;
/// g_printerr!("test: {}", x);
/// g_printerr!("test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_printerr!("test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_printerr {
    ($format:expr $(, $arg:expr)* $(,)?) => {{
        $crate::g_print_inner!(g_printerr, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib structured logging system.
///
/// The structured data is provided inside braces as key-value pairs using the `=>` token and
/// separated by semicolons. The key can be a string literal or an expression that satisfies
/// [`AsRef<GStr>`]. The value can be a format string with arguments, or a single expression that
/// satisfies `AsRef<[u8]>`.
///
/// See [`g_log_structured`][gls] for more details.
///
/// [gls]: https://docs.gtk.org/glib/func.log_structured.html
/// [`AsRef<GStr>`]: crate::GStr
///
/// Example:
///
/// ```no_run
/// use glib::{GString, LogLevel, log_structured};
/// use std::ffi::CString;
///
/// log_structured!(
///     "test",
///     LogLevel::Debug,
///     {
///         // a normal string field
///         "MY_FIELD" => "123";
///         // fields can also take format arguments
///         "MY_FIELD2" => "abc {}", "def";
///         // single argument can be a &str or a &[u8] or anything else satisfying AsRef<[u8]>
///         "MY_FIELD3" => CString::new("my string").unwrap().to_bytes();
///         // field names can also be dynamic
///         GString::from("MY_FIELD4") => b"a binary string".to_owned();
///         // the main log message goes in the MESSAGE field
///         "MESSAGE" => "test: {} {}", 1, 2, ;
///     }
/// );
/// ```
#[macro_export]
macro_rules! log_structured {
    ($log_domain:expr, $log_level:expr, {$($key:expr => $format:expr $(,$arg:expr)* $(,)?);+ $(;)?} $(,)?) => {{
        let log_domain = <Option<&str> as std::convert::From<_>>::from($log_domain);
        let log_domain_str = log_domain.unwrap_or_default();
        let level: $crate::LogLevel = $log_level;
        let field_count =
            <[()]>::len(&[$($crate::log_structured_inner!(@clear $key)),+])
            + log_domain.map(|_| 2usize).unwrap_or(1usize)
            + 3;

        let mut line = [0u8; 32]; // 32 decimal digits of line numbers should be enough!
        let line = {
            use std::io::Write;

            let mut cursor = std::io::Cursor::new(&mut line[..]);
            std::write!(&mut cursor, "{}", line!()).unwrap();
            let pos = cursor.position() as usize;
            &line[..pos]
        };

        $crate::log_structured_array(
            level,
            &[
                $crate::LogField::new(
                    $crate::gstr!("PRIORITY"),
                    level.priority().as_bytes(),
                ),
                $crate::LogField::new(
                    $crate::gstr!("CODE_FILE"),
                    file!().as_bytes(),
                ),
                $crate::LogField::new(
                    $crate::gstr!("CODE_LINE"),
                    line,
                ),
                $crate::LogField::new(
                    $crate::gstr!("CODE_FUNC"),
                    $crate::function_name!().as_bytes(),
                ),
                $(
                    $crate::LogField::new(
                        $crate::log_structured_inner!(@key $key),
                        $crate::log_structured_inner!(@value $format $(,$arg)*),
                    ),
                )+
                $crate::LogField::new(
                    $crate::gstr!("GLIB_DOMAIN"),
                    log_domain_str.as_bytes(),
                ),
            ][0..field_count],
        )
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! log_structured_inner {
    (@clear $($_:tt)*) => { () };
    (@key $key:literal) => { $crate::gstr!($key) };
    (@key $key:expr) => { std::convert::AsRef::<$crate::GStr>::as_ref(&$key) };
    (@value $value:expr) => { std::convert::AsRef::<[u8]>::as_ref(&$value) };
    (@value $format:expr $(,$arg:expr)+) => {
        {
            let mut builder = $crate::GStringBuilder::default();
            if std::fmt::Write::write_fmt(&mut builder, format_args!($format, $($arg),+)).is_err() {
                return;
            }
            builder.into_string()
        }.as_str().as_bytes()
    };
}

#[doc(alias = "g_log_structured_array")]
#[inline]
pub fn log_structured_array(log_level: LogLevel, fields: &[LogField<'_>]) {
    unsafe {
        ffi::g_log_structured_array(
            log_level.into_glib(),
            fields.as_ptr() as *const ffi::GLogField,
            fields.len(),
        )
    }
}

#[doc(alias = "g_log_variant")]
#[inline]
pub fn log_variant(log_domain: Option<&str>, log_level: LogLevel, fields: &crate::Variant) {
    unsafe {
        ffi::g_log_variant(
            log_domain.to_glib_none().0,
            log_level.into_glib(),
            fields.to_glib_none().0,
        );
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_log_writer_supports_color")]
#[inline]
pub fn log_writer_supports_color(output_fd: impl AsFd) -> bool {
    unsafe {
        from_glib(ffi::g_log_writer_supports_color(
            output_fd.as_fd().as_raw_fd(),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_log_writer_is_journald")]
#[inline]
pub fn log_writer_is_journald(output_fd: impl AsFd) -> bool {
    unsafe { from_glib(ffi::g_log_writer_is_journald(output_fd.as_fd().as_raw_fd())) }
}

#[doc(alias = "g_log_writer_format_fields")]
#[inline]
pub fn log_writer_format_fields(
    log_level: LogLevel,
    fields: &[LogField<'_>],
    use_color: bool,
) -> GString {
    unsafe {
        from_glib_full(ffi::g_log_writer_format_fields(
            log_level.into_glib(),
            fields.as_ptr() as *const ffi::GLogField,
            fields.len(),
            use_color.into_glib(),
        ))
    }
}

#[doc(alias = "g_log_writer_journald")]
#[inline]
pub fn log_writer_journald(log_level: LogLevel, fields: &[LogField<'_>]) -> LogWriterOutput {
    unsafe {
        from_glib(ffi::g_log_writer_journald(
            log_level.into_glib(),
            fields.as_ptr() as *const ffi::GLogField,
            fields.len(),
            std::ptr::null_mut(),
        ))
    }
}

#[doc(alias = "g_log_writer_standard_streams")]
#[inline]
pub fn log_writer_standard_streams(
    log_level: LogLevel,
    fields: &[LogField<'_>],
) -> LogWriterOutput {
    unsafe {
        from_glib(ffi::g_log_writer_standard_streams(
            log_level.into_glib(),
            fields.as_ptr() as *const ffi::GLogField,
            fields.len(),
            std::ptr::null_mut(),
        ))
    }
}

#[doc(alias = "g_log_writer_default")]
#[inline]
pub fn log_writer_default(log_level: LogLevel, fields: &[LogField<'_>]) -> LogWriterOutput {
    unsafe {
        from_glib(ffi::g_log_writer_default(
            log_level.into_glib(),
            fields.as_ptr() as *const ffi::GLogField,
            fields.len(),
            std::ptr::null_mut(),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Sets whether GLib log functions output to stderr or stdout.
///
/// By default, log messages of level [`LogLevel::Info`] and [`LogLevel::Debug`] are sent to stdout,
/// and other log messages are sent to stderr. Passing `true` will send all messages to stderr.
///
/// # Safety
///
/// This function sets global state and is not thread-aware, as such it should be called before any
/// threads may try to use GLib logging.
#[cfg(feature = "v2_68")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
#[doc(alias = "g_log_writer_default_set_use_stderr")]
#[inline]
pub unsafe fn log_writer_default_set_use_stderr(use_stderr: bool) {
    ffi::g_log_writer_default_set_use_stderr(use_stderr.into_glib());
}

#[cfg(feature = "v2_68")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
#[doc(alias = "g_log_writer_default_would_drop")]
#[inline]
pub fn log_writer_default_would_drop(log_level: LogLevel, log_domain: Option<&str>) -> bool {
    unsafe {
        from_glib(ffi::g_log_writer_default_would_drop(
            log_level.into_glib(),
            log_domain.to_glib_none().0,
        ))
    }
}
