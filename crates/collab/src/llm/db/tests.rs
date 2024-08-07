mod provider_tests;
mod usage_tests;

use gpui::BackgroundExecutor;
use parking_lot::Mutex;
use rand::prelude::*;
use sea_orm::ConnectionTrait;
use sqlx::migrate::MigrateDatabase;
use std::sync::Arc;
use std::time::Duration;

use crate::migrations::run_database_migrations;

use super::*;

pub struct TestLlmDb {
    pub db: Option<Arc<LlmDatabase>>,
    pub connection: Option<sqlx::AnyConnection>,
}

impl TestLlmDb {
    pub fn sqlite(background: BackgroundExecutor) -> Self {
        let url = "sqlite::memory:";
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let mut db = runtime.block_on(async {
            let mut options = ConnectOptions::new(url);
            options.max_connections(5);
            let db = LlmDatabase::new(options, Executor::Deterministic(background))
                .await
                .unwrap();
            let sql = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/migrations_llm.sqlite/20240806182921_test_schema.sql"
            ));
            db.pool
                .execute(sea_orm::Statement::from_string(
                    db.pool.get_database_backend(),
                    sql,
                ))
                .await
                .unwrap();
            db
        });

        db.runtime = Some(runtime);

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn postgres(background: BackgroundExecutor) -> Self {
        static LOCK: Mutex<()> = Mutex::new(());

        let _guard = LOCK.lock();
        let mut rng = StdRng::from_entropy();
        let url = format!(
            "postgres://postgres@localhost/zed-llm-test-{}",
            rng.gen::<u128>()
        );
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let mut db = runtime.block_on(async {
            sqlx::Postgres::create_database(&url)
                .await
                .expect("failed to create test db");
            let mut options = ConnectOptions::new(url);
            options
                .max_connections(5)
                .idle_timeout(Duration::from_secs(0));
            let db = LlmDatabase::new(options, Executor::Deterministic(background))
                .await
                .unwrap();
            let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations_llm");
            run_database_migrations(db.options(), migrations_path)
                .await
                .unwrap();
            db
        });

        db.runtime = Some(runtime);

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn db(&self) -> &Arc<LlmDatabase> {
        self.db.as_ref().unwrap()
    }
}

#[macro_export]
macro_rules! test_both_llm_dbs {
    ($test_name:ident, $postgres_test_name:ident, $sqlite_test_name:ident) => {
        #[cfg(target_os = "macos")]
        #[gpui::test]
        async fn $postgres_test_name(cx: &mut gpui::TestAppContext) {
            let test_db = $crate::llm::db::TestLlmDb::postgres(cx.executor().clone());
            $test_name(test_db.db()).await;
        }

        #[gpui::test]
        async fn $sqlite_test_name(cx: &mut gpui::TestAppContext) {
            let test_db = $crate::llm::db::TestLlmDb::sqlite(cx.executor().clone());
            $test_name(test_db.db()).await;
        }
    };
}

impl Drop for TestLlmDb {
    fn drop(&mut self) {
        let db = self.db.take().unwrap();
        if let sea_orm::DatabaseBackend::Postgres = db.pool.get_database_backend() {
            db.runtime.as_ref().unwrap().block_on(async {
                use util::ResultExt;
                let query = "
                        SELECT pg_terminate_backend(pg_stat_activity.pid)
                        FROM pg_stat_activity
                        WHERE
                            pg_stat_activity.datname = current_database() AND
                            pid <> pg_backend_pid();
                    ";
                db.pool
                    .execute(sea_orm::Statement::from_string(
                        db.pool.get_database_backend(),
                        query,
                    ))
                    .await
                    .log_err();
                sqlx::Postgres::drop_database(db.options.get_url())
                    .await
                    .log_err();
            })
        }
    }
}
