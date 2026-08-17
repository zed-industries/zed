//-
// Copyright 2018, 2020 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support code for the `rusty_fork_test!` macro and similar.
//!
//! Some functionality in this module is useful to other implementors and
//! unlikely to change. This subset is documented and considered stable.

use std::process::Command;

use crate::child_wrapper::ChildWrapper;

/// Run Rust tests in subprocesses.
///
/// The basic usage is to simply put this macro around your `#[test]`
/// functions.
///
/// ```
/// use rusty_fork::rusty_fork_test;
///
/// rusty_fork_test! {
/// # /*
///     #[test]
/// # */
///     fn my_test() {
///         assert_eq!(2, 1 + 1);
///     }
///
///     // more tests...
/// }
/// #
/// # fn main() { my_test(); }
/// ```
///
/// Each test will be run in its own process. If the subprocess exits
/// unsuccessfully for any reason, including due to signals, the test fails.
///
/// It is also possible to specify a timeout which is applied to all tests in
/// the block, like so:
///
/// ```
/// use rusty_fork::rusty_fork_test;
///
/// rusty_fork_test! {
///     #![rusty_fork(timeout_ms = 1000)]
/// # /*
///     #[test]
/// # */
///     fn my_test() {
///         do_some_expensive_computation();
///     }
///
///     // more tests...
/// }
/// # fn do_some_expensive_computation() { }
/// # fn main() { my_test(); }
/// ```
///
/// If any individual test takes more than the given timeout, the child is
/// terminated and the test panics.
///
/// Using the timeout feature requires the `timeout` feature for this crate to
/// be enabled (which it is by default).
#[macro_export]
macro_rules! rusty_fork_test {
    (#![rusty_fork(timeout_ms = $timeout:expr)]
     $(
         $(#[$meta:meta])*
         fn $test_name:ident() $body:block
    )*) => { $(
        $(#[$meta])*
        fn $test_name() {
            // Eagerly convert everything to function pointers so that all
            // tests use the same instantiation of `fork`.
            fn body_fn() $body
            let body: fn () = body_fn;

            fn supervise_fn(child: &mut $crate::ChildWrapper,
                            _file: &mut ::std::fs::File) {
                $crate::fork_test::supervise_child(child, $timeout)
            }
            let supervise:
                fn (&mut $crate::ChildWrapper, &mut ::std::fs::File) =
                supervise_fn;

            $crate::fork(
                $crate::rusty_fork_test_name!($test_name),
                $crate::rusty_fork_id!(),
                $crate::fork_test::no_configure_child,
                supervise, body).expect("forking test failed")
        }
    )* };

    ($(
         $(#[$meta:meta])*
         fn $test_name:ident() $body:block
    )*) => {
        rusty_fork_test! {
            #![rusty_fork(timeout_ms = 0)]

            $($(#[$meta])* fn $test_name() $body)*
        }
    };
}

/// Given the unqualified name of a `#[test]` function, produce a
/// `&'static str` corresponding to the name of the test as filtered by the
/// standard test harness.
///
/// This is internally used by `rusty_fork_test!` but is made available since
/// other test wrapping implementations will likely need it too.
///
/// This does not currently produce a constant expression.
#[macro_export]
macro_rules! rusty_fork_test_name {
    ($function_name:ident) => {
        $crate::fork_test::fix_module_path(
            concat!(module_path!(), "::", stringify!($function_name)))
    }
}

#[allow(missing_docs)]
#[doc(hidden)]
pub fn supervise_child(child: &mut ChildWrapper, timeout_ms: u64) {
    if timeout_ms > 0 {
        wait_timeout(child, timeout_ms)
    } else {
        let status = child.wait().expect("failed to wait for child");
        assert!(status.success(),
                "child exited unsuccessfully with {}", status);
    }
}

#[allow(missing_docs)]
#[doc(hidden)]
pub fn no_configure_child(_child: &mut Command) { }

/// Transform a string representing a qualified path as generated via
/// `module_path!()` into a qualified path as expected by the standard Rust
/// test harness.
pub fn fix_module_path(path: &str) -> &str {
    path.find("::").map(|ix| &path[ix+2..]).unwrap_or(path)
}

#[cfg(feature = "timeout")]
fn wait_timeout(child: &mut ChildWrapper, timeout_ms: u64) {
    use std::time::Duration;

    let timeout = Duration::from_millis(timeout_ms);
    let status = child.wait_timeout(timeout).expect("failed to wait for child");
    if let Some(status) = status {
        assert!(status.success(),
                "child exited unsuccessfully with {}", status);
    } else {
        panic!("child process exceeded {} ms timeout", timeout_ms);
    }
}

#[cfg(not(feature = "timeout"))]
fn wait_timeout(_: &mut ChildWrapper, _: u64) {
    panic!("Using the timeout feature of rusty_fork_test! requires \
            enabling the `timeout` feature on the rusty-fork crate.");
}

#[cfg(test)]
mod test {
    rusty_fork_test! {
        #[test]
        fn trivial() { }

        #[test]
        #[should_panic]
        fn panicking_child() {
            panic!("just testing a panic, nothing to see here");
        }

        #[test]
        #[should_panic]
        fn aborting_child() {
            ::std::process::abort();
        }
    }

    rusty_fork_test! {
        #![rusty_fork(timeout_ms = 1000)]

        #[test]
        #[cfg(feature = "timeout")]
        fn timeout_passes() { }

        #[test]
        #[should_panic]
        #[cfg(feature = "timeout")]
        fn timeout_fails() {
            println!("hello from child");
            ::std::thread::sleep(
                ::std::time::Duration::from_millis(10000));
            println!("goodbye from child");
        }
    }
}
