use super::*;
use crate::{IUnknown, IUnknown_Vtbl, Interface, GUID, HRESULT};
use core::ffi::c_void;
use core::mem::{transmute, transmute_copy};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicIsize, Ordering};

#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub const fn new() -> Self {
        Self(AtomicIsize::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        self.0
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |count_or_pointer| {
                bool::then_some(!is_weak_ref(count_or_pointer), count_or_pointer + 1)
            })
            .map(|u| u as u32 + 1)
            .unwrap_or_else(|pointer| unsafe { TearOff::decode(pointer).strong_count.add_ref() })
    }

    #[inline(always)]
    pub fn is_one(&self) -> bool {
        self.0.load(Ordering::Acquire) == 1
    }

    pub fn release(&self) -> u32 {
        self.0
            .fetch_update(Ordering::Release, Ordering::Relaxed, |count_or_pointer| {
                bool::then_some(!is_weak_ref(count_or_pointer), count_or_pointer - 1)
            })
            .map(|u| u as u32 - 1)
            .unwrap_or_else(|pointer| unsafe {
                let tear_off = TearOff::decode(pointer);
                let remaining = tear_off.strong_count.release();

                // If this is the last strong reference, we can release the weak reference implied by the strong reference.
                // There may still be weak references, so the WeakRelease is called to handle such possibilities.
                if remaining == 0 {
                    TearOff::WeakRelease(&mut tear_off.weak_vtable as *mut _ as _);
                }

                remaining
            })
    }

    /// # Safety
    pub unsafe fn query(&self, iid: &GUID, object: *mut c_void) -> *mut c_void {
        unsafe {
            if iid != &IWeakReferenceSource::IID {
                return null_mut();
            }

            let mut count_or_pointer = self.0.load(Ordering::Relaxed);

            if is_weak_ref(count_or_pointer) {
                return TearOff::from_encoding(count_or_pointer);
            }

            let tear_off = TearOff::new(object, count_or_pointer as u32);
            let tear_off_ptr: *mut c_void = transmute_copy(&tear_off);
            let encoding: usize = ((tear_off_ptr as usize) >> 1) | (1 << (usize::BITS - 1));

            loop {
                match self.0.compare_exchange_weak(
                    count_or_pointer,
                    encoding as isize,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let result: *mut c_void = transmute(tear_off);
                        TearOff::from_strong_ptr(result).strong_count.add_ref();
                        return result;
                    }
                    Err(pointer) => count_or_pointer = pointer,
                }

                if is_weak_ref(count_or_pointer) {
                    return TearOff::from_encoding(count_or_pointer);
                }

                TearOff::from_strong_ptr(tear_off_ptr)
                    .strong_count
                    .0
                    .store(count_or_pointer as i32, Ordering::SeqCst);
            }
        }
    }
}

fn is_weak_ref(value: isize) -> bool {
    value < 0
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_Vtbl,
    weak_vtable: *const IWeakReference_Vtbl,
    object: *mut c_void,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    #[allow(clippy::new_ret_no_self)]
    unsafe fn new(object: *mut c_void, strong_count: u32) -> IWeakReferenceSource {
        unsafe {
            transmute(Box::new(TearOff {
                strong_vtable: &Self::STRONG_VTABLE,
                weak_vtable: &Self::WEAK_VTABLE,
                object,
                strong_count: RefCount::new(strong_count),
                weak_count: RefCount::new(1),
            }))
        }
    }

    unsafe fn from_encoding(encoding: isize) -> *mut c_void {
        unsafe {
            let tear_off = TearOff::decode(encoding);
            tear_off.strong_count.add_ref();
            tear_off as *mut _ as *mut _
        }
    }

    const STRONG_VTABLE: IWeakReferenceSource_Vtbl = IWeakReferenceSource_Vtbl {
        base__: IUnknown_Vtbl {
            QueryInterface: Self::StrongQueryInterface,
            AddRef: Self::StrongAddRef,
            Release: Self::StrongRelease,
        },
        GetWeakReference: Self::StrongDowngrade,
    };

    const WEAK_VTABLE: IWeakReference_Vtbl = IWeakReference_Vtbl {
        base__: IUnknown_Vtbl {
            QueryInterface: Self::WeakQueryInterface,
            AddRef: Self::WeakAddRef,
            Release: Self::WeakRelease,
        },
        Resolve: Self::WeakUpgrade,
    };

    unsafe fn from_strong_ptr<'a>(this: *mut c_void) -> &'a mut Self {
        unsafe { &mut *(this as *mut *mut c_void as *mut Self) }
    }

    unsafe fn from_weak_ptr<'a>(this: *mut c_void) -> &'a mut Self {
        unsafe { &mut *((this as *mut *mut c_void).sub(1) as *mut Self) }
    }

    unsafe fn decode<'a>(value: isize) -> &'a mut Self {
        unsafe { transmute(value << 1) }
    }

    unsafe fn query_interface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        unsafe {
            ((*(*(self.object as *mut *mut IUnknown_Vtbl))).QueryInterface)(
                self.object,
                iid,
                interface,
            )
        }
    }

    unsafe extern "system" fn StrongQueryInterface(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_strong_ptr(ptr);

            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }

            // Only directly respond to queries for the the tear-off's strong interface. This is
            // effectively a self-query.
            if *iid == IWeakReferenceSource::IID {
                *interface = ptr;
                this.strong_count.add_ref();
                return HRESULT(0);
            }

            // As the tear-off is sharing the identity of the object, simply delegate any remaining
            // queries to the object.
            this.query_interface(iid, interface)
        }
    }

    unsafe extern "system" fn WeakQueryInterface(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_weak_ptr(ptr);

            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }

            // While the weak vtable is packed into the same allocation as the strong vtable and
            // tear-off, it represents a distinct COM identity and thus does not share or delegate to
            // the object.

            *interface = if *iid == IWeakReference::IID
                || *iid == IUnknown::IID
                || *iid == IAgileObject::IID
            {
                ptr
            } else {
                #[cfg(windows)]
                if *iid == IMarshal::IID {
                    this.weak_count.add_ref();
                    return marshaler(transmute::<*mut c_void, IUnknown>(ptr), interface);
                }

                null_mut()
            };

            if (*interface).is_null() {
                E_NOINTERFACE
            } else {
                this.weak_count.add_ref();
                HRESULT(0)
            }
        }
    }

    unsafe extern "system" fn StrongAddRef(ptr: *mut c_void) -> u32 {
        unsafe {
            let this = Self::from_strong_ptr(ptr);

            // Implement `AddRef` directly as we own the strong reference.
            this.strong_count.add_ref()
        }
    }

    unsafe extern "system" fn WeakAddRef(ptr: *mut c_void) -> u32 {
        unsafe {
            let this = Self::from_weak_ptr(ptr);

            // Implement `AddRef` directly as we own the weak reference.
            this.weak_count.add_ref()
        }
    }

    unsafe extern "system" fn StrongRelease(ptr: *mut c_void) -> u32 {
        unsafe {
            let this = Self::from_strong_ptr(ptr);

            // Forward strong `Release` to the object so that it can destroy itself. It will then
            // decrement its weak reference and allow the tear-off to be released as needed.
            ((*(*(this.object as *mut *mut IUnknown_Vtbl))).Release)(this.object)
        }
    }

    unsafe extern "system" fn WeakRelease(ptr: *mut c_void) -> u32 {
        unsafe {
            let this = Self::from_weak_ptr(ptr);

            // Implement `Release` directly as we own the weak reference.
            let remaining = this.weak_count.release();

            // If there are no remaining references, it means that the object has already been
            // destroyed. Go ahead and destroy the tear-off.
            if remaining == 0 {
                let _ = Box::from_raw(this);
            }

            remaining
        }
    }

    unsafe extern "system" fn StrongDowngrade(
        ptr: *mut c_void,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_strong_ptr(ptr);

            // The strong vtable hands out a reference to the weak vtable. This is always safe and
            // straightforward since a strong reference guarantees there is at least one weak
            // reference.
            *interface = &mut this.weak_vtable as *mut _ as _;
            this.weak_count.add_ref();
            HRESULT(0)
        }
    }

    unsafe extern "system" fn WeakUpgrade(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_weak_ptr(ptr);

            this.strong_count
                .0
                .fetch_update(Ordering::Acquire, Ordering::Relaxed, |count| {
                    // Attempt to acquire a strong reference count to stabilize the object for the duration
                    // of the `QueryInterface` call.
                    bool::then_some(count != 0, count + 1)
                })
                .map(|_| {
                    // Let the object respond to the upgrade query.
                    let result = this.query_interface(iid, interface);
                    // Decrement the temporary reference account used to stabilize the object.
                    this.strong_count.0.fetch_sub(1, Ordering::Relaxed);
                    // Return the result of the query.
                    result
                })
                .unwrap_or_else(|_| {
                    *interface = null_mut();
                    HRESULT(0)
                })
        }
    }
}
