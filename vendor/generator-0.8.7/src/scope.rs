//! # yield
//!
//! generator yield implementation
//!

use std::marker::PhantomData;
use std::sync::atomic;

use crate::gen_impl::Generator;
use crate::rt::{Context, ContextStack, Error};
use crate::yield_::raw_yield_now;

/// passed in scope type
/// it not use the context to pass data, but keep it's own data ref
/// this struct provide both compile type info and runtime data
pub struct Scope<'scope, 'a, A, T> {
    para: &'a mut Option<A>,
    ret: &'a mut Option<T>,
    scope: PhantomData<&'scope mut &'scope ()>,
}

impl<'a, A, T> Scope<'_, 'a, A, T> {
    /// create a new scope object
    pub(crate) fn new(para: &'a mut Option<A>, ret: &'a mut Option<T>) -> Self {
        Scope {
            para,
            ret,
            scope: PhantomData,
        }
    }

    /// set current generator return value
    #[inline]
    fn set_ret(&mut self, v: T) {
        *self.ret = Some(v);
    }

    /// raw yield without catch passed in para
    #[inline]
    fn raw_yield(&mut self, env: &ContextStack, context: &mut Context, v: T) {
        // check the context
        if !context.is_generator() {
            panic!("yield from none generator context");
        }

        self.set_ret(v);
        context._ref -= 1;
        raw_yield_now(env, context);

        // here we just panic to exit the func
        if context._ref != 1 {
            std::panic::panic_any(Error::Cancel);
        }
    }

    /// yield something without catch passed in para
    #[inline]
    pub fn yield_with(&mut self, v: T) {
        let env = ContextStack::current();
        let context = env.top();
        self.raw_yield(&env, context, v);
    }

    /// get current generator send para
    #[inline]
    pub fn get_yield(&mut self) -> Option<A> {
        self.para.take()
    }

    /// yield and get the send para
    /// # Safety
    /// When yield out, the reference of the captured data must be still valid
    /// normally, you should always call the `drop` of the generator
    #[inline]
    pub unsafe fn yield_unsafe(&mut self, v: T) -> Option<A> {
        self.yield_with(v);
        atomic::compiler_fence(atomic::Ordering::Acquire);
        self.get_yield()
    }

    /// `yield_from_unsafe`
    /// the from generator must has the same type as itself
    /// # Safety
    /// When yield out, the reference of the captured data must be still valid
    /// normally, you should always call the `drop` of the generator
    pub unsafe fn yield_from_unsafe(&mut self, mut g: Generator<A, T>) -> Option<A> {
        let env = ContextStack::current();
        let context = env.top();
        let mut p = self.get_yield();
        while !g.is_done() {
            match g.raw_send(p) {
                None => return None,
                Some(r) => self.raw_yield(&env, context, r),
            }
            p = self.get_yield();
        }
        drop(g); // explicitly consume g
        p
    }
}

impl<A, T> Scope<'_, 'static, A, T> {
    /// yield and get the send para
    // it's totally safe that we can refer to the function block
    // since we will come back later
    #[inline]
    pub fn yield_(&mut self, v: T) -> Option<A> {
        unsafe { self.yield_unsafe(v) }
    }

    /// `yield_from`
    /// the from generator must has the same type as itself
    pub fn yield_from(&mut self, g: Generator<A, T>) -> Option<A> {
        unsafe { self.yield_from_unsafe(g) }
    }
}
