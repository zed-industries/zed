/// Assert that two arguments refer to the same underlying file.
macro_rules! assert_same_file {
    ($left:expr, $right:expr) => ({
        let (left, right) = (&($left), &($right));
        if crate::rustix::fs::is_different_file(left, right).unwrap() {
            panic!("assertion failed: left is a different file from right\n  left: `{:?}`,\n right: `{:?}`",
                   left, right);
        }
    });
    ($left:expr, $right:expr, ) => ({
        assert_le!($left, $right);
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if crate::rustix::fs::is_different_file(left, right).unwrap() {
            panic!("assertion failed: left is a different file from right\n  left: `{:?}`,\n right: `{:?}`: {}",
                   left, right, format_args!($($msg_args)+));
        }
    })
}

/// Assert that two arguments are metadata for the same underlying file.
macro_rules! assert_same_file_metadata {
    ($left:expr, $right:expr) => ({
        let (left, right) = (&($left), &($right));
        if crate::rustix::fs::is_different_file_metadata(left, right).unwrap() {
            panic!("assertion failed: left is a different file from right\n  left: `{:?}`,\n right: `{:?}`",
                   left, right);
        }
    });
    ($left:expr, $right:expr, ) => ({
        assert_le!($left, $right);
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if crate::rustix::fs::is_different_file_metadata(left, right).unwrap() {
            panic!("assertion failed: left is a different file from right\n  left: `{:?}`,\n right: `{:?}`: {}",
                   left, right, format_args!($($msg_args)+));
        }
    })
}

/// Assert that two arguments refer to different underlying files.
#[allow(unused_macros)]
macro_rules! assert_different_file {
    ($left:expr, $right:expr) => ({
        let (left, right) = (&($left), &($right));
        if !crate::rustix::fs::is_different_file(left, right).unwrap() {
            panic!("assertion failed: left is the same file as right\n  left: `{:?}`,\n right: `{:?}`",
                   left, right);
        }
    });
    ($left:expr, $right:expr, ) => ({
        assert_le!($left, $right);
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !crate::rustix::fs::is_different_file(left, right).unwrap() {
            panic!("assertion failed: left is the same file as right\n  left: `{:?}`,\n right: `{:?}`: {}",
                   left, right, format_args!($($msg_args)+));
        }
    })
}

/// Assert that two arguments are metadata for the same underlying file.
#[allow(unused_macros)]
macro_rules! assert_different_metadata {
    ($left:expr, $right:expr) => ({
        let (left, right) = (&($left), &($right));
        if !crate::rustix::fs::is_different_file_metadata(left, right).unwrap() {
            panic!("assertion failed: left is the same file as right\n  left: `{:?}`,\n right: `{:?}`",
                   left, right);
        }
    });
    ($left:expr, $right:expr, ) => ({
        assert_le!($left, $right);
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !crate::rustix::fs::is_different_file_metadata(left, right).unwrap() {
            panic!("assertion failed: left is the same file as right\n  left: `{:?}`,\n right: `{:?}`: {}",
                   left, right, format_args!($($msg_args)+));
        }
    })
}
