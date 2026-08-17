pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    let mut stackinfo = std::mem::MaybeUninit::<libc::stack_t>::uninit();
    let res = libc::pthread_stackseg_np(libc::pthread_self(), stackinfo.as_mut_ptr());
    if res != 0 {
        return None;
    }
    let stackinfo = stackinfo.assume_init();
    Some(stackinfo.ss_sp as usize - stackinfo.ss_size)
}
