// FIXME: this can be greatly simplified via $()?
// as soon as MRSV hits 1.32

/// Build [`Diagnostic`](struct.Diagnostic.html) instance from provided arguments.
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! diagnostic {
    // from alias
    ($err:expr) => { $crate::Diagnostic::from($err) };

    // span, message, help
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+ ; $($rest:tt)+) => {{
        #[allow(unused_imports)]
        use $crate::__export::{
            ToTokensAsSpanRange,
            Span2AsSpanRange,
            SpanAsSpanRange,
            SpanRangeAsSpanRange
        };
        use $crate::DiagnosticExt;
        let span_range = (&$span).FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange();

        let diag = $crate::Diagnostic::spanned_range(
            span_range,
            $level,
            format!($fmt, $($args),*)
        );
        $crate::__pme__suggestions!(diag $($rest)*);
        diag
    }};

    ($span:expr, $level:expr, $msg:expr ; $($rest:tt)+) => {{
        #[allow(unused_imports)]
        use $crate::__export::{
            ToTokensAsSpanRange,
            Span2AsSpanRange,
            SpanAsSpanRange,
            SpanRangeAsSpanRange
        };
        use $crate::DiagnosticExt;
        let span_range = (&$span).FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange();

        let diag = $crate::Diagnostic::spanned_range(span_range, $level, $msg.to_string());
        $crate::__pme__suggestions!(diag $($rest)*);
        diag
    }};

    // span, message, no help
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+) => {{
        #[allow(unused_imports)]
        use $crate::__export::{
            ToTokensAsSpanRange,
            Span2AsSpanRange,
            SpanAsSpanRange,
            SpanRangeAsSpanRange
        };
        use $crate::DiagnosticExt;
        let span_range = (&$span).FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange();

        $crate::Diagnostic::spanned_range(
            span_range,
            $level,
            format!($fmt, $($args),*)
        )
    }};

    ($span:expr, $level:expr, $msg:expr) => {{
        #[allow(unused_imports)]
        use $crate::__export::{
            ToTokensAsSpanRange,
            Span2AsSpanRange,
            SpanAsSpanRange,
            SpanRangeAsSpanRange
        };
        use $crate::DiagnosticExt;
        let span_range = (&$span).FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange();

        $crate::Diagnostic::spanned_range(span_range, $level, $msg.to_string())
    }};


    // trailing commas

    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+, ; $($rest:tt)+) => {
        $crate::diagnostic!($span, $level, $fmt, $($args),* ; $($rest)*)
    };
    ($span:expr, $level:expr, $msg:expr, ; $($rest:tt)+) => {
        $crate::diagnostic!($span, $level, $msg ; $($rest)*)
    };
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+,) => {
        $crate::diagnostic!($span, $level, $fmt, $($args),*)
    };
    ($span:expr, $level:expr, $msg:expr,) => {
        $crate::diagnostic!($span, $level, $msg)
    };
    // ($err:expr,) => { $crate::diagnostic!($err) };
}

/// Abort proc-macro execution right now and display the error.
///
/// # Syntax
///
/// See [the guide](index.html#guide).
#[macro_export]
macro_rules! abort {
    ($err:expr) => {
        $crate::diagnostic!($err).abort()
    };

    ($span:expr, $($tts:tt)*) => {
        $crate::diagnostic!($span, $crate::Level::Error, $($tts)*).abort()
    };
}

/// Shortcut for `abort!(Span::call_site(), msg...)`. This macro
/// is still preferable over plain panic, panics are not for error reporting.
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! abort_call_site {
    ($($tts:tt)*) => {
        $crate::abort!($crate::__export::proc_macro2::Span::call_site(), $($tts)*)
    };
}

/// Emit an error while not aborting the proc-macro right away.
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! emit_error {
    ($err:expr) => {
        $crate::diagnostic!($err).emit()
    };

    ($span:expr, $($tts:tt)*) => {{
        let level = $crate::Level::Error;
        $crate::diagnostic!($span, level, $($tts)*).emit()
    }};
}

/// Shortcut for `emit_error!(Span::call_site(), ...)`. This macro
/// is still preferable over plain panic, panics are not for error reporting..
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! emit_call_site_error {
    ($($tts:tt)*) => {
        $crate::emit_error!($crate::__export::proc_macro2::Span::call_site(), $($tts)*)
    };
}

/// Emit a warning. Warnings are not errors and compilation won't fail because of them.
///
/// **Does nothing on stable**
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! emit_warning {
    ($span:expr, $($tts:tt)*) => {
        $crate::diagnostic!($span, $crate::Level::Warning, $($tts)*).emit()
    };
}

/// Shortcut for `emit_warning!(Span::call_site(), ...)`.
///
/// **Does nothing on stable**
///
/// # Syntax
///
/// See [the guide](index.html#guide).
///
#[macro_export]
macro_rules! emit_call_site_warning {
    ($($tts:tt)*) => {{
        $crate::emit_warning!($crate::__export::proc_macro2::Span::call_site(), $($tts)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __pme__suggestions {
    ($var:ident) => ();

    ($var:ident $help:ident =? $msg:expr) => {
        let $var = if let Some(msg) = $msg {
            $var.suggestion(stringify!($help), msg.to_string())
        } else {
            $var
        };
    };
    ($var:ident $help:ident =? $span:expr => $msg:expr) => {
        let $var = if let Some(msg) = $msg {
            $var.span_suggestion($span.into(), stringify!($help), msg.to_string())
        } else {
            $var
        };
    };

    ($var:ident $help:ident =? $msg:expr ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help =? $msg);
        $crate::__pme__suggestions!($var $($rest)*);
    };
    ($var:ident $help:ident =? $span:expr => $msg:expr ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help =? $span => $msg);
        $crate::__pme__suggestions!($var $($rest)*);
    };


    ($var:ident $help:ident = $msg:expr) => {
        let $var = $var.suggestion(stringify!($help), $msg.to_string());
    };
    ($var:ident $help:ident = $fmt:expr, $($args:expr),+) => {
        let $var = $var.suggestion(
            stringify!($help),
            format!($fmt, $($args),*)
        );
    };
    ($var:ident $help:ident = $span:expr => $msg:expr) => {
        let $var = $var.span_suggestion($span.into(), stringify!($help), $msg.to_string());
    };
    ($var:ident $help:ident = $span:expr => $fmt:expr, $($args:expr),+) => {
        let $var = $var.span_suggestion(
            $span.into(),
            stringify!($help),
            format!($fmt, $($args),*)
        );
    };

    ($var:ident $help:ident = $msg:expr ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $msg);
        $crate::__pme__suggestions!($var $($rest)*);
    };
    ($var:ident $help:ident = $fmt:expr, $($args:expr),+ ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $fmt, $($args),*);
        $crate::__pme__suggestions!($var $($rest)*);
    };
    ($var:ident $help:ident = $span:expr => $msg:expr ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $span => $msg);
        $crate::__pme__suggestions!($var $($rest)*);
    };
    ($var:ident $help:ident = $span:expr => $fmt:expr, $($args:expr),+ ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $span => $fmt, $($args),*);
        $crate::__pme__suggestions!($var $($rest)*);
    };

    // trailing commas

    ($var:ident $help:ident = $msg:expr,) => {
        $crate::__pme__suggestions!($var $help = $msg)
    };
    ($var:ident $help:ident = $fmt:expr, $($args:expr),+,) => {
        $crate::__pme__suggestions!($var $help = $fmt, $($args)*)
    };
    ($var:ident $help:ident = $span:expr => $msg:expr,) => {
        $crate::__pme__suggestions!($var $help = $span => $msg)
    };
    ($var:ident $help:ident = $span:expr => $fmt:expr, $($args:expr),*,) => {
        $crate::__pme__suggestions!($var $help = $span => $fmt, $($args)*)
    };
    ($var:ident $help:ident = $msg:expr, ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $msg; $($rest)*)
    };
    ($var:ident $help:ident = $fmt:expr, $($args:expr),+, ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $fmt, $($args),*; $($rest)*)
    };
    ($var:ident $help:ident = $span:expr => $msg:expr, ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $span => $msg; $($rest)*)
    };
    ($var:ident $help:ident = $span:expr => $fmt:expr, $($args:expr),+, ; $($rest:tt)*) => {
        $crate::__pme__suggestions!($var $help = $span => $fmt, $($args),*; $($rest)*)
    };
}
