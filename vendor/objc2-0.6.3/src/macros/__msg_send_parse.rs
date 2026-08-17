#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_parse {
    // No arguments
    {
        // Intentionally empty
        ()
        ()
        ($selector:ident $(,)?)

        ($($error_data:tt)*)
        ($($data:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($selector)
            ()
            ()

            ($($error_data)*)
            ($($data)*)

            ($out_macro)
            $($macro_args)*
        }
    };

    // tt-munch remaining `selector: argument` pairs, looking for a pattern
    // that ends with `sel: _`.
    {
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ()

        ($($error_data:tt)*)
        ($($data:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => ({
        $out_macro! {
            $($macro_args)*

            ($($data)*)
            ($($selector_output)*)
            ($($argument_output)*)
        }
    });
    {
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ($selector:ident: _ $(,)?)

        ($($error_data:tt)*)
        ($($data:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($($selector_output)* $selector:)
            // Don't pass an argument
            ($($argument_output)*)
            ()

            // Instead, we change the data to the error data.
            ($($error_data)*)
            ($($error_data)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    {
        ($($selector_output:tt)*)
        ($($argument_output:tt)*)
        ($selector:ident : $argument:expr $(, $($rest:tt)*)?)

        ($($error_data:tt)*)
        ($($data:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__msg_send_parse! {
            ($($selector_output)* $selector:)
            ($($argument_output)* $argument,)
            ($($($rest)*)?)

            ($($error_data)*)
            ($($data)*)

            ($out_macro)
            $($macro_args)*
        }
    };

    // Handle calls without comma between `selector: argument` pair.
    {
        // Intentionally empty
        ()
        ()
        ($($selector:ident : $argument:expr)*)

        ($($error_data:tt)*)
        ($($data:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {{
        $crate::__missing_comma_between_args!(
            ($($data)*)

            ($(
                ", ",
                $crate::__macro_helpers::stringify!($selector),
                ": ",
                $crate::__macro_helpers::stringify!($argument),
            )+)

            $($macro_args)*
        );

        $crate::__msg_send_parse! {
            ()
            ()
            ($($selector : $argument),*)

            ($($error_data)*)
            ($($data)*)

            ($out_macro)
            $($macro_args)*
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __missing_comma_between_args {
    (
        (MsgSendSuper::send_super_message_static)
        ($($args:tt)*)
        ($obj:expr)
        ()
    ) => {
        $crate::__missing_comma_between_args_inner!(
            "super", $crate::__macro_helpers::stringify!(($obj)), $($args)*
        );
    };
    (
        (MsgSendSuper::send_super_message)
        ($($args:tt)*)
        ($obj:expr, $superclass:expr)
        ()
    ) => {
        $crate::__missing_comma_between_args_inner!(
            "super", $crate::__macro_helpers::stringify!(($obj, $superclass)), $($args)*
        );
    };
    (
        (MsgSend::send_message)
        ($($args:tt)*)
        ($obj:expr)
        ()
    ) => {
        $crate::__missing_comma_between_args_inner!(
            $crate::__macro_helpers::stringify!($obj), $($args)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __missing_comma_between_args_inner {
    ($($args:tt)*) => {{
        #[deprecated = $crate::__macro_helpers::concat!(
            "using msg_send! without a comma between arguments is ",
            "technically not valid macro syntax, and may break in a future ",
            "version of Rust. You should use the following instead:\n",
            "msg_send![", $($args)* "]"
        )]
        #[inline]
        fn __missing_comma() {}
        __missing_comma();
    }};
}
