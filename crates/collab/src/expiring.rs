use std::{future::Future, time::Instant};

use async_std::sync::Mutex;

#[derive(Default)]
pub struct Expiring<T>(Mutex<Option<ExpiringState<T>>>);

pub struct ExpiringState<T> {
    value: T,
    expires_at: Instant,
}

impl<T: Clone> Expiring<T> {
    pub async fn get_or_refresh<F, G>(&self, f: F) -> tide::Result<T>
    where
        F: FnOnce() -> G,
        G: Future<Output = tide::Result<(T, Instant)>>,
    {
        let mut state = self.0.lock().await;

        if let Some(state) = state.as_mut() {
            if Instant::now() >= state.expires_at {
                let (value, expires_at) = f().await?;
                state.value = value.clone();
                state.expires_at = expires_at;
                Ok(value)
            } else {
                Ok(state.value.clone())
            }
        } else {
            let (value, expires_at) = f().await?;
            *state = Some(ExpiringState {
                value: value.clone(),
                expires_at,
            });
            Ok(value)
        }
    }

    pub async fn clear(&self) {
        self.0.lock().await.take();
    }
}
