use gpui::{
    Action,
    actions,
    elements::*,
    impl_actions,
    keymap_matcher::{Binding, KeymapContext, Keystroke},
    platform::{MouseButton, MouseButtonEvent},
    serde_json,
    text_layout::*,
    util::post_inc,
    window::ChildView,
    AnyViewHandle, AnyWindowHandle, AppContext, Entity, ModelContext, ModelHandle, TestAppContext,
    View, ViewContext, ViewHandle,
};use gpui::LayoutContext;
use gpui::platform::Event;
use itertools::Itertools;
use postage::{sink::Sink, stream::Stream};
use serde::Deserialize;
use smol::future::poll_once;
use std::{
    cell::{Cell, RefCell},
    mem,
    ops::Range,
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
    sync::{Arc, Mutex},
};

#[crate::test(self)]
fn test_model_handles(cx: &mut AppContext) {
    struct Model {
        other: Option<ModelHandle<Model>>,
        events: Vec<String>,
    }

    impl Entity for Model {
        type Event = usize;
    }

    impl Model {
        fn new(other: Option<ModelHandle<Self>>, cx: &mut ModelContext<Self>) -> Self {
            if let Some(other) = other.as_ref() {
                cx.observe(other, |me, _, _| {
                    me.events.push("notified".into());
                })
                .detach();
                cx.subscribe(other, |me, _, event, _| {
                    me.events.push(format!("observed event {}", event));
                })
                .detach();
            }

            Self {
                other,
                events: Vec::new(),
            }
        }
    }

    let handle_1 = cx.add_model(|cx| Model::new(None, cx));
    let handle_2 = cx.add_model(|cx| Model::new(Some(handle_1.clone()), cx));
    assert_eq!(cx.models.len(), 2);

    handle_1.update(cx, |model, cx| {
        model.events.push("updated".into());
        cx.emit(1);
        cx.notify();
        cx.emit(2);
    });
    assert_eq!(handle_1.read(cx).events, vec!["updated".to_string()]);
    assert_eq!(
        handle_2.read(cx).events,
        vec![
            "observed event 1".to_string(),
            "notified".to_string(),
            "observed event 2".to_string(),
        ]
    );

    handle_2.update(cx, |model, _| {
        drop(handle_1);
        model.other.take();
    });

    assert_eq!(cx.models.len(), 1);
    assert!(cx.subscriptions.is_empty());
    assert!(cx.observations.is_empty());
}

#[crate::test(self)]
fn test_model_events(cx: &mut AppContext) {
    #[derive(Default)]
    struct Model {
        events: Vec<usize>,
    }

    impl Entity for Model {
        type Event = usize;
    }

    let handle_1 = cx.add_model(|_| Model::default());
    let handle_2 = cx.add_model(|_| Model::default());

    handle_1.update(cx, |_, cx| {
        cx.subscribe(&handle_2, move |model: &mut Model, emitter, event, cx| {
            model.events.push(*event);

            cx.subscribe(&emitter, |model, _, event, _| {
                model.events.push(*event * 2);
            })
            .detach();
        })
        .detach();
    });

    handle_2.update(cx, |_, c| c.emit(7));
    assert_eq!(handle_1.read(cx).events, vec![7]);

    handle_2.update(cx, |_, c| c.emit(5));
    assert_eq!(handle_1.read(cx).events, vec![7, 5, 10]);
}

#[crate::test(self)]
fn test_model_emit_before_subscribe_in_same_update_cycle(cx: &mut AppContext) {
    #[derive(Default)]
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    cx.add_model(|cx| {
        drop(cx.subscribe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _, _| events.borrow_mut().push("dropped before flush")
        }));
        cx.subscribe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _, _| events.borrow_mut().push("before emit")
        })
        .detach();
        cx.emit(());
        cx.subscribe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _, _| events.borrow_mut().push("after emit")
        })
        .detach();
        Model
    });
    assert_eq!(*events.borrow(), ["before emit"]);
}

#[crate::test(self)]
fn test_observe_and_notify_from_model(cx: &mut AppContext) {
    #[derive(Default)]
    struct Model {
        count: usize,
        events: Vec<usize>,
    }

    impl Entity for Model {
        type Event = ();
    }

    let handle_1 = cx.add_model(|_| Model::default());
    let handle_2 = cx.add_model(|_| Model::default());

    handle_1.update(cx, |_, c| {
        c.observe(&handle_2, move |model, observed, c| {
            model.events.push(observed.read(c).count);
            c.observe(&observed, |model, observed, c| {
                model.events.push(observed.read(c).count * 2);
            })
            .detach();
        })
        .detach();
    });

    handle_2.update(cx, |model, c| {
        model.count = 7;
        c.notify()
    });
    assert_eq!(handle_1.read(cx).events, vec![7]);

    handle_2.update(cx, |model, c| {
        model.count = 5;
        c.notify()
    });
    assert_eq!(handle_1.read(cx).events, vec![7, 5, 10])
}

#[crate::test(self)]
fn test_model_notify_before_observe_in_same_update_cycle(cx: &mut AppContext) {
    #[derive(Default)]
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    cx.add_model(|cx| {
        drop(cx.observe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _| events.borrow_mut().push("dropped before flush")
        }));
        cx.observe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _| events.borrow_mut().push("before notify")
        })
        .detach();
        cx.notify();
        cx.observe(&cx.handle(), {
            let events = events.clone();
            move |_, _, _| events.borrow_mut().push("after notify")
        })
        .detach();
        Model
    });
    assert_eq!(*events.borrow(), ["before notify"]);
}

#[crate::test(self)]
fn test_defer_and_after_window_update(cx: &mut TestAppContext) {
    struct View {
        render_count: usize,
    }

    impl Entity for View {
        type Event = usize;
    }

    impl gpui::View for View {
        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            post_inc(&mut self.render_count);
            Empty::new().into_any()
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    let window = cx.add_window(|_| View { render_count: 0 });
    let called_defer = Rc::new(AtomicBool::new(false));
    let called_after_window_update = Rc::new(AtomicBool::new(false));

    window.root(cx).update(cx, |this, cx| {
        assert_eq!(this.render_count, 1);
        cx.defer({
            let called_defer = called_defer.clone();
            move |this, _| {
                assert_eq!(this.render_count, 1);
                called_defer.store(true, SeqCst);
            }
        });
        cx.after_window_update({
            let called_after_window_update = called_after_window_update.clone();
            move |this, cx| {
                assert_eq!(this.render_count, 2);
                called_after_window_update.store(true, SeqCst);
                cx.notify();
            }
        });
        assert!(!called_defer.load(SeqCst));
        assert!(!called_after_window_update.load(SeqCst));
        cx.notify();
    });

    assert!(called_defer.load(SeqCst));
    assert!(called_after_window_update.load(SeqCst));
    assert_eq!(window.read_root_with(cx, |view, _| view.render_count), 3);
}

#[crate::test(self)]
fn test_view_handles(cx: &mut TestAppContext) {
    struct View {
        other: Option<ViewHandle<View>>,
        events: Vec<String>,
    }

    impl Entity for View {
        type Event = usize;
    }

    impl gpui::View for View {
        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    impl View {
        fn new(other: Option<ViewHandle<View>>, cx: &mut ViewContext<Self>) -> Self {
            if let Some(other) = other.as_ref() {
                cx.subscribe(other, |me, _, event, _| {
                    me.events.push(format!("observed event {}", event));
                })
                .detach();
            }
            Self {
                other,
                events: Vec::new(),
            }
        }
    }

    let window = cx.add_window(|cx| View::new(None, cx));
    let handle_1 = window.add_view(cx, |cx| View::new(None, cx));
    let handle_2 = window.add_view(cx, |cx| View::new(Some(handle_1.clone()), cx));
    assert_eq!(cx.read(|cx| cx.views.len()), 3);

    handle_1.update(cx, |view, cx| {
        view.events.push("updated".into());
        cx.emit(1);
        cx.emit(2);
    });
    handle_1.read_with(cx, |view, _| {
        assert_eq!(view.events, vec!["updated".to_string()]);
    });
    handle_2.read_with(cx, |view, _| {
        assert_eq!(
            view.events,
            vec![
                "observed event 1".to_string(),
                "observed event 2".to_string(),
            ]
        );
    });

    handle_2.update(cx, |view, _| {
        drop(handle_1);
        view.other.take();
    });

    cx.read(|cx| {
        assert_eq!(cx.views.len(), 2);
        assert!(cx.subscriptions.is_empty());
        assert!(cx.observations.is_empty());
    });
}

#[crate::test(self)]
fn test_add_window(cx: &mut AppContext) {
    struct View {
        mouse_down_count: Arc<AtomicUsize>,
    }

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            enum Handler {}
            let mouse_down_count = self.mouse_down_count.clone();
            MouseEventHandler::new::<Handler, _>(0, cx, |_, _| Empty::new())
                .on_down(MouseButton::Left, move |_, _, _| {
                    mouse_down_count.fetch_add(1, SeqCst);
                })
                .into_any()
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    let mouse_down_count = Arc::new(AtomicUsize::new(0));
    let window = cx.add_window(Default::default(), |_| View {
        mouse_down_count: mouse_down_count.clone(),
    });

    window.update(cx, |cx| {
        // Ensure window's root element is in a valid lifecycle state.
        cx.dispatch_event(
            Event::MouseDown(MouseButtonEvent {
                position: Default::default(),
                button: MouseButton::Left,
                modifiers: Default::default(),
                click_count: 1,
            }),
            false,
        );
        assert_eq!(mouse_down_count.load(SeqCst), 1);
    });
}

#[crate::test(self)]
fn test_entity_release_hooks(cx: &mut TestAppContext) {
    struct Model {
        released: Rc<Cell<bool>>,
    }

    struct View {
        released: Rc<Cell<bool>>,
    }

    impl Entity for Model {
        type Event = ();

        fn release(&mut self, _: &mut AppContext) {
            self.released.set(true);
        }
    }

    impl Entity for View {
        type Event = ();

        fn release(&mut self, _: &mut AppContext) {
            self.released.set(true);
        }
    }

    impl gpui::View for View {
        fn ui_name() -> &'static str {
            "View"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }

    let model_released = Rc::new(Cell::new(false));
    let model_release_observed = Rc::new(Cell::new(false));
    let view_released = Rc::new(Cell::new(false));
    let view_release_observed = Rc::new(Cell::new(false));

    let model = cx.add_model(|_| Model {
        released: model_released.clone(),
    });
    let window = cx.add_window(|_| View {
        released: view_released.clone(),
    });
    let view = window.root(cx);

    assert!(!model_released.get());
    assert!(!view_released.get());

    cx.update(|cx| {
        cx.observe_release(&model, {
            let model_release_observed = model_release_observed.clone();
            move |_, _| model_release_observed.set(true)
        })
        .detach();
        cx.observe_release(&view, {
            let view_release_observed = view_release_observed.clone();
            move |_, _| view_release_observed.set(true)
        })
        .detach();
    });

    cx.update(move |_| {
        drop(model);
    });
    assert!(model_released.get());
    assert!(model_release_observed.get());

    drop(view);
    window.update(cx, |cx| cx.remove_window());
    assert!(view_released.get());
    assert!(view_release_observed.get());
}

#[crate::test(self)]
fn test_view_events(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = String;
    }

    let window = cx.add_window(|_| TestView::default());
    let handle_1 = window.root(cx);
    let handle_2 = window.add_view(cx, |_| TestView::default());
    let handle_3 = cx.add_model(|_| Model);

    handle_1.update(cx, |_, cx| {
        cx.subscribe(&handle_2, move |me, emitter, event, cx| {
            me.events.push(event.clone());

            cx.subscribe(&emitter, |me, _, event, _| {
                me.events.push(format!("{event} from inner"));
            })
            .detach();
        })
        .detach();

        cx.subscribe(&handle_3, |me, _, event, _| {
            me.events.push(event.clone());
        })
        .detach();
    });

    handle_2.update(cx, |_, c| c.emit("7".into()));
    handle_1.read_with(cx, |view, _| assert_eq!(view.events, ["7"]));

    handle_2.update(cx, |_, c| c.emit("5".into()));
    handle_1.read_with(cx, |view, _| {
        assert_eq!(view.events, ["7", "5", "5 from inner"])
    });

    handle_3.update(cx, |_, c| c.emit("9".into()));
    handle_1.read_with(cx, |view, _| {
        assert_eq!(view.events, ["7", "5", "5 from inner", "9"])
    });
}

#[crate::test(self)]
fn test_global_events(cx: &mut AppContext) {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct GlobalEvent(u64);

    let events = Rc::new(RefCell::new(Vec::new()));
    let first_subscription;
    let second_subscription;

    {
        let events = events.clone();
        first_subscription = cx.subscribe_global(move |e: &GlobalEvent, _| {
            events.borrow_mut().push(("First", e.clone()));
        });
    }

    {
        let events = events.clone();
        second_subscription = cx.subscribe_global(move |e: &GlobalEvent, _| {
            events.borrow_mut().push(("Second", e.clone()));
        });
    }

    cx.update(|cx| {
        cx.emit_global(GlobalEvent(1));
        cx.emit_global(GlobalEvent(2));
    });

    drop(first_subscription);

    cx.update(|cx| {
        cx.emit_global(GlobalEvent(3));
    });

    drop(second_subscription);

    cx.update(|cx| {
        cx.emit_global(GlobalEvent(4));
    });

    assert_eq!(
        &*events.borrow(),
        &[
            ("First", GlobalEvent(1)),
            ("Second", GlobalEvent(1)),
            ("First", GlobalEvent(2)),
            ("Second", GlobalEvent(2)),
            ("Second", GlobalEvent(3)),
        ]
    );
}

#[crate::test(self)]
fn test_global_events_emitted_before_subscription_in_same_update_cycle(cx: &mut AppContext) {
    let events = Rc::new(RefCell::new(Vec::new()));
    cx.update(|cx| {
        {
            let events = events.clone();
            drop(cx.subscribe_global(move |_: &(), _| {
                events.borrow_mut().push("dropped before emit");
            }));
        }

        {
            let events = events.clone();
            cx.subscribe_global(move |_: &(), _| {
                events.borrow_mut().push("before emit");
            })
            .detach();
        }

        cx.emit_global(());

        {
            let events = events.clone();
            cx.subscribe_global(move |_: &(), _| {
                events.borrow_mut().push("after emit");
            })
            .detach();
        }
    });

    assert_eq!(*events.borrow(), ["before emit"]);
}

#[crate::test(self)]
fn test_global_nested_events(cx: &mut AppContext) {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct GlobalEvent(u64);

    let events = Rc::new(RefCell::new(Vec::new()));

    {
        let events = events.clone();
        cx.subscribe_global(move |e: &GlobalEvent, cx| {
            events.borrow_mut().push(("Outer", e.clone()));

            if e.0 == 1 {
                let events = events.clone();
                cx.subscribe_global(move |e: &GlobalEvent, _| {
                    events.borrow_mut().push(("Inner", e.clone()));
                })
                .detach();
            }
        })
        .detach();
    }

    cx.update(|cx| {
        cx.emit_global(GlobalEvent(1));
        cx.emit_global(GlobalEvent(2));
        cx.emit_global(GlobalEvent(3));
    });
    cx.update(|cx| {
        cx.emit_global(GlobalEvent(4));
    });

    assert_eq!(
        &*events.borrow(),
        &[
            ("Outer", GlobalEvent(1)),
            ("Outer", GlobalEvent(2)),
            ("Outer", GlobalEvent(3)),
            ("Outer", GlobalEvent(4)),
            ("Inner", GlobalEvent(4)),
        ]
    );
}

#[crate::test(self)]
fn test_global(cx: &mut AppContext) {
    type Global = usize;

    let observation_count = Rc::new(RefCell::new(0));
    let subscription = cx.observe_global::<Global, _>({
        let observation_count = observation_count.clone();
        move |_| {
            *observation_count.borrow_mut() += 1;
        }
    });

    assert!(!cx.has_global::<Global>());
    assert_eq!(cx.default_global::<Global>(), &0);
    assert_eq!(*observation_count.borrow(), 1);
    assert!(cx.has_global::<Global>());
    assert_eq!(
        cx.update_global::<Global, _, _>(|global, _| {
            *global = 1;
            "Update Result"
        }),
        "Update Result"
    );
    assert_eq!(*observation_count.borrow(), 2);
    assert_eq!(cx.global::<Global>(), &1);

    drop(subscription);
    cx.update_global::<Global, _, _>(|global, _| {
        *global = 2;
    });
    assert_eq!(*observation_count.borrow(), 2);

    type OtherGlobal = f32;

    let observation_count = Rc::new(RefCell::new(0));
    cx.observe_global::<OtherGlobal, _>({
        let observation_count = observation_count.clone();
        move |_| {
            *observation_count.borrow_mut() += 1;
        }
    })
    .detach();

    assert_eq!(
        cx.update_default_global::<OtherGlobal, _, _>(|global, _| {
            assert_eq!(global, &0.0);
            *global = 2.0;
            "Default update result"
        }),
        "Default update result"
    );
    assert_eq!(cx.global::<OtherGlobal>(), &2.0);
    assert_eq!(*observation_count.borrow(), 1);
}

#[crate::test(self)]
fn test_dropping_subscribers(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let window = cx.add_window(|_| TestView::default());
    let observing_view = window.add_view(cx, |_| TestView::default());
    let emitting_view = window.add_view(cx, |_| TestView::default());
    let observing_model = cx.add_model(|_| Model);
    let observed_model = cx.add_model(|_| Model);

    observing_view.update(cx, |_, cx| {
        cx.subscribe(&emitting_view, |_, _, _, _| {}).detach();
        cx.subscribe(&observed_model, |_, _, _, _| {}).detach();
    });
    observing_model.update(cx, |_, cx| {
        cx.subscribe(&observed_model, |_, _, _, _| {}).detach();
    });

    cx.update(|_| {
        drop(observing_view);
        drop(observing_model);
    });

    emitting_view.update(cx, |_, cx| cx.emit(Default::default()));
    observed_model.update(cx, |_, cx| cx.emit(()));
}

#[crate::test(self)]
fn test_view_emit_before_subscribe_in_same_update_cycle(cx: &mut AppContext) {
    let window = cx.add_window::<TestView, _>(Default::default(), |cx| {
        drop(cx.subscribe(&cx.handle(), {
            move |this, _, _, _| this.events.push("dropped before flush".into())
        }));
        cx.subscribe(&cx.handle(), {
            move |this, _, _, _| this.events.push("before emit".into())
        })
        .detach();
        cx.emit("the event".into());
        cx.subscribe(&cx.handle(), {
            move |this, _, _, _| this.events.push("after emit".into())
        })
        .detach();
        TestView { events: Vec::new() }
    });

    window.read_root_with(cx, |view, _| assert_eq!(view.events, ["before emit"]));
}

#[crate::test(self)]
fn test_observe_and_notify_from_view(cx: &mut TestAppContext) {
    #[derive(Default)]
    struct Model {
        state: String,
    }

    impl Entity for Model {
        type Event = ();
    }

    let window = cx.add_window(|_| TestView::default());
    let view = window.root(cx);
    let model = cx.add_model(|_| Model {
        state: "old-state".into(),
    });

    view.update(cx, |_, c| {
        c.observe(&model, |me, observed, cx| {
            me.events.push(observed.read(cx).state.clone())
        })
        .detach();
    });

    model.update(cx, |model, cx| {
        model.state = "new-state".into();
        cx.notify();
    });
    view.read_with(cx, |view, _| assert_eq!(view.events, ["new-state"]));
}

#[crate::test(self)]
fn test_view_notify_before_observe_in_same_update_cycle(cx: &mut AppContext) {
    let window = cx.add_window::<TestView, _>(Default::default(), |cx| {
        drop(cx.observe(&cx.handle(), {
            move |this, _, _| this.events.push("dropped before flush".into())
        }));
        cx.observe(&cx.handle(), {
            move |this, _, _| this.events.push("before notify".into())
        })
        .detach();
        cx.notify();
        cx.observe(&cx.handle(), {
            move |this, _, _| this.events.push("after notify".into())
        })
        .detach();
        TestView { events: Vec::new() }
    });

    window.read_root_with(cx, |view, _| assert_eq!(view.events, ["before notify"]));
}

#[crate::test(self)]
fn test_notify_and_drop_observe_subscription_in_same_update_cycle(cx: &mut TestAppContext) {
    struct Model;
    impl Entity for Model {
        type Event = ();
    }

    let model = cx.add_model(|_| Model);
    let window = cx.add_window(|_| TestView::default());
    let view = window.root(cx);

    view.update(cx, |_, cx| {
        model.update(cx, |_, cx| cx.notify());
        drop(cx.observe(&model, move |this, _, _| {
            this.events.push("model notified".into());
        }));
        model.update(cx, |_, cx| cx.notify());
    });

    for _ in 0..3 {
        model.update(cx, |_, cx| cx.notify());
    }
    view.read_with(cx, |view, _| assert_eq!(view.events, Vec::<&str>::new()));
}

#[crate::test(self)]
fn test_dropping_observers(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let window = cx.add_window(|_| TestView::default());
    let observing_view = window.add_view(cx, |_| TestView::default());
    let observing_model = cx.add_model(|_| Model);
    let observed_model = cx.add_model(|_| Model);

    observing_view.update(cx, |_, cx| {
        cx.observe(&observed_model, |_, _, _| {}).detach();
    });
    observing_model.update(cx, |_, cx| {
        cx.observe(&observed_model, |_, _, _| {}).detach();
    });

    cx.update(|_| {
        drop(observing_view);
        drop(observing_model);
    });

    observed_model.update(cx, |_, cx| cx.notify());
}

#[crate::test(self)]
fn test_dropping_subscriptions_during_callback(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = u64;
    }

    // Events
    let observing_model = cx.add_model(|_| Model);
    let observed_model = cx.add_model(|_| Model);

    let events = Rc::new(RefCell::new(Vec::new()));

    observing_model.update(cx, |_, cx| {
        let events = events.clone();
        let subscription = Rc::new(RefCell::new(None));
        *subscription.borrow_mut() = Some(cx.subscribe(&observed_model, {
            let subscription = subscription.clone();
            move |_, _, e, _| {
                subscription.borrow_mut().take();
                events.borrow_mut().push(*e);
            }
        }));
    });

    observed_model.update(cx, |_, cx| {
        cx.emit(1);
        cx.emit(2);
    });

    assert_eq!(*events.borrow(), [1]);

    // Global Events
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct GlobalEvent(u64);

    let events = Rc::new(RefCell::new(Vec::new()));

    {
        let events = events.clone();
        let subscription = Rc::new(RefCell::new(None));
        *subscription.borrow_mut() = Some(cx.subscribe_global({
            let subscription = subscription.clone();
            move |e: &GlobalEvent, _| {
                subscription.borrow_mut().take();
                events.borrow_mut().push(e.clone());
            }
        }));
    }

    cx.update(|cx| {
        cx.emit_global(GlobalEvent(1));
        cx.emit_global(GlobalEvent(2));
    });

    assert_eq!(*events.borrow(), [GlobalEvent(1)]);

    // Model Observation
    let observing_model = cx.add_model(|_| Model);
    let observed_model = cx.add_model(|_| Model);

    let observation_count = Rc::new(RefCell::new(0));

    observing_model.update(cx, |_, cx| {
        let observation_count = observation_count.clone();
        let subscription = Rc::new(RefCell::new(None));
        *subscription.borrow_mut() = Some(cx.observe(&observed_model, {
            let subscription = subscription.clone();
            move |_, _, _| {
                subscription.borrow_mut().take();
                *observation_count.borrow_mut() += 1;
            }
        }));
    });

    observed_model.update(cx, |_, cx| {
        cx.notify();
    });

    observed_model.update(cx, |_, cx| {
        cx.notify();
    });

    assert_eq!(*observation_count.borrow(), 1);

    // View Observation
    struct View;

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    let window = cx.add_window(|_| View);
    let observing_view = window.add_view(cx, |_| View);
    let observed_view = window.add_view(cx, |_| View);

    let observation_count = Rc::new(RefCell::new(0));
    observing_view.update(cx, |_, cx| {
        let observation_count = observation_count.clone();
        let subscription = Rc::new(RefCell::new(None));
        *subscription.borrow_mut() = Some(cx.observe(&observed_view, {
            let subscription = subscription.clone();
            move |_, _, _| {
                subscription.borrow_mut().take();
                *observation_count.borrow_mut() += 1;
            }
        }));
    });

    observed_view.update(cx, |_, cx| {
        cx.notify();
    });

    observed_view.update(cx, |_, cx| {
        cx.notify();
    });

    assert_eq!(*observation_count.borrow(), 1);

    // Global Observation
    let observation_count = Rc::new(RefCell::new(0));
    let subscription = Rc::new(RefCell::new(None));
    *subscription.borrow_mut() = Some(cx.observe_global::<(), _>({
        let observation_count = observation_count.clone();
        let subscription = subscription.clone();
        move |_| {
            subscription.borrow_mut().take();
            *observation_count.borrow_mut() += 1;
        }
    }));

    cx.update(|cx| {
        cx.default_global::<()>();
        cx.set_global(());
    });
    assert_eq!(*observation_count.borrow(), 1);
}

#[crate::test(self)]
fn test_focus(cx: &mut TestAppContext) {
    struct View {
        name: String,
        events: Arc<Mutex<Vec<String>>>,
        child: Option<AnyViewHandle>,
    }

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            self.child
                .as_ref()
                .map(|child| ChildView::new(child, cx).into_any())
                .unwrap_or(Empty::new().into_any())
        }

        fn ui_name() -> &'static str {
            "View"
        }

        fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
            if cx.handle().id() == focused.id() {
                self.events.lock().push(format!("{} focused", &self.name));
            }
        }

        fn focus_out(&mut self, blurred: AnyViewHandle, cx: &mut ViewContext<Self>) {
            if cx.handle().id() == blurred.id() {
                self.events.lock().push(format!("{} blurred", &self.name));
            }
        }
    }

    let view_events: Arc<Mutex<Vec<String>>> = Default::default();
    let window = cx.add_window(|_| View {
        events: view_events.clone(),
        name: "view 1".to_string(),
        child: None,
    });
    let view_1 = window.root(cx);
    let view_2 = window.update(cx, |cx| {
        let view_2 = cx.add_view(|_| View {
            events: view_events.clone(),
            name: "view 2".to_string(),
            child: None,
        });
        view_1.update(cx, |view_1, cx| {
            view_1.child = Some(view_2.clone().into_any());
            cx.notify();
        });
        view_2
    });

    let observed_events: Arc<Mutex<Vec<String>>> = Default::default();
    view_1.update(cx, |_, cx| {
        cx.observe_focus(&view_2, {
            let observed_events = observed_events.clone();
            move |this, view, focused, cx| {
                let label = if focused { "focus" } else { "blur" };
                observed_events.lock().push(format!(
                    "{} observed {}'s {}",
                    this.name,
                    view.read(cx).name,
                    label
                ))
            }
        })
        .detach();
    });
    view_2.update(cx, |_, cx| {
        cx.observe_focus(&view_1, {
            let observed_events = observed_events.clone();
            move |this, view, focused, cx| {
                let label = if focused { "focus" } else { "blur" };
                observed_events.lock().push(format!(
                    "{} observed {}'s {}",
                    this.name,
                    view.read(cx).name,
                    label
                ))
            }
        })
        .detach();
    });
    assert_eq!(mem::take(&mut *view_events.lock()), ["view 1 focused"]);
    assert_eq!(mem::take(&mut *observed_events.lock()), Vec::<&str>::new());

    view_1.update(cx, |_, cx| {
        // Ensure only the last focus event is honored.
        cx.focus(&view_2);
        cx.focus(&view_1);
        cx.focus(&view_2);
    });

    assert_eq!(
        mem::take(&mut *view_events.lock()),
        ["view 1 blurred", "view 2 focused"],
    );
    assert_eq!(
        mem::take(&mut *observed_events.lock()),
        [
            "view 2 observed view 1's blur",
            "view 1 observed view 2's focus"
        ]
    );

    view_1.update(cx, |_, cx| cx.focus(&view_1));
    assert_eq!(
        mem::take(&mut *view_events.lock()),
        ["view 2 blurred", "view 1 focused"],
    );
    assert_eq!(
        mem::take(&mut *observed_events.lock()),
        [
            "view 1 observed view 2's blur",
            "view 2 observed view 1's focus"
        ]
    );

    view_1.update(cx, |_, cx| cx.focus(&view_2));
    assert_eq!(
        mem::take(&mut *view_events.lock()),
        ["view 1 blurred", "view 2 focused"],
    );
    assert_eq!(
        mem::take(&mut *observed_events.lock()),
        [
            "view 2 observed view 1's blur",
            "view 1 observed view 2's focus"
        ]
    );

    println!("=====================");
    view_1.update(cx, |view, _| {
        drop(view_2);
        view.child = None;
    });
    assert_eq!(mem::take(&mut *view_events.lock()), ["view 1 focused"]);
    assert_eq!(mem::take(&mut *observed_events.lock()), Vec::<&str>::new());
}

#[crate::test(self)]
fn test_deserialize_actions(cx: &mut AppContext) {
    #[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
    pub struct ComplexAction {
        arg: String,
        count: usize,
    }

    actions!(test::something, [SimpleAction]);
    impl_actions!(test::something, [ComplexAction]);

    cx.add_global_action(move |_: &SimpleAction, _: &mut AppContext| {});
    cx.add_global_action(move |_: &ComplexAction, _: &mut AppContext| {});

    let action1 = cx
        .deserialize_action(
            "test::something::ComplexAction",
            Some(serde_json::from_str(r#"{"arg": "a", "count": 5}"#).unwrap()),
        )
        .unwrap();
    let action2 = cx
        .deserialize_action("test::something::SimpleAction", None)
        .unwrap();
    assert_eq!(
        action1.as_any().downcast_ref::<ComplexAction>().unwrap(),
        &ComplexAction {
            arg: "a".to_string(),
            count: 5,
        }
    );
    assert_eq!(
        action2.as_any().downcast_ref::<SimpleAction>().unwrap(),
        &SimpleAction
    );
}

#[crate::test(self)]
fn test_dispatch_action(cx: &mut TestAppContext) {
    struct ViewA {
        id: usize,
        child: Option<AnyViewHandle>,
    }

    impl Entity for ViewA {
        type Event = ();
    }

    impl gpui::View for ViewA {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            self.child
                .as_ref()
                .map(|child| ChildView::new(child, cx).into_any())
                .unwrap_or(Empty::new().into_any())
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    struct ViewB {
        id: usize,
        child: Option<AnyViewHandle>,
    }

    impl Entity for ViewB {
        type Event = ();
    }

    impl gpui::View for ViewB {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            self.child
                .as_ref()
                .map(|child| ChildView::new(child, cx).into_any())
                .unwrap_or(Empty::new().into_any())
        }

        fn ui_name() -> &'static str {
            "View"
        }
    }

    #[derive(Clone, Default, Deserialize, PartialEq)]
    pub struct Action(pub String);

    impl_actions!(test, [Action]);

    let actions = Rc::new(RefCell::new(Vec::new()));
    let observed_actions = Rc::new(RefCell::new(Vec::new()));

    cx.update(|cx| {
        cx.add_global_action({
            let actions = actions.clone();
            move |_: &Action, _: &mut AppContext| {
                actions.borrow_mut().push("global".to_string());
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewA, action: &Action, cx| {
                assert_eq!(action.0, "bar");
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} a", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewA, _: &Action, cx| {
                if view.id != 1 {
                    cx.add_view(|cx| {
                        cx.propagate_action(); // Still works on a nested ViewContext
                        ViewB { id: 5, child: None }
                    });
                }
                actions.borrow_mut().push(format!("{} b", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewB, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} c", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewB, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} d", view.id));
            }
        });

        cx.capture_action({
            let actions = actions.clone();
            move |view: &mut ViewA, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} capture", view.id));
            }
        });

        cx.observe_actions({
            let observed_actions = observed_actions.clone();
            move |action_id, _| observed_actions.borrow_mut().push(action_id)
        })
        .detach();
    });

    let window = cx.add_window(|_| ViewA { id: 1, child: None });
    let view_1 = window.root(cx);
    let view_2 = window.update(cx, |cx| {
        let child = cx.add_view(|_| ViewB { id: 2, child: None });
        view_1.update(cx, |view, cx| {
            view.child = Some(child.clone().into_any());
            cx.notify();
        });
        child
    });
    let view_3 = window.update(cx, |cx| {
        let child = cx.add_view(|_| ViewA { id: 3, child: None });
        view_2.update(cx, |view, cx| {
            view.child = Some(child.clone().into_any());
            cx.notify();
        });
        child
    });
    let view_4 = window.update(cx, |cx| {
        let child = cx.add_view(|_| ViewB { id: 4, child: None });
        view_3.update(cx, |view, cx| {
            view.child = Some(child.clone().into_any());
            cx.notify();
        });
        child
    });

    window.update(cx, |cx| {
        cx.dispatch_action(Some(view_4.id()), &Action("bar".to_string()))
    });

    assert_eq!(
        *actions.borrow(),
        vec![
            "1 capture",
            "3 capture",
            "4 d",
            "4 c",
            "3 b",
            "3 a",
            "2 d",
            "2 c",
            "1 b"
        ]
    );
    assert_eq!(*observed_actions.borrow(), [Action::default().id()]);

    // Remove view_1, which doesn't propagate the action

    let window = cx.add_window(|_| ViewB { id: 2, child: None });
    let view_2 = window.root(cx);
    let view_3 = window.update(cx, |cx| {
        let child = cx.add_view(|_| ViewA { id: 3, child: None });
        view_2.update(cx, |view, cx| {
            view.child = Some(child.clone().into_any());
            cx.notify();
        });
        child
    });
    let view_4 = window.update(cx, |cx| {
        let child = cx.add_view(|_| ViewB { id: 4, child: None });
        view_3.update(cx, |view, cx| {
            view.child = Some(child.clone().into_any());
            cx.notify();
        });
        child
    });

    actions.borrow_mut().clear();
    window.update(cx, |cx| {
        cx.dispatch_action(Some(view_4.id()), &Action("bar".to_string()))
    });

    assert_eq!(
        *actions.borrow(),
        vec![
            "3 capture",
            "4 d",
            "4 c",
            "3 b",
            "3 a",
            "2 d",
            "2 c",
            "global"
        ]
    );
    assert_eq!(
        *observed_actions.borrow(),
        [Action::default().id(), Action::default().id()]
    );
}

#[crate::test(self)]
fn test_dispatch_keystroke(cx: &mut AppContext) {
    #[derive(Clone, Deserialize, PartialEq)]
    pub struct Action(String);

    impl_actions!(test, [Action]);

    struct View {
        id: usize,
        keymap_context: KeymapContext,
        child: Option<AnyViewHandle>,
    }

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            self.child
                .as_ref()
                .map(|child| ChildView::new(child, cx).into_any())
                .unwrap_or(Empty::new().into_any())
        }

        fn ui_name() -> &'static str {
            "View"
        }

        fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
            *keymap = self.keymap_context.clone();
        }
    }

    impl View {
        fn new(id: usize) -> Self {
            View {
                id,
                keymap_context: KeymapContext::default(),
                child: None,
            }
        }
    }

    let mut view_1 = View::new(1);
    let mut view_2 = View::new(2);
    let mut view_3 = View::new(3);
    view_1.keymap_context.add_identifier("a");
    view_2.keymap_context.add_identifier("a");
    view_2.keymap_context.add_identifier("b");
    view_3.keymap_context.add_identifier("a");
    view_3.keymap_context.add_identifier("b");
    view_3.keymap_context.add_identifier("c");

    let window = cx.add_window(Default::default(), |cx| {
        let view_2 = cx.add_view(|cx| {
            let view_3 = cx.add_view(|cx| {
                cx.focus_self();
                view_3
            });
            view_2.child = Some(view_3.into_any());
            view_2
        });
        view_1.child = Some(view_2.into_any());
        view_1
    });

    // This binding only dispatches an action on view 2 because that view will have
    // "a" and "b" in its context, but not "c".
    cx.add_bindings(vec![Binding::new(
        "a",
        Action("a".to_string()),
        Some("a && b && !c"),
    )]);

    cx.add_bindings(vec![Binding::new("b", Action("b".to_string()), None)]);

    // This binding only dispatches an action on views 2 and 3, because they have
    // a parent view with a in its context
    cx.add_bindings(vec![Binding::new(
        "c",
        Action("c".to_string()),
        Some("b > c"),
    )]);

    // This binding only dispatches an action on view 2, because they have
    // a parent view with a in its context
    cx.add_bindings(vec![Binding::new(
        "d",
        Action("d".to_string()),
        Some("a && !b > b"),
    )]);

    let actions = Rc::new(RefCell::new(Vec::new()));
    cx.add_action({
        let actions = actions.clone();
        move |view: &mut View, action: &Action, cx| {
            actions
                .borrow_mut()
                .push(format!("{} {}", view.id, action.0));

            if action.0 == "b" {
                cx.propagate_action();
            }
        }
    });

    cx.add_global_action({
        let actions = actions.clone();
        move |action: &Action, _| {
            actions.borrow_mut().push(format!("global {}", action.0));
        }
    });

    window.update(cx, |cx| {
        cx.dispatch_keystroke(&Keystroke::parse("a").unwrap())
    });
    assert_eq!(&*actions.borrow(), &["2 a"]);
    actions.borrow_mut().clear();

    window.update(cx, |cx| {
        cx.dispatch_keystroke(&Keystroke::parse("b").unwrap());
    });

    assert_eq!(&*actions.borrow(), &["3 b", "2 b", "1 b", "global b"]);
    actions.borrow_mut().clear();

    window.update(cx, |cx| {
        cx.dispatch_keystroke(&Keystroke::parse("c").unwrap());
    });
    assert_eq!(&*actions.borrow(), &["3 c"]);
    actions.borrow_mut().clear();

    window.update(cx, |cx| {
        cx.dispatch_keystroke(&Keystroke::parse("d").unwrap());
    });
    assert_eq!(&*actions.borrow(), &["2 d"]);
    actions.borrow_mut().clear();
}

#[crate::test(self)]
fn test_keystrokes_for_action(cx: &mut TestAppContext) {
    actions!(test, [Action1, Action2, GlobalAction]);

    struct View1 {
        child: ViewHandle<View2>,
    }
    struct View2 {}

    impl Entity for View1 {
        type Event = ();
    }
    impl Entity for View2 {
        type Event = ();
    }

    impl gpui::View for View1 {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            ChildView::new(&self.child, cx).into_any()
        }
        fn ui_name() -> &'static str {
            "View1"
        }
    }
    impl gpui::View for View2 {
        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
        fn ui_name() -> &'static str {
            "View2"
        }
    }

    let window = cx.add_window(|cx| {
        let view_2 = cx.add_view(|cx| {
            cx.focus_self();
            View2 {}
        });
        View1 { child: view_2 }
    });
    let view_1 = window.root(cx);
    let view_2 = view_1.read_with(cx, |view, _| view.child.clone());

    cx.update(|cx| {
        cx.add_action(|_: &mut View1, _: &Action1, _cx| {});
        cx.add_action(|_: &mut View2, _: &Action2, _cx| {});
        cx.add_global_action(|_: &GlobalAction, _| {});
        cx.add_bindings(vec![
            Binding::new("a", Action1, Some("View1")),
            Binding::new("b", Action2, Some("View1 > View2")),
            Binding::new("c", GlobalAction, Some("View3")), // View 3 does not exist
        ]);
    });

    let view_1_id = view_1.id();
    view_1.update(cx, |_, cx| {
        view_2.update(cx, |_, cx| {
            // Sanity check
            let mut new_parents = Default::default();
            let mut notify_views_if_parents_change = Default::default();
            let mut layout_cx = LayoutContext::new(
                cx,
                &mut new_parents,
                &mut notify_views_if_parents_change,
                false,
            );
            assert_eq!(
                layout_cx
                    .keystrokes_for_action(view_1_id, &Action1)
                    .unwrap()
                    .as_slice(),
                &[Keystroke::parse("a").unwrap()]
            );
            assert_eq!(
                layout_cx
                    .keystrokes_for_action(view_2.id(), &Action2)
                    .unwrap()
                    .as_slice(),
                &[Keystroke::parse("b").unwrap()]
            );

            // The 'a' keystroke propagates up the view tree from view_2
            // to view_1. The action, Action1, is handled by view_1.
            assert_eq!(
                layout_cx
                    .keystrokes_for_action(view_2.id(), &Action1)
                    .unwrap()
                    .as_slice(),
                &[Keystroke::parse("a").unwrap()]
            );

            // Actions that are handled below the current view don't have bindings
            assert_eq!(layout_cx.keystrokes_for_action(view_1_id, &Action2), None);

            // Actions that are handled in other branches of the tree should not have a binding
            assert_eq!(
                layout_cx.keystrokes_for_action(view_2.id(), &GlobalAction),
                None
            );
        });
    });

    // Check that global actions do not have a binding, even if a binding does exist in another view
    assert_eq!(
        &available_actions(window.into(), view_1.id(), cx),
        &[
            ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
            ("test::GlobalAction", vec![])
        ],
    );

    // Check that view 1 actions and bindings are available even when called from view 2
    assert_eq!(
        &available_actions(window.into(), view_2.id(), cx),
        &[
            ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
            ("test::Action2", vec![Keystroke::parse("b").unwrap()]),
            ("test::GlobalAction", vec![]),
        ],
    );

    // Produces a list of actions and key bindings
    fn available_actions(
        window: AnyWindowHandle,
        view_id: usize,
        cx: &TestAppContext,
    ) -> Vec<(&'static str, Vec<Keystroke>)> {
        cx.available_actions(window.into(), view_id)
            .into_iter()
            .map(|(action_name, _, bindings)| {
                (
                    action_name,
                    bindings
                        .iter()
                        .map(|binding| binding.keystrokes()[0].clone())
                        .collect::<Vec<_>>(),
                )
            })
            .sorted_by(|(name1, _), (name2, _)| name1.cmp(name2))
            .collect()
    }
}

#[crate::test(self)]
fn test_keystrokes_for_action_with_data(cx: &mut TestAppContext) {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct ActionWithArg {
        #[serde(default)]
        arg: bool,
    }

    struct View;
    impl Entity for View {
        type Event = ();
    }
    impl gpui::View for View {
        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
        fn ui_name() -> &'static str {
            "View"
        }
    }

    impl_actions!(test, [ActionWithArg]);

    let window = cx.add_window(|_| View);
    let view = window.root(cx);
    cx.update(|cx| {
        cx.add_global_action(|_: &ActionWithArg, _| {});
        cx.add_bindings(vec![
            Binding::new("a", ActionWithArg { arg: false }, None),
            Binding::new("shift-a", ActionWithArg { arg: true }, None),
        ]);
    });

    let actions = cx.available_actions(window.into(), view.id());
    assert_eq!(
        actions[0].1.as_any().downcast_ref::<ActionWithArg>(),
        Some(&ActionWithArg { arg: false })
    );
    assert_eq!(
        actions[0]
            .2
            .iter()
            .map(|b| b.keystrokes()[0].clone())
            .collect::<Vec<_>>(),
        vec![Keystroke::parse("a").unwrap()],
    );
}

#[crate::test(self)]
async fn test_model_condition(cx: &mut TestAppContext) {
    struct Counter(usize);

    impl Entity for Counter {
        type Event = ();
    }

    impl Counter {
        fn inc(&mut self, cx: &mut ModelContext<Self>) {
            self.0 += 1;
            cx.notify();
        }
    }

    let model = cx.add_model(|_| Counter(0));

    let condition1 = model.condition(cx, |model, _| model.0 == 2);
    let condition2 = model.condition(cx, |model, _| model.0 == 3);
    smol::pin!(condition1, condition2);

    model.update(cx, |model, cx| model.inc(cx));
    assert_eq!(poll_once(&mut condition1).await, None);
    assert_eq!(poll_once(&mut condition2).await, None);

    model.update(cx, |model, cx| model.inc(cx));
    assert_eq!(poll_once(&mut condition1).await, Some(()));
    assert_eq!(poll_once(&mut condition2).await, None);

    model.update(cx, |model, cx| model.inc(cx));
    assert_eq!(poll_once(&mut condition2).await, Some(()));

    model.update(cx, |_, cx| cx.notify());
}

#[crate::test(self)]
#[should_panic]
async fn test_model_condition_timeout(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let model = cx.add_model(|_| Model);
    model.condition(cx, |_, _| false).await;
}

#[crate::test(self)]
#[should_panic(expected = "model dropped with pending condition")]
async fn test_model_condition_panic_on_drop(cx: &mut TestAppContext) {
    struct Model;

    impl Entity for Model {
        type Event = ();
    }

    let model = cx.add_model(|_| Model);
    let condition = model.condition(cx, |_, _| false);
    cx.update(|_| drop(model));
    condition.await;
}

#[crate::test(self)]
async fn test_view_condition(cx: &mut TestAppContext) {
    struct Counter(usize);

    impl Entity for Counter {
        type Event = ();
    }

    impl View for Counter {
        fn ui_name() -> &'static str {
            "test view"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }

    impl Counter {
        fn inc(&mut self, cx: &mut ViewContext<Self>) {
            self.0 += 1;
            cx.notify();
        }
    }

    let window = cx.add_window(|_| Counter(0));
    let view = window.root(cx);

    let condition1 = view.condition(cx, |view, _| view.0 == 2);
    let condition2 = view.condition(cx, |view, _| view.0 == 3);
    smol::pin!(condition1, condition2);

    view.update(cx, |view, cx| view.inc(cx));
    assert_eq!(poll_once(&mut condition1).await, None);
    assert_eq!(poll_once(&mut condition2).await, None);

    view.update(cx, |view, cx| view.inc(cx));
    assert_eq!(poll_once(&mut condition1).await, Some(()));
    assert_eq!(poll_once(&mut condition2).await, None);

    view.update(cx, |view, cx| view.inc(cx));
    assert_eq!(poll_once(&mut condition2).await, Some(()));
    view.update(cx, |_, cx| cx.notify());
}

#[crate::test(self)]
#[should_panic]
async fn test_view_condition_timeout(cx: &mut TestAppContext) {
    let window = cx.add_window(|_| TestView::default());
    window.root(cx).condition(cx, |_, _| false).await;
}

#[crate::test(self)]
#[should_panic(expected = "view dropped with pending condition")]
async fn test_view_condition_panic_on_drop(cx: &mut TestAppContext) {
    let window = cx.add_window(|_| TestView::default());
    let view = window.add_view(cx, |_| TestView::default());

    let condition = view.condition(cx, |_, _| false);
    cx.update(|_| drop(view));
    condition.await;
}

#[crate::test(self)]
fn test_refresh_windows(cx: &mut TestAppContext) {
    struct View(usize);

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn ui_name() -> &'static str {
            "test view"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any_named(format!("render count: {}", post_inc(&mut self.0)))
        }
    }

    let window = cx.add_window(|_| View(0));
    let root_view = window.root(cx);
    window.update(cx, |cx| {
        assert_eq!(
            cx.window().rendered_views[&root_view.id()].name(),
            Some("render count: 0")
        );
    });

    let view = window.update(cx, |cx| {
        cx.refresh_windows();
        cx.add_view(|_| View(0))
    });

    window.update(cx, |cx| {
        assert_eq!(
            cx.window().rendered_views[&root_view.id()].name(),
            Some("render count: 1")
        );
        assert_eq!(
            cx.window().rendered_views[&view.id()].name(),
            Some("render count: 0")
        );
    });

    cx.update(|cx| cx.refresh_windows());

    window.update(cx, |cx| {
        assert_eq!(
            cx.window().rendered_views[&root_view.id()].name(),
            Some("render count: 2")
        );
        assert_eq!(
            cx.window().rendered_views[&view.id()].name(),
            Some("render count: 1")
        );
    });

    cx.update(|cx| {
        cx.refresh_windows();
        drop(view);
    });

    window.update(cx, |cx| {
        assert_eq!(
            cx.window().rendered_views[&root_view.id()].name(),
            Some("render count: 3")
        );
        assert_eq!(cx.window().rendered_views.len(), 1);
    });
}

#[crate::test(self)]
async fn test_labeled_tasks(cx: &mut TestAppContext) {
    assert_eq!(None, cx.update(|cx| cx.active_labeled_tasks().next()));
    let (mut sender, mut receiver) = postage::oneshot::channel::<()>();
    let task =
        cx.update(|cx| cx.spawn_labeled("Test Label", |_| async move { receiver.recv().await }));

    assert_eq!(
        Some("Test Label"),
        cx.update(|cx| cx.active_labeled_tasks().next())
    );
    sender
        .send(())
        .await
        .expect("Could not send message to complete task");
    task.await;

    assert_eq!(None, cx.update(|cx| cx.active_labeled_tasks().next()));
}

#[crate::test(self)]
async fn test_window_activation(cx: &mut TestAppContext) {
    struct View(&'static str);

    impl Entity for View {
        type Event = ();
    }

    impl gpui::View for View {
        fn ui_name() -> &'static str {
            "test view"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    let window_1 = cx.add_window(|cx: &mut ViewContext<View>| {
        cx.observe_window_activation({
            let events = events.clone();
            move |this, active, _| events.borrow_mut().push((this.0, active))
        })
        .detach();
        View("window 1")
    });
    assert_eq!(mem::take(&mut *events.borrow_mut()), [("window 1", true)]);

    let window_2 = cx.add_window(|cx: &mut ViewContext<View>| {
        cx.observe_window_activation({
            let events = events.clone();
            move |this, active, _| events.borrow_mut().push((this.0, active))
        })
        .detach();
        View("window 2")
    });
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [("window 1", false), ("window 2", true)]
    );

    let window_3 = cx.add_window(|cx: &mut ViewContext<View>| {
        cx.observe_window_activation({
            let events = events.clone();
            move |this, active, _| events.borrow_mut().push((this.0, active))
        })
        .detach();
        View("window 3")
    });
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [("window 2", false), ("window 3", true)]
    );

    window_2.simulate_activation(cx);
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [("window 3", false), ("window 2", true)]
    );

    window_1.simulate_activation(cx);
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [("window 2", false), ("window 1", true)]
    );

    window_3.simulate_activation(cx);
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [("window 1", false), ("window 3", true)]
    );

    window_3.simulate_activation(cx);
    assert_eq!(mem::take(&mut *events.borrow_mut()), []);
}

#[crate::test(self)]
fn test_child_view(cx: &mut TestAppContext) {
    struct Child {
        rendered: Rc<Cell<bool>>,
        dropped: Rc<Cell<bool>>,
    }

    impl Entity for Child {
        type Event = ();
    }

    impl View for Child {
        fn ui_name() -> &'static str {
            "child view"
        }

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            self.rendered.set(true);
            Empty::new().into_any()
        }
    }

    impl Drop for Child {
        fn drop(&mut self) {
            self.dropped.set(true);
        }
    }

    struct Parent {
        child: Option<ViewHandle<Child>>,
    }

    impl Entity for Parent {
        type Event = ();
    }

    impl View for Parent {
        fn ui_name() -> &'static str {
            "parent view"
        }

        fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
            if let Some(child) = self.child.as_ref() {
                ChildView::new(child, cx).into_any()
            } else {
                Empty::new().into_any()
            }
        }
    }

    let child_rendered = Rc::new(Cell::new(false));
    let child_dropped = Rc::new(Cell::new(false));
    let window = cx.add_window(|cx| Parent {
        child: Some(cx.add_view(|_| Child {
            rendered: child_rendered.clone(),
            dropped: child_dropped.clone(),
        })),
    });
    let root_view = window.root(cx);
    assert!(child_rendered.take());
    assert!(!child_dropped.take());

    root_view.update(cx, |view, cx| {
        view.child.take();
        cx.notify();
    });
    assert!(!child_rendered.take());
    assert!(child_dropped.take());
}

#[derive(Default)]
struct TestView {
    events: Vec<String>,
}

impl Entity for TestView {
    type Event = String;
}

impl View for TestView {
    fn ui_name() -> &'static str {
        "TestView"
    }

    fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
        Empty::new().into_any()
    }
}
