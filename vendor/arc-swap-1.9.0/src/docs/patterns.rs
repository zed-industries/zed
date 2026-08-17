//! Common use patterns
//!
//! Here are some common patterns one can use for inspiration. These are mostly covered by examples
//! at the right type in the crate, but this lists them at a single place.
//!
//! # Sharing of configuration data
//!
//! We want to share configuration from some source with rare updates to some high performance
//! worker threads. It can be configuration in its true sense, or a routing table.
//!
//! The idea here is, each new version is a newly allocated in its own [`Arc`]. It is then stored
//! into a *shared* `ArcSwap` instance.
//!
//! Each worker then loads the current version before each work chunk. In case a new version is
//! stored, the worker keeps using the loaded one until it ends the work chunk and, if it's the
//! last one to have the version, deallocates it automatically by dropping the [`Guard`]
//!
//! Note that the configuration needs to be passed through a *single shared* [`ArcSwap`]. That
//! means we need to share that instance and we do so through an [`Arc`] (one could use a global
//! variable instead).
//!
//! Therefore, what we have is `Arc<ArcSwap<Config>>`.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use std::sync::atomic::{AtomicBool, Ordering};
//! # use std::thread;
//! # use std::time::Duration;
//! #
//! # use arc_swap::ArcSwap;
//! # struct Work;
//! # impl Work { fn fetch() -> Self { Work } fn perform(&self, _: &Config) {} }
//! #
//! #[derive(Debug, Default)]
//! struct Config {
//!     // ... Stuff in here ...
//! }
//!
//! // We wrap the ArcSwap into an Arc, so we can share it between threads.
//! let config = Arc::new(ArcSwap::from_pointee(Config::default()));
//!
//! let terminate = Arc::new(AtomicBool::new(false));
//! let mut threads = Vec::new();
//!
//! // The configuration thread
//! threads.push(thread::spawn({
//!     let config = Arc::clone(&config);
//!     let terminate = Arc::clone(&terminate);
//!     move || {
//!         while !terminate.load(Ordering::Relaxed) {
//!             thread::sleep(Duration::from_secs(6));
//!             // Actually, load it from somewhere
//!             let new_config = Arc::new(Config::default());
//!             config.store(new_config);
//!         }
//!     }
//! }));
//!
//! // The worker thread
//! for _ in 0..10 {
//!     threads.push(thread::spawn({
//!         let config = Arc::clone(&config);
//!         let terminate = Arc::clone(&terminate);
//!         move || {
//!             while !terminate.load(Ordering::Relaxed) {
//!                 let work = Work::fetch();
//!                 let config = config.load();
//!                 work.perform(&config);
//!             }
//!         }
//!     }));
//! }
//!
//! // Terminate gracefully
//! terminate.store(true, Ordering::Relaxed);
//! for thread in threads {
//!     thread.join().unwrap();
//! }
//! ```
//!
//! # Consistent snapshots
//!
//! While one probably wants to get a fresh instance every time a work chunk is available,
//! therefore there would be one [`load`] for each work chunk, it is often also important that the
//! configuration doesn't change in the *middle* of processing of one chunk. Therefore, one
//! commonly wants *exactly* one [`load`] for the work chunk, not *at least* one. If the processing
//! had multiple phases, one would use something like this:
//!
//! ```rust
//! # use std::sync::Arc;
//! #
//! # use arc_swap::ArcSwap;
//! # struct Config;
//! # struct Work;
//! # impl Work {
//! #     fn fetch() -> Self { Work }
//! #     fn phase_1(&self, _: &Config) {}
//! #     fn phase_2(&self, _: &Config) {}
//! # }
//! # let config = Arc::new(ArcSwap::from_pointee(Config));
//! let work = Work::fetch();
//! let config = config.load();
//! work.phase_1(&config);
//! // We keep the same config value here
//! work.phase_2(&config);
//! ```
//!
//! Over this:
//!
//! ```rust
//! # use std::sync::Arc;
//! #
//! # use arc_swap::ArcSwap;
//! # struct Config;
//! # struct Work;
//! # impl Work {
//! #     fn fetch() -> Self { Work }
//! #     fn phase_1(&self, _: &Config) {}
//! #     fn phase_2(&self, _: &Config) {}
//! # }
//! # let config = Arc::new(ArcSwap::from_pointee(Config));
//! let work = Work::fetch();
//! work.phase_1(&config.load());
//! // WARNING!! This is broken, because in between phase_1 and phase_2, the other thread could
//! // have replaced the config. Then each phase would be performed with a different one and that
//! // could lead to surprises.
//! work.phase_2(&config.load());
//! ```
//!
//! # Caching of the configuration
//!
//! Let's say that the work chunks are really small, but there's *a lot* of them to work on. Maybe
//! we are routing packets and the configuration is the routing table that can sometimes change,
//! but mostly doesn't.
//!
//! There's an overhead to [`load`]. If the work chunks are small enough, that could be measurable.
//! We can reach for [`Cache`]. It makes loads much faster (in the order of accessing local
//! variables) in case nothing has changed. It has two costs, it makes the load slightly slower in
//! case the thing *did* change (which is rare) and if the worker is inactive, it holds the old
//! cached value alive.
//!
//! This is OK for our use case, because the routing table is usually small enough so some stale
//! instances taking a bit of memory isn't an issue.
//!
//! The part that takes care of updates stays the same as above.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use std::thread;
//! # use std::sync::atomic::{AtomicBool, Ordering};
//! # use arc_swap::{ArcSwap, Cache};
//! # struct Packet; impl Packet { fn receive() -> Self { Packet } }
//!
//! #[derive(Debug, Default)]
//! struct RoutingTable {
//!     // ... Stuff in here ...
//! }
//!
//! impl RoutingTable {
//!     fn route(&self, _: Packet) {
//!         // ... Interesting things are done here ...
//!     }
//! }
//!
//! let routing_table = Arc::new(ArcSwap::from_pointee(RoutingTable::default()));
//!
//! let terminate = Arc::new(AtomicBool::new(false));
//! let mut threads = Vec::new();
//!
//! for _ in 0..10 {
//!     let t = thread::spawn({
//!         let routing_table = Arc::clone(&routing_table);
//!         let terminate = Arc::clone(&terminate);
//!         move || {
//!             let mut routing_table = Cache::new(routing_table);
//!             while !terminate.load(Ordering::Relaxed) {
//!                 let packet = Packet::receive();
//!                 // This load is cheaper, because we cache in the private Cache thing.
//!                 // But if the above receive takes a long time, the Cache will keep the stale
//!                 // value  alive until this time (when it will get replaced by up to date value).
//!                 let current = routing_table.load();
//!                 current.route(packet);
//!             }
//!         }
//!     });
//!     threads.push(t);
//! }
//!
//! // Shut down properly
//! terminate.store(true, Ordering::Relaxed);
//! for thread in threads {
//!     thread.join().unwrap();
//! }
//! ```
//!
//! # Projecting into configuration field
//!
//! We have a larger application, composed of multiple components. Each component has its own
//! `ComponentConfig` structure. Then, the whole application has a `Config` structure that contains
//! a component config for each component:
//!
//! ```rust
//! # struct ComponentConfig;
//!
//! struct Config {
//!     component: ComponentConfig,
//!     // ... Some other components and things ...
//! }
//! # let c = Config { component: ComponentConfig };
//! # let _ = c.component;
//! ```
//!
//! We would like to use [`ArcSwap`] to push updates to the components. But for various reasons,
//! it's not a good idea to put the whole `ArcSwap<Config>` to each component, eg:
//!
//! * That would make each component depend on the top level config, which feels reversed.
//! * It doesn't allow reusing the same component in multiple applications, as these would have
//!   different `Config` structures.
//! * One needs to build the whole `Config` for tests.
//! * There's a risk of entanglement, that the component would start looking at configuration of
//!   different parts of code, which would be hard to debug.
//!
//! We also could have a separate `ArcSwap<ComponentConfig>` for each component, but that also
//! doesn't feel right, as we would have to push updates to multiple places and they could be
//! inconsistent for a while and we would have to decompose the `Config` structure into the parts,
//! because we need our things in [`Arc`]s to be put into [`ArcSwap`].
//!
//! This is where the [`Access`] trait comes into play. The trait abstracts over things that can
//! give access to up to date version of specific T. That can be a [`Constant`] (which is useful
//! mostly for the tests, where one doesn't care about the updating), it can be an
//! [`ArcSwap<T>`][`ArcSwap`] itself, but it also can be an [`ArcSwap`] paired with a closure to
//! project into the specific field. The [`DynAccess`] is similar, but allows type erasure. That's
//! more convenient, but a little bit slower.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use arc_swap::ArcSwap;
//! # use arc_swap::access::{DynAccess, Map};
//!
//! #[derive(Debug, Default)]
//! struct ComponentConfig;
//!
//! struct Component {
//!     config: Box<dyn DynAccess<ComponentConfig>>,
//! }
//!
//! #[derive(Debug, Default)]
//! struct Config {
//!     component: ComponentConfig,
//! }
//!
//! let config = Arc::new(ArcSwap::from_pointee(Config::default()));
//!
//! let component = Component {
//!     config: Box::new(Map::new(Arc::clone(&config), |config: &Config| &config.component)),
//! };
//! # let _ = component.config;
//! ```
//!
//! One would use `Box::new(Constant(ComponentConfig))` in unittests instead as the `config` field.
//!
//! The [`Cache`] has its own [`Access`][crate::cache::Access] trait for similar purposes.
//!
//! [`Arc`]: std::sync::Arc
//! [`Guard`]: crate::Guard
//! [`load`]: crate::ArcSwapAny::load
//! [`ArcSwap`]: crate::ArcSwap
//! [`Cache`]: crate::cache::Cache
//! [`Access`]: crate::access::Access
//! [`DynAccess`]: crate::access::DynAccess
//! [`Constant`]: crate::access::Constant
