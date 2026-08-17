//! A hybrid strategy.
//!
//! This is based on debts ‒ an Arc may owe a reference, but it is marked in the debt. It is either
//! put back (by stopping using it), or if the pointer is replaced, the writer bumps the reference
//! count and removes the debt.
//!
//! The strategy uses two different slots for the debts. The first ones are faster, but fallible.
//! If they fail (either because there's interference from a writer at the same time, or because
//! they are full), the secondary one that is slower, but always succeeds, is used. In the latter
//! case, the reference is bumped and this secondary debt slot is released, so it is available for
//! further loads.
//!
//! See the [crate::debt] module for the actual slot manipulation. Here we just wrap them into the
//! strategy.

use core::borrow::Borrow;
use core::mem::{self, ManuallyDrop};
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::*;

use super::sealed::{CaS, InnerStrategy, Protected};
use crate::debt::{Debt, LocalNode};
use crate::ref_cnt::RefCnt;

pub struct HybridProtection<T: RefCnt> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}

impl<T: RefCnt> HybridProtection<T> {
    pub(super) unsafe fn new(ptr: *const T::Base, debt: Option<&'static Debt>) -> Self {
        Self {
            debt,
            ptr: ManuallyDrop::new(T::from_ptr(ptr)),
        }
    }

    /// Try getting a dept into a fast slot.
    #[inline]
    fn attempt(node: &LocalNode, storage: &AtomicPtr<T::Base>) -> Option<Self> {
        // SeqCst needed, because of we potentially use it in the "Debt already paid" scenario. And
        // we need to timeline it before the debt gets paid.
        let ptr = storage.load(SeqCst);
        // Try to get a debt slot. If not possible, fail.
        let debt = node.new_fast(ptr as usize)?;

        // Acquire to get the data.
        //
        // SeqCst to make sure the storage vs. the debt are well ordered.
        let confirm = storage.load(SeqCst);
        if ptr == confirm {
            // Successfully got a debt
            // NOTE: we *must* use `confirm` here instead of `ptr`. The address may compare equal,
            // but they could have different provenance, if the pointer is freed and then
            // subsequently reused.
            Some(unsafe { Self::new(confirm, Some(debt)) })
        } else if debt.pay::<T>(ptr) {
            // It changed in the meantime, we return the debt (that is on the outdated pointer,
            // possibly destroyed) and fail.
            None
        } else {
            // It changed in the meantime, but the debt for the previous pointer was already paid
            // for by someone else, so we are fine using it.
            Some(unsafe { Self::new(ptr, None) })
        }
    }

    /// Get a debt slot using the slower but always successful mechanism.
    fn fallback(node: &LocalNode, storage: &AtomicPtr<T::Base>) -> Self {
        // First, we claim a debt slot and store the address of the atomic pointer there, so the
        // writer can optionally help us out with loading and protecting something.
        let gen = node.new_helping(storage as *const _ as usize);
        // Need SeqCst to make sure the candidate is not outdated and already freed. Otherwise, we
        // could "successfully" protect an already freed pointer.
        let candidate = storage.load(SeqCst);

        // Try to replace the debt with our candidate. If it works, we get the debt slot to use. If
        // not, we get a replacement value, already protected and a debt to take care of.
        match node.confirm_helping(gen, candidate as usize) {
            Ok(debt) => {
                // The fast path -> we got the debt confirmed alright.
                Self::from_inner(unsafe { Self::new(candidate, Some(debt)).into_inner() })
            }
            Err((unused_debt, replacement)) => {
                // The debt is on the candidate we provided and it is unused, we so we just pay it
                // back right away.
                if !unused_debt.pay::<T>(candidate) {
                    unsafe { T::dec(candidate) };
                }
                // We got a (possibly) different pointer out. But that one is already protected and
                // the slot is paid back.
                unsafe { Self::new(replacement as *mut _, None) }
            }
        }
    }

    #[inline]
    fn as_ptr(&self) -> *const T::Base {
        T::as_ptr(self.ptr.deref())
    }
}

impl<T: RefCnt> Drop for HybridProtection<T> {
    #[inline]
    fn drop(&mut self) {
        match self.debt.take() {
            // We have our own copy of Arc, so we don't need a protection. Do nothing (but release
            // the Arc below).
            None => (),
            // If we owed something, just return the debt. We don't have a pointer owned, so
            // nothing to release.
            Some(debt) => {
                let ptr = T::as_ptr(&self.ptr);
                if debt.pay::<T>(ptr) {
                    return;
                }
                // But if the debt was already paid for us, we need to release the pointer, as we
                // were effectively already in the Unprotected mode.
            }
        }
        // Equivalent to T::dec(ptr)
        unsafe { ManuallyDrop::drop(&mut self.ptr) };
    }
}

impl<T: RefCnt> Protected<T> for HybridProtection<T> {
    #[inline]
    fn from_inner(ptr: T) -> Self {
        Self {
            debt: None,
            ptr: ManuallyDrop::new(ptr),
        }
    }

    #[inline]
    fn into_inner(mut self) -> T {
        // Drop any debt and release any lock held by the given guard and return a
        // full-featured value that even can outlive the ArcSwap it originated from.
        match self.debt.take() {
            None => (), // We have a fully loaded ref-counted pointer.
            Some(debt) => {
                let ptr = T::inc(&self.ptr);
                if !debt.pay::<T>(ptr) {
                    unsafe { T::dec(ptr) };
                }
            }
        }

        // The ptr::read & forget is something like a cheating move. We can't move it out, because
        // we have a destructor and Rust doesn't allow us to do that.
        let inner = unsafe { ptr::read(self.ptr.deref()) };
        mem::forget(self);
        inner
    }
}

impl<T: RefCnt> Borrow<T> for HybridProtection<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ptr
    }
}

pub trait Config {
    // Mostly for testing, way to disable the fast slo
    const USE_FAST: bool;
}

#[derive(Clone, Default)]
pub struct DefaultConfig;

impl Config for DefaultConfig {
    const USE_FAST: bool = true;
}

#[derive(Clone, Default)]
pub struct HybridStrategy<Cfg> {
    pub(crate) _config: Cfg,
}

impl<T, Cfg> InnerStrategy<T> for HybridStrategy<Cfg>
where
    T: RefCnt,
    Cfg: Config,
{
    type Protected = HybridProtection<T>;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        LocalNode::with(|node| {
            let fast = if Cfg::USE_FAST {
                HybridProtection::attempt(node, storage)
            } else {
                None
            };
            fast.unwrap_or_else(|| HybridProtection::fallback(node, storage))
        })
    }
    unsafe fn wait_for_readers(&self, old: *const T::Base, storage: &AtomicPtr<T::Base>) {
        // The pay_all may need to provide fresh replacement values if someone else is loading from
        // this particular storage. We do so by the exact same way, by `load` ‒ it's OK, a writer
        // does not hold a slot and the reader doesn't recurse back into writer, so we won't run
        // out of slots.
        let replacement = || self.load(storage).into_inner();
        Debt::pay_all::<T, _>(old, storage as *const _ as usize, replacement);
    }
}

impl<T: RefCnt, Cfg: Config> CaS<T> for HybridStrategy<Cfg> {
    unsafe fn compare_and_swap<C: crate::as_raw::AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        loop {
            let old = <Self as InnerStrategy<T>>::load(self, storage);
            // Observation of their inequality is enough to make a verdict
            if old.as_ptr() != current.as_raw() {
                return old;
            }
            // If they are still equal, put the new one in.
            let new_raw = T::as_ptr(&new);
            if storage
                .compare_exchange_weak(current.as_raw(), new_raw, SeqCst, Relaxed)
                .is_ok()
            {
                // We successfully put the new value in. The ref count went in there too.
                T::into_ptr(new);
                <Self as InnerStrategy<T>>::wait_for_readers(self, old.as_ptr(), storage);
                // We just got one ref count out of the storage and we have one in old. We don't
                // need two.
                T::dec(old.as_ptr());
                return old;
            }
        }
    }
}
