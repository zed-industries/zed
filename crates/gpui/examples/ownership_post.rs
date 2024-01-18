use gpui::{prelude::*, App, AppContext, EventEmitter, Model, ModelContext};

struct Counter {
    count: usize,
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let counter: Model<Counter> = cx.new_model(|_cx| Counter { count: 0 });
        let observer = cx.new_model(|cx: &mut ModelContext<Counter>| {
            cx.observe(&counter, |observer, observed, cx| {
                observer.count = observed.read(cx).count * 2;
            })
            .detach();

            Counter {
                count: counter.read(cx).count * 2,
            }
        });

        counter.update(cx, |counter, cx| {
            counter.count += 1;
            cx.notify();
        });

        assert_eq!(observer.read(cx).count, 2);
    });
}

struct Change {
    delta: isize,
}

impl EventEmitter<Change> for Counter {}
