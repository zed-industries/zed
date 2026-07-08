//! The guest half of `crates/gpui_embedded/tests/shared_entities.rs`: a zoo of guest-homed
//! entities exercising calls, capability refs, attenuation, and fully dynamic dispatch.

use anyhow::anyhow;
use gpui::{AnyView, App, Context, Entity, Task, Window, div, prelude::*};
use gpui_embedded_shared::test_schema::{
    Bump, ChameleonSnapshot, ChameleonSpec, CreateItem, FactorySnapshot, FactorySpec,
    GatekeeperSnapshot, GatekeeperSpec, Guard, ItemSnapshot, ItemSpec, TestCounterSnapshot,
    TestCounterSpec, TestIncrement, VaultSnapshot, VaultSpec,
};
use gpui_embedded_shared::{decode, encode};
use gpui_plugin::shared::{HandleShared, Remote, SharedEntitySource, SharedRef};
use gpui_plugin::{Plugin, register_plugin};

/// Named shares borrow their entities (the sharer owns the lifetime), so the plugin must
/// keep them alive; anonymous shares own theirs until released.
struct TestGuest {
    _counter: Entity<Counter>,
    _factory: Entity<Factory>,
    _chameleon: Entity<Chameleon>,
    _gatekeeper: Entity<Gatekeeper>,
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
                methods.on::<CreateItem>();
            },
            cx,
        );

        let gatekeeper = cx.new(|_| Gatekeeper { guarded: 0 });
        gpui_plugin::shared::share::<GatekeeperSpec, _>(
            &gatekeeper,
            "gatekeeper",
            |methods| {
                methods.on::<Guard>();
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
            _gatekeeper: gatekeeper,
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

struct Gatekeeper {
    guarded: u32,
}

impl SharedEntitySource<GatekeeperSpec> for Gatekeeper {
    fn snapshot(&self, _cx: &App) -> GatekeeperSnapshot {
        GatekeeperSnapshot {
            guarded: self.guarded,
        }
    }
}

/// A membrane: wraps a vault capability (itself a projection of a *host*-homed entity)
/// in a caretaker the guest controls. Callers get a ref that behaves exactly like the
/// vault — every method is forwarded asynchronously — until the guest revokes it by
/// dropping the wrapped capability.
struct Caretaker {
    vault: Option<Remote<VaultSpec>>,
}

impl SharedEntitySource<VaultSpec> for Caretaker {
    fn snapshot(&self, cx: &App) -> VaultSnapshot {
        match &self.vault {
            Some(vault) => vault
                .replica()
                .read(cx)
                .state
                .clone()
                .unwrap_or(VaultSnapshot {
                    label: "pending".to_string(),
                }),
            None => VaultSnapshot {
                label: "revoked".to_string(),
            },
        }
    }
}

impl HandleShared<Guard> for Gatekeeper {
    fn handle(&mut self, message: Guard, cx: &mut Context<Self>) -> SharedRef<VaultSpec> {
        self.guarded += 1;
        cx.notify();
        let vault = gpui_plugin::shared::remote_from_ref::<VaultSpec>(message.vault, cx);
        let caretaker = cx.new(|cx| {
            cx.observe(vault.replica(), |_: &mut Caretaker, _, cx| cx.notify())
                .detach();
            Caretaker { vault: Some(vault) }
        });
        gpui_plugin::shared::share_anonymous::<VaultSpec, _>(
            &caretaker,
            |methods| {
                methods
                    .on_raw("revoke", |entity, _method, _payload, cx| {
                        entity.update(cx, |caretaker, cx| {
                            // Dropping the Remote is the revocation: its guard sends
                            // `$release`, and the vault's home drops its strong handle.
                            caretaker.vault = None;
                            cx.notify();
                            encode(&())
                        })
                    })
                    .on_raw_async("*", |entity, method, payload, cx| {
                        let receipt = entity
                            .read(cx)
                            .vault
                            .as_ref()
                            .map(|vault| vault.forward(method, payload.to_vec()));
                        match receipt {
                            Some(receipt) => cx.spawn(async move |_| receipt.await),
                            None => Task::ready(Err(anyhow!("capability revoked"))),
                        }
                    });
            },
            cx,
        )
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
