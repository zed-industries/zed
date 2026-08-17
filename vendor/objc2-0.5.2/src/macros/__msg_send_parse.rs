#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_parse {
    // No arguments
    {
        ($error_fn:ident)
        // Intentionally empty
        ()
        ()
        ($selector:ident $(,)?)
        ($fn:ident)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($error_fn)
            ($selector)
            ()
            ()
            ($fn)

            ($out_macro)
            $($macro_args)*
        }
    };

    // tt-munch remaining `selector: argument` pairs, looking for a pattern
    // that ends with `sel: _`.
    {
        ($_error_fn:ident)
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ()
        ($fn:ident)

        ($out_macro:path)
        $($macro_args:tt)*
    } => ({
        $out_macro! {
            $($macro_args)*

            ($fn)
            ($($selector_output)*)
            ($($argument_output)*)
        }
    });
    {
        ($error_fn:ident)
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ($selector:ident: _ $(,)?)
        ($fn:ident)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($error_fn)
            ($($selector_output)* $selector:)
            // Don't pass an argument
            ($($argument_output)*)
            ()
            // Instead, we change the called function to the error function.
            ($error_fn)

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($error_fn:ident)
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ($selector:ident : $argument:expr $(, $($rest:tt)*)?)
        ($fn:ident)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($error_fn)
            ($($selector_output)* $selector:)
            ($($argument_output)* $argument,)
            ($($($rest)*)?)
            ($fn)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Handle calls without comma between `selector: argument` pair.
    {
        ($error_fn:ident)
        // Intentionally empty
        ()
        ()
        ($($selector:ident : $argument:expr)*)
        ($fn:ident)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {{
        $crate::__comma_between_args!(
            ($fn)

            ($(
                ", ",
                $crate::__macro_helpers::stringify!($selector),
                ": ",
                $crate::__macro_helpers::stringify!($argument),
            )+)

            $($macro_args)*
        );

        $crate::__msg_send_parse! {
            ($error_fn)
            ()
            ()
            ($($selector : $argument),*)
            ($fn)

            ($out_macro)
            $($macro_args)*
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-msg-send-always-comma"))]
macro_rules! __comma_between_args {
    ($($args:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "unstable-msg-send-always-comma")]
macro_rules! __comma_between_args {
    // msg_send!
    (
        (send_super_message_static)
        ($($args:tt)*)
        ($obj:expr)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send")
            ("super", $crate::__macro_helpers::stringify!(($obj)), $($args)*)
        }
    };
    (
        (send_super_message)
        ($($args:tt)*)
        ($obj:expr, $superclass:expr)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send")
            ("super", $crate::__macro_helpers::stringify!(($obj, $superclass)), $($args)*)
        }
    };
    (
        (send_message)
        ($($args:tt)*)
        ($obj:expr)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send")
            ($crate::__macro_helpers::stringify!($obj), $($args)*)
        }
    };
    // msg_send_id!
    (
        (send_super_message_id_static)
        ($($args:tt)*)
        ($obj:expr)
        ()
        (MsgSendSuperId)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send_id")
            ("super", $crate::__macro_helpers::stringify!(($obj)), $($args)*)
        }
    };
    (
        (send_super_message_id)
        ($($args:tt)*)
        ($obj:expr, $superclass:expr)
        ()
        (MsgSendSuperId)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send_id")
            ("super", $crate::__macro_helpers::stringify!(($obj, $superclass)), $($args)*)
        }
    };
    (
        (send_message_id)
        ($($args:tt)*)
        ($obj:expr)
        ()
        (MsgSendId)
    ) => {
        $crate::__comma_between_args_inner! {
            ("msg_send_id")
            ($crate::__macro_helpers::stringify!($obj), $($args)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "unstable-msg-send-always-comma")]
macro_rules! __comma_between_args_inner {
    (
        ($macro_name:literal)
        ($($args:tt)*)
    ) => {
        #[deprecated = $crate::__macro_helpers::concat!(
            "using ", $macro_name, "! without a comma between arguments is ",
            "technically not valid macro syntax, and may break in a future ",
            "version of Rust. You should use the following instead:\n",
            $macro_name, "![", $($args)* "]"
        )]
        #[inline]
        fn __msg_send_missing_comma() {}
        __msg_send_missing_comma();
    };
}
