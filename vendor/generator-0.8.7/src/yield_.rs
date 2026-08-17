//! # yield
//!
//! generator yield implementation
//!
use std::any::Any;
use std::sync::atomic;

use crate::gen_impl::{unlikely, Generator};
use crate::reg_context::RegContext;
use crate::rt::{is_generator, Context, ContextStack, Error};

/// it's a special return instruction that yield nothing
/// but only terminate the generator safely
#[macro_export]
macro_rules! done {
    () => {{
        return $crate::done();
    }};
}

/// don't use it directly, use done!() macro instead
/// would panic if use in none generator context
#[doc(hidden)]
#[inline]
pub fn done<T>() -> T {
    assert!(is_generator(), "done is only possible in a generator");
    std::panic::panic_any(Error::Done)
}

/// switch back to parent context
#[inline]
pub fn yield_now() {
    let env = ContextStack::current();
    let cur = env.top();
    raw_yield_now(&env, cur);
}

#[inline]
pub fn raw_yield_now(env: &ContextStack, cur: &mut Context) {
    let parent = env.pop_context(cur as *mut _);
    RegContext::swap(&mut cur.regs, &parent.regs);
}

/// raw yield without catch passed in para
#[inline]
fn raw_yield<T: Any>(env: &ContextStack, context: &mut Context, v: T) {
    // check the context
    if unlikely(!context.is_generator()) {
        panic!("yield from none generator context");
    }

    context.set_ret(v);
    context._ref -= 1;
    raw_yield_now(env, context);

    // here we just panic to exit the func
    if unlikely(context._ref != 1) {
        std::panic::panic_any(Error::Cancel);
    }
}

/// yield something without catch passed in para
#[inline]
#[deprecated(since = "0.6.18", note = "please use `scope` version instead")]
pub fn yield_with<T: Any>(v: T) {
    let env = ContextStack::current();
    let context = env.top();
    raw_yield(&env, context, v);
}

/// get the passed in para
#[inline]
#[deprecated(since = "0.6.18", note = "please use `scope` version instead")]
pub fn get_yield<A: Any>() -> Option<A> {
    let context = ContextStack::current().top();
    raw_get_yield(context)
}

/// get the passed in para from context
#[inline]
fn raw_get_yield<A: Any>(context: &mut Context) -> Option<A> {
    // check the context
    if unlikely(!context.is_generator()) {
        {
            error!("get yield from none generator context");
            std::panic::panic_any(Error::ContextErr);
        }
    }

    context.get_para()
}

/// yield and get the send para
// here yield need to return a static lifetime value, which is Any required
// this is fine, but it's totally safe that we can refer to the function block
// since we will come back later
#[inline]
#[deprecated(since = "0.6.18", note = "please use `scope` version instead")]
pub fn yield_<A: Any, T: Any>(v: T) -> Option<A> {
    let env = ContextStack::current();
    let context = env.top();
    raw_yield(&env, context, v);
    atomic::compiler_fence(atomic::Ordering::Acquire);
    raw_get_yield(context)
}

/// `yield_from`
#[deprecated(since = "0.6.18", note = "please use `scope` version instead")]
pub fn yield_from<A: Any, T: Any>(mut g: Generator<A, T>) -> Option<A> {
    let env = ContextStack::current();
    let context = env.top();
    let mut p = context.get_para();
    while unlikely(!g.is_done()) {
        match g.raw_send(p) {
            None => return None,
            Some(r) => raw_yield(&env, context, r),
        }
        p = context.get_para();
    }
    drop(g); // explicitly consume g
    p
}

/// coroutine yield
pub fn co_yield_with<T: Any>(v: T) {
    let env = ContextStack::current();
    let context = env.co_ctx().unwrap();

    // check the context, already checked in co_ctx()
    // if !context.is_generator() {
    //     info!("yield from none coroutine context");
    //     // do nothing, just return
    //     return;
    // }

    // here we just panic to exit the func
    if unlikely(context._ref != 1) {
        std::panic::panic_any(Error::Cancel);
    }

    context.co_set_ret(v);
    context._ref -= 1;

    let parent = env.pop_context(context);
    let top = unsafe { &mut *context.parent };
    // here we should use the top regs
    RegContext::swap(&mut top.regs, &parent.regs);
}

/// coroutine get passed in yield para
pub fn co_get_yield<A: Any>() -> Option<A> {
    ContextStack::current()
        .co_ctx()
        .and_then(|ctx| ctx.co_get_para())
}

/// set current coroutine para in user space
pub fn co_set_para<A: Any>(para: A) {
    if let Some(ctx) = ContextStack::current().co_ctx() {
        ctx.co_set_para(para)
    }
}
