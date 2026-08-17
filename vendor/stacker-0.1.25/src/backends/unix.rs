#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "illumos"))]
use libc::pthread_attr_get_np as get_attr;
#[cfg(any(target_os = "linux", target_os = "solaris", target_os = "netbsd", target_os = "haiku"))]
use libc::pthread_getattr_np as get_attr;

pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    let mut attr = PthreadAttr::new()?;
    (get_attr(libc::pthread_self(), &mut attr.0) == 0).then_some(())?;
    let mut stackaddr = std::ptr::null_mut();
    let mut stacksize = 0;
    (libc::pthread_attr_getstack(&attr.0, &mut stackaddr, &mut stacksize) == 0).then_some(())?;
    Some(stackaddr as usize)
}

struct PthreadAttr(libc::pthread_attr_t);

impl Drop for PthreadAttr {
    fn drop(&mut self) {
        unsafe {
            let ret = libc::pthread_attr_destroy(&mut self.0);
            if ret != 0 {
                let err = std::io::Error::last_os_error();
                panic!(
                    "pthread_attr_destroy failed with error code {}: {}",
                    ret, err
                );
            }
        }
    }
}

impl PthreadAttr {
    unsafe fn new() -> Option<Self> {
        let mut attr = std::mem::MaybeUninit::<libc::pthread_attr_t>::uninit();
        if libc::pthread_attr_init(attr.as_mut_ptr()) != 0 {
            return None;
        }
        Some(PthreadAttr(attr.assume_init()))
    }
}
