use crate::db::UserId;
use dashmap::DashMap;
use parking_lot::Mutex;
use std::any::TypeId;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

trait RateLimit: 'static {
    fn capacity() -> usize;
    fn refill_duration() -> Duration;
    fn id() -> &'static str;
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }
}

struct RateLimiter {
    buckets: DashMap<(UserId, TypeId), Arc<Mutex<Bucket>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            buckets: DashMap::new(),
        }
    }

    pub fn allow<K: RateLimit>(&self, user_id: UserId) -> bool {
        let type_id = K::type_id();
        let user_key = (user_id, type_id);

        let bucket = self
            .buckets
            .entry(user_key)
            .or_insert_with(|| {
                Arc::new(Mutex::new(Bucket::new(K::capacity(), K::refill_duration())))
            })
            .value()
            .clone();

        let allowed = bucket.lock().allow();
        allowed
    }
}

struct Bucket {
    capacity: usize,
    tokens: AtomicUsize,
    refill_time_per_token: Duration,
    last_refill: Instant,
}

impl Bucket {
    fn new(capacity: usize, refill_duration: Duration) -> Self {
        Bucket {
            capacity,
            tokens: AtomicUsize::new(capacity),
            refill_time_per_token: refill_duration / capacity as u32,
            last_refill: Instant::now(),
        }
    }

    fn allow(&mut self) -> bool {
        self.refill();
        let current_tokens = self.tokens.load(Ordering::SeqCst);
        if current_tokens > 0 {
            self.tokens.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        if elapsed >= self.refill_time_per_token {
            let new_tokens =
                (elapsed.as_millis() / self.refill_time_per_token.as_millis()) as usize;

            let current_tokens = self.tokens.fetch_add(new_tokens, Ordering::SeqCst);
            if current_tokens > self.capacity {
                self.tokens.store(self.capacity, Ordering::SeqCst);
            }
            self.last_refill += self.refill_time_per_token * new_tokens as u32;
        }
    }
}

// Example implementation of RateLimit for a specific action type
struct DownloadActionKey;

impl RateLimit for DownloadActionKey {
    fn capacity() -> usize {
        5
    }

    fn refill_duration() -> Duration {
        Duration::from_secs(60)
    }

    fn id() -> &'static str {
        "download-action"
    }
}
