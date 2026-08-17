//! This module defines a load-balanced pool of services that adds new services when load is high.
//!
//! The pool uses `poll_ready` as a signal indicating whether additional services should be spawned
//! to handle the current level of load. Specifically, every time `poll_ready` on the inner service
//! returns `Ready`, [`Pool`] consider that a 0, and every time it returns `Pending`, [`Pool`]
//! considers it a 1. [`Pool`] then maintains an [exponential moving
//! average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) over those
//! samples, which gives an estimate of how often the underlying service has been ready when it was
//! needed "recently" (see [`Builder::urgency`]). If the service is loaded (see
//! [`Builder::loaded_above`]), a new service is created and added to the underlying [`Balance`].
//! If the service is underutilized (see [`Builder::underutilized_below`]) and there are two or
//! more services, then the latest added service is removed. In either case, the load estimate is
//! reset to its initial value (see [`Builder::initial`] to prevent services from being rapidly
//! added or removed.
#![deny(missing_docs)]

use super::p2c::Balance;
use crate::discover::Change;
use crate::load::Load;
use crate::make::MakeService;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use slab::Slab;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Level {
    /// Load is low -- remove a service instance.
    Low,
    /// Load is normal -- keep the service set as it is.
    Normal,
    /// Load is high -- add another service instance.
    High,
}

pin_project! {
    /// A wrapper around `MakeService` that discovers a new service when load is high, and removes a
    /// service when load is low. See [`Pool`].
    pub struct PoolDiscoverer<MS, Target, Request>
    where
        MS: MakeService<Target, Request>,
    {
        maker: MS,
        #[pin]
        making: Option<MS::Future>,
        target: Target,
        load: Level,
        services: Slab<()>,
        died_tx: tokio::sync::mpsc::UnboundedSender<usize>,
        #[pin]
        died_rx: tokio::sync::mpsc::UnboundedReceiver<usize>,
        limit: Option<usize>,
    }
}

impl<MS, Target, Request> fmt::Debug for PoolDiscoverer<MS, Target, Request>
where
    MS: MakeService<Target, Request> + fmt::Debug,
    Target: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoolDiscoverer")
            .field("maker", &self.maker)
            .field("making", &self.making.is_some())
            .field("target", &self.target)
            .field("load", &self.load)
            .field("services", &self.services)
            .field("limit", &self.limit)
            .finish()
    }
}

impl<MS, Target, Request> Stream for PoolDiscoverer<MS, Target, Request>
where
    MS: MakeService<Target, Request>,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    type Item = Result<Change<usize, DropNotifyService<MS::Service>>, MS::MakeError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        while let Poll::Ready(Some(sid)) = this.died_rx.as_mut().poll_recv(cx) {
            this.services.remove(sid);
            tracing::trace!(
                pool.services = this.services.len(),
                message = "removing dropped service"
            );
        }

        if this.services.is_empty() && this.making.is_none() {
            let _ = ready!(this.maker.poll_ready(cx))?;
            tracing::trace!("construct initial pool connection");
            this.making
                .set(Some(this.maker.make_service(this.target.clone())));
        }

        if let Level::High = this.load {
            if this.making.is_none() {
                if this
                    .limit
                    .map(|limit| this.services.len() >= limit)
                    .unwrap_or(false)
                {
                    return Poll::Pending;
                }

                tracing::trace!(
                    pool.services = this.services.len(),
                    message = "decided to add service to loaded pool"
                );
                ready!(this.maker.poll_ready(cx))?;
                tracing::trace!("making new service");
                // TODO: it'd be great if we could avoid the clone here and use, say, &Target
                this.making
                    .set(Some(this.maker.make_service(this.target.clone())));
            }
        }

        if let Some(fut) = this.making.as_mut().as_pin_mut() {
            let svc = ready!(fut.poll(cx))?;
            this.making.set(None);

            let id = this.services.insert(());
            let svc = DropNotifyService {
                svc,
                id,
                notify: this.died_tx.clone(),
            };
            tracing::trace!(
                pool.services = this.services.len(),
                message = "finished creating new service"
            );
            *this.load = Level::Normal;
            return Poll::Ready(Some(Ok(Change::Insert(id, svc))));
        }

        match this.load {
            Level::High => {
                unreachable!("found high load but no Service being made");
            }
            Level::Normal => Poll::Pending,
            Level::Low if this.services.len() == 1 => Poll::Pending,
            Level::Low => {
                *this.load = Level::Normal;
                // NOTE: this is a little sad -- we'd prefer to kill short-living services
                let rm = this.services.iter().next().unwrap().0;
                // note that we _don't_ remove from self.services here
                // that'll happen automatically on drop
                tracing::trace!(
                    pool.services = this.services.len(),
                    message = "removing service for over-provisioned pool"
                );
                Poll::Ready(Some(Ok(Change::Remove(rm))))
            }
        }
    }
}

/// A [builder] that lets you configure how a [`Pool`] determines whether the underlying service is
/// loaded or not. See the [module-level documentation](self) and the builder's methods for
/// details.
///
///  [builder]: https://rust-lang-nursery.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
#[derive(Copy, Clone, Debug)]
pub struct Builder {
    low: f64,
    high: f64,
    init: f64,
    alpha: f64,
    limit: Option<usize>,
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            init: 0.1,
            low: 0.00001,
            high: 0.2,
            alpha: 0.03,
            limit: None,
        }
    }
}

impl Builder {
    /// Create a new builder with default values for all load settings.
    ///
    /// If you just want to use the defaults, you can just use [`Pool::new`].
    pub fn new() -> Self {
        Self::default()
    }

    /// When the estimated load (see the [module-level docs](self)) drops below this
    /// threshold, and there are at least two services active, a service is removed.
    ///
    /// The default value is 0.01. That is, when one in every 100 `poll_ready` calls return
    /// `Pending`, then the underlying service is considered underutilized.
    pub fn underutilized_below(&mut self, low: f64) -> &mut Self {
        self.low = low;
        self
    }

    /// When the estimated load (see the [module-level docs](self)) exceeds this
    /// threshold, and no service is currently in the process of being added, a new service is
    /// scheduled to be added to the underlying [`Balance`].
    ///
    /// The default value is 0.5. That is, when every other call to `poll_ready` returns
    /// `Pending`, then the underlying service is considered highly loaded.
    pub fn loaded_above(&mut self, high: f64) -> &mut Self {
        self.high = high;
        self
    }

    /// The initial estimated load average.
    ///
    /// This is also the value that the estimated load will be reset to whenever a service is added
    /// or removed.
    ///
    /// The default value is 0.1.
    pub fn initial(&mut self, init: f64) -> &mut Self {
        self.init = init;
        self
    }

    /// How aggressively the estimated load average is updated.
    ///
    /// This is the α parameter of the formula for the [exponential moving
    /// average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average), and
    /// dictates how quickly new samples of the current load affect the estimated load. If the
    /// value is closer to 1, newer samples affect the load average a lot (when α is 1, the load
    /// average is immediately set to the current load). If the value is closer to 0, newer samples
    /// affect the load average very little at a time.
    ///
    /// The given value is clamped to `[0,1]`.
    ///
    /// The default value is 0.05, meaning, in very approximate terms, that each new load sample
    /// affects the estimated load by 5%.
    pub fn urgency(&mut self, alpha: f64) -> &mut Self {
        self.alpha = alpha.max(0.0).min(1.0);
        self
    }

    /// The maximum number of backing `Service` instances to maintain.
    ///
    /// When the limit is reached, the load estimate is clamped to the high load threshhold, and no
    /// new service is spawned.
    ///
    /// No maximum limit is imposed by default.
    pub fn max_services(&mut self, limit: Option<usize>) -> &mut Self {
        self.limit = limit;
        self
    }

    /// See [`Pool::new`].
    pub fn build<MS, Target, Request>(
        &self,
        make_service: MS,
        target: Target,
    ) -> Pool<MS, Target, Request>
    where
        MS: MakeService<Target, Request>,
        MS::Service: Load,
        <MS::Service as Load>::Metric: std::fmt::Debug,
        MS::MakeError: Into<crate::BoxError>,
        MS::Error: Into<crate::BoxError>,
        Target: Clone,
    {
        let (died_tx, died_rx) = tokio::sync::mpsc::unbounded_channel();
        let d = PoolDiscoverer {
            maker: make_service,
            making: None,
            target,
            load: Level::Normal,
            services: Slab::new(),
            died_tx,
            died_rx,
            limit: self.limit,
        };

        Pool {
            balance: Balance::new(Box::pin(d)),
            options: *self,
            ewma: self.init,
        }
    }
}

/// A dynamically sized, load-balanced pool of `Service` instances.
pub struct Pool<MS, Target, Request>
where
    MS: MakeService<Target, Request>,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    // the Pin<Box<_>> here is needed since Balance requires the Service to be Unpin
    balance: Balance<Pin<Box<PoolDiscoverer<MS, Target, Request>>>, Request>,
    options: Builder,
    ewma: f64,
}

impl<MS, Target, Request> fmt::Debug for Pool<MS, Target, Request>
where
    MS: MakeService<Target, Request> + fmt::Debug,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone + fmt::Debug,
    MS::Service: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pool")
            .field("balance", &self.balance)
            .field("options", &self.options)
            .field("ewma", &self.ewma)
            .finish()
    }
}

impl<MS, Target, Request> Pool<MS, Target, Request>
where
    MS: MakeService<Target, Request>,
    MS::Service: Load,
    <MS::Service as Load>::Metric: std::fmt::Debug,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    /// Construct a new dynamically sized `Pool`.
    ///
    /// If many calls to `poll_ready` return `Pending`, `new_service` is used to
    /// construct another `Service` that is then added to the load-balanced pool.
    /// If many calls to `poll_ready` succeed, the most recently added `Service`
    /// is dropped from the pool.
    pub fn new(make_service: MS, target: Target) -> Self {
        Builder::new().build(make_service, target)
    }
}

type PinBalance<S, Request> = Balance<Pin<Box<S>>, Request>;

impl<MS, Target, Req> Service<Req> for Pool<MS, Target, Req>
where
    MS: MakeService<Target, Req>,
    MS::Service: Load,
    <MS::Service as Load>::Metric: std::fmt::Debug,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    type Response = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Response;
    type Error = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Error;
    type Future = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Poll::Ready(()) = self.balance.poll_ready(cx)? {
            // services was ready -- there are enough services
            // update ewma with a 0 sample
            self.ewma *= 1.0 - self.options.alpha;

            let discover = self.balance.discover_mut().as_mut().project();
            if self.ewma < self.options.low {
                if *discover.load != Level::Low {
                    tracing::trace!({ ewma = %self.ewma }, "pool is over-provisioned");
                }
                *discover.load = Level::Low;

                if discover.services.len() > 1 {
                    // reset EWMA so we don't immediately try to remove another service
                    self.ewma = self.options.init;
                }
            } else {
                if *discover.load != Level::Normal {
                    tracing::trace!({ ewma = %self.ewma }, "pool is appropriately provisioned");
                }
                *discover.load = Level::Normal;
            }

            return Poll::Ready(Ok(()));
        }

        let discover = self.balance.discover_mut().as_mut().project();
        if discover.making.is_none() {
            // no services are ready -- we're overloaded
            // update ewma with a 1 sample
            self.ewma = self.options.alpha + (1.0 - self.options.alpha) * self.ewma;

            if self.ewma > self.options.high {
                if *discover.load != Level::High {
                    tracing::trace!({ ewma = %self.ewma }, "pool is under-provisioned");
                }
                *discover.load = Level::High;

                // don't reset the EWMA -- in theory, poll_ready should now start returning
                // `Ready`, so we won't try to launch another service immediately.
                // we clamp it to high though in case the # of services is limited.
                self.ewma = self.options.high;

                // we need to call balance again for PoolDiscover to realize
                // it can make a new service
                return self.balance.poll_ready(cx);
            } else {
                *discover.load = Level::Normal;
            }
        }

        Poll::Pending
    }

    fn call(&mut self, req: Req) -> Self::Future {
        self.balance.call(req)
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct DropNotifyService<Svc> {
    svc: Svc,
    id: usize,
    notify: tokio::sync::mpsc::UnboundedSender<usize>,
}

impl<Svc> Drop for DropNotifyService<Svc> {
    fn drop(&mut self) {
        let _ = self.notify.send(self.id).is_ok();
    }
}

impl<Svc: Load> Load for DropNotifyService<Svc> {
    type Metric = Svc::Metric;
    fn load(&self) -> Self::Metric {
        self.svc.load()
    }
}

impl<Request, Svc: Service<Request>> Service<Request> for DropNotifyService<Svc> {
    type Response = Svc::Response;
    type Future = Svc::Future;
    type Error = Svc::Error;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.svc.call(req)
    }
}
