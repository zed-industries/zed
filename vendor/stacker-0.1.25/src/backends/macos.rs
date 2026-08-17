pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    Some(
        libc::pthread_get_stackaddr_np(libc::pthread_self()) as usize
            - libc::pthread_get_stacksize_np(libc::pthread_self()) as usize,
    )
}
