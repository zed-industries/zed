use std::{future::Future, time::Duration};

#[cfg(test)]
use gpui::BackgroundExecutor;

#[derive(Clone)]
pub enum Executor {
    Production,
    #[cfg(test)]
    Deterministic(BackgroundExecutor),
}

impl Executor {
    pub fn spawn_detached<F>(&self, future: F)
    where
        F: 'static + Send + Future<Output = ()>,
    {
        match self {
            Executor::Production => {
                tokio::spawn(future);
            }
            #[cfg(test)]
            Executor::Deterministic(background) => {
                background.spawn(future).detach();
            }
        }
    }

    pub fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + use<> {
        let this = self.clone();
        async move {
            match this {
                Executor::Production => tokio::time::sleep(duration).await,
                #[cfg(test)]
                Executor::Deterministic(background) => background.timer(duration).await,
            }
        }
    }
}
