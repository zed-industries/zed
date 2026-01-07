//! Example demonstrating GPUI's testing infrastructure.
//!
//! When run normally, this displays an interactive counter window.
//! The tests below demonstrate various GPUI testing patterns.
//!
//! Run the app: cargo run -p gpui --example testing
//! Run tests:   cargo test -p gpui --example testing --features test-support

use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, Render, Task, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, size,
};

actions!(counter, [Increment, Decrement]);

struct Counter {
    count: i32,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
}

/// Event emitted by Counter
struct CounterEvent;

impl gpui::EventEmitter<CounterEvent> for Counter {}

impl Counter {
    fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe_self(|this: &mut Self, _event: &CounterEvent, _cx| {
            this.count = 999;
        });

        Self {
            count: 0,
            focus_handle: cx.focus_handle(),
            _subscription: subscription,
        }
    }

    fn increment(&mut self, _: &Increment, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }

    fn decrement(&mut self, _: &Decrement, _window: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.notify();
    }

    fn load(&self, cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            // Simulate loading data (e.g., from disk or network)
            this.update(cx, |counter, _| {
                counter.count = 100;
            })
            .ok();
        })
    }

    fn reload(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            // Simulate reloading data in the background
            this.update(cx, |counter, _| {
                counter.count += 50;
            })
            .ok();
        })
        .detach();
    }
}

impl Focusable for Counter {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("counter")
            .key_context("Counter")
            .on_action(cx.listener(Self::increment))
            .on_action(cx.listener(Self::decrement))
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_4()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_3xl()
                    .text_color(rgb(0xcdd6f4))
                    .child(format!("{}", self.count)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("decrement")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x313244))
                            .hover(|s| s.bg(rgb(0x45475a)))
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(rgb(0xcdd6f4))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.decrement(&Decrement, window, cx)
                            }))
                            .child("−"),
                    )
                    .child(
                        div()
                            .id("increment")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x313244))
                            .hover(|s| s.bg(rgb(0x45475a)))
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(rgb(0xcdd6f4))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.increment(&Increment, window, cx)
                            }))
                            .child("+"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("load")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x313244))
                            .hover(|s| s.bg(rgb(0x45475a)))
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(rgb(0xcdd6f4))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.load(cx).detach();
                            }))
                            .child("Load"),
                    )
                    .child(
                        div()
                            .id("reload")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x313244))
                            .hover(|s| s.bg(rgb(0x45475a)))
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(rgb(0xcdd6f4))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.reload(cx);
                            }))
                            .child("Reload"),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6c7086))
                    .child("Press ↑/↓ or click buttons"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, Some("Counter")),
            gpui::KeyBinding::new("down", Decrement, Some("Counter")),
        ]);

        let bounds = Bounds::centered(None, size(px(300.), px(200.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let counter = cx.new(|cx| Counter::new(cx));
                counter.focus_handle(cx).focus(window, cx);
                counter
            },
        )
        .unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use rand::prelude::*;

    /// Here's a basic GPUI test. Just add the macro and take a TestAppContext as an argument!
    ///
    /// Note that synchronous side effects run immediately after your "update*" calls complete.
    #[gpui::test]
    fn basic_testing(cx: &mut TestAppContext) {
        let counter = cx.new(|cx| Counter::new(cx));

        counter.update(cx, |counter, _| {
            counter.count = 42;
        });

        // Note that TestAppContext doesn't support `read(cx)`
        let updated = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(updated, 42);

        // Emit an event - the subscriber will run immediately after the update finishes
        counter.update(cx, |_, cx| {
            cx.emit(CounterEvent);
        });

        let count_after_update = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(
            count_after_update, 999,
            "Side effects should run after update completes"
        );
    }

    /// Tests which involve the window require you to construct a VisualTestContext.
    /// Just like synchronous side effects, the window will be drawn after every "update*"
    /// call, so you can test render-dependent behavior.
    #[gpui::test]
    fn test_counter_in_window(cx: &mut TestAppContext) {
        let window = cx.update(|cx| {
            cx.open_window(Default::default(), |_, cx| cx.new(|cx| Counter::new(cx)))
                .unwrap()
        });

        let mut cx = VisualTestContext::from_window(window.into(), cx);
        let counter = window.root(&mut cx).unwrap();

        // Action dispatch depends on the element tree to resolve which action handler
        // to call, and this works exactly as you'd expect in a test.
        let focus_handle = counter.read_with(&cx, |counter, _| counter.focus_handle.clone());
        cx.update(|window, cx| {
            focus_handle.dispatch_action(&Increment, window, cx);
        });

        let count_after = counter.read_with(&cx, |counter, _| counter.count);
        assert_eq!(
            count_after, 1,
            "Action dispatched via focus handle should increment"
        );
    }

    /// GPUI tests can also be async, simply add the async keyword before the test.
    /// Note that the test executor is single thread, so async side effects (including
    /// background tasks) won't run until you explicitly yield control.
    #[gpui::test]
    async fn test_async_operations(cx: &mut TestAppContext) {
        let counter = cx.new(|cx| Counter::new(cx));

        // Tasks can be awaited directly
        counter.update(cx, |counter, cx| counter.load(cx)).await;

        let count = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(count, 100, "Load task should have set count to 100");

        // But side effects don't run until you yield control
        counter.update(cx, |counter, cx| counter.reload(cx));

        let count = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(count, 100, "Detached reload task shouldn't have run yet");

        // This runs all pending tasks
        cx.run_until_parked();

        let count = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(count, 150, "Reload task should have run after parking");
    }

    /// Note that the test executor panics if you await a future that waits on
    /// something outside GPUI's control, like a reading a file or network IO.
    /// You should mock external systems where possible, as this feature can be used
    /// to detect potential deadlocks in your async code.
    ///
    /// However, if you want to disable this check use `allow_parking()`
    #[gpui::test]
    async fn test_allow_parking(cx: &mut TestAppContext) {
        // Allow the thread to park
        cx.executor().allow_parking();

        // Simulate an external system (like a file system) with an OS thread
        let (tx, rx) = futures::channel::oneshot::channel();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(5));
            tx.send(42).ok();
        });

        // Without allow_parking(), this await would panic because GPUI's
        // scheduler runs out of tasks while waiting for the external thread.
        let result = rx.await.unwrap();
        assert_eq!(result, 42);
    }

    /// GPUI also provides support for property testing, via the iterations flag
    #[gpui::test(iterations = 10)]
    fn test_counter_random_operations(cx: &mut TestAppContext, mut rng: StdRng) {
        let counter = cx.new(|cx| Counter::new(cx));

        // Perform random increments/decrements
        let mut expected = 0i32;
        for _ in 0..100 {
            if rng.random_bool(0.5) {
                expected += 1;
                counter.update(cx, |counter, _| counter.count += 1);
            } else {
                expected -= 1;
                counter.update(cx, |counter, _| counter.count -= 1);
            }
        }

        let actual = counter.read_with(cx, |counter, _| counter.count);
        assert_eq!(
            actual, expected,
            "Counter should match expected after random ops"
        );
    }

    /// Now, all of those tests are good, but GPUI also provides strong support for testing distributed systems.
    /// Let's setup a mock network and enhance the counter to send messages over it.
    mod distributed_systems {
        use std::sync::{Arc, Mutex};

        /// A mock network that delivers messages between two peers.
        struct MockNetwork {
            a_to_b: Vec<i32>,
            b_to_a: Vec<i32>,
        }

        impl MockNetwork {
            fn new() -> Arc<Mutex<Self>> {
                Arc::new(Mutex::new(Self {
                    a_to_b: Vec::new(),
                    b_to_a: Vec::new(),
                }))
            }

            fn a_client(network: &Arc<Mutex<Self>>) -> NetworkClient {
                NetworkClient {
                    network: network.clone(),
                    is_a: true,
                }
            }

            fn b_client(network: &Arc<Mutex<Self>>) -> NetworkClient {
                NetworkClient {
                    network: network.clone(),
                    is_a: false,
                }
            }
        }

        /// A client handle for sending/receiving messages over the mock network.
        #[derive(Clone)]
        struct NetworkClient {
            network: Arc<Mutex<MockNetwork>>,
            is_a: bool,
        }

        impl NetworkClient {
            fn send(&self, value: i32) {
                let mut network = self.network.lock().unwrap();
                if self.is_a {
                    network.b_to_a.push(value);
                } else {
                    network.a_to_b.push(value);
                }
            }

            fn receive_all(&self) -> Vec<i32> {
                let mut network = self.network.lock().unwrap();
                if self.is_a {
                    network.a_to_b.drain(..).collect()
                } else {
                    network.b_to_a.drain(..).collect()
                }
            }
        }

        use gpui::Context;

        /// A networked counter that can send/receive over a mock network.
        struct NetworkedCounter {
            count: i32,
            client: NetworkClient,
        }

        impl NetworkedCounter {
            fn new(client: NetworkClient) -> Self {
                Self { count: 0, client }
            }

            /// Increment the counter and broadcast the change.
            fn increment(&mut self, delta: i32, cx: &mut Context<Self>) {
                self.count += delta;

                cx.background_spawn({
                    let client = self.client.clone();
                    async move {
                        client.send(delta);
                    }
                })
                .detach();
            }

            /// Process incoming increment requests.
            fn sync(&mut self) {
                for delta in self.client.receive_all() {
                    self.count += delta;
                }
            }

            /// Like increment, but tracks when the background send executes.
            fn increment_tracked(
                &mut self,
                delta: i32,
                cx: &mut Context<Self>,
                order: Arc<Mutex<Vec<i32>>>,
            ) {
                self.count += delta;

                cx.background_spawn({
                    let client = self.client.clone();
                    async move {
                        order.lock().unwrap().push(delta);
                        client.send(delta);
                    }
                })
                .detach();
            }
        }

        use super::*;

        /// You can simulate distributed systems with multiple app contexts, simply by adding
        /// additional parameters.
        #[gpui::test]
        fn test_app_sync(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
            let network = MockNetwork::new();

            let a = cx_a.new(|_| NetworkedCounter::new(MockNetwork::a_client(&network)));
            let b = cx_b.new(|_| NetworkedCounter::new(MockNetwork::b_client(&network)));

            // B increments locally and broadcasts the delta
            b.update(cx_b, |b, cx| b.increment(42, cx));
            b.read_with(cx_b, |b, _| assert_eq!(b.count, 42)); // B's count is set immediately
            a.read_with(cx_a, |a, _| assert_eq!(a.count, 0)); // A's count is in a side effect

            cx_b.run_until_parked(); // Send the delta from B
            a.update(cx_a, |a, _| a.sync()); // Receive the delta at A

            b.read_with(cx_b, |b, _| assert_eq!(b.count, 42)); // Both counts now match
            a.read_with(cx_a, |a, _| assert_eq!(a.count, 42));
        }

        /// Multiple apps can run concurrently, and to capture this each test app shares
        /// a dispatcher. Whenever you call `run_until_parked`, the dispatcher will randomly
        /// pick which app's tasks to run next. This allows you to test that your distributed code
        /// is robust to different execution orderings.
        #[gpui::test(iterations = 10)]
        fn test_random_interleaving(
            cx_a: &mut TestAppContext,
            cx_b: &mut TestAppContext,
            mut rng: StdRng,
        ) {
            let network = MockNetwork::new();

            // Track execution order
            let actual_order = Arc::new(Mutex::new(Vec::new()));
            let mut original_order = Vec::new();
            let a = cx_a.new(|_| NetworkedCounter::new(MockNetwork::a_client(&network)));
            let b = cx_b.new(|_| NetworkedCounter::new(MockNetwork::b_client(&network)));

            let num_operations: usize = rng.random_range(3..8);

            for i in 0..num_operations {
                let id = i as i32;
                let which = rng.random_bool(0.5);

                original_order.push(id);
                if which {
                    b.update(cx_b, |b, cx| {
                        b.increment_tracked(id, cx, actual_order.clone())
                    });
                } else {
                    a.update(cx_a, |a, cx| {
                        a.increment_tracked(id, cx, actual_order.clone())
                    });
                }
            }

            // This will send all of the pending increment messages, from both a and b
            cx_a.run_until_parked();

            a.update(cx_a, |a, _| a.sync());
            b.update(cx_b, |b, _| b.sync());

            let a_count = a.read_with(cx_a, |a, _| a.count);
            let b_count = b.read_with(cx_b, |b, _| b.count);

            assert_eq!(a_count, b_count, "A and B should have the same count");

            // Nicely format the execution order output.
            // Run this test with `-- --nocapture` to see it!
            let actual = actual_order.lock().unwrap();
            let spawned: Vec<_> = original_order.iter().map(|n| format!("{}", n)).collect();
            let ran: Vec<_> = actual.iter().map(|n| format!("{}", n)).collect();
            let diff: Vec<_> = original_order
                .iter()
                .zip(actual.iter())
                .map(|(o, a)| {
                    if o == a {
                        " ".to_string()
                    } else {
                        "^".to_string()
                    }
                })
                .collect();
            println!("spawned: [{}]", spawned.join(", "));
            println!("ran:     [{}]", ran.join(", "));
            println!("         [{}]", diff.join(", "));
        }
    }
}
