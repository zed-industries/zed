//! # generator stack
//!
//!

use std::error::Error;
use std::fmt::{self, Display};
use std::io;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::ptr;

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod sys;

pub use sys::overflow;

// must align with StackBoxHeader
const ALIGN: usize = std::mem::size_of::<StackBoxHeader>();
const HEADER_SIZE: usize = std::mem::size_of::<StackBoxHeader>() / std::mem::size_of::<usize>();

struct StackBoxHeader {
    // track the stack
    stack: Stack,
    // track how big the data is (in usize)
    data_size: usize,
    // non zero dealloc the stack
    need_drop: usize,
}

/// A pointer type for stack allocation.
pub struct StackBox<T> {
    // the stack memory
    ptr: ptr::NonNull<T>,
}

impl<T> StackBox<T> {
    /// create uninit stack box
    fn new_uninit(stack: &mut Stack, need_drop: usize) -> MaybeUninit<Self> {
        // cheat #[warn(clippy::needless_pass_by_ref_mut)]
        // we need mutable ref for ownership
        let _ = stack as *mut Stack;

        let offset = unsafe { &mut *stack.get_offset() };
        // alloc the data
        let layout = std::alloc::Layout::new::<T>();
        let align = std::cmp::max(layout.align(), ALIGN);
        let size = ((layout.size() + align - 1) & !(align - 1)) / std::mem::size_of::<usize>();
        let u_align = align / std::mem::size_of::<usize>();
        let pad_size = u_align - (*offset + size) % u_align;
        let data_size = size + pad_size;
        *offset += data_size;
        let ptr = unsafe { ptr::NonNull::new_unchecked(stack.end() as *mut T) };

        // init the header
        *offset += HEADER_SIZE;
        unsafe {
            let mut header = ptr::NonNull::new_unchecked(stack.end() as *mut StackBoxHeader);
            let header = header.as_mut();
            header.data_size = data_size;
            header.need_drop = need_drop;
            header.stack = stack.shadow_clone();
            MaybeUninit::new(StackBox { ptr })
        }
    }

    fn get_header(&self) -> &StackBoxHeader {
        unsafe {
            let header = (self.ptr.as_ptr() as *mut usize).offset(0 - HEADER_SIZE as isize);
            &*(header as *const StackBoxHeader)
        }
    }

    /// move data into the box
    pub(crate) unsafe fn init(&mut self, data: T) {
        ptr::write(self.ptr.as_ptr(), data);
    }

    // get the stack ptr
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Constructs a StackBox from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    #[inline]
    pub(crate) unsafe fn from_raw(raw: *mut T) -> Self {
        StackBox {
            ptr: ptr::NonNull::new_unchecked(raw),
        }
    }

    // Consumes the `StackBox`, returning a wrapped raw pointer.
    // #[inline]
    // pub(crate) fn into_raw(b: StackBox<T>) -> *mut T {
    //     let ret = b.ptr.as_ptr();
    //     std::mem::forget(b);
    //     ret
    // }
}

pub struct Func {
    data: *mut (),
    size: usize,
    offset: *mut usize,
    func: fn(*mut ()),
    drop: fn(*mut ()),
}

impl Func {
    pub fn call_once(mut self) {
        let data = self.data;
        self.data = ptr::null_mut();
        (self.func)(data);
    }
}

impl Drop for Func {
    fn drop(&mut self) {
        if !self.data.is_null() {
            (self.drop)(self.data);
        }
        unsafe { *self.offset -= self.size };
    }
}

impl<F: FnOnce()> StackBox<F> {
    fn call_once(data: *mut ()) {
        unsafe {
            let data = data as *mut F;
            let f = data.read();
            f();
        }
    }

    fn drop_inner(data: *mut ()) {
        unsafe {
            let data = data as *mut F;
            ptr::drop_in_place(data);
        }
    }

    /// create a functor on the stack
    pub(crate) fn new_fn_once(stack: &mut Stack, data: F) -> Func {
        unsafe {
            let mut d = Self::new_uninit(stack, 0);
            (*d.as_mut_ptr()).init(data);
            let d = d.assume_init();
            let header = d.get_header();
            let f = Func {
                data: d.ptr.as_ptr() as *mut (),
                size: header.data_size + HEADER_SIZE,
                offset: stack.get_offset(),
                func: Self::call_once,
                drop: Self::drop_inner,
            };
            std::mem::forget(d);
            f
        }
    }
}

impl<T> std::ops::Deref for StackBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> std::ops::DerefMut for StackBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_mut() }
    }
}

impl<T> Drop for StackBox<T> {
    fn drop(&mut self) {
        let header = self.get_header();
        unsafe {
            *header.stack.get_offset() -= header.data_size + HEADER_SIZE;
            ptr::drop_in_place(self.ptr.as_ptr());
            if header.need_drop != 0 {
                header.stack.drop_stack();
            }
        }
    }
}

/// Error type returned by stack allocation methods.
#[derive(Debug)]
pub enum StackError {
    /// Contains the maximum amount of memory allowed to be allocated as stack space.
    ExceedsMaximumSize(usize),

    /// Returned if some kind of I/O error happens during allocation.
    IoError(io::Error),
}

impl Display for StackError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StackError::ExceedsMaximumSize(size) => write!(
                fmt,
                "Requested more than max size of {size} bytes for a stack"
            ),
            StackError::IoError(ref e) => e.fmt(fmt),
        }
    }
}

impl Error for StackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            StackError::ExceedsMaximumSize(_) => None,
            StackError::IoError(ref e) => Some(e),
        }
    }
}

/// Represents any kind of stack memory.
///
/// `FixedSizeStack` as well as `ProtectedFixedSizeStack`
/// can be used to allocate actual stack space.
#[derive(Debug)]
pub struct SysStack {
    top: *mut c_void,
    bottom: *mut c_void,
}

impl SysStack {
    /// Creates a (non-owning) representation of some stack memory.
    ///
    /// It is unsafe because it is your responsibility to make sure that `top` and `bottom` are valid
    /// addresses.
    #[inline]
    pub unsafe fn new(top: *mut c_void, bottom: *mut c_void) -> SysStack {
        debug_assert!(top >= bottom);

        SysStack { top, bottom }
    }

    /// Returns the top of the stack from which on it grows downwards towards bottom().
    #[inline]
    pub fn top(&self) -> *mut c_void {
        self.top
    }

    /// Returns the bottom of the stack and thus it's end.
    #[inline]
    pub fn bottom(&self) -> *mut c_void {
        self.bottom
    }

    /// Returns the size of the stack between top() and bottom().
    #[inline]
    pub fn len(&self) -> usize {
        self.top as usize - self.bottom as usize
    }

    /// Returns the minimal stack size allowed by the current platform.
    #[inline]
    pub fn min_size() -> usize {
        sys::min_stack_size()
    }

    /// Allocates a new stack of `size`.
    fn allocate(mut size: usize, protected: bool) -> Result<SysStack, StackError> {
        let page_size = sys::page_size();
        let min_stack_size = sys::min_stack_size();
        let max_stack_size = sys::max_stack_size();
        let add_shift = i32::from(protected);
        let add = page_size << add_shift;

        if size < min_stack_size {
            size = min_stack_size;
        }

        size = (size - 1) & !(page_size.overflowing_sub(1).0);

        if let Some(size) = size.checked_add(add) {
            if size <= max_stack_size {
                let mut ret = unsafe { sys::allocate_stack(size) };

                if protected {
                    if let Ok(stack) = ret {
                        ret = unsafe { sys::protect_stack(&stack) };
                    }
                }

                return ret.map_err(StackError::IoError);
            }
        }

        Err(StackError::ExceedsMaximumSize(max_stack_size - add))
    }
}

unsafe impl Send for SysStack {}

/// generator stack
/// this struct will not dealloc the memory
/// instead StackBox<> would track it's usage and dealloc it
pub struct Stack {
    buf: SysStack,
}

impl Stack {
    /// Allocate a new stack of `size`. If size = 0, this is a `dummy_stack`
    pub fn new(size: usize) -> Stack {
        let track = (size & 1) != 0;

        let bytes = usize::max(size * std::mem::size_of::<usize>(), SysStack::min_size());

        let buf = SysStack::allocate(bytes, true).expect("failed to alloc sys stack");

        let stk = Stack { buf };

        // if size is not even we do the full foot print test
        let count = if track {
            stk.size()
        } else {
            // we only check the last few words
            8
        };

        unsafe {
            let buf = stk.buf.bottom as *mut usize;
            ptr::write_bytes(buf, 0xEE, count);
        }

        // init the stack box usage
        let offset = stk.get_offset();
        unsafe { *offset = 1 };

        stk
    }

    /// get used stack size
    pub fn get_used_size(&self) -> usize {
        let mut offset: usize = 0;
        unsafe {
            let mut magic: usize = 0xEE;
            ptr::write_bytes(&mut magic, 0xEE, 1);
            let mut ptr = self.buf.bottom as *mut usize;
            while *ptr == magic {
                offset += 1;
                ptr = ptr.offset(1);
            }
        }
        let cap = self.size();
        cap - offset
    }

    /// get the stack cap
    #[inline]
    pub fn size(&self) -> usize {
        self.buf.len() / std::mem::size_of::<usize>()
    }

    /// Point to the high end of the allocated stack
    pub fn end(&self) -> *mut usize {
        let offset = self.get_offset();
        unsafe { (self.buf.top as *mut usize).offset(0 - *offset as isize) }
    }

    /// Point to the low end of the allocated stack
    pub fn begin(&self) -> *mut usize {
        self.buf.bottom as *mut _
    }

    /// alloc buffer on this stack
    pub fn alloc_uninit_box<T>(&mut self) -> MaybeUninit<StackBox<T>> {
        // the first obj should set need drop to non zero
        StackBox::<T>::new_uninit(self, 1)
    }

    // get offset
    fn get_offset(&self) -> *mut usize {
        unsafe { (self.buf.top as *mut usize).offset(-1) }
    }

    // dealloc the stack
    fn drop_stack(&self) {
        if self.buf.len() == 0 {
            return;
        }
        let page_size = sys::page_size();
        let guard = (self.buf.bottom as usize - page_size) as *mut c_void;
        let size_with_guard = self.buf.len() + page_size;
        unsafe {
            sys::deallocate_stack(guard, size_with_guard);
        }
    }

    fn shadow_clone(&self) -> Self {
        Stack {
            buf: SysStack {
                top: self.buf.top,
                bottom: self.buf.bottom,
            },
        }
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = self.get_offset();
        write!(f, "Stack<{:?}, Offset={}>", self.buf, unsafe { *offset })
    }
}
