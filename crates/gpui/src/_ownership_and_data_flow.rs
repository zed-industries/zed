//! In GPUI, every model or view in the application is actually owned by a single top-level object called the `App`. When a new entity or view is created (referred to collectively as _entities_), the application is given ownership of their state to enable their participation in a variety of app services and interaction with other entities.
//!
//! To illustrate, consider the trivial app below. We start the app by calling `run` with a callback, which is passed a reference to the `App` that owns all the state for the application. This `App` is our gateway to all application-level services, such as opening windows, presenting dialogs, etc. It also has an `insert_entity` method, which is called below to create an entity and give ownership of it to the application.
//!
//! ```no_run
//! # use gpui::{App, AppContext, Application, Entity};
//! # struct Counter {
//! #     count: usize,
//! # }
//! Application::new().run(|cx: &mut App| {
//!     let _counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
//!     // ...
//! });
//! ```
//!
//! The call to `new_entity` returns an _entity handle_, which carries a type parameter based on the type of object it references. By itself, this `Entity<Counter>` handle doesn't provide access to the entity's state. It's merely an inert identifier plus a compile-time type tag, and it maintains a reference counted pointer to the underlying `Counter` object that is owned by the app.
//!
//! Much like an `Rc` from the Rust standard library, this reference count is incremented when the handle is cloned and decremented when it is dropped to enable shared ownership over the underlying model, but unlike an `Rc` it only provides access to the model's state when a reference to an `App` is available. The handle doesn't truly _own_ the state, but it can be used to access the state from its true owner, the `App`. Stripping away some of the setup code for brevity:
//!
//! ```no_run
//! # use gpui::{App, AppContext, Application, Context, Entity};
//! # struct Counter {
//! #     count: usize,
//! # }
//! Application::new().run(|cx: &mut App| {
//!     let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
//!     // Call `update` to access the model's state.
//!     counter.update(cx, |counter: &mut Counter, _cx: &mut Context<Counter>| {
//!         counter.count += 1;
//!     });
//! });
//! ```
//!
//! To update the counter, we call `update` on the handle, passing the context reference and a callback. The callback is yielded a mutable reference to the counter, which can be used to manipulate state.
//!
//! The callback is also provided a second `Context<Counter>` reference. This reference is similar to the `App` reference provided to the `run` callback. A `Context` is actually a wrapper around the `App`, including some additional data to indicate which particular entity it is tied to; in this case the counter.
//!
//! In addition to the application-level services provided by `App`, a `Context` provides access to entity-level services. For example, it can be used it to inform observers of this entity that its state has changed. Let's add that to our example, by calling `cx.notify()`.
//!
//! ```no_run
//! # use gpui::{App, AppContext, Application, Entity};
//! # struct Counter {
//! #     count: usize,
//! # }
//! Application::new().run(|cx: &mut App| {
//!     let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
//!     counter.update(cx, |counter, cx| {
//!         counter.count += 1;
//!         cx.notify(); // Notify observers
//!     });
//! });
//! ```
//!
//! Next, these notifications need to be observed and reacted to. Before updating the counter, we'll construct a second counter that observes it. Whenever the first counter changes, twice its count is assigned to the second counter. Note how `observe` is called on the `Context` belonging to our second counter to arrange for it to be notified whenever the first counter notifies. The call to `observe` returns a `Subscription`, which is `detach`ed to preserve this behavior for as long as both counters exist. We could also store this subscription and drop it at a time of our choosing to cancel this behavior.
//!
//! The `observe` callback is passed a mutable reference to the observer and a _handle_ to the observed counter, whose state we access with the `read` method.
//!
//! ```no_run
//!  # use gpui::{App, AppContext, Application, Entity, prelude::*};
//!  # struct Counter {
//!  #     count: usize,
//!  # }
//!  Application::new().run(|cx: &mut App| {
//!      let first_counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
//!
//!      let second_counter = cx.new(|cx: &mut Context<Counter>| {
//!          // Note we can set up the callback before the Counter is even created!
//!          cx.observe(
//!              &first_counter,
//!              |second: &mut Counter, first: Entity<Counter>, cx| {
//!                  second.count = first.read(cx).count * 2;
//!              },
//!          )
//!          .detach();
//!
//!          Counter { count: 0 }
//!      });
//!
//!      first_counter.update(cx, |counter, cx| {
//!          counter.count += 1;
//!          cx.notify();
//!      });
//!
//!      assert_eq!(second_counter.read(cx).count, 2);
//!  });
//! ```
//!
//! After updating the first counter, it can be noted that the observing counter's state is maintained according to our subscription.
//!
//! In addition to `observe` and `notify`, which indicate that an entity's state has changed, GPUI also offers `subscribe` and `emit`, which enables entities to emit typed events. To opt into this system, the emitting object must implement the `EventEmitter` trait.
//!
//! Let's introduce a new event type called `CounterChangeEvent`, then indicate that `Counter` can emit this type of event:
//!
//! ```no_run
//! use gpui::EventEmitter;
//! # struct Counter {
//! #     count: usize,
//! # }
//! struct CounterChangeEvent {
//!     increment: usize,
//! }
//!
//! impl EventEmitter<CounterChangeEvent> for Counter {}
//! ```
//!
//! Next, the example should be updated, replacing the observation with a subscription. Whenever the counter is incremented, a `Change` event is emitted to indicate the magnitude of the increase.
//!
//! ```no_run
//! # use gpui::{App, AppContext, Application, Context, Entity, EventEmitter};
//! # struct Counter {
//! #     count: usize,
//! # }
//! # struct CounterChangeEvent {
//! #     increment: usize,
//! # }
//! # impl EventEmitter<CounterChangeEvent> for Counter {}
//! Application::new().run(|cx: &mut App| {
//!     let first_counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
//!
//!     let second_counter = cx.new(|cx: &mut Context<Counter>| {
//!         // Note we can set up the callback before the Counter is even created!
//!         cx.subscribe(&first_counter, |second: &mut Counter, _first: Entity<Counter>, event, _cx| {
//!             second.count += event.increment * 2;
//!         })
//!         .detach();
//!
//!         Counter {
//!             count: first_counter.read(cx).count * 2,
//!         }
//!     });
//!
//!     first_counter.update(cx, |first, cx| {
//!         first.count += 2;
//!         cx.emit(CounterChangeEvent { increment: 2 });
//!         cx.notify();
//!     });
//!
//!     assert_eq!(second_counter.read(cx).count, 4);
//! });
//! ```
