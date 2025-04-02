mod billing_subscription_tests;
mod buffer_tests;
mod channel_tests;
mod contributor_tests;
mod db_tests;
// we only run postgres tests on macos right now
#[cfg(target_os = "macos")]
mod embedding_tests;
mod extension_tests;
mod feature_flag_tests;
mod message_tests;
mod processed_stripe_event_tests;
mod user_tests;

use crate::migrations::run_database_migrations;

use super::*;
use gpui::BackgroundExecutor;
use parking_lot::Mutex;
use sea_orm::ConnectionTrait;
use sqlx::migrate::MigrateDatabase;
use std::sync::{
    Arc,
    atomic::{AtomicI32, AtomicU32, Ordering::SeqCst},
};

pub struct TestDb {
    pub db: Option<Arc<Database>>,
    pub connection: Option<sqlx::AnyConnection>,
}

impl TestDb {
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
            let mut db = Database::new(options, Executor::Deterministic(background))
                .await
                .unwrap();
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
            "postgres://postgres@localhost/zed-test-{}",
            rng.r#gen::<u128>()
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
            let mut db = Database::new(options, Executor::Deterministic(background))
                .await
                .unwrap();
            let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
            run_database_migrations(db.options(), migrations_path)
                .await
                .unwrap();
            db.initialize_notification_kinds().await.unwrap();
            db
        });

        db.runtime = Some(runtime);

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn db(&self) -> &Arc<Database> {
        self.db.as_ref().unwrap()
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

fn channel_tree(channels: &[(ChannelId, &[ChannelId], &'static str)]) -> Vec<Channel> {
    channels
        .iter()
        .map(|(id, parent_path, name)| Channel {
            id: *id,
            name: name.to_string(),
            visibility: ChannelVisibility::Members,
            parent_path: parent_path.to_vec(),
        })
        .collect()
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

static TEST_CONNECTION_ID: AtomicU32 = AtomicU32::new(1);
fn new_test_connection(server: ServerId) -> ConnectionId {
    ConnectionId {
        id: TEST_CONNECTION_ID.fetch_add(1, SeqCst),
        owner_id: server.0 as u32,
    }
}
