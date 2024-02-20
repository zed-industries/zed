use crate::db::{Database, UserId};
use anyhow::Result;
use dashmap::DashMap;
use governor::{
    nanos::Nanos, state::{InMemoryState, StateStore as GovernorStateStore}, Quota,
    RateLimiter as GovernorRateLimiter,
};
use std::sync::{atomic::AtomicU64, Arc};

struct StateStore {
    rate_state: DashMap<String, AtomicU64>,

}

impl StateStore {
    pub(crate) fn measure_and_replace_one<T, F, E>(&self, mut f: F) -> Result<T, E>
        where
            F: FnMut(Option<Nanos>) -> Result<(T, Nanos), E>,
        {
            let mut prev = self.0.load(Ordering::Acquire);
            let mut decision = f(NonZeroU64::new(prev).map(|n| n.get().into()));
            while let Ok((result, new_data)) = decision {
                match self.0.compare_exchange_weak(
                    prev,
                    new_data.into(),
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return Ok(result),
                    Err(next_prev) => prev = next_prev,
                }
                decision = f(NonZeroU64::new(prev).map(|n| n.get().into()));
            }
            // This map shouldn't be needed, as we only get here in the error case, but the compiler
            // can't see it.
            decision.map(|(result, _)| result)
        }

}

impl GovernorStateStore for StateStore {
    type Key = String;

    fn measure_and_replace<T, F, E>(&self, key: &Self::Key, f: F) -> Result<T, E>
    where
        F: Fn(Option<Nanos>) -> Result<(T, Nanos), E>,
    {
        if let Some(v) = self.rate_state.get(key) {
            // fast path: measure existing entry
            return v.measure_and_replace_one(f);
        }

        // make an entry and measure that:
        let entry = self.entry(key.clone()).or_default();
        (*entry).measure_and_replace_one(f)
    }
}

pub struct RateLimiter {
    db: Arc<Database>,
    rate_state: DashMap<String, InMemoryState>,
    governer: GovernorRateLimiter<String>,
}

impl RateLimiter {
    pub async fn check(&self, user_id: UserId, key: &str) -> Result<()> {


        if !self.rate_state.contains_key((user_id, key)) {

            load_key()await
            // insert into rate state

        }

        // do the the governor thing
        // GovernorRateLimiter::keyed(Quota)

        // let rate_state = self.db.rate_stae
    }
}
