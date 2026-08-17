#[cfg(unix)]
#[test]
fn fd_value() {
    use std::os::fd::AsFd;
    use zvariant::{Basic, Fd, LE};

    #[macro_use]
    mod common {
        include!("common.rs");
    }

    let stdout = std::io::stdout();
    let fd = stdout.as_fd();
    fd_value_test!(LE, DBus, Fd::from(fd), 4, 4, 8);
    #[cfg(feature = "gvariant")]
    fd_value_test!(LE, GVariant, Fd::from(fd), 4, 4, 6);
}
