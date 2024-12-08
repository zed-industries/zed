use gpui::{prelude::*, App, AppContext, EventEmitter, Model};

struct Counter {
    count: usize,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let counter: Model<Counter> = cx.new_model(|_model, _cx| Counter { count: 0 });
        let subscriber = cx.new_model(|model: &Model<Counter>, cx: &mut AppContext| {
            model
                .subscribe(
                    &counter,
                    cx,
                    |subscriber: &mut Counter, _emitter, event: &Change, _model, _cx| {
                        subscriber.count += event.increment * 2;
                    },
                )
                .detach();

            Counter {
                count: counter.read(cx).count * 2,
            }
        });

        counter.update(cx, |counter, model, cx| {
            counter.count += 2;
            model.notify(cx);
            model.emit(cx, Change { increment: 2 });
        });

        assert_eq!(subscriber.read(cx).count, 4);
    });
}
