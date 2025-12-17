mod buffer_tests;
mod channel_tests;
mod contributor_tests;
mod db_tests;
mod extension_tests;
mod migrations;

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering::SeqCst};
use std::time::Duration;

use gpui::BackgroundExecutor;
use parking_lot::Mutex;
use rand::prelude::*;
use sea_orm::ConnectionTrait;
use sqlx::migrate::MigrateDatabase;

use self::migrations::run_database_migrations;

use super::*;

pub struct TestDb {
    pub db: Option<Arc<Database>>,
    pub connection: Option<sqlx::AnyConnection>,
}

impl TestDb {
    pub fn sqlite(executor: BackgroundExecutor) -> Self {
        let url = "sqlite::memory:";
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let mut db = runtime.block_on(async {
            let mut options = ConnectOptions::new(url);
            options.max_connections(5);
            let mut db = Database::new(options).await.unwrap();
            let sql = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/migrations.sqlite/20221109000000_test_schema.sql"
            ));
            db.pool
                .execute(sea_orm::Statement::from_string(
                    db.pool.get_database_backend(),
                    sql,
                ))
                .await
                .unwrap();
            db.initialize_notification_kinds().await.unwrap();
            db
        });

        db.test_options = Some(DatabaseTestOptions {
            executor,
            runtime,
            query_failure_probability: parking_lot::Mutex::new(0.0),
        });

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn postgres(executor: BackgroundExecutor) -> Self {
        static LOCK: Mutex<()> = Mutex::new(());

        let _guard = LOCK.lock();
        let mut rng = StdRng::from_os_rng();
        let url = format!(
            "postgres://postgres@localhost/zed-test-{}",
            rng.random::<u128>()
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
            let mut db = Database::new(options).await.unwrap();
            let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
            run_database_migrations(db.options(), migrations_path)
                .await
                .unwrap();
            db.initialize_notification_kinds().await.unwrap();
            db
        });

        db.test_options = Some(DatabaseTestOptions {
            executor,
            runtime,
            query_failure_probability: parking_lot::Mutex::new(0.0),
        });

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn db(&self) -> &Arc<Database> {
        self.db.as_ref().unwrap()
    }

    pub fn set_query_failure_probability(&self, probability: f64) {
        let database = self.db.as_ref().unwrap();
        let test_options = database.test_options.as_ref().unwrap();
        *test_options.query_failure_probability.lock() = probability;
    }
}

#[macro_export]
macro_rules! test_both_dbs {
    ($test_name:ident, $postgres_test_name:ident, $sqlite_test_name:ident) => {
        #[cfg(target_os = "macos")]
        #[gpui::test]
        async fn $postgres_test_name(cx: &mut gpui::TestAppContext) {
            let test_db = $crate::db::TestDb::postgres(cx.executor().clone());
            $test_name(test_db.db()).await;
        }

        #[gpui::test]
        async fn $sqlite_test_name(cx: &mut gpui::TestAppContext) {
            let test_db = $crate::db::TestDb::sqlite(cx.executor().clone());
            $test_name(test_db.db()).await;
        }
    };
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let db = self.db.take().unwrap();
        if let sea_orm::DatabaseBackend::Postgres = db.pool.get_database_backend() {
            db.test_options.as_ref().unwrap().runtime.block_on(async {
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

#[track_caller]
fn assert_channel_tree_matches(actual: Vec<Channel>, expected: Vec<Channel>) {
    let expected_channels = expected.into_iter().collect::<HashSet<_>>();
    let actual_channels = actual.into_iter().collect::<HashSet<_>>();
    pretty_assertions::assert_eq!(expected_channels, actual_channels);
}

fn channel_tree(channels: &[(ChannelId, &[ChannelId], &'static str)]) -> Vec<Channel> {
    use std::collections::HashMap;

    let mut result = Vec::new();
    let mut order_by_parent: HashMap<Vec<ChannelId>, i32> = HashMap::new();

    for (id, parent_path, name) in channels {
        let parent_key = parent_path.to_vec();
        let order = if parent_key.is_empty() {
            1
        } else {
            *order_by_parent
                .entry(parent_key.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1)
        };

        result.push(Channel {
            id: *id,
            name: (*name).to_owned(),
            visibility: ChannelVisibility::Members,
            parent_path: parent_key,
            channel_order: order,
        });
    }

    result
}

static GITHUB_USER_ID: AtomicI32 = AtomicI32::new(5);

async fn new_test_user(db: &Arc<Database>, email: &str) -> UserId {
    db.create_user(
        email,
        None,
        false,
        NewUserParams {
            github_login: email[0..email.find('@').unwrap()].to_string(),
            github_user_id: GITHUB_USER_ID.fetch_add(1, SeqCst),
        },
    )
    .await
    .unwrap()
    .user_id
}
