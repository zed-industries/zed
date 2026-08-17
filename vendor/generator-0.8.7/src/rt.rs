//! # generator run time support
//!
//! generator run time context management
//!
use std::any::Any;
use std::cell::Cell;
use std::mem::MaybeUninit;
use std::ptr;

use crate::reg_context::RegContext;

thread_local! {
    // each thread has it's own generator context stack
    static ROOT_CONTEXT_P: Cell<*mut Context> = const { Cell::new(ptr::null_mut()) };
}

/// yield panic error types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// Done panic
    Done,
    /// Cancel panic
    Cancel,
    /// Type mismatch panic
    TypeErr,
    /// Stack overflow panic
    StackErr,
    /// Wrong Context panic
    ContextErr,
}

/// generator context
#[repr(C)]
#[repr(align(128))]
pub struct Context {
    /// generator regs context
    pub regs: RegContext,
    /// child context
    child: *mut Context,
    /// parent context
    pub parent: *mut Context,
    /// passed in para for send
    pub para: MaybeUninit<*mut dyn Any>,
    /// this is just a buffer for the return value
    pub ret: MaybeUninit<*mut dyn Any>,
    /// track generator ref, yield will -1, send will +1
    pub _ref: usize,
    /// context local storage
    pub local_data: *mut u8,
    /// propagate panic
    pub err: Option<Box<dyn Any + Send>>,
    /// cached stack guard for fast path
    pub stack_guard: (usize, usize),
}

impl Context {
    /// return a default generator context
    pub fn new() -> Context {
        Context {
            regs: RegContext::empty(),
            para: MaybeUninit::zeroed(),
            ret: MaybeUninit::zeroed(),
            _ref: 1, // none zero means it's not running
            err: None,
            child: ptr::null_mut(),
            parent: ptr::null_mut(),
            local_data: ptr::null_mut(),
            stack_guard: (0, 0),
        }
    }

    /// judge it's generator context
    #[inline]
    pub fn is_generator(&self) -> bool {
        !std::ptr::eq(self.parent, self)
    }

    /// get current generator send para
    #[inline]
    pub fn get_para<A>(&mut self) -> Option<A>
    where
        A: Any,
    {
        let para = unsafe {
            let para_ptr = *self.para.as_mut_ptr();
            assert!(!para_ptr.is_null());
            &mut *para_ptr
        };
        match para.downcast_mut::<Option<A>>() {
            Some(v) => v.take(),
            None => type_error::<A>("get yield type mismatch error detected"),
        }
    }

    /// get coroutine send para
    #[inline]
    pub fn co_get_para<A>(&mut self) -> Option<A> {
        let para = unsafe {
            let para_ptr = *self.para.as_mut_ptr();
            debug_assert!(!para_ptr.is_null());
            &mut *(para_ptr as *mut Option<A>)
        };
        para.take()
    }

    // /// set current generator send para
    // #[inline]
    // pub fn set_para<A>(&self, data: A)
    // where
    //     A: Any,
    // {
    //     let para = unsafe { &mut *self.para };
    //     match para.downcast_mut::<Option<A>>() {
    //         Some(v) => *v = Some(data),
    //         None => type_error::<A>("set yield type mismatch error detected"),
    //     }
    // }

    /// set coroutine send para
    /// without check the data type for coroutine performance reason
    #[inline]
    pub fn co_set_para<A>(&mut self, data: A) {
        let para = unsafe {
            let para_ptr = *self.para.as_mut_ptr();
            debug_assert!(!para_ptr.is_null());
            &mut *(para_ptr as *mut Option<A>)
        };
        *para = Some(data);
    }

    /// set current generator return value
    #[inline]
    pub fn set_ret<T>(&mut self, v: T)
    where
        T: Any,
    {
        let ret = unsafe {
            let ret_ptr = *self.ret.as_mut_ptr();
            assert!(!ret_ptr.is_null());
            &mut *ret_ptr
        };
        match ret.downcast_mut::<Option<T>>() {
            Some(r) => *r = Some(v),
            None => type_error::<T>("yield type mismatch error detected"),
        }
    }

    /// set coroutine return value
    /// without check the data type for coroutine performance reason
    #[inline]
    pub fn co_set_ret<T>(&mut self, v: T) {
        let ret = unsafe {
            let ret_ptr = *self.ret.as_mut_ptr();
            debug_assert!(!ret_ptr.is_null());
            &mut *(ret_ptr as *mut Option<T>)
        };
        *ret = Some(v);
    }
}

/// Coroutine managing environment
pub struct ContextStack {
    pub(crate) root: *mut Context,
}

impl ContextStack {
    #[cold]
    fn init_root() -> *mut Context {
        let root = Box::leak(Box::new(Context::new()));
        root.parent = root; // init top to current
        ROOT_CONTEXT_P.set(root);
        root
    }

    /// get the current context stack
    pub fn current() -> ContextStack {
        let mut root = ROOT_CONTEXT_P.get();

        if root.is_null() {
            root = Self::init_root();
        }

        // for windows and macos release version
        // seems add these two lines could fix the bug!!!
        // the MAY project test could fail due to this bug
        // this bug is appeared since rust 1.89 (I have no clue yet)
        #[cfg(all(not(debug_assertions), any(windows, target_os = "macos")))]
        {
            let _thread = std::thread::current();
            let _thread = std::thread::current();
        }

        ContextStack { root }
    }

    /// get the top context
    #[inline]
    pub fn top(&self) -> &'static mut Context {
        unsafe {
            let root = &*self.root;
            &mut *root.parent
        }
    }

    /// get the coroutine context
    #[inline]
    pub fn co_ctx(&self) -> Option<&'static mut Context> {
        // search from top
        let mut ctx = self.top();

        while !std::ptr::eq(ctx, self.root) {
            if !ctx.local_data.is_null() {
                return Some(ctx);
            }
            ctx = unsafe { &mut *ctx.parent };
        }
        // not find any coroutine
        None
    }

    /// push the context to the thread context list
    #[inline]
    pub fn push_context(&self, ctx: *mut Context) {
        let root = unsafe { &mut *self.root };
        let ctx = unsafe { &mut *ctx };
        let top = unsafe { &mut *root.parent };
        let new_top = ctx.parent;

        // link top and new ctx
        top.child = ctx;
        ctx.parent = top;

        // save the new top
        root.parent = new_top;
    }

    /// pop the context from the thread context list and return it's parent context
    #[inline]
    pub fn pop_context(&self, ctx: *mut Context) -> &'static mut Context {
        let root = unsafe { &mut *self.root };
        let ctx = unsafe { &mut *ctx };
        let parent = unsafe { &mut *ctx.parent };

        // save the old top in ctx's parent
        ctx.parent = root.parent;
        // unlink ctx and it's parent
        parent.child = ptr::null_mut();

        // save the new top
        root.parent = parent;

        parent
    }
}

#[inline]
#[cold]
fn type_error<A>(msg: &str) -> ! {
    error!("{msg}, expected type: {}", std::any::type_name::<A>());
    std::panic::panic_any(Error::TypeErr)
}

/// check the current context if it's generator
#[inline]
pub fn is_generator() -> bool {
    let env = ContextStack::current();
    let root = unsafe { &mut *env.root };
    !root.child.is_null()
}

/// get the current context local data
/// only coroutine support local data
#[inline]
pub fn get_local_data() -> *mut u8 {
    let env = ContextStack::current();
    env.co_ctx().map_or(ptr::null_mut(), |ctx| ctx.local_data)
}

pub mod guard {
    use crate::is_generator;
    use crate::rt::ContextStack;
    use crate::stack::sys::page_size;
    use std::ops::Range;

    pub type Guard = Range<usize>;

    pub fn current() -> Guard {
        assert!(is_generator());
        let guard = unsafe { (*(*ContextStack::current().root).child).stack_guard };

        guard.0 - page_size()..guard.1
    }
}

#[cfg(test)]
mod test {
    use super::is_generator;

    #[test]
    fn test_is_context() {
        // this is the root context
        assert!(!is_generator());
    }

    #[test]
    fn test_overflow() {
        use crate::*;
        use std::panic::catch_unwind;

        // test signal mask
        for _ in 0..2 {
            let result = catch_unwind(|| {
                let mut g = Gn::new_scoped(move |_s: Scope<(), ()>| {
                    let guard = super::guard::current();

                    // make sure the compiler does not apply any optimization on it
                    std::hint::black_box(unsafe { *(guard.start as *const usize) });

                    eprintln!("entered unreachable code");
                    std::process::abort();
                });

                g.next();
            });

            assert!(matches!(
                result.map_err(|err| *err.downcast::<Error>().unwrap()),
                Err(Error::StackErr)
            ));
        }
    }
}
