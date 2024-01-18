use gpui::{prelude::*, App, AppContext, EventEmitter, Model, ModelContext};

struct Counter {
    count: usize,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let counter: Model<Counter> = cx.new_model(|_cx| Counter { count: 0 });
        let subscriber = cx.new_model(|cx: &mut ModelContext<Counter>| {
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
