/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use [`io::stdout().flush()`][flush] to ensure the output is emitted
/// immediately.
///
/// Use `print!` only for the primary output of your program. Use
/// [`eprint!`] instead to print error and progress messages.
///
/// [`println!`]: macro.println.html
/// [flush]: io/trait.Write.html#tymethod.flush
/// [`eprint!`]: macro.eprint.html
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
/// use async_std::print;
///
/// print!("this ").await;
/// print!("will ").await;
/// print!("be ").await;
/// print!("on ").await;
/// print!("the ").await;
/// print!("same ").await;
/// print!("line ").await;
///
/// io::stdout().flush().await.unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n").await;
///
/// io::stdout().flush().await.unwrap();
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)))
}

/// Prints to the standard output, with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// Use the [`format!`] syntax to write data to the standard output.
/// See [`std::fmt`] for more information.
///
/// Use `println!` only for the primary output of your program. Use
/// [`eprintln!`] instead to print error and progress messages.
///
/// [`format!`]: macro.format.html
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`eprintln!`]: macro.eprintln.html
/// # Panics
///
/// Panics if writing to `io::stdout` fails.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::println;
///
/// println!().await; // prints just a newline
/// println!("hello there!").await;
/// println!("format {} arguments", "some").await;
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (async {
        $crate::io::_print(format_args!($($arg)*)).await;
        $crate::io::_print(format_args!("\n")).await;
    })
}

/// Prints to the standard error.
///
/// Equivalent to the [`print!`] macro, except that output goes to
/// [`io::stderr`] instead of `io::stdout`. See [`print!`] for
/// example usage.
///
/// Use `eprint!` only for error and progress messages. Use `print!`
/// instead for the primary output of your program.
///
/// [`io::stderr`]: io/struct.Stderr.html
/// [`print!`]: macro.print.html
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::eprint;
///
/// eprint!("Error: Could not complete task").await;
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::io::_eprint(format_args!($($arg)*)))
}

/// Prints to the standard error, with a newline.
///
/// Equivalent to the [`println!`] macro, except that output goes to
/// [`io::stderr`] instead of `io::stdout`. See [`println!`] for
/// example usage.
///
/// Use `eprintln!` only for error and progress messages. Use `println!`
/// instead for the primary output of your program.
///
/// [`io::stderr`]: io/struct.Stderr.html
/// [`println!`]: macro.println.html
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::eprintln;
///
/// eprintln!("Error: Could not complete task").await;
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[macro_export]
macro_rules! eprintln {
    () => (async { $crate::eprint!("\n").await; });
    ($($arg:tt)*) => (
        async {
            $crate::io::_eprint(format_args!($($arg)*)).await;
            $crate::io::_eprint(format_args!("\n")).await;
        }
    );
}

/// Declares task-local values.
///
/// The macro wraps any number of static declarations and makes them task-local. Attributes and
/// visibility modifiers are allowed.
///
/// Each declared value is of the accessor type [`LocalKey`].
///
/// [`LocalKey`]: task/struct.LocalKey.html
///
/// # Examples
///
/// ```
/// #
/// use std::cell::Cell;
///
/// use async_std::prelude::*;
/// use async_std::task;
///
/// task_local! {
///     static VAL: Cell<u32> = Cell::new(5);
/// }
///
/// task::block_on(async {
///     let v = VAL.with(|c| c.get());
///     assert_eq!(v, 5);
/// });
/// ```
#[cfg(feature = "default")]
#[macro_export]
macro_rules! task_local {
    () => ();

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => (
        $(#[$attr])* $vis static $name: $crate::task::LocalKey<$t> = {
            #[inline]
            fn __init() -> $t {
                $init
            }

            $crate::task::LocalKey {
                __init,
                __key: ::std::sync::atomic::AtomicU32::new(0),
            }
        };
    );

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
        $crate::task_local!($(#[$attr])* $vis static $name: $t = $init);
        $crate::task_local!($($rest)*);
    );
}
