#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{App, Context, Entity, EventEmitter, prelude::*};
use gpui_platform::application;

struct Counter {
    count: usize,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

fn run_example() {
    application().run(|cx: &mut App| {
        let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
        let subscriber = cx.new(|cx: &mut Context<Counter>| {
            cx.subscribe(&counter, |subscriber, _emitter, event, _cx| {
                subscriber.count += event.increment * 2;
            })
            .detach();

            Counter {
                count: counter.read(cx).count * 2,
            }
        });

        counter.update(cx, |counter, cx| {
            counter.count += 2;
            cx.notify();
            cx.emit(Change { increment: 2 });
        });

        assert_eq!(subscriber.read(cx).count, 4);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
