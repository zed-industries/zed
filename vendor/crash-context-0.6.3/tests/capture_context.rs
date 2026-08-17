#[test]
fn captures() {
    fn one() {
        two();
    }

    fn two() {
        three();
    }

    #[allow(unsafe_code)]
    fn three() {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "windows")] {
                let ctx = unsafe {
                    let mut ctx = std::mem::MaybeUninit::zeroed();
                    crash_context::capture_context(ctx.as_mut_ptr());

                    ctx.assume_init()
                };

                cfg_if::cfg_if! {
                    if #[cfg(target_arch = "x86_64")] {
                        assert!(ctx.Rbp != 0);
                        assert!(ctx.Rsp != 0);
                        assert!(ctx.Rip != 0);
                    } else if #[cfg(target_arch = "x86")] {
                        assert!(ctx.Ebp != 0);
                        assert!(ctx.Esp != 0);
                        assert!(ctx.Eip != 0);
                    }
                }
            } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                let ctx = unsafe {
                    let mut ctx = std::mem::MaybeUninit::zeroed();
                    assert_eq!(crash_context::crash_context_getcontext(ctx.as_mut_ptr()), 0);
                    ctx.assume_init()
                };

                let gregs = &ctx.uc_mcontext.gregs;
                assert!(gregs[libc::REG_RBP as usize] != 0);
                assert!(gregs[libc::REG_RSP as usize] != 0);
                assert!(gregs[libc::REG_RIP as usize] != 0);
            }
        }
    }

    one();
}
