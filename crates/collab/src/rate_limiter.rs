use crate::db::UserId;
use crate::{Database, Error};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use parking_lot::Mutex;
use sea_orm::prelude::DateTimeUtc;
use std::any::TypeId;
use std::sync::Arc;
use util::ResultExt;

trait RateLimit: 'static {
    fn capacity() -> usize;
    fn refill_duration() -> Duration;
    fn db_name() -> &'static str;
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }
}

struct RateLimiter {
    buckets: DashMap<(UserId, TypeId), Arc<Mutex<RateBucket>>>,
    db: Arc<Database>,
}

impl RateLimiter {
    pub fn new(db: Arc<Database>) -> Self {
        RateLimiter {
            buckets: DashMap::new(),
            db,
        }
    }

    pub async fn allow<T: RateLimit>(&self, user_id: UserId) -> bool {
        let type_id = T::type_id();
        let bucket_key = (user_id, type_id);

        // Attempt to fetch the bucket from the database if it hasn't been cached
        // For now, we keep buckets in memory for the lifetime of the process rather than expiring them,
        // but this enforces limits across restarts so long as the database is reachable.
        if !self.buckets.contains_key(&bucket_key) {
            if let Some(bucket) = self.load_bucket::<T>(user_id).await.log_err().flatten() {
                self.buckets
                    .insert(bucket_key, Arc::new(Mutex::new(bucket)));
            }
        }

        let bucket = self
            .buckets
            .entry(bucket_key)
            .or_insert_with(|| {
                Arc::new(Mutex::new(RateBucket::new(
                    T::capacity(),
                    T::refill_duration(),
                )))
            })
            .value()
            .clone();

        let allowed = bucket.lock().allow();
        allowed
    }

    async fn load_bucket<K: RateLimit>(
        &self,
        user_id: UserId,
    ) -> Result<Option<RateBucket>, Error> {
        Ok(self
            .db
            .get_rate_bucket(user_id, K::db_name())
            .await?
            .map(|saved_bucket| RateBucket {
                capacity: K::capacity(),
                refill_time_per_token: K::refill_duration(),
                token_count: saved_bucket.token_count as usize,
                last_refill: DateTime::from_naive_utc_and_offset(saved_bucket.last_refill, Utc),
            }))
    }
}

struct RateBucket {
    capacity: usize,
    token_count: usize,
    refill_time_per_token: Duration,
    last_refill: DateTimeUtc,
}

impl RateBucket {
    fn new(capacity: usize, refill_duration: Duration) -> Self {
        RateBucket {
            capacity,
            token_count: capacity,
            refill_time_per_token: refill_duration / capacity as i32,
            last_refill: Utc::now(),
        }
    }

    fn allow(&mut self) -> bool {
        self.refill();
        if self.token_count > 0 {
            self.token_count -= 1;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = now - self.last_refill;
        if elapsed >= self.refill_time_per_token {
            let new_tokens =
                elapsed.num_milliseconds() / self.refill_time_per_token.num_milliseconds();

            self.token_count = (self.token_count + new_tokens as usize).min(self.capacity);
            self.last_refill = now;
        }
    }
}
