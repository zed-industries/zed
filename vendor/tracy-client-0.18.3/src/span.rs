use crate::{adjust_stack_depth, Client};
use std::ffi::CString;

/// A handle representing a span of execution.
///
/// The trace span will be ended when this type is dropped.
pub struct Span {
    #[cfg(feature = "enable")]
    client: Client,
    #[cfg(feature = "enable")]
    zone: sys::___tracy_c_zone_context,
    #[cfg(feature = "enable")]
    _no_send_sync: std::marker::PhantomData<*mut sys::___tracy_c_zone_context>,
    #[cfg(not(feature = "enable"))]
    _no_send_sync: std::marker::PhantomData<*mut ()>,
}

/// A statically allocated location information for a span.
///
/// Construct with the [`span_location!`](crate::span_location) macro.
pub struct SpanLocation {
    #[cfg(feature = "enable")]
    pub(crate) _function_name: CString,
    #[cfg(feature = "enable")]
    pub(crate) data: sys::___tracy_source_location_data,
    #[cfg(not(feature = "enable"))]
    pub(crate) _internal: (),
}

unsafe impl Send for SpanLocation {}
unsafe impl Sync for SpanLocation {}

/// Instrumentation for timed regions, spans or zones of execution.
impl Client {
    /// Start a new Tracy span/zone.
    ///
    /// In order to obtain a [`SpanLocation`] value to provide to this function use the
    /// [`span_location!`](crate::span_location) macro.
    ///
    /// Specifying a non-zero `callstack_depth` will enable collection of callstack for this
    /// message. The number provided will limit the number of call frames collected. Note that
    /// enabling callstack collection introduces a non-trivial amount of overhead to this call. On
    /// some systems this value may be clamped to a maximum value supported by the target.
    ///
    /// The [`span!`](crate::span!) macro is a convenience wrapper over this method.
    ///
    /// # Example
    ///
    /// In the following example the span is created with the location at which the
    /// `span_location!` macro appears and will measure the execution of the 100ms long sleep.
    ///
    /// ```rust
    /// use tracy_client::{Client, span_location};
    /// let client = Client::start();
    /// {
    ///     let _span = client.span(span_location!("sleeping"), 100);
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    /// } // _span ends
    /// ```
    #[inline]
    #[must_use]
    pub fn span(self, loc: &'static SpanLocation, callstack_depth: u16) -> Span {
        #[cfg(feature = "enable")]
        unsafe {
            let zone = if callstack_depth == 0 {
                sys::___tracy_emit_zone_begin(&loc.data, 1)
            } else {
                let stack_depth = adjust_stack_depth(callstack_depth).into();
                sys::___tracy_emit_zone_begin_callstack(&loc.data, stack_depth, 1)
            };
            Span {
                client: self,
                zone,
                _no_send_sync: std::marker::PhantomData,
            }
        }
        #[cfg(not(feature = "enable"))]
        Span {
            _no_send_sync: std::marker::PhantomData,
        }
    }

    /// Start a new Tracy span/zone.
    ///
    /// This function allocates the span information on the heap until it is read out by the
    /// profiler. Prefer the [`Client::span`] as a allocation-free and faster alternative when
    /// possible.
    ///
    /// Specifying a non-zero `callstack_depth` will enable collection of callstack for this
    /// message. The number provided will limit the number of call frames collected. Note that
    /// enabling callstack collection introduces a non-trivial amount of overhead to this call. On
    /// some systems this value may be clamped to a maximum value supported by the target.
    ///
    /// # Example
    ///
    /// In the following example the span is created with custom span source data and will measure
    /// the execution of the 100ms long sleep.
    ///
    /// ```rust
    /// use tracy_client::Client;
    /// let client = Client::start();
    /// {
    ///     let _span = client.span_alloc(Some("hello"), "my_function", "hello.rs", 42, 100);
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    /// } // _span ends
    /// ```
    #[inline]
    #[must_use]
    pub fn span_alloc(
        self,
        name: Option<&str>,
        function: &str,
        file: &str,
        line: u32,
        callstack_depth: u16,
    ) -> Span {
        #[cfg(feature = "enable")]
        unsafe {
            let loc = sys::___tracy_alloc_srcloc_name(
                line,
                file.as_ptr().cast(),
                file.len(),
                function.as_ptr().cast(),
                function.len(),
                name.map_or(std::ptr::null(), |n| n.as_ptr().cast()),
                name.unwrap_or("").len(),
                0,
            );
            let zone = if callstack_depth == 0 {
                sys::___tracy_emit_zone_begin_alloc(loc, 1)
            } else {
                let stack_depth = adjust_stack_depth(callstack_depth).into();
                sys::___tracy_emit_zone_begin_alloc_callstack(loc, stack_depth, 1)
            };
            Span {
                client: self,
                zone,
                _no_send_sync: std::marker::PhantomData,
            }
        }
        #[cfg(not(feature = "enable"))]
        Span {
            _no_send_sync: std::marker::PhantomData,
        }
    }
}

impl Span {
    /// Emit a numeric value associated with this span.
    pub fn emit_value(&self, value: u64) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
            let () = sys::___tracy_emit_zone_value(self.zone, value);
        }
    }

    /// Emit some text associated with this span.
    pub fn emit_text(&self, text: &str) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
            let () = sys::___tracy_emit_zone_text(self.zone, text.as_ptr().cast(), text.len());
        }
    }

    /// Emit a color associated with this span.
    ///
    /// The color is specified as RGB. It is most straightforward to specify them as hex literals
    /// such as `0xFF0000` for red, `0x00FF00` for green or `0x0000FF` for blue.
    pub fn emit_color(&self, color: u32) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
            // TODO: verify if we need to shift by 8 or not...?
            let () = sys::___tracy_emit_zone_color(self.zone, color);
        }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: The only way to construct `Span` is by creating a valid tracy zone context. We
            // also still have an owned Client handle.
            let () = sys::___tracy_emit_zone_end(self.zone);
            std::convert::identity(&self.client);
        }
    }
}

/// Construct a <code>&'static [SpanLocation]</code>.
///
/// The returned `SpanLocation` is allocated statically and is cached between invocations. This
/// `SpanLocation` will refer to the file and line at which this macro has been invoked, as well as
/// to the item containing this macro invocation.
///
/// The resulting value may be used as an argument for the [`Client::span`] method.
///
/// # Example
///
/// ```rust
/// let location: &'static tracy_client::SpanLocation = tracy_client::span_location!("some name");
/// ```
#[macro_export]
macro_rules! span_location {
    () => {{
        struct S;
        // String processing in `const` when, Oli?
        static LOC: $crate::internal::Lazy<$crate::internal::SpanLocation> =
            $crate::internal::Lazy::new(|| {
                $crate::internal::make_span_location(
                    $crate::internal::type_name::<S>(),
                    $crate::internal::null(),
                    concat!(file!(), "\0").as_ptr(),
                    line!(),
                )
            });
        &*LOC
    }};
    ($name: expr) => {{
        struct S;
        // String processing in `const` when, Oli?
        static LOC: $crate::internal::Lazy<$crate::internal::SpanLocation> =
            $crate::internal::Lazy::new(|| {
                $crate::internal::make_span_location(
                    $crate::internal::type_name::<S>(),
                    concat!($name, "\0").as_ptr(),
                    concat!(file!(), "\0").as_ptr(),
                    line!(),
                )
            });
        &*LOC
    }};
}

/// Start a new Tracy span with function, file, and line determined automatically.
///
/// # Panics
///
/// `span!` will panic if the Client isn't running at the time this macro is invoked.
///
/// # Examples
///
/// Begin a span region, which will be terminated once `_span` goes out of scope:
///
/// ```
/// use tracy_client::{Client, span};
/// # let _client = tracy_client::Client::start();
/// let _span = span!("some span");
/// ```
///
/// It is also possible to enable collection of the callstack by specifying a limit of call stack
/// frames to record:
///
/// ```
/// use tracy_client::span;
/// # let _client = tracy_client::Client::start();
/// let _span = span!("some span", 32);
/// ```
///
/// Note, however, that collecting callstack introduces a non-trivial overhead at the point of
/// instrumentation.
#[macro_export]
macro_rules! span {
    () => {
        $crate::Client::running()
            .expect("span! without a running Client")
            .span($crate::span_location!(), 0)
    };
    ($name: expr) => {
        $crate::span!($name, 0)
    };
    ($name: expr, $callstack_depth: expr) => {{
        let location = $crate::span_location!($name);
        $crate::Client::running()
            .expect("span! without a running Client")
            .span(location, $callstack_depth)
    }};
}
