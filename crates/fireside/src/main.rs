use gpui::*;

struct Counter {
    value: usize,
}

impl Counter {
    fn increment(&mut self, cx: &mut ModelContext<Self>) {
        self.value += 1;
        cx.notify();
    }
}

struct Observer {
    value: usize,
}

impl Observer {
    fn new(counter: &Model<Counter>, cx: &mut ModelContext<Self>) -> Self {
        cx.observe(counter, Self::counter_changed).detach();
        Self {
            value: counter.read(cx).value * 2,
        }
    }

    fn counter_changed(&mut self, counter: Model<Counter>, cx: &mut ModelContext<Self>) {
        self.value = counter.read(cx).value * 2;
        cx.notify();
    }
}

struct RootView {
    counter: Model<Counter>,
    observer: Model<Observer>,
}

impl RootView {
    fn new(counter: Model<Counter>, observer: Model<Observer>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&counter, |_, _, cx| cx.notify()).detach();
        cx.observe(&observer, |_, _, cx| cx.notify()).detach();
        Self { counter, observer }
    }
}

impl Render for RootView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .border()
            .border_color(rgb(0x2B2F31))
            .bg(rgb(0x202425))
            .text_color(rgb(0xECEDEE))
            .flex()
            .flex_col()
            .gap_2()
            .child(format!("Counter: {}", self.counter.read(cx).value))
            .child(format!("Observer: {}", self.observer.read(cx).value))
            .child(
                div()
                    .child("Increment")
                    .w_32()
                    .h_32()
                    .bg(red())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, cx| {
                            this.counter.update(cx, |counter, cx| {
                                counter.increment(cx);
                            })
                        }),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx| {
        cx.activate(true);

        let counter: Model<Counter> = cx.new_model(|_| Counter { value: 0 });
        dbg!(counter.read(cx).value);

        let observer = cx.new_model(|cx| Observer::new(&counter, cx));

        counter.update(cx, |counter, cx| {
            println!("about to increment counter...");
            counter.increment(cx);
            dbg!(counter.value);
        });

        let window = cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| RootView::new(counter, observer, cx))
        });
    });
}
