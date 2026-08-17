pub mod io;

#[allow(unused)]
macro_rules! check {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with: {}", stringify!($e), e),
        }
    };
}

#[cfg(windows)]
#[allow(unused)]
macro_rules! error {
    ($e:expr, $s:expr) => {
        match $e {
            Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
            Err(ref err) => assert!(
                err.raw_os_error() == Some($s),
                format!("`{}` did not have a code of `{}`", err, $s)
            ),
        }
    };
}

#[cfg(any(unix, target_os = "wasi"))]
#[allow(unused)]
macro_rules! error {
    ($e:expr, $s:expr) => {
        error_contains!($e, $s)
    };
}

#[allow(unused)]
macro_rules! error_contains {
    ($e:expr, $s:expr) => {
        match $e {
            Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
            Err(ref err) => assert!(
                err.to_string().contains($s),
                format!("`{}` did not contain `{}`", err, $s)
            ),
        }
    };
}
