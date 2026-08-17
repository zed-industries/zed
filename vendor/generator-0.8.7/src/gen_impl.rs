//! # generator
//!
//! Rust generator implementation
//!

use crate::detail::gen_init;
use crate::reg_context::RegContext;
use crate::rt::{Context, ContextStack, Error};
use crate::scope::Scope;
use crate::stack::{Func, Stack, StackBox};

use std::any::Any;
use std::fmt;
use std::marker::PhantomData;
use std::panic;
use std::thread;

/// The default stack size for generators, in bytes.
// windows has a minimal size as 0x4a8!!!!
pub const DEFAULT_STACK_SIZE: usize = 0x1000;

#[inline]
#[cold]
fn cold() {}

// #[inline]
// fn likely(b: bool) -> bool {
//     if !b { cold() }
//     b
// }

#[inline]
pub(crate) fn unlikely(b: bool) -> bool {
    if b {
        cold()
    }
    b
}

/// the generator obj type, the functor passed to it must be Send
pub struct GeneratorObj<'a, A, T, const LOCAL: bool> {
    gen: StackBox<GeneratorImpl<'a, A, T>>,
}

/// the generator type, the functor passed to it must be Send
pub type Generator<'a, A, T> = GeneratorObj<'a, A, T, false>;

// only when A, T and Functor are all sendable, the generator could be send
unsafe impl<A: Send, T: Send> Send for Generator<'static, A, T> {}

impl<'a, A, T> Generator<'a, A, T> {
    /// init a heap based generator with scoped closure
    pub fn scoped_init<F>(&mut self, f: F)
    where
        for<'scope> F: FnOnce(Scope<'scope, 'a, A, T>) -> T + Send + 'a,
        T: Send + 'a,
        A: Send + 'a,
    {
        self.gen.scoped_init(f);
    }

    /// init a heap based generator
    // it's can be used to re-init a 'done' generator before it's get dropped
    pub fn init_code<F: FnOnce() -> T + Send + 'a>(&mut self, f: F)
    where
        T: Send + 'a,
    {
        self.gen.init_code(f);
    }
}

/// the local generator type, can't Send
pub type LocalGenerator<'a, A, T> = GeneratorObj<'a, A, T, true>;

impl<'a, A, T> LocalGenerator<'a, A, T> {
    /// init a heap based generator with scoped closure
    pub fn scoped_init<F>(&mut self, f: F)
    where
        for<'scope> F: FnOnce(Scope<'scope, 'a, A, T>) -> T + 'a,
        T: 'a,
        A: 'a,
    {
        self.gen.scoped_init(f);
    }
}

impl<'a, A, T, const LOCAL: bool> GeneratorObj<'a, A, T, LOCAL> {
    /// Constructs a Generator from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw(raw: *mut usize) -> Self {
        GeneratorObj {
            gen: StackBox::from_raw(raw as *mut GeneratorImpl<'a, A, T>),
        }
    }

    /// Consumes the `Generator`, returning a wrapped raw pointer.
    #[inline]
    pub fn into_raw(self) -> *mut usize {
        let ret = self.gen.as_ptr() as *mut usize;
        std::mem::forget(self);
        ret
    }

    /// prefetch the generator into cache
    #[inline]
    pub fn prefetch(&self) {
        self.gen.prefetch();
    }

    /// prepare the para that passed into generator before send
    #[inline]
    pub fn set_para(&mut self, para: A) {
        self.gen.set_para(para);
    }

    /// set the generator local data
    #[inline]
    pub fn set_local_data(&mut self, data: *mut u8) {
        self.gen.set_local_data(data);
    }

    /// get the generator local data
    #[inline]
    pub fn get_local_data(&self) -> *mut u8 {
        self.gen.get_local_data()
    }

    /// get the generator panic data
    #[inline]
    pub fn get_panic_data(&mut self) -> Option<Box<dyn Any + Send>> {
        self.gen.get_panic_data()
    }

    /// resume the generator without touch the para
    /// you should call `set_para` before this method
    #[inline]
    pub fn resume(&mut self) -> Option<T> {
        self.gen.resume()
    }

    /// `raw_send`
    #[inline]
    pub fn raw_send(&mut self, para: Option<A>) -> Option<T> {
        self.gen.raw_send(para)
    }

    /// send interface
    pub fn send(&mut self, para: A) -> T {
        self.gen.send(para)
    }

    /// cancel the generator
    /// this will trigger a Cancel panic to unwind the stack and finish the generator
    pub fn cancel(&mut self) {
        self.gen.cancel()
    }

    /// is finished
    #[inline]
    pub fn is_done(&self) -> bool {
        self.gen.is_done()
    }

    /// get stack total size and used size in word
    pub fn stack_usage(&self) -> (usize, usize) {
        self.gen.stack_usage()
    }
}

impl<T, const LOCAL: bool> Iterator for GeneratorObj<'_, (), T, LOCAL> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.resume()
    }
}

impl<A, T, const LOCAL: bool> fmt::Debug for GeneratorObj<'_, A, T, LOCAL> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generator<{}, Output={}, Local={}> {{ ... }}",
            std::any::type_name::<A>(),
            std::any::type_name::<T>(),
            LOCAL
        )
    }
}

/// Generator helper
pub struct Gn<A = ()> {
    dummy: PhantomData<A>,
}

impl<A> Gn<A> {
    /// create a scoped generator with default stack size
    pub fn new_scoped<'a, T, F>(f: F) -> Generator<'a, A, T>
    where
        for<'scope> F: FnOnce(Scope<'scope, 'a, A, T>) -> T + Send + 'a,
        T: Send + 'a,
        A: Send + 'a,
    {
        Self::new_scoped_opt(DEFAULT_STACK_SIZE, f)
    }

    /// create a scoped local generator with default stack size
    pub fn new_scoped_local<'a, T, F>(f: F) -> LocalGenerator<'a, A, T>
    where
        F: FnOnce(Scope<A, T>) -> T + 'a,
        T: 'a,
        A: 'a,
    {
        Self::new_scoped_opt_local(DEFAULT_STACK_SIZE, f)
    }

    /// create a scoped generator with specified stack size
    pub fn new_scoped_opt<'a, T, F>(size: usize, f: F) -> Generator<'a, A, T>
    where
        for<'scope> F: FnOnce(Scope<'scope, 'a, A, T>) -> T + Send + 'a,
        T: Send + 'a,
        A: Send + 'a,
    {
        let mut gen = GeneratorImpl::<A, T>::new(Stack::new(size));
        gen.scoped_init(f);
        Generator { gen }
    }

    /// create a scoped local generator with specified stack size
    pub fn new_scoped_opt_local<'a, T, F>(size: usize, f: F) -> LocalGenerator<'a, A, T>
    where
        F: FnOnce(Scope<A, T>) -> T + 'a,
        T: 'a,
        A: 'a,
    {
        let mut gen = GeneratorImpl::<A, T>::new(Stack::new(size));
        gen.scoped_init(f);
        LocalGenerator { gen }
    }
}

impl<A: Any> Gn<A> {
    /// create a new generator with default stack size
    #[allow(clippy::new_ret_no_self)]
    #[deprecated(since = "0.6.18", note = "please use `scope` version instead")]
    pub fn new<'a, T: Any, F>(f: F) -> Generator<'a, A, T>
    where
        F: FnOnce() -> T + Send + 'a,
    {
        Self::new_opt(DEFAULT_STACK_SIZE, f)
    }

    /// create a new generator with specified stack size
    // the `may` library use this API so we can't deprecated it yet.
    pub fn new_opt<'a, T: Any, F>(size: usize, f: F) -> Generator<'a, A, T>
    where
        F: FnOnce() -> T + Send + 'a,
    {
        let mut gen = GeneratorImpl::<A, T>::new(Stack::new(size));
        gen.init_context();
        gen.init_code(f);
        Generator { gen }
    }
}

/// `GeneratorImpl`
#[repr(C)]
struct GeneratorImpl<'a, A, T> {
    // run time context
    context: Context,
    // stack
    stack: Stack,
    // save the input
    para: Option<A>,
    // save the output
    ret: Option<T>,
    // boxed functor
    f: Option<Func>,
    // phantom lifetime
    phantom: PhantomData<&'a T>,
}

impl<A: Any, T: Any> GeneratorImpl<'_, A, T> {
    /// create a new generator with default stack size
    fn init_context(&mut self) {
        unsafe {
            std::ptr::write(
                self.context.para.as_mut_ptr(),
                &mut self.para as &mut dyn Any,
            );
            std::ptr::write(self.context.ret.as_mut_ptr(), &mut self.ret as &mut dyn Any);
        }
    }
}

impl<'a, A, T> GeneratorImpl<'a, A, T> {
    /// create a new generator with specified stack size
    fn new(mut stack: Stack) -> StackBox<Self> {
        // the stack box would finally dealloc the stack!
        unsafe {
            let mut stack_box = stack.alloc_uninit_box::<GeneratorImpl<'a, A, T>>();
            (*stack_box.as_mut_ptr()).init(GeneratorImpl {
                para: None,
                stack,
                ret: None,
                f: None,
                context: Context::new(),
                phantom: PhantomData,
            });
            stack_box.assume_init()
        }
    }

    /// prefetch the generator into cache
    #[inline]
    pub fn prefetch(&self) {
        self.context.regs.prefetch();
    }

    /// init a heap based generator with scoped closure
    fn scoped_init<F>(&mut self, f: F)
    where
        for<'scope> F: FnOnce(Scope<'scope, 'a, A, T>) -> T + 'a,
        T: 'a,
        A: 'a,
    {
        use std::mem::transmute;
        let scope: Scope<A, T> = unsafe { transmute(Scope::new(&mut self.para, &mut self.ret)) };
        self.init_code(move || f(scope));
    }

    /// init a heap based generator
    // it's can be used to re-init a 'done' generator before it's get dropped
    fn init_code<F: FnOnce() -> T + 'a>(&mut self, f: F)
    where
        T: 'a,
    {
        // make sure the last one is finished
        if self.f.is_none() && self.context._ref == 0 {
            self.cancel();
        } else {
            let _ = self.f.take();
        }

        // init ctx parent to itself, this would be the new top
        self.context.parent = &mut self.context;

        // init the ref to 0 means that it's ready to start
        self.context._ref = 0;
        let ret = &mut self.ret as *mut _;
        // alloc the function on stack
        let func = StackBox::new_fn_once(&mut self.stack, move || {
            let r = f();
            unsafe { *ret = Some(r) };
        });

        self.f = Some(func);

        let guard = (self.stack.begin() as usize, self.stack.end() as usize);
        self.context.stack_guard = guard;
        self.context.regs.init_with(
            gen_init,
            0,
            &mut self.f as *mut _ as *mut usize,
            &self.stack,
        );
    }

    /// resume the generator
    #[inline]
    fn resume_gen(&mut self) {
        let env = ContextStack::current();
        // get the current regs
        let cur = &mut env.top().regs;

        // switch to new context, always use the top context's reg
        // for normal generator self.context.parent == self.context
        // for coroutine self.context.parent == top generator context
        debug_assert!(!self.context.parent.is_null());
        let top = unsafe { &mut *self.context.parent };

        // save current generator context on stack
        env.push_context(&mut self.context);

        // swap to the generator
        RegContext::swap(cur, &top.regs);

        // comes back, check the panic status
        // this would propagate the panic until root context
        // if it's a coroutine just stop propagate
        if !self.context.local_data.is_null() {
            return;
        }

        if let Some(err) = self.context.err.take() {
            // pass the error to the parent until root
            panic::resume_unwind(err);
        }
    }

    #[inline]
    fn is_started(&self) -> bool {
        // when the f is consumed we think it's running
        self.f.is_none()
    }

    /// prepare the para that passed into generator before send
    #[inline]
    fn set_para(&mut self, para: A) {
        self.para = Some(para);
    }

    /// set the generator local data
    #[inline]
    fn set_local_data(&mut self, data: *mut u8) {
        self.context.local_data = data;
    }

    /// get the generator local data
    #[inline]
    fn get_local_data(&self) -> *mut u8 {
        self.context.local_data
    }

    /// get the generator panic data
    #[inline]
    fn get_panic_data(&mut self) -> Option<Box<dyn Any + Send>> {
        self.context.err.take()
    }

    /// resume the generator without touch the para
    /// you should call `set_para` before this method
    #[inline]
    fn resume(&mut self) -> Option<T> {
        if unlikely(self.is_done()) {
            return None;
        }

        // every time we call the function, increase the ref count
        // yield will decrease it and return will not
        self.context._ref += 1;
        self.resume_gen();

        self.ret.take()
    }

    /// `raw_send`
    #[inline]
    fn raw_send(&mut self, para: Option<A>) -> Option<T> {
        if unlikely(self.is_done()) {
            return None;
        }

        // this is the passed in value of the send primitive
        // the yield part would read out this value in the next round
        self.para = para;

        // every time we call the function, increase the ref count
        // yield will decrease it and return will not
        self.context._ref += 1;
        self.resume_gen();

        self.ret.take()
    }

    /// send interface
    fn send(&mut self, para: A) -> T {
        let ret = self.raw_send(Some(para));
        ret.expect("send got None return")
    }

    /// cancel the generator without any check
    #[inline]
    fn raw_cancel(&mut self) {
        // tell the func to panic
        // so that we can stop the inner func
        self.context._ref = 2;
        // save the old panic hook, we don't want to print anything for the Cancel
        let old = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        self.resume_gen();
        panic::set_hook(old);
    }

    /// cancel the generator
    /// this will trigger a Cancel panic to unwind the stack
    fn cancel(&mut self) {
        if self.is_done() {
            return;
        }

        // consume the fun if it's not started
        if !self.is_started() {
            self.f.take();
            self.context._ref = 1;
        } else {
            self.raw_cancel();
        }
    }

    /// is finished
    #[inline]
    fn is_done(&self) -> bool {
        self.is_started() && (self.context._ref & 0x3) != 0
    }

    /// get stack total size and used size in word
    fn stack_usage(&self) -> (usize, usize) {
        (self.stack.size(), self.stack.get_used_size())
    }
}

impl<A, T> Drop for GeneratorImpl<'_, A, T> {
    fn drop(&mut self) {
        // when the thread is already panic, do nothing
        if thread::panicking() {
            return;
        }

        if !self.is_started() {
            // not started yet, just drop the gen
            return;
        }

        if !self.is_done() {
            trace!("generator is not done while drop");
            self.raw_cancel()
        }

        assert!(self.is_done());

        let (total_stack, used_stack) = self.stack_usage();
        if used_stack < total_stack {
            // here we should record the stack in the class
            // next time will just use
            // set_stack_size::<F>(used_stack);
        } else {
            error!("stack overflow detected!");
            panic::panic_any(Error::StackErr);
        }
    }
}
