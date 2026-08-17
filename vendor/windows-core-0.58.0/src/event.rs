use super::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{size_of, transmute_copy};
use core::ptr::null_mut;
use std::sync::Mutex;

/// A type that you can use to declare and implement an event of a specified delegate type.
///
/// The implementation is thread-safe and designed to avoid contention between events being
/// raised and delegates being added or removed.
pub struct Event<T: Interface> {
    swap: Mutex<()>,
    change: Mutex<()>,
    delegates: Array<T>,
}

impl<T: Interface> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Interface> Event<T> {
    /// Creates a new, empty `Event<T>`.
    pub fn new() -> Self {
        Self {
            delegates: Array::new(),
            swap: Mutex::default(),
            change: Mutex::default(),
        }
    }

    /// Registers a delegate with the event object.
    pub fn add(&mut self, delegate: &T) -> Result<i64> {
        let mut _lock_free_drop = Array::new();
        Ok({
            let _change_lock = self.change.lock().unwrap();
            let mut new_delegates = Array::with_capacity(self.delegates.len() + 1)?;
            for delegate in self.delegates.as_slice() {
                new_delegates.push(delegate.clone());
            }
            let delegate = Delegate::new(delegate)?;
            let token = delegate.to_token();
            new_delegates.push(delegate);

            let _swap_lock = self.swap.lock().unwrap();
            _lock_free_drop = self.delegates.swap(new_delegates);
            token
        })
    }

    /// Revokes a delegate's registration from the event object.
    pub fn remove(&mut self, token: i64) -> Result<()> {
        let mut _lock_free_drop = Array::new();
        {
            let _change_lock = self.change.lock().unwrap();
            if self.delegates.is_empty() {
                return Ok(());
            }
            let mut capacity = self.delegates.len() - 1;
            let mut new_delegates = Array::new();
            let mut removed = false;
            if capacity == 0 {
                removed = self.delegates.as_slice()[0].to_token() == token;
            } else {
                new_delegates = Array::with_capacity(capacity)?;
                for delegate in self.delegates.as_slice() {
                    if !removed && delegate.to_token() == token {
                        removed = true;
                        continue;
                    }
                    if capacity == 0 {
                        break;
                    }
                    new_delegates.push(delegate.clone());
                    capacity -= 1;
                }
            }
            if removed {
                let _swap_lock = self.swap.lock().unwrap();
                _lock_free_drop = self.delegates.swap(new_delegates);
            }
        }
        Ok(())
    }

    /// Clears the event, removing all delegates.
    pub fn clear(&mut self) {
        let mut _lock_free_drop = Array::new();
        {
            let _change_lock = self.change.lock().unwrap();
            if self.delegates.is_empty() {
                return;
            }
            let _swap_lock = self.swap.lock().unwrap();
            _lock_free_drop = self.delegates.swap(Array::new());
        }
    }

    /// Invokes all of the event object's registered delegates with the provided callback.
    pub fn call<F: FnMut(&T) -> Result<()>>(&mut self, mut callback: F) -> Result<()> {
        let lock_free_calls = {
            let _swap_lock = self.swap.lock().unwrap();
            self.delegates.clone()
        };
        for delegate in lock_free_calls.as_slice() {
            if let Err(error) = delegate.call(&mut callback) {
                const RPC_E_SERVER_UNAVAILABLE: HRESULT = HRESULT(-2147023174); // HRESULT_FROM_WIN32(RPC_S_SERVER_UNAVAILABLE)
                if matches!(
                    error.code(),
                    imp::RPC_E_DISCONNECTED | imp::JSCRIPT_E_CANTEXECUTE | RPC_E_SERVER_UNAVAILABLE
                ) {
                    self.remove(delegate.to_token())?;
                }
            }
        }
        Ok(())
    }
}

/// A thread-safe reference-counted array of delegates.
struct Array<T: Interface> {
    buffer: *mut Buffer<T>,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T: Interface> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Interface> Array<T> {
    /// Creates a new, empty `Array<T>` with no capacity.
    fn new() -> Self {
        Self {
            buffer: null_mut(),
            len: 0,
            _phantom: PhantomData,
        }
    }

    /// Creates a new, empty `Array<T>` with the specified capacity.
    fn with_capacity(capacity: usize) -> Result<Self> {
        Ok(Self {
            buffer: Buffer::new(capacity)?,
            len: 0,
            _phantom: PhantomData,
        })
    }

    /// Swaps the contents of two `Array<T>` objects.
    fn swap(&mut self, mut other: Self) -> Self {
        unsafe { core::ptr::swap(&mut self.buffer, &mut other.buffer) };
        core::mem::swap(&mut self.len, &mut other.len);
        other
    }

    /// Returns `true` if the array contains no delegates.
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of delegates in the array.
    fn len(&self) -> usize {
        self.len
    }

    /// Appends a delegate to the back of the array.
    fn push(&mut self, delegate: Delegate<T>) {
        unsafe {
            (*self.buffer).as_mut_ptr().add(self.len).write(delegate);
            self.len += 1;
        }
    }

    /// Returns a slice containing of all delegates.
    fn as_slice(&self) -> &[Delegate<T>] {
        if self.is_empty() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts((*self.buffer).as_ptr(), self.len) }
        }
    }

    /// Returns a mutable slice of all delegates.
    fn as_mut_slice(&mut self) -> &mut [Delegate<T>] {
        if self.is_empty() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut((*self.buffer).as_mut_ptr(), self.len) }
        }
    }
}

impl<T: Interface> Clone for Array<T> {
    fn clone(&self) -> Self {
        if !self.is_empty() {
            unsafe { (*self.buffer).0.add_ref() };
        }
        Self {
            buffer: self.buffer,
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

impl<T: Interface> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.is_empty() && (*self.buffer).0.release() == 0 {
                core::ptr::drop_in_place(self.as_mut_slice());
                heap_free(self.buffer as _)
            }
        }
    }
}

/// A reference-counted buffer.
#[repr(C)]
#[repr(align(8))]
struct Buffer<T>(imp::RefCount, PhantomData<T>);

impl<T: Interface> Buffer<T> {
    /// Creates a new `Buffer` with the specified size in bytes.
    fn new(len: usize) -> Result<*mut Self> {
        if len == 0 {
            Ok(null_mut())
        } else {
            let alloc_size = size_of::<Self>() + len * size_of::<Delegate<T>>();
            let header = heap_alloc(alloc_size)? as *mut Self;
            unsafe {
                header.write(Self(imp::RefCount::new(1), PhantomData));
            }
            Ok(header)
        }
    }

    /// Returns a raw pointer to the buffer's contents. The resulting pointer might be uninititalized.
    fn as_ptr(&self) -> *const Delegate<T> {
        unsafe { (self as *const Self).add(1) as *const _ }
    }

    /// Returns a raw mutable pointer to the buffer's contents. The resulting pointer might be uninititalized.
    fn as_mut_ptr(&mut self) -> *mut Delegate<T> {
        unsafe { (self as *mut Self).add(1) as *mut _ }
    }
}

/// Holds either a direct or indirect reference to a delegate. A direct reference is typically
/// agile while an indirect reference is an agile wrapper.
#[derive(Clone)]
enum Delegate<T> {
    Direct(T),
    Indirect(AgileReference<T>),
}

impl<T: Interface> Delegate<T> {
    /// Creates a new `Delegate<T>`, containing a suitable reference to the specified delegate.
    fn new(delegate: &T) -> Result<Self> {
        if delegate.cast::<imp::IAgileObject>().is_ok() {
            Ok(Self::Direct(delegate.clone()))
        } else {
            Ok(Self::Indirect(AgileReference::new(delegate)?))
        }
    }

    /// Returns an encoded token to identify the delegate.
    fn to_token(&self) -> i64 {
        unsafe {
            match self {
                Self::Direct(delegate) => imp::EncodePointer(transmute_copy(delegate)) as i64,
                Self::Indirect(delegate) => imp::EncodePointer(transmute_copy(delegate)) as i64,
            }
        }
    }

    /// Invokes the delegates with the provided callback.
    fn call<F: FnMut(&T) -> Result<()>>(&self, mut callback: F) -> Result<()> {
        match self {
            Self::Direct(delegate) => callback(delegate),
            Self::Indirect(delegate) => callback(&delegate.resolve()?),
        }
    }
}

/// Allocate memory of size `bytes` using `malloc` - the `Event` implementation does not
/// need to use any particular allocator so `HeapAlloc` need not be used.
fn heap_alloc(bytes: usize) -> crate::Result<*mut c_void> {
    let ptr: *mut c_void = unsafe {
        extern "C" {
            fn malloc(bytes: usize) -> *mut c_void;
        }

        malloc(bytes)
    };

    if ptr.is_null() {
        Err(Error::from_hresult(imp::E_OUTOFMEMORY))
    } else {
        Ok(ptr)
    }
}

/// Free memory allocated by `heap_alloc`.
unsafe fn heap_free(ptr: *mut c_void) {
    extern "C" {
        fn free(ptr: *mut c_void);
    }

    free(ptr);
}
