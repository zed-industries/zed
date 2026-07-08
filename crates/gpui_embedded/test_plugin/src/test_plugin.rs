//! The guest half of `crates/gpui_embedded/tests/shared_entities.rs`: a zoo of guest-homed
//! entities exercising calls, capability refs, attenuation, and fully dynamic dispatch.

use anyhow::anyhow;
use gpui::{AnyView, App, Context, Entity, Window, div, prelude::*};
use gpui_embedded_shared::test_schema::{
    Bump, ChameleonSnapshot, ChameleonSpec, CreateItem, CreateReadonlyItem, FactorySnapshot,
    FactorySpec, ItemSnapshot, ItemSpec, TestCounterSnapshot, TestCounterSpec, TestIncrement,
};
use gpui_embedded_shared::{decode, encode};
use gpui_plugin::shared::{HandleShared, SharedEntitySource, SharedRef};
use gpui_plugin::{Plugin, register_plugin};

/// Named shares borrow their entities (the sharer owns the lifetime), so the plugin must
/// keep them alive; anonymous shares own theirs until released.
struct TestGuest {
    _counter: Entity<Counter>,
    _factory: Entity<Factory>,
    _chameleon: Entity<Chameleon>,
}

impl Plugin for TestGuest {
    fn new(cx: &mut App) -> Self {
        let counter = cx.new(|_| Counter { count: 0 });
        gpui_plugin::shared::share::<TestCounterSpec, _>(
            &counter,
            "guest-counter",
            |methods| {
                methods.on::<TestIncrement>();
            },
            cx,
        );

        let factory = cx.new(|_| Factory { created: 0 });
        gpui_plugin::shared::share::<FactorySpec, _>(
            &factory,
            "factory",
            |methods| {
                methods.on::<CreateItem>().on::<CreateReadonlyItem>();
            },
            cx,
        );

        let chameleon = cx.new(|_| Chameleon {
            mode: "echo".to_string(),
            pokes: 0,
        });
        // Entirely dynamic dispatch: one wildcard handler interprets every method name at
        // runtime and can change its own behavior ("become").
        gpui_plugin::shared::share::<ChameleonSpec, _>(
            &chameleon,
            "chameleon",
            |methods| {
                methods.on_raw("*", |entity, method, payload, cx| {
                    entity.update(cx, |this, cx| match method {
                        "become" => {
                            this.mode = decode(payload)?;
                            cx.notify();
                            encode(&())
                        }
                        "poke" => {
                            this.pokes += 1;
                            cx.notify();
                            let input: String = decode(payload)?;
                            match this.mode.as_str() {
                                "echo" => encode(&input),
                                "shout" => encode(&input.to_uppercase()),
                                "reverse" => encode(&input.chars().rev().collect::<String>()),
                                other => Err(anyhow!("chameleon has no mode {other:?}")),
                            }
                        }
                        other => Err(anyhow!("chameleon does not understand {other:?}")),
                    })
                });
            },
            cx,
        );

        TestGuest {
            _counter: counter,
            _factory: factory,
            _chameleon: chameleon,
        }
    }

    fn create_view(&mut self, _view_id: u32, _window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| EmptyView).into()
    }
}

register_plugin!(TestGuest);

struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

struct Counter {
    count: u32,
}

impl SharedEntitySource<TestCounterSpec> for Counter {
    fn snapshot(&self, _cx: &App) -> TestCounterSnapshot {
        TestCounterSnapshot { count: self.count }
    }
}

impl HandleShared<TestIncrement> for Counter {
    fn handle(&mut self, message: TestIncrement, cx: &mut Context<Self>) -> u32 {
        self.count += message.by;
        cx.notify();
        self.count
    }
}

struct Factory {
    created: u32,
}

impl SharedEntitySource<FactorySpec> for Factory {
    fn snapshot(&self, _cx: &App) -> FactorySnapshot {
        FactorySnapshot {
            created: self.created,
        }
    }
}

struct Item {
    label: String,
    bumps: u32,
}

impl SharedEntitySource<ItemSpec> for Item {
    fn snapshot(&self, _cx: &App) -> ItemSnapshot {
        ItemSnapshot {
            label: self.label.clone(),
            bumps: self.bumps,
        }
    }
}

impl HandleShared<Bump> for Item {
    fn handle(&mut self, _message: Bump, cx: &mut Context<Self>) -> u32 {
        self.bumps += 1;
        cx.notify();
        self.bumps
    }
}

impl HandleShared<CreateItem> for Factory {
    fn handle(&mut self, message: CreateItem, cx: &mut Context<Self>) -> SharedRef<ItemSpec> {
        self.created += 1;
        cx.notify();
        let item: Entity<Item> = cx.new(|_| Item {
            label: message.label,
            bumps: 0,
        });
        gpui_plugin::shared::share_anonymous::<ItemSpec, _>(
            &item,
            |methods| {
                methods.on::<Bump>();
            },
            cx,
        )
    }
}

impl HandleShared<CreateReadonlyItem> for Factory {
    fn handle(
        &mut self,
        message: CreateReadonlyItem,
        cx: &mut Context<Self>,
    ) -> SharedRef<ItemSpec> {
        self.created += 1;
        cx.notify();
        let item: Entity<Item> = cx.new(|_| Item {
            label: message.label,
            bumps: 0,
        });
        // Attenuation: the same entity kind, shared with an empty method table. Holders can
        // subscribe and read, but every write is rejected by dispatch.
        gpui_plugin::shared::share_anonymous::<ItemSpec, _>(&item, |_methods| {}, cx)
    }
}

struct Chameleon {
    mode: String,
    pokes: u32,
}

impl SharedEntitySource<ChameleonSpec> for Chameleon {
    fn snapshot(&self, _cx: &App) -> ChameleonSnapshot {
        ChameleonSnapshot {
            mode: self.mode.clone(),
            pokes: self.pokes,
        }
    }
}
