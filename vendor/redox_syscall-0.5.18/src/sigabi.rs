use core::sync::atomic::{AtomicUsize, Ordering};

/// Signal runtime struct for the entire process
#[derive(Debug)]
#[repr(C, align(4096))]
pub struct SigProcControl {
    pub pending: AtomicU64,
    pub actions: [RawAction; 64],
    pub sender_infos: [AtomicU64; 32],
    //pub queue: [RealtimeSig; 32], TODO
    // qhead, qtail TODO
}
/*#[derive(Debug)]
#[repr(transparent)]
pub struct RealtimeSig {
    pub arg: NonatomicUsize,
}*/
#[derive(Debug, Default)]
#[repr(C, align(16))]
pub struct RawAction {
    /// Only two MSBs are interesting for the kernel. If bit 63 is set, signal is ignored. If bit
    /// 62 is set and the signal is SIGTSTP/SIGTTIN/SIGTTOU, it's equivalent to the action of
    /// Stop.
    pub first: AtomicU64,
    /// Completely ignored by the kernel, but exists so userspace can (when 16-byte atomics exist)
    /// atomically set both the handler, sigaction flags, and sigaction mask.
    pub user_data: AtomicU64,
}

/// Signal runtime struct for a thread
#[derive(Debug, Default)]
#[repr(C)]
pub struct Sigcontrol {
    // composed of [lo "pending" | lo "unmasked", hi "pending" | hi "unmasked"]
    pub word: [AtomicU64; 2],

    // lo = sender pid, hi = sender ruid
    pub sender_infos: [AtomicU64; 32],

    pub control_flags: SigatomicUsize,

    pub saved_ip: NonatomicUsize,          // rip/eip/pc
    pub saved_archdep_reg: NonatomicUsize, // rflags(x64)/eflags(x86)/x0(aarch64)/t0(riscv64)
}
#[derive(Clone, Copy, Debug)]
pub struct SenderInfo {
    pub pid: u32,
    pub ruid: u32,
}
impl SenderInfo {
    #[inline]
    pub fn raw(self) -> u64 {
        u64::from(self.pid) | (u64::from(self.ruid) << 32)
    }
    #[inline]
    pub const fn from_raw(raw: u64) -> Self {
        Self {
            pid: raw as u32,
            ruid: (raw >> 32) as u32,
        }
    }
}

impl Sigcontrol {
    pub fn currently_pending_unblocked(&self, proc: &SigProcControl) -> u64 {
        let proc_pending = proc.pending.load(Ordering::Relaxed);
        let [w0, w1] = core::array::from_fn(|i| {
            let w = self.word[i].load(Ordering::Relaxed);
            ((w | (proc_pending >> (i * 32))) & 0xffff_ffff) & (w >> 32)
        });
        //core::sync::atomic::fence(Ordering::Acquire);
        w0 | (w1 << 32)
    }
    pub fn set_allowset(&self, new_allowset: u64) -> u64 {
        //core::sync::atomic::fence(Ordering::Release);
        let [w0, w1] = self.word.each_ref().map(|w| w.load(Ordering::Relaxed));
        let old_a0 = w0 & 0xffff_ffff_0000_0000;
        let old_a1 = w1 & 0xffff_ffff_0000_0000;
        let new_a0 = (new_allowset & 0xffff_ffff) << 32;
        let new_a1 = new_allowset & 0xffff_ffff_0000_0000;

        let prev_w0 = self.word[0].fetch_add(new_a0.wrapping_sub(old_a0), Ordering::Relaxed);
        let prev_w1 = self.word[0].fetch_add(new_a1.wrapping_sub(old_a1), Ordering::Relaxed);
        //core::sync::atomic::fence(Ordering::Acquire);
        let up0 = prev_w0 & (prev_w0 >> 32);
        let up1 = prev_w1 & (prev_w1 >> 32);

        up0 | (up1 << 32)
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct SigatomicUsize(AtomicUsize);

impl SigatomicUsize {
    #[inline]
    pub fn load(&self, ordering: Ordering) -> usize {
        let value = self.0.load(Ordering::Relaxed);
        if ordering != Ordering::Relaxed {
            core::sync::atomic::compiler_fence(ordering);
        }
        value
    }
    #[inline]
    pub fn store(&self, value: usize, ordering: Ordering) {
        if ordering != Ordering::Relaxed {
            core::sync::atomic::compiler_fence(ordering);
        }
        self.0.store(value, Ordering::Relaxed);
    }
}
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct NonatomicUsize(AtomicUsize);

impl NonatomicUsize {
    #[inline]
    pub const fn new(a: usize) -> Self {
        Self(AtomicUsize::new(a))
    }

    #[inline]
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn set(&self, value: usize) {
        self.0.store(value, Ordering::Relaxed);
    }
}

pub fn sig_bit(sig: usize) -> u64 {
    1 << (sig - 1)
}
impl SigProcControl {
    // TODO: Move to redox_rt?
    pub fn signal_will_ign(&self, sig: usize, is_parent_sigchld: bool) -> bool {
        let flags = self.actions[sig - 1].first.load(Ordering::Relaxed);
        let will_ign = flags & (1 << 63) != 0;
        let sig_specific = flags & (1 << 62) != 0; // SA_NOCLDSTOP if sig == SIGCHLD

        will_ign || (sig == SIGCHLD && is_parent_sigchld && sig_specific)
    }
    // TODO: Move to redox_rt?
    pub fn signal_will_stop(&self, sig: usize) -> bool {
        use crate::flag::*;
        matches!(sig, SIGTSTP | SIGTTIN | SIGTTOU)
            && self.actions[sig - 1].first.load(Ordering::Relaxed) & (1 << 62) != 0
    }
}

#[cfg(not(target_arch = "x86"))]
pub use core::sync::atomic::AtomicU64;

use crate::SIGCHLD;

#[cfg(target_arch = "x86")]
pub use self::atomic::AtomicU64;

#[cfg(target_arch = "x86")]
mod atomic {
    use core::{cell::UnsafeCell, sync::atomic::Ordering};

    #[derive(Debug, Default)]
    pub struct AtomicU64(UnsafeCell<u64>);

    unsafe impl Send for AtomicU64 {}
    unsafe impl Sync for AtomicU64 {}

    impl AtomicU64 {
        pub const fn new(inner: u64) -> Self {
            Self(UnsafeCell::new(inner))
        }
        pub fn compare_exchange(
            &self,
            old: u64,
            new: u64,
            _success: Ordering,
            _failure: Ordering,
        ) -> Result<u64, u64> {
            let old_hi = (old >> 32) as u32;
            let old_lo = old as u32;
            let new_hi = (new >> 32) as u32;
            let new_lo = new as u32;
            let mut out_hi;
            let mut out_lo;

            unsafe {
                core::arch::asm!("lock cmpxchg8b [{}]", in(reg) self.0.get(), inout("edx") old_hi => out_hi, inout("eax") old_lo => out_lo, in("ecx") new_hi, in("ebx") new_lo);
            }

            if old_hi == out_hi && old_lo == out_lo {
                Ok(old)
            } else {
                Err(u64::from(out_lo) | (u64::from(out_hi) << 32))
            }
        }
        pub fn load(&self, ordering: Ordering) -> u64 {
            match self.compare_exchange(0, 0, ordering, ordering) {
                Ok(new) => new,
                Err(new) => new,
            }
        }
        pub fn store(&self, new: u64, ordering: Ordering) {
            let mut old = 0;

            loop {
                match self.compare_exchange(old, new, ordering, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(new) => {
                        old = new;
                        core::hint::spin_loop();
                    }
                }
            }
        }
        pub fn fetch_update(
            &self,
            set_order: Ordering,
            fetch_order: Ordering,
            mut f: impl FnMut(u64) -> Option<u64>,
        ) -> Result<u64, u64> {
            let mut old = self.load(fetch_order);

            loop {
                let new = f(old).ok_or(old)?;
                match self.compare_exchange(old, new, set_order, Ordering::Relaxed) {
                    Ok(_) => return Ok(new),
                    Err(changed) => {
                        old = changed;
                        core::hint::spin_loop();
                    }
                }
            }
        }
        pub fn fetch_or(&self, bits: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b | bits))
                .unwrap()
        }
        pub fn fetch_and(&self, bits: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b & bits))
                .unwrap()
        }
        pub fn fetch_add(&self, term: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b.wrapping_add(term)))
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    };

    #[cfg(not(loom))]
    use std::{sync::Mutex, thread};
    #[cfg(not(loom))]
    fn model(f: impl FnOnce()) {
        f()
    }

    #[cfg(loom)]
    use loom::{model, sync::Mutex, thread};

    use crate::{RawAction, SigProcControl, Sigcontrol};

    struct FakeThread {
        ctl: Sigcontrol,
        pctl: SigProcControl,
        ctxt: Mutex<()>,
    }
    impl Default for FakeThread {
        fn default() -> Self {
            Self {
                ctl: Sigcontrol::default(),
                pctl: SigProcControl {
                    pending: AtomicU64::new(0),
                    actions: core::array::from_fn(|_| RawAction::default()),
                    sender_infos: Default::default(),
                },
                ctxt: Default::default(),
            }
        }
    }

    #[test]
    fn singlethread_mask() {
        model(|| {
            let fake_thread = Arc::new(FakeThread::default());

            let thread = {
                let fake_thread = Arc::clone(&fake_thread);

                thread::spawn(move || {
                    fake_thread.ctl.set_allowset(!0);
                    {
                        let _g = fake_thread.ctxt.lock();
                        if fake_thread
                            .ctl
                            .currently_pending_unblocked(&fake_thread.pctl)
                            == 0
                        {
                            drop(_g);
                            thread::park();
                        }
                    }
                })
            };

            for sig in 1..=64 {
                let _g = fake_thread.ctxt.lock();

                let idx = sig - 1;
                let bit = 1 << (idx % 32);

                fake_thread.ctl.word[idx / 32].fetch_or(bit, Ordering::Relaxed);
                let w = fake_thread.ctl.word[idx / 32].load(Ordering::Relaxed);

                if w & (w >> 32) != 0 {
                    thread.thread().unpark();
                }
            }

            thread.join().unwrap();
        });
    }
}
