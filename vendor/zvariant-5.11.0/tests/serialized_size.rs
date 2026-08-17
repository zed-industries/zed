use zvariant::{LE, serialized::Context, serialized_size};

#[cfg(unix)]
use zvariant::Fd;

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn test_serialized_size() {
    let ctxt = Context::new_dbus(LE, 0);
    let l = serialized_size(ctxt, &()).unwrap();
    assert_eq!(*l, 0);

    #[cfg(unix)]
    {
        let stdout = std::io::stdout();
        let l = serialized_size(ctxt, &Fd::from(&stdout)).unwrap();
        assert_eq!(*l, 4);
        assert_eq!(l.num_fds(), 1);
    }

    let l = serialized_size(ctxt, &('a', "abc", &(1_u32, 2))).unwrap();
    assert_eq!(*l, 24);

    let v = vec![1, 2];
    let l = serialized_size(ctxt, &('a', "abc", &v)).unwrap();
    assert_eq!(*l, 28);
}
