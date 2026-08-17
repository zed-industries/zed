// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "debug")]
#[doc(hidden)]
#[allow(unused)]
macro_rules! sysinfo_debug {
    ($($x:tt)*) => {{
        eprintln!($($x)*);
    }}
}

#[cfg(not(feature = "debug"))]
#[doc(hidden)]
#[allow(unused)]
macro_rules! sysinfo_debug {
    ($($x:tt)*) => {{}};
}

#[cfg(feature = "system")]
macro_rules! declare_signals {
    ($kind:ty, _ => None,) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[]
        }
    );

    ($kind:ty, $(Signal::$signal:ident => $map:expr,)+ _ => None,) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[$(Signal::$signal,)*]
        }

        #[inline]
        pub(crate) fn convert_signal(s: Signal) -> Option<$kind> {
            match s {
                $(Signal::$signal => Some($map),)*
                _ => None,
            }
        }
    );

    ($kind:ty, $(Signal::$signal:ident => $map:expr,)+) => (
        use crate::Signal;

        pub(crate) const fn supported_signals() -> &'static [Signal] {
            &[$(Signal::$signal,)*]
        }

        #[inline]
        pub(crate) fn convert_signal(s: Signal) -> Option<$kind> {
            match s {
                $(Signal::$signal => Some($map),)*
            }
        }
    )
}

#[cfg(all(unix, not(feature = "unknown-ci")))]
#[allow(unused_macros)]
macro_rules! retry_eintr {
    (set_to_0 => $($t:tt)+) => {{
        let errno = crate::unix::libc_errno();
        if !errno.is_null() {
            *errno = 0;
        }
        retry_eintr!($($t)+)
    }};
    ($errno_value:ident => $($t:tt)+) => {{
        loop {
            let ret = $($t)+;
            if ret < 0 {
                let tmp = std::io::Error::last_os_error();
                if tmp.kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                $errno_value = tmp.raw_os_error().unwrap_or(0);
            }
            break ret;
        }
    }};
    ($($t:tt)+) => {{
        loop {
            let ret = $($t)+;
            if ret < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            break ret;
        }
    }};
}

//FIXME: Remove this code if https://github.com/rust-lang/cfg-if/pull/78 is ever merged.
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    (
        $(
            if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        ) else+
        else { $( $e_tokens:tt )* }
    ) => {
        cfg_if! {
            @__items () ;
            $(
                (( $i_meta ) ( $( $i_tokens )* )) ,
            )+
            (() ( $( $e_tokens )* )) ,
        }
    };

    // Allow to multiple conditions in a same call.
    (
        $(
            if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        ) else+
        else { $( $e_tokens:tt )* }
        if $($extra_conditions:tt)+
    ) => {
        cfg_if! {
            @__items () ;
            $(
                (( $i_meta ) ( $( $i_tokens )* )) ,
            )+
            (() ( $( $e_tokens )* )) ,
        }
        cfg_if! {
            if $($extra_conditions)+
        }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* }
        )*
    ) => {
        cfg_if! {
            @__items () ;
            (( $i_meta ) ( $( $i_tokens )* )) ,
            $(
                (( $e_meta ) ( $( $e_tokens )* )) ,
            )*
        }
    };

    // Allow to multiple conditions in a same call.
    (
        if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* }
        )*
        if $($extra_conditions:tt)+
    ) => {
        cfg_if! {
            @__items () ;
            (( $i_meta ) ( $( $i_tokens )* )) ,
            $(
                (( $e_meta ) ( $( $e_tokens )* )) ,
            )*
        }
        cfg_if! {
            if $($extra_conditions)+
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the previous cfgs in a list at the beginning, so they can be
    // negated. After the semicolon is all the remaining items.
    (@__items ( $( $_:meta , )* ) ; ) => {};
    (
        @__items ( $( $no:meta , )* ) ;
        (( $( $yes:meta )? ) ( $( $tokens:tt )* )) ,
        $( $rest:tt , )*
    ) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$yes` matchers specified and must also negate
        // all previous matchers.
        #[cfg(all(
            $( $yes , )?
            not(any( $( $no ),* ))
        ))]
        cfg_if! { @__identity $( $tokens )* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$yes` matchers to the list of `$no` matchers as future emissions
        // will have to negate everything we just matched as well.
        cfg_if! {
            @__items ( $( $no , )* $( $yes , )? ) ;
            $( $rest , )*
        }
    };

    // Internal macro to make __apply work out right for different match types,
    // because of how macros match/expand stuff.
    (@__identity $( $tokens:tt )* ) => {
        $( $tokens )*
    };
}

#[cfg(test)]
#[allow(unexpected_cfgs)]
mod tests {
    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option2;
            fn works1() -> Option2<u32> { Some(1) }
        } else {
            fn works1() -> Option<u32> { None }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works2() -> bool { false }
        } else if #[cfg(test)] {
            fn works2() -> bool { true }
        } else {
            fn works2() -> bool { false }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works3() -> bool { false }
        } else {
            fn works3() -> bool { true }
        }
    }

    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option3;
            fn works4() -> Option3<u32> { Some(1) }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works5() -> bool { false }
        } else if #[cfg(test)] {
            fn works5() -> bool { true }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works6() -> bool { false }
        } else if #[cfg(test)] {
            fn works6() -> bool { true }
        }
        if #[cfg(test)] {
            fn works7() -> bool { true }
        } else {
            fn works7() -> bool { false }
        }
    }

    cfg_if! {
        if #[cfg(test)] {
            fn works8() -> bool { true }
        } else if #[cfg(foo)] {
            fn works8() -> bool { false }
        }
        if #[cfg(foo)] {
            fn works9() -> bool { false }
        } else if #[cfg(test)] {
            fn works9() -> bool { true }
        }
    }

    #[test]
    fn it_works() {
        assert!(works1().is_some());
        assert!(works2());
        assert!(works3());
        assert!(works4().is_some());
        assert!(works5());
        assert!(works6());
        assert!(works7());
        assert!(works8());
        assert!(works9());
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_usage_within_a_function() {
        cfg_if! {if #[cfg(debug_assertions)] {
            // we want to put more than one thing here to make sure that they
            // all get configured properly.
            assert!(cfg!(debug_assertions));
            assert_eq!(4, 2+2);
        } else {
            assert!(works1().is_some());
            assert_eq!(10, 5+5);
        }}
    }

    #[allow(dead_code)]
    trait Trait {
        fn blah(&self);
    }

    #[allow(dead_code)]
    struct Struct;

    impl Trait for Struct {
        cfg_if! {
            if #[cfg(feature = "blah")] {
                fn blah(&self) {
                    unimplemented!();
                }
            } else {
                fn blah(&self) {
                    unimplemented!();
                }
            }
        }
    }
}
