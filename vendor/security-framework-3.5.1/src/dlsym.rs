// dlsym.rs is taken from mio
// https://github.com/carllerche/mio/blob/master/src/sys/unix/dlsym.rs

use std::marker;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};

use libc;

macro_rules! dlsym {
    (fn $name:ident($($t:ty),*) -> $ret:ty) => (
        #[allow(bad_style)]
        static $name: $crate::dlsym::DlSym<unsafe extern fn($($t),*) -> $ret> =
            $crate::dlsym::DlSym {
                name: concat!(stringify!($name), "\0"),
                addr: ::std::sync::atomic::AtomicUsize::new(0),
                _marker: ::std::marker::PhantomData,
            };
    )
}

pub struct DlSym<F> {
    pub name: &'static str,
    pub addr: AtomicUsize,
    pub _marker: marker::PhantomData<F>,
}

impl<F> DlSym<F> {
    pub fn get(&self) -> Option<&F> {
        assert_eq!(mem::size_of::<F>(), mem::size_of::<usize>());
        unsafe {
            if self.addr.load(Ordering::SeqCst) == 0 {
                self.addr.store(fetch(self.name), Ordering::SeqCst);
            }
            if self.addr.load(Ordering::SeqCst) == 1 {
                None
            } else {
                mem::transmute::<&AtomicUsize, Option<&F>>(&self.addr)
            }
        }
    }
}

unsafe fn fetch(name: &str) -> usize {
    assert_eq!(name.as_bytes()[name.len() - 1], 0);
    match libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr() as *const _) as usize {
        0 => 1,
        n => n,
    }
}
