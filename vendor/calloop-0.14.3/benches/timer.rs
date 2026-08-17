use std::time::Duration;

use calloop::timer::TimeoutAction;
use criterion::{criterion_group, criterion_main, Criterion};

fn single(c: &mut Criterion) {
    let mut event_loop = calloop::EventLoop::<()>::try_new().unwrap();
    let loop_handle = event_loop.handle();

    let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
    let mut timeout_token = loop_handle
        .insert_source(timer, |_, _, _| TimeoutAction::Drop)
        .unwrap();

    c.bench_function("extend_single", |b| {
        b.iter(|| {
            loop_handle.remove(timeout_token);

            let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
            timeout_token = loop_handle
                .insert_source(timer, |_, _, _| TimeoutAction::Drop)
                .unwrap();

            event_loop.dispatch(Some(Duration::ZERO), &mut ()).unwrap();
        });
    });
}

fn mixed(c: &mut Criterion) {
    let mut event_loop = calloop::EventLoop::<()>::try_new().unwrap();
    let loop_handle = event_loop.handle();

    let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10 - 1));
    loop_handle
        .insert_source(timer, |_, _, _| TimeoutAction::Drop)
        .unwrap();

    let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
    let mut timeout_token = loop_handle
        .insert_source(timer, |_, _, _| TimeoutAction::Drop)
        .unwrap();

    let timer = calloop::timer::Timer::from_duration(Duration::from_secs(90 * 10));
    loop_handle
        .insert_source(timer, |_, _, _| TimeoutAction::Drop)
        .unwrap();

    c.bench_function("extend_mixed", |b| {
        b.iter(|| {
            loop_handle.remove(timeout_token);

            let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
            timeout_token = loop_handle
                .insert_source(timer, |_, _, _| TimeoutAction::Drop)
                .unwrap();

            event_loop.dispatch(Some(Duration::ZERO), &mut ()).unwrap();
        });
    });
}

fn mixed_multiple(c: &mut Criterion) {
    let mut event_loop = calloop::EventLoop::<()>::try_new().unwrap();
    let loop_handle = event_loop.handle();

    for _ in 0..1000 {
        let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10 - 1));
        loop_handle
            .insert_source(timer, |_, _, _| TimeoutAction::Drop)
            .unwrap();
    }

    let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
    let mut timeout_token = loop_handle
        .insert_source(timer, |_, _, _| TimeoutAction::Drop)
        .unwrap();

    for _ in 0..1000 {
        let timer = calloop::timer::Timer::from_duration(Duration::from_secs(90 * 10));
        loop_handle
            .insert_source(timer, |_, _, _| TimeoutAction::Drop)
            .unwrap();
    }

    c.bench_function("extend_mixed_many", |b| {
        b.iter(|| {
            loop_handle.remove(timeout_token);

            let timer = calloop::timer::Timer::from_duration(Duration::from_secs(60 * 10));
            timeout_token = loop_handle
                .insert_source(timer, |_, _, _| TimeoutAction::Drop)
                .unwrap();

            event_loop.dispatch(Some(Duration::ZERO), &mut ()).unwrap();
        });
    });
}

criterion_group!(benches, single, mixed, mixed_multiple);
criterion_main!(benches);
