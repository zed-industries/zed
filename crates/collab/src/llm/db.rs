mod ids;
mod queries;
mod seed;
mod tables;

#[cfg(test)]
mod tests;

use collections::HashMap;
pub use ids::*;
use rpc::LanguageModelProvider;
pub use seed::*;
pub use tables::*;

#[cfg(test)]
pub use tests::TestLlmDb;
use usage_measure::UsageMeasure;

use std::future::Future;
use std::sync::Arc;

use anyhow::anyhow;
pub use queries::usages::ActiveUserCount;
use sea_orm::prelude::*;
pub use sea_orm::ConnectOptions;
use sea_orm::{
    ActiveValue, DatabaseConnection, DatabaseTransaction, IsolationLevel, TransactionTrait,
};

use crate::db::TransactionHandle;
use crate::executor::Executor;
use crate::Result;

/// The database for the LLM service.
pub struct LlmDatabase {
    options: ConnectOptions,
    pool: DatabaseConnection,
    #[allow(unused)]
    executor: Executor,
    provider_ids: HashMap<LanguageModelProvider, ProviderId>,
    models: HashMap<(LanguageModelProvider, String), model::Model>,
    usage_measure_ids: HashMap<UsageMeasure, UsageMeasureId>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

impl LlmDatabase {
    /// Connects to the database with the given options
    pub async fn new(options: ConnectOptions, executor: Executor) -> Result<Self> {
        sqlx::any::install_default_drivers();
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
            executor,
            provider_ids: HashMap::default(),
            models: HashMap::default(),
            usage_measure_ids: HashMap::default(),
            #[cfg(test)]
            runtime: None,
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        self.initialize_providers().await?;
        self.initialize_models().await?;
        self.initialize_usage_measures().await?;
        Ok(())
    }

    /// Returns the names of the known models for the given [`LanguageModelProvider`].
    pub fn model_names_for_provider(&self, provider: LanguageModelProvider) -> Vec<String> {
        self.models
            .keys()
            .filter_map(|(model_provider, model_name)| {
                if model_provider == &provider {
                    Some(model_name)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn model(&self, provider: LanguageModelProvider, name: &str) -> Result<&model::Model> {
        Ok(self
            .models
            .get(&(provider, name.to_string()))
            .ok_or_else(|| anyhow!("unknown model {provider:?}:{name}"))?)
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }

    pub async fn transaction<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            let (tx, result) = self.with_transaction(&f).await?;
            match result {
                Ok(result) => match tx.commit().await.map_err(Into::into) {
                    Ok(()) => return Ok(result),
                    Err(error) => {
                        return Err(error);
                    }
                },
                Err(error) => {
                    tx.rollback().await?;
                    return Err(error);
                }
            }
        };

        self.run(body).await
    }

    async fn with_transaction<F, Fut, T>(&self, f: &F) -> Result<(DatabaseTransaction, Result<T>)>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let tx = self
            .pool
            .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
            .await?;

        let mut tx = Arc::new(Some(tx));
        let result = f(TransactionHandle(tx.clone())).await;
        let Some(tx) = Arc::get_mut(&mut tx).and_then(|tx| tx.take()) else {
            return Err(anyhow!(
                "couldn't complete transaction because it's still in use"
            ))?;
        };

        Ok((tx, result))
    }

    async fn run<F, T>(&self, future: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        #[cfg(test)]
        {
            if let Executor::Deterministic(executor) = &self.executor {
                executor.simulate_random_delay().await;
            }

            self.runtime.as_ref().unwrap().block_on(future)
        }

        #[cfg(not(test))]
        {
            future.await
        }
    }
}
