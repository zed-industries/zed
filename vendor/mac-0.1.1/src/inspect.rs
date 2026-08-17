//! Inspect Macros

/// Evaluates an expression, prints a stringified version of the expression
/// along with the evaluated value, and then returns that value.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
///
/// # fn main() {
/// fn lcm_2_to_4() -> u32 {
///     let mut i = 1;
///     loop {
///         if inspect!(i % 2, i % 3, i % 4) == (0, 0, 0) {
///             return inspect!("done: i = " => i);
///         }
///         i += 1;
///     }
/// }
/// assert_eq!(lcm_2_to_4(), 12);
/// # }
/// ```
///
/// Returns `12`, and prints the following to stdout:
///
/// ```ignore
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 1, 1)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 2, 2)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 0, 3)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 1, 0)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 2, 1)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 0, 2)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 1, 3)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 2, 0)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 0, 1)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 1, 2)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (1, 2, 3)
/// src/inspect.rs:94 - (i % 2, i % 3, i % 4) = (0, 0, 0)
/// src/inspect.rs:95 - done: i = 12
/// ```

#[macro_export]
macro_rules! inspect {
    ($prefix:expr => $expr:expr) => {{
        let val = $expr;
        println!("{}:{} - {}{:?}", file!(), line!(), $prefix, val);
        val
    }};
    ($expr:expr) => {
        inspect!(concat!(stringify!($expr), " = ") => $expr)
    };
    ($prefix:expr => $($expr:expr),+) => {
        inspect!($prefix => ($($expr),+))
    };
    ($($expr:expr),+) => {
        inspect!(($($expr),+))
    };
}

#[test]
fn test_inspect() {
    assert_eq!(inspect!("foo"), "foo");
    assert_eq!(inspect!("" => "foo"), "foo");
    assert_eq!(inspect!(1 + 2, 2 + 3, 3 + 4), (3, 5, 7));
    assert_eq!(inspect!("" => 1 + 2, 2 + 3, 3 + 4), (3, 5, 7));

    fn fib(n: u64) -> u64 {
        inspect!("fib :: n = " => n);
        inspect! { "ret = " => match n {
            0 | 1 => n,
            n => fib(n-1) + fib(n-2)
        }}
    }

    fn fib_iter(n: u64) -> u64 {
        inspect!("fib_iter :: n = " => n);
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            inspect!(a, b);
            let tmp = b;
            b += a;
            a = tmp;
        }
        inspect!("ret = " => a)
    }

    assert_eq!(fib(4), 3);
    assert_eq!(fib_iter(7), 13);

    // Uncomment the following to see the output in `cargo test`.
    // panic!()
}
