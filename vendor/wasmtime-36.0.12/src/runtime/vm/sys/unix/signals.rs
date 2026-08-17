//! Trap handling on Unix based on POSIX signals.

use crate::prelude::*;
use crate::runtime::vm::sys::traphandlers::wasmtime_longjmp;
use crate::runtime::vm::traphandlers::{TrapRegisters, TrapTest, tls};
use std::cell::RefCell;
use std::io;
use std::mem;
use std::ptr::{self, null_mut};

/// Function which may handle custom signals while processing traps.
pub type SignalHandler =
    Box<dyn Fn(libc::c_int, *const libc::siginfo_t, *const libc::c_void) -> bool + Send + Sync>;

const UNINIT_SIGACTION: libc::sigaction = unsafe { mem::zeroed() };
static mut PREV_SIGSEGV: libc::sigaction = UNINIT_SIGACTION;
static mut PREV_SIGBUS: libc::sigaction = UNINIT_SIGACTION;
static mut PREV_SIGILL: libc::sigaction = UNINIT_SIGACTION;
static mut PREV_SIGFPE: libc::sigaction = UNINIT_SIGACTION;

pub struct TrapHandler;

impl TrapHandler {
    /// Installs all trap handlers.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because it's not safe to call concurrently and
    /// it's not safe to call if the trap handlers have already been initialized
    /// for this process.
    pub unsafe fn new(macos_use_mach_ports: bool) -> TrapHandler {
        // Either mach ports shouldn't be in use or we shouldn't be on macOS,
        // otherwise the `machports.rs` module should be used instead.
        assert!(!macos_use_mach_ports || !cfg!(target_vendor = "apple"));

        foreach_handler(|slot, signal| {
            let mut handler: libc::sigaction = unsafe { mem::zeroed() };
            // The flags here are relatively careful, and they are...
            //
            // SA_SIGINFO gives us access to information like the program
            // counter from where the fault happened.
            //
            // SA_ONSTACK allows us to handle signals on an alternate stack,
            // so that the handler can run in response to running out of
            // stack space on the main stack. Rust installs an alternate
            // stack with sigaltstack, so we rely on that.
            //
            // SA_NODEFER allows us to reenter the signal handler if we
            // crash while handling the signal, and fall through to the
            // Breakpad handler by testing handlingSegFault.
            handler.sa_flags = libc::SA_SIGINFO | libc::SA_NODEFER | libc::SA_ONSTACK;
            handler.sa_sigaction = trap_handler as usize;
            unsafe {
                libc::sigemptyset(&mut handler.sa_mask);
                if libc::sigaction(signal, &handler, slot) != 0 {
                    panic!(
                        "unable to install signal handler: {}",
                        io::Error::last_os_error(),
                    );
                }
            }
        });

        TrapHandler
    }

    pub fn validate_config(&self, macos_use_mach_ports: bool) {
        assert!(!macos_use_mach_ports || !cfg!(target_vendor = "apple"));
    }
}

fn foreach_handler(mut f: impl FnMut(*mut libc::sigaction, i32)) {
    // Allow handling OOB with signals on all architectures
    f(&raw mut PREV_SIGSEGV, libc::SIGSEGV);

    // Handle `unreachable` instructions which execute `ud2` right now
    f(&raw mut PREV_SIGILL, libc::SIGILL);

    // x86 and s390x use SIGFPE to report division by zero
    if cfg!(target_arch = "x86_64") || cfg!(target_arch = "s390x") {
        f(&raw mut PREV_SIGFPE, libc::SIGFPE);
    }

    // Sometimes we need to handle SIGBUS too:
    // - On Darwin, guard page accesses are raised as SIGBUS.
    if cfg!(target_vendor = "apple") || cfg!(target_os = "freebsd") {
        f(&raw mut PREV_SIGBUS, libc::SIGBUS);
    }

    // TODO(#1980): x86-32, if we support it, will also need a SIGFPE handler.
    // TODO(#1173): ARM32, if we support it, will also need a SIGBUS handler.
}

impl Drop for TrapHandler {
    fn drop(&mut self) {
        unsafe {
            foreach_handler(|slot, signal| {
                let mut prev: libc::sigaction = mem::zeroed();

                // Restore the previous handler that this signal had.
                if libc::sigaction(signal, slot, &mut prev) != 0 {
                    eprintln!(
                        "unable to reinstall signal handler: {}",
                        io::Error::last_os_error(),
                    );
                    libc::abort();
                }

                // If our trap handler wasn't currently listed for this process
                // then that's a problem because we have just corrupted the
                // signal handler state and don't know how to remove ourselves
                // from the signal handling state. Inform the user of this and
                // abort the process.
                if prev.sa_sigaction != trap_handler as usize {
                    eprintln!(
                        "
Wasmtime's signal handler was not the last signal handler to be installed
in the process so it's not certain how to unload signal handlers. In this
situation the Engine::unload_process_handlers API is not applicable and requires
perhaps initializing libraries in a different order. The process will be aborted
now.
"
                    );
                    libc::abort();
                }
            });
        }
    }
}

unsafe extern "C" fn trap_handler(
    signum: libc::c_int,
    siginfo: *mut libc::siginfo_t,
    context: *mut libc::c_void,
) {
    let previous = match signum {
        libc::SIGSEGV => &raw const PREV_SIGSEGV,
        libc::SIGBUS => &raw const PREV_SIGBUS,
        libc::SIGFPE => &raw const PREV_SIGFPE,
        libc::SIGILL => &raw const PREV_SIGILL,
        _ => panic!("unknown signal: {signum}"),
    };
    let handled = tls::with(|info| {
        // If no wasm code is executing, we don't handle this as a wasm
        // trap.
        let info = match info {
            Some(info) => info,
            None => return false,
        };

        // If we hit an exception while handling a previous trap, that's
        // quite bad, so bail out and let the system handle this
        // recursive segfault.
        //
        // Otherwise flag ourselves as handling a trap, do the trap
        // handling, and reset our trap handling flag. Then we figure
        // out what to do based on the result of the trap handling.
        let faulting_addr = match signum {
            libc::SIGSEGV | libc::SIGBUS => unsafe { Some((*siginfo).si_addr() as usize) },
            _ => None,
        };
        let regs = unsafe { get_trap_registers(context, signum) };
        let test = info.test_if_trap(regs, faulting_addr, |handler| {
            handler(signum, siginfo, context)
        });

        // Figure out what to do based on the result of this handling of
        // the trap. Note that our sentinel value of 1 means that the
        // exception was handled by a custom exception handler, so we
        // keep executing.
        let jmp_buf = match test {
            TrapTest::NotWasm => {
                if let Some(faulting_addr) = faulting_addr {
                    let range = unsafe { &info.vm_store_context.as_ref().async_guard_range };
                    if range.start.addr() <= faulting_addr && faulting_addr < range.end.addr() {
                        abort_stack_overflow();
                    }
                }
                return false;
            }
            TrapTest::HandledByEmbedder => return true,
            TrapTest::Trap { jmp_buf } => jmp_buf,
        };
        // On macOS this is a bit special, unfortunately. If we were to
        // `siglongjmp` out of the signal handler that notably does
        // *not* reset the sigaltstack state of our signal handler. This
        // seems to trick the kernel into thinking that the sigaltstack
        // is still in use upon delivery of the next signal, meaning
        // that the sigaltstack is not ever used again if we immediately
        // call `wasmtime_longjmp` here.
        //
        // Note that if we use `longjmp` instead of `siglongjmp` then
        // the problem is fixed. The problem with that, however, is that
        // `setjmp` is much slower than `sigsetjmp` due to the
        // preservation of the process's signal mask. The reason
        // `longjmp` appears to work is that it seems to call a function
        // (according to published macOS sources) called
        // `_sigunaltstack` which updates the kernel to say the
        // sigaltstack is no longer in use. We ideally want to call that
        // here but I don't think there's a stable way for us to call
        // that.
        //
        // Given all that, on macOS only, we do the next best thing. We
        // return from the signal handler after updating the register
        // context. This will cause control to return to our shim
        // function defined here which will perform the
        // `wasmtime_longjmp` (`siglongjmp`) for us. The reason this
        // works is that by returning from the signal handler we'll
        // trigger all the normal machinery for "the signal handler is
        // done running" which will clear the sigaltstack flag and allow
        // reusing it for the next signal. Then upon resuming in our custom
        // code we blow away the stack anyway with a longjmp.
        if cfg!(target_vendor = "apple") {
            unsafe extern "C" fn wasmtime_longjmp_shim(jmp_buf: *const u8) {
                unsafe { wasmtime_longjmp(jmp_buf) }
            }
            unsafe {
                set_pc(context, wasmtime_longjmp_shim as usize, jmp_buf as usize);
            }
            return true;
        }
        unsafe { wasmtime_longjmp(jmp_buf) }
    });

    if handled {
        return;
    }

    unsafe { delegate_signal_to_previous_handler(previous, signum, siginfo, context) }
}

pub unsafe fn delegate_signal_to_previous_handler(
    previous: *const libc::sigaction,
    signum: libc::c_int,
    siginfo: *mut libc::siginfo_t,
    context: *mut libc::c_void,
) {
    // This signal is not for any compiled wasm code we expect, so we
    // need to forward the signal to the next handler. If there is no
    // next handler (SIG_IGN or SIG_DFL), then it's time to crash. To do
    // this, we set the signal back to its original disposition and
    // return. This will cause the faulting op to be re-executed which
    // will crash in the normal way. If there is a next handler, call
    // it. It will either crash synchronously, fix up the instruction
    // so that execution can continue and return, or trigger a crash by
    // returning the signal to it's original disposition and returning.
    unsafe {
        let previous = *previous;
        if previous.sa_flags & libc::SA_SIGINFO != 0 {
            mem::transmute::<
                usize,
                extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void),
            >(previous.sa_sigaction)(signum, siginfo, context)
        } else if previous.sa_sigaction == libc::SIG_DFL || previous.sa_sigaction == libc::SIG_IGN {
            libc::sigaction(signum, &previous as *const _, ptr::null_mut());
        } else {
            mem::transmute::<usize, extern "C" fn(libc::c_int)>(previous.sa_sigaction)(signum)
        }
    }
}

pub fn abort_stack_overflow() -> ! {
    unsafe {
        let msg = "execution on async fiber has overflowed its stack";
        libc::write(libc::STDERR_FILENO, msg.as_ptr().cast(), msg.len());
        libc::abort();
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "too fiddly to handle and wouldn't help much anyway"
)]
unsafe fn get_trap_registers(cx: *mut libc::c_void, _signum: libc::c_int) -> TrapRegisters {
    cfg_if::cfg_if! {
        if #[cfg(all(any(target_os = "linux", target_os = "android", target_os = "illumos"), target_arch = "x86_64"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.gregs[libc::REG_RIP as usize] as usize,
                fp: cx.uc_mcontext.gregs[libc::REG_RBP as usize] as usize,
            }
        } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.gregs[libc::REG_EIP as usize] as usize,
                fp: cx.uc_mcontext.gregs[libc::REG_EBP as usize] as usize,
            }
        } else if #[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "aarch64"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.pc as usize,
                fp: cx.uc_mcontext.regs[29] as usize,
            }
        } else if #[cfg(all(target_os = "linux", target_arch = "s390x"))] {
            // On s390x, SIGILL and SIGFPE are delivered with the PSW address
            // pointing *after* the faulting instruction, while SIGSEGV and
            // SIGBUS are delivered with the PSW address pointing *to* the
            // faulting instruction.  To handle this, the code generator registers
            // any trap that results in one of "late" signals on the last byte
            // of the instruction, and any trap that results in one of the "early"
            // signals on the first byte of the instruction (as usual).  This
            // means we simply need to decrement the reported PSW address by
            // one in the case of a "late" signal here to ensure we always
            // correctly find the associated trap handler.
            let trap_offset = match _signum {
                libc::SIGILL | libc::SIGFPE => 1,
                _ => 0,
            };
            unsafe {
                let cx = &*(cx as *const libc::ucontext_t);
                TrapRegisters {
                    pc: (cx.uc_mcontext.psw.addr - trap_offset) as usize,
                    fp: *(cx.uc_mcontext.gregs[15] as *const usize),
                }
            }
        } else if #[cfg(all(target_vendor = "apple", target_arch = "x86_64"))] {
            unsafe {
                let cx = &*(cx as *const libc::ucontext_t);
                TrapRegisters {
                    pc: (*cx.uc_mcontext).__ss.__rip as usize,
                    fp: (*cx.uc_mcontext).__ss.__rbp as usize,
                }
            }
        } else if #[cfg(all(target_vendor = "apple", target_arch = "aarch64"))] {
            unsafe {
                let cx = &*(cx as *const libc::ucontext_t);
                TrapRegisters {
                    pc: (*cx.uc_mcontext).__ss.__pc as usize,
                    fp: (*cx.uc_mcontext).__ss.__fp as usize,
                }
            }
        } else if #[cfg(all(target_os = "freebsd", target_arch = "x86_64"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.mc_rip as usize,
                fp: cx.uc_mcontext.mc_rbp as usize,
            }
        } else if #[cfg(all(target_os = "linux", target_arch = "riscv64"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.__gregs[libc::REG_PC] as usize,
                fp: cx.uc_mcontext.__gregs[libc::REG_S0] as usize,
            }
        } else if #[cfg(all(target_os = "freebsd", target_arch = "aarch64"))] {
            let cx = unsafe { &*(cx as *const libc::mcontext_t) };
            TrapRegisters {
                pc: cx.mc_gpregs.gp_elr as usize,
                fp: cx.mc_gpregs.gp_x[29] as usize,
            }
        } else if #[cfg(all(target_os = "openbsd", target_arch = "x86_64"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.sc_rip as usize,
                fp: cx.sc_rbp as usize,
            }
        } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
            let cx = unsafe { &*(cx as *const libc::ucontext_t) };
            TrapRegisters {
                pc: cx.uc_mcontext.arm_pc as usize,
                fp: cx.uc_mcontext.arm_fp as usize,
            }
        } else {
            compile_error!("unsupported platform");
            panic!();
        }
    }
}

// This is only used on macOS targets for calling an unwinding shim
// function to ensure that we return from the signal handler.
//
// See more comments above where this is called for what it's doing.
unsafe fn set_pc(cx: *mut libc::c_void, pc: usize, arg1: usize) {
    cfg_if::cfg_if! {
        if #[cfg(not(target_vendor = "apple"))] {
            let _ = (cx, pc, arg1);
            unreachable!(); // not used on these platforms
        } else if #[cfg(target_arch = "x86_64")] {
            unsafe {
                let cx = &mut *(cx as *mut libc::ucontext_t);
                (*cx.uc_mcontext).__ss.__rip = pc as u64;
                (*cx.uc_mcontext).__ss.__rdi = arg1 as u64;
                // We're simulating a "pseudo-call" so we need to ensure
                // stack alignment is properly respected, notably that on a
                // `call` instruction the stack is 8/16-byte aligned, then
                // the function adjusts itself to be 16-byte aligned.
                //
                // Most of the time the stack pointer is 16-byte aligned at
                // the time of the trap but for more robust-ness with JIT
                // code where it may ud2 in a prologue check before the
                // stack is aligned we double-check here.
                if (*cx.uc_mcontext).__ss.__rsp % 16 == 0 {
                    (*cx.uc_mcontext).__ss.__rsp -= 8;
                }
            }
        } else if #[cfg(target_arch = "aarch64")] {
            unsafe {
                let cx = &mut *(cx as *mut libc::ucontext_t);
                (*cx.uc_mcontext).__ss.__pc = pc as u64;
                (*cx.uc_mcontext).__ss.__x[0] = arg1 as u64;
            }
        } else {
            compile_error!("unsupported apple target architecture");
        }
    }
}

/// A function for registering a custom alternate signal stack (sigaltstack).
///
/// Rust's libstd installs an alternate stack with size `SIGSTKSZ`, which is not
/// always large enough for our signal handling code. Override it by creating
/// and registering our own alternate stack that is large enough and has a guard
/// page.
///
/// Note that one might reasonably ask why do this at all? Why not remove
/// `SA_ONSTACK` from our signal handlers entirely? The basic reason for that is
/// because we want to print a message on stack overflow. The Rust standard
/// library will print this message by default and by us overriding the
/// `SIGSEGV` handler above we're now sharing responsibility for that as well.
/// We must have `SA_ONSTACK` to even attempt to being able to printing this
/// message, and so we leave it turned on. Wasmtime will determine a stack
/// overflow fault isn't caused by wasm and then forward to libstd's signal
/// handler which will actually print-and-abort.
///
/// Another reasonable question might be why we need to increase the size of the
/// sigaltstack at all? This is something which we may want to reconsider in the
/// future. For now it helps keep debug builds working which consume more stack
/// when handling normal wasm out-of-bounds and faults. Perhaps in the future we
/// could optimize this more or maybe even do something clever like lazily
/// allocate the sigaltstack on the fault itself. (e.g. trampoline from a tiny
/// stack to the "big stack" during a wasm fault or something like that)
#[cold]
pub fn lazy_per_thread_init() {
    // This is a load-bearing requirement to keep address-sanitizer working and
    // prevent crashes during fuzzing. The general idea here is that we skip the
    // sigaltstack setup below entirely on asan builds, aka fuzzing. The exact
    // reason for this is not entirely known, but the closest guess we have at
    // this time is something like:
    //
    // * ASAN builds intercept mmap/munmap to keep track of what's going on.
    // * The sigaltstack below registers a TLS destructor for when the current
    //   thread exits to deallocate the stack.
    // * ASAN looks to also have TLS destructors for its own internal state.
    // * The current assumption is that the order of these TLS destructors can
    //   cause corruption in ASAN state where if we run after asan's destructor
    //   it may intercept munmap and then asan doesn't know it's been
    //   de-initialized yet.
    //
    // The reproduction of this involved a standalone project built with
    // `-Zsanitizer=address` where internally it would spawn two threads. Each
    // thread would build a "hello world" module and then one of the threads
    // would execute a noop exported function. If this was run thousands of
    // times in a loop in the same process it would eventually crash under asan.
    //
    // It's notably not quite so simple as frobbing TLS destructors. There's
    // clearly something else going on with ASAN state internally which we don't
    // fully understand at this time. An attempt to make a standalone C++
    // reproduction, for example, was not successful. In lieu of that the best
    // we have for now is to disable our custom and larger sigaltstack in asan
    // builds.
    //
    // The exact source was
    // https://gist.github.com/alexcrichton/6815a5d57a3c5ca94a8d816a9fcc91af for
    // future reference if necessary.
    if cfg!(asan) {
        return;
    }

    // This thread local is purely used to register a `Stack` to get deallocated
    // when the thread exists. Otherwise this function is only ever called at
    // most once per-thread.
    std::thread_local! {
        static STACK: RefCell<Option<Stack>> = const { RefCell::new(None) };
    }

    /// The size of the sigaltstack (not including the guard, which will be
    /// added). Make this large enough to run our signal handlers.
    ///
    /// The main current requirement of the signal handler in terms of stack
    /// space is that `malloc`/`realloc` are called to create a `Backtrace` of
    /// wasm frames.
    ///
    /// Historically this was 16k. Turns out jemalloc requires more than 16k of
    /// stack space in debug mode, so this was bumped to 64k.
    const MIN_STACK_SIZE: usize = 64 * 4096;

    struct Stack {
        mmap_ptr: *mut libc::c_void,
        mmap_size: usize,
    }

    return STACK.with(|s| {
        *s.borrow_mut() = unsafe { allocate_sigaltstack() };
    });

    unsafe fn allocate_sigaltstack() -> Option<Stack> {
        // Check to see if the existing sigaltstack, if it exists, is big
        // enough. If so we don't need to allocate our own.
        let mut old_stack = unsafe { mem::zeroed() };
        let r = unsafe { libc::sigaltstack(ptr::null(), &mut old_stack) };
        assert_eq!(
            r,
            0,
            "learning about sigaltstack failed: {}",
            io::Error::last_os_error()
        );
        if old_stack.ss_flags & libc::SS_DISABLE == 0 && old_stack.ss_size >= MIN_STACK_SIZE {
            return None;
        }

        // ... but failing that we need to allocate our own, so do all that
        // here.
        let page_size = crate::runtime::vm::host_page_size();
        let guard_size = page_size;
        let alloc_size = guard_size + MIN_STACK_SIZE;

        let ptr = unsafe {
            rustix::mm::mmap_anonymous(
                null_mut(),
                alloc_size,
                rustix::mm::ProtFlags::empty(),
                rustix::mm::MapFlags::PRIVATE,
            )
            .expect("failed to allocate memory for sigaltstack")
        };

        // Prepare the stack with readable/writable memory and then register it
        // with `sigaltstack`.
        let stack_ptr = (ptr as usize + guard_size) as *mut std::ffi::c_void;
        unsafe {
            rustix::mm::mprotect(
                stack_ptr,
                MIN_STACK_SIZE,
                rustix::mm::MprotectFlags::READ | rustix::mm::MprotectFlags::WRITE,
            )
            .expect("mprotect to configure memory for sigaltstack failed");
        }
        let new_stack = libc::stack_t {
            ss_sp: stack_ptr,
            ss_flags: 0,
            ss_size: MIN_STACK_SIZE,
        };
        let r = unsafe { libc::sigaltstack(&new_stack, ptr::null_mut()) };
        assert_eq!(
            r,
            0,
            "registering new sigaltstack failed: {}",
            io::Error::last_os_error()
        );

        Some(Stack {
            mmap_ptr: ptr,
            mmap_size: alloc_size,
        })
    }

    impl Drop for Stack {
        fn drop(&mut self) {
            unsafe {
                // Deallocate the stack memory.
                let r = rustix::mm::munmap(self.mmap_ptr, self.mmap_size);
                debug_assert!(r.is_ok(), "munmap failed during thread shutdown");
            }
        }
    }
}
