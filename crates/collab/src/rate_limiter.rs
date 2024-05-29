use crate::{db::UserId, executor::Executor, Database, Error, Result};
use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use dashmap::{DashMap, DashSet};
use sea_orm::prelude::DateTimeUtc;
use std::sync::Arc;
use util::ResultExt;

pub trait RateLimit: 'static {
    fn capacity() -> usize;
    fn refill_duration() -> Duration;
    fn db_name() -> &'static str;
}

/// Used to enforce per-user rate limits
pub struct RateLimiter {
    buckets: DashMap<(UserId, String), RateBucket>,
    dirty_buckets: DashSet<(UserId, String)>,
    db: Arc<Database>,
}

impl RateLimiter {
    pub fn new(db: Arc<Database>) -> Self {
        RateLimiter {
            buckets: DashMap::new(),
            dirty_buckets: DashSet::new(),
            db,
        }
    }

    /// Spawns a new task that periodically saves rate limit data to the database.
    pub fn save_periodically(rate_limiter: Arc<Self>, executor: Executor) {
        const RATE_LIMITER_SAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(10);

        executor.clone().spawn_detached(async move {
            loop {
                executor.sleep(RATE_LIMITER_SAVE_INTERVAL).await;
                rate_limiter.save().await.log_err();
            }
        });
    }

    /// Returns an error if the user has exceeded the specified `RateLimit`.
    /// Attempts to read the from the database if no cached RateBucket currently exists.
    pub async fn check<T: RateLimit>(&self, user_id: UserId) -> Result<()> {
        self.check_internal::<T>(user_id, Utc::now()).await
    }

    async fn check_internal<T: RateLimit>(&self, user_id: UserId, now: DateTimeUtc) -> Result<()> {
        let bucket_key = (user_id, T::db_name().to_string());

        // Attempt to fetch the bucket from the database if it hasn't been cached.
        // For now, we keep buckets in memory for the lifetime of the process rather than expiring them,
        // but this enforces limits across restarts so long as the database is reachable.
        if !self.buckets.contains_key(&bucket_key) {
            if let Some(bucket) = self.load_bucket::<T>(user_id).await.log_err().flatten() {
                self.buckets.insert(bucket_key.clone(), bucket);
                self.dirty_buckets.insert(bucket_key.clone());
            }
        }

        let mut bucket = self
            .buckets
            .entry(bucket_key.clone())
            .or_insert_with(|| RateBucket::new::<T>(now));

        if bucket.value_mut().allow(now) {
            self.dirty_buckets.insert(bucket_key);
            Ok(())
        } else {
            Err(anyhow!("rate limit exceeded"))?
        }
    }

    async fn load_bucket<T: RateLimit>(
        &self,
        user_id: UserId,
    ) -> Result<Option<RateBucket>, Error> {
        Ok(self
            .db
            .get_rate_bucket(user_id, T::db_name())
            .await?
            .map(|saved_bucket| {
                RateBucket::from_db::<T>(
                    saved_bucket.token_count as usize,
                    DateTime::from_naive_utc_and_offset(saved_bucket.last_refill, Utc),
                )
            }))
    }

    pub async fn save(&self) -> Result<()> {
        let mut buckets = Vec::new();
        self.dirty_buckets.retain(|key| {
            if let Some(bucket) = self.buckets.get(&key) {
                buckets.push(crate::db::rate_buckets::Model {
                    user_id: key.0,
                    rate_limit_name: key.1.clone(),
                    token_count: bucket.token_count as i32,
                    last_refill: bucket.last_refill.naive_utc(),
                });
            }
            false
        });

        match self.db.save_rate_buckets(&buckets).await {
            Ok(()) => Ok(()),
            Err(err) => {
                for bucket in buckets {
                    self.dirty_buckets
                        .insert((bucket.user_id, bucket.rate_limit_name));
                }
                Err(err)
            }
        }
    }
}

#[derive(Clone)]
struct RateBucket {
    capacity: usize,
    token_count: usize,
    refill_time_per_token: Duration,
    last_refill: DateTimeUtc,
}

impl RateBucket {
    fn new<T: RateLimit>(now: DateTimeUtc) -> Self {
        Self {
            capacity: T::capacity(),
            token_count: T::capacity(),
            refill_time_per_token: T::refill_duration() / T::capacity() as i32,
            last_refill: now,
        }
    }

    fn from_db<T: RateLimit>(token_count: usize, last_refill: DateTimeUtc) -> Self {
        Self {
            capacity: T::capacity(),
            token_count,
            refill_time_per_token: T::refill_duration() / T::capacity() as i32,
            last_refill,
        }
    }

    fn allow(&mut self, now: DateTimeUtc) -> bool {
        self.refill(now);
        if self.token_count > 0 {
            self.token_count -= 1;
            true
        } else {
            false
        }
    }

    fn refill(&mut self, now: DateTimeUtc) {
        let elapsed = now - self.last_refill;
        if elapsed >= self.refill_time_per_token {
            let new_tokens =
                elapsed.num_milliseconds() / self.refill_time_per_token.num_milliseconds();
            self.token_count = (self.token_count + new_tokens as usize).min(self.capacity);

            let unused_refill_time = Duration::milliseconds(
                elapsed.num_milliseconds() % self.refill_time_per_token.num_milliseconds(),
            );
            self.last_refill = now - unused_refill_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{NewUserParams, TestDb};
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_rate_limiter(cx: &mut TestAppContext) {
        let test_db = TestDb::sqlite(cx.executor().clone());
        let db = test_db.db().clone();
        let user_1 = db
            .create_user(
                "user-1@zed.dev",
                false,
                NewUserParams {
                    github_login: "user-1".into(),
                    github_user_id: 1,
                },
            )
            .await
            .unwrap()
            .user_id;
        let user_2 = db
            .create_user(
                "user-2@zed.dev",
                false,
                NewUserParams {
                    github_login: "user-2".into(),
                    github_user_id: 2,
                },
            )
            .await
            .unwrap()
            .user_id;

        let mut now = Utc::now();

        let rate_limiter = RateLimiter::new(db.clone());

        // User 1 can access resource A two times before being rate-limited.
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap_err();

        // User 2 can access resource A and user 1 can access resource B.
        rate_limiter
            .check_internal::<RateLimitB>(user_2, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitB>(user_1, now)
            .await
            .unwrap();

        // After 1.5s, user 1 can make another request before being rate-limited again.
        now += Duration::milliseconds(1500);
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap_err();

        // After 500ms, user 1 can make another request before being rate-limited again.
        now += Duration::milliseconds(500);
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap_err();

        rate_limiter.save().await.unwrap();

        // Rate limits are reloaded from the database, so user A is still rate-limited
        // for resource A.
        let rate_limiter = RateLimiter::new(db.clone());
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap_err();

        // After 1s, user 1 can make another request before being rate-limited again.
        now += Duration::seconds(1);
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap();
        rate_limiter
            .check_internal::<RateLimitA>(user_1, now)
            .await
            .unwrap_err();
    }

    struct RateLimitA;

    impl RateLimit for RateLimitA {
        fn capacity() -> usize {
            2
        }

        fn refill_duration() -> Duration {
            Duration::seconds(2)
        }

        fn db_name() -> &'static str {
            "rate-limit-a"
        }
    }

    struct RateLimitB;

    impl RateLimit for RateLimitB {
        fn capacity() -> usize {
            10
        }

        fn refill_duration() -> Duration {
            Duration::seconds(3)
        }

        fn db_name() -> &'static str {
            "rate-limit-b"
        }
    }
}
