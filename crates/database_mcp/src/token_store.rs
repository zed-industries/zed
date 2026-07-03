//! One-shot, TTL-expiring storage for write proposals produced by
//! `propose_write` and redeemed by `apply_write`.
//!
//! A token is minted for a proposed write statement and is valid for exactly
//! one [`TokenStore::take`] call within the configured TTL; after that (or
//! once expired) it is gone. Storage lives in the MCP server process's
//! memory only — nothing here is persisted across restarts.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub connection: String,
    pub database: String,
    pub sql: String,
    pub previewed_rows_affected: u64,
}

pub struct TokenStore {
    ttl: Duration,
    entries: HashMap<String, (Proposal, Instant)>,
}

impl TokenStore {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            entries: HashMap::new(),
        }
    }

    /// Stores a proposal, returns its fresh one-shot token (uuid v4).
    pub fn insert(&mut self, proposal: Proposal, now: Instant) -> String {
        let token = uuid::Uuid::new_v4().to_string();
        self.entries.insert(token.clone(), (proposal, now));
        token
    }

    /// Removes and returns the proposal iff the token exists and is not
    /// expired. Expired or unknown tokens return None. Also prunes expired
    /// entries.
    pub fn take(&mut self, token: &str, now: Instant) -> Option<Proposal> {
        let ttl = self.ttl;
        self.entries
            .retain(|_, (_, created)| now.duration_since(*created) <= ttl);
        self.entries.remove(token).map(|(proposal, _)| proposal)
    }

    pub fn ttl_seconds(&self) -> u64 {
        self.ttl.as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn proposal() -> Proposal {
        Proposal {
            connection: "dev".into(),
            database: "shop".into(),
            sql: "DELETE FROM t WHERE id=1".into(),
            previewed_rows_affected: 1,
        }
    }

    #[test]
    fn token_is_one_shot() {
        let mut store = TokenStore::new(Duration::from_secs(300));
        let now = Instant::now();
        let token = store.insert(proposal(), now);
        assert!(store.take(&token, now).is_some());
        assert!(store.take(&token, now).is_none(), "second take must fail");
    }

    #[test]
    fn expired_token_is_rejected() {
        let mut store = TokenStore::new(Duration::from_secs(300));
        let now = Instant::now();
        let token = store.insert(proposal(), now);
        let later = now + Duration::from_secs(301);
        assert!(store.take(&token, later).is_none());
    }

    #[test]
    fn unknown_token_is_none() {
        let mut store = TokenStore::new(Duration::from_secs(300));
        assert!(store.take("nope", Instant::now()).is_none());
    }

    #[test]
    fn distinct_tokens_are_independent() {
        let mut store = TokenStore::new(Duration::from_secs(300));
        let now = Instant::now();
        let a = store.insert(proposal(), now);
        let b = store.insert(proposal(), now);
        assert_ne!(a, b);
        assert!(store.take(&a, now).is_some());
        assert!(store.take(&b, now).is_some());
    }

    #[test]
    fn take_preserves_previewed_rows_affected() {
        let mut store = TokenStore::new(Duration::from_secs(300));
        let now = Instant::now();
        let mut proposed = proposal();
        proposed.previewed_rows_affected = 42;
        let token = store.insert(proposed, now);
        let taken = store.take(&token, now).expect("token should be present");
        assert_eq!(taken.previewed_rows_affected, 42);
    }
}
