/// Forward selector and arguments to `MsgSend::send_message[_error]`.
///
/// Note: We can't forward to `msg_send!` since that doesn't support selectors
/// with space between.
#[doc(hidden)]
#[macro_export]
macro_rules! __method_msg_send {
    // Selector with no arguments
    (
        ($receiver:expr)
        ($sel:ident)
        ()

        ()
        ()
        ($($method_family:tt)*)
    ) => {
        $crate::__msg_send_helper! {
            ($receiver)
            ($($method_family)*)
            (MsgSend::send_message)
            ($sel)
            ()
        }
    };

    // Skip using `MainThreadMarker` in the message send.
    //
    // This is a purely textual match, and using e.g. `objc2::MainThreadMarker`
    // would fail - but that would just be detected as giving a wrong number
    // of arguments, so it's fine for now.
    (
        ($receiver:expr)
        ($($sel_rest:tt)*)
        ($arg:ident: MainThreadMarker $(, $($params_rest:tt)*)?)

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => ({
        let _ = $arg;
        $crate::__method_msg_send! {
            ($receiver)
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($sel_parsed)*)
            ($($arg_parsed)*)
            ($($method_family)*)
        }
    });

    // Parse each argument-selector pair
    (
        ($receiver:expr)
        ($($sel:ident)? : $($sel_rest:tt)*)
        ($arg:ident : $_arg_ty:ty $(, $($params_rest:tt)*)?)

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => {
        $crate::__method_msg_send! {
            ($receiver)
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($sel_parsed)* $($sel)? :)
            ($($arg_parsed)* $arg,)
            ($($method_family)*)
        }
    };
    // Handle path separator token
    (
        ($receiver:expr)
        ($($sel:ident)? :: $($sel_rest:tt)*)
        ($arg1:ident : $_arg_ty1:ty, $arg2:ident : $_arg_ty2:ty $(, $($params_rest:tt)*)?)

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => {
        $crate::__method_msg_send! {
            ($receiver)
            ($($sel_rest)*)
            ($($($params_rest)*)?)

            ($($sel_parsed)* $($sel)? : :)
            ($($arg_parsed)* $arg1, $arg2,)
            ($($method_family)*)
        }
    };

    // Normal return
    (
        ($receiver:expr)
        ()
        ()

        // Notice the "+" here; we must make sure we actually _did_ parse
        // a selector, and haven't just gotten an empty `#[unsafe(method())]`.
        ($($sel_parsed:tt)+)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => {
        $crate::__msg_send_helper! {
            ($receiver)
            ($($method_family)*)
            (MsgSend::send_message)
            ($($sel_parsed)*)
            ($($arg_parsed)*)
        }
    };

    // Error return
    (
        ($receiver:expr)
        // `sel:_` without a corresponding argument
        ($sel:ident : _)
        ()

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => {
        $crate::__msg_send_helper! {
            ($receiver)
            ($($method_family)*)
            // Use error method
            (MsgSendError::send_message_error)
            ($($sel_parsed)* $sel :)
            ($($arg_parsed)*)
        }
    };

    // Variadic method
    (
        ($receiver:expr)
        ($($sel:ident : _)?)
        ($($arg:ident :)? ...)

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => ({
        $crate::__macro_helpers::compile_error!(
            "variadic methods are not yet supported"
        )
    });

    // Mismatched selector/argument
    (
        ($receiver:expr)
        ($($sel_rest:tt)*)
        ($($params_rest:tt)*)

        ($($sel_parsed:tt)*)
        ($($arg_parsed:tt)*)
        ($($method_family:tt)*)
    ) => ({
        $crate::__macro_helpers::compile_error!(
            "number of arguments in function and selector did not match"
        )
    });
}
