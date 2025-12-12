#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, Priority, RealtimePriority, RunnableMeta,
    RunnableVariant, THREAD_TIMINGS, TaskLabel, TaskTiming, ThreadTaskTimings,
};

use anyhow::Context;
use async_task::Runnable;
use mach2::{
    kern_return::KERN_SUCCESS,
    mach_time::mach_timebase_info_data_t,
    thread_policy::{
        THREAD_EXTENDED_POLICY, THREAD_EXTENDED_POLICY_COUNT, THREAD_PRECEDENCE_POLICY,
        THREAD_PRECEDENCE_POLICY_COUNT, THREAD_TIME_CONSTRAINT_POLICY,
        THREAD_TIME_CONSTRAINT_POLICY_COUNT, thread_extended_policy_data_t,
        thread_precedence_policy_data_t, thread_time_constraint_policy_data_t,
    },
};
use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use std::{
    ffi::c_void,
    mem::MaybeUninit,
    ptr::{NonNull, addr_of},
    time::{Duration, Instant},
};
use util::ResultExt;

/// All items in the generated file are marked as pub, so we're gonna wrap it in a separate mod to prevent
/// these pub items from leaking into public API.
pub(crate) mod dispatch_sys {
    include!(concat!(env!("OUT_DIR"), "/dispatch_sys.rs"));
}

use dispatch_sys::*;
pub(crate) fn dispatch_get_main_queue() -> dispatch_queue_t {
    addr_of!(_dispatch_main_q) as *const _ as dispatch_queue_t
}

pub(crate) struct MacDispatcher;

impl PlatformDispatcher for MacDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        let global_timings = GLOBAL_THREAD_TIMINGS.lock();
        ThreadTaskTimings::convert(&global_timings)
    }

    fn get_current_thread_timings(&self) -> Vec<TaskTiming> {
        THREAD_TIMINGS.with(|timings| {
            let timings = &timings.lock().timings;

            let mut vec = Vec::with_capacity(timings.len());

            let (s1, s2) = timings.as_slices();
            vec.extend_from_slice(s1);
            vec.extend_from_slice(s2);
            vec
        })
    }

    fn is_main_thread(&self) -> bool {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        is_main_thread == YES
    }

    fn dispatch(&self, runnable: RunnableVariant, _: Option<TaskLabel>, priority: Priority) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };

        let queue_priority = match priority {
            Priority::Realtime(_) => unreachable!(),
            Priority::High => DISPATCH_QUEUE_PRIORITY_HIGH as isize,
            Priority::Medium => DISPATCH_QUEUE_PRIORITY_DEFAULT as isize,
            Priority::Low => DISPATCH_QUEUE_PRIORITY_LOW as isize,
        };

        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(queue_priority, 0),
                context,
                trampoline,
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };
        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), context, trampoline);
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(when, queue, context, trampoline);
        }
    }

    fn spawn_realtime(&self, priority: RealtimePriority, f: Box<dyn FnOnce() + Send>) {
        std::thread::spawn(move || {
            match priority {
                RealtimePriority::Audio => set_audio_thread_priority(),
                RealtimePriority::Other => set_high_thread_priority(),
            }
            .context(format!("for priority {:?}", priority))
            .log_err();

            f();
        });
    }
}

fn set_high_thread_priority() -> anyhow::Result<()> {
    // SAFETY: always safe to call
    let thread_id = unsafe { libc::pthread_self() };

    // SAFETY: all sched_param members are valid when initialized to zero.
    let mut sched_param = unsafe { MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
    sched_param.sched_priority = 45;

    let result = unsafe { libc::pthread_setschedparam(thread_id, libc::SCHED_FIFO, &sched_param) };
    if result != 0 {
        anyhow::bail!("failed to set realtime thread priority")
    }

    Ok(())
}

fn set_audio_thread_priority() -> anyhow::Result<()> {
    // https://chromium.googlesource.com/chromium/chromium/+/master/base/threading/platform_thread_mac.mm#93

    // SAFETY: always safe to call
    let thread_id = unsafe { libc::pthread_self() };

    // SAFETY: thread_id is a valid thread id
    let thread_id = unsafe { libc::pthread_mach_thread_np(thread_id) };

    // Fixed priority thread
    let mut policy = thread_extended_policy_data_t { timeshare: 0 };

    // SAFETY: thread_id is a valid thread id
    // SAFETY: thread_extended_policy_data_t is passed as THREAD_EXTENDED_POLICY
    let result = unsafe {
        mach2::thread_policy::thread_policy_set(
            thread_id,
            THREAD_EXTENDED_POLICY,
            &mut policy as *mut _ as *mut _,
            THREAD_EXTENDED_POLICY_COUNT,
        )
    };

    if result != KERN_SUCCESS {
        anyhow::bail!("failed to set thread extended policy");
    }

    // relatively high priority
    let mut precedence = thread_precedence_policy_data_t { importance: 63 };

    // SAFETY: thread_id is a valid thread id
    // SAFETY: thread_precedence_policy_data_t is passed as THREAD_PRECEDENCE_POLICY
    let result = unsafe {
        mach2::thread_policy::thread_policy_set(
            thread_id,
            THREAD_PRECEDENCE_POLICY,
            &mut precedence as *mut _ as *mut _,
            THREAD_PRECEDENCE_POLICY_COUNT,
        )
    };

    if result != KERN_SUCCESS {
        anyhow::bail!("failed to set thread precedence policy");
    }

    const GUARANTEED_AUDIO_DUTY_CYCLE: f32 = 0.75;
    const MAX_AUDIO_DUTY_CYCLE: f32 = 0.85;

    // ~128 frames @ 44.1KHz
    const TIME_QUANTUM: f32 = 2.9;

    const AUDIO_TIME_NEEDED: f32 = GUARANTEED_AUDIO_DUTY_CYCLE * TIME_QUANTUM;
    const MAX_TIME_ALLOWED: f32 = MAX_AUDIO_DUTY_CYCLE * TIME_QUANTUM;

    let mut timebase_info = mach_timebase_info_data_t { numer: 0, denom: 0 };
    // SAFETY: timebase_info is a valid pointer to a mach_timebase_info_data_t struct
    unsafe { mach2::mach_time::mach_timebase_info(&mut timebase_info) };

    let ms_to_abs_time = ((timebase_info.denom as f32) / (timebase_info.numer as f32)) * 1000000f32;

    let mut time_constraints = thread_time_constraint_policy_data_t {
        period: (TIME_QUANTUM * ms_to_abs_time) as u32,
        computation: (AUDIO_TIME_NEEDED * ms_to_abs_time) as u32,
        constraint: (MAX_TIME_ALLOWED * ms_to_abs_time) as u32,
        preemptible: 0,
    };

    // SAFETY: thread_id is a valid thread id
    // SAFETY: thread_precedence_pthread_time_constraint_policy_data_t is passed as THREAD_TIME_CONSTRAINT_POLICY
    let result = unsafe {
        mach2::thread_policy::thread_policy_set(
            thread_id,
            THREAD_TIME_CONSTRAINT_POLICY,
            &mut time_constraints as *mut _ as *mut _,
            THREAD_TIME_CONSTRAINT_POLICY_COUNT,
        )
    };

    if result != KERN_SUCCESS {
        anyhow::bail!("failed to set thread time constraint policy");
    }

    Ok(())
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task =
        unsafe { Runnable::<RunnableMeta>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };

    let location = task.metadata().location;

    let start = Instant::now();
    let timing = TaskTiming {
        location,
        start,
        end: None,
    };

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        if let Some(last_timing) = timings.iter_mut().rev().next() {
            if last_timing.location == timing.location {
                return;
            }
        }

        timings.push_back(timing);
    });

    task.run();
    let end = Instant::now();

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        let Some(last_timing) = timings.iter_mut().rev().next() else {
            return;
        };
        last_timing.end = Some(end);
    });
}

extern "C" fn trampoline_compat(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };

    let location = core::panic::Location::caller();

    let start = Instant::now();
    let timing = TaskTiming {
        location,
        start,
        end: None,
    };
    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        if let Some(last_timing) = timings.iter_mut().rev().next() {
            if last_timing.location == timing.location {
                return;
            }
        }

        timings.push_back(timing);
    });

    task.run();
    let end = Instant::now();

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        let Some(last_timing) = timings.iter_mut().rev().next() else {
            return;
        };
        last_timing.end = Some(end);
    });
}
