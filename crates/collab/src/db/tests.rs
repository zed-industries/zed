mod buffer_tests;
mod channel_tests;
mod db_tests;
mod feature_flag_tests;
mod message_tests;

use super::*;
use gpui::executor::Background;
use parking_lot::Mutex;
use rpc::proto::ChannelEdge;
use sea_orm::ConnectionTrait;
use sqlx::migrate::MigrateDatabase;
use std::sync::Arc;

const TEST_RELEASE_CHANNEL: &'static str = "test";

pub struct TestDb {
    pub db: Option<Arc<Database>>,
    pub connection: Option<sqlx::AnyConnection>,
}

impl TestDb {
    pub fn sqlite(background: Arc<Background>) -> Self {
        let url = format!("sqlite::memory:");
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let mut db = runtime.block_on(async {
            let mut options = ConnectOptions::new(url);
            options.max_connections(5);
            let db = Database::new(options, Executor::Deterministic(background))
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
            db
        });

        db.runtime = Some(runtime);

        Self {
            db: Some(Arc::new(db)),
            connection: None,
        }
    }

    pub fn postgres(background: Arc<Background>) -> Self {
        static LOCK: Mutex<()> = Mutex::new(());

        let _guard = LOCK.lock();
        let mut rng = StdRng::from_entropy();
        let url = format!(
            "postgres://postgres@localhost/zed-test-{}",
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
            let db = Database::new(options, Executor::Deterministic(background))
                .await
                .unwrap();
            let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
            db.migrate(Path::new(migrations_path), false).await.unwrap();
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
        #[gpui::test]
        async fn $postgres_test_name() {
            let test_db = crate::db::TestDb::postgres(
                gpui::executor::Deterministic::new(0).build_background(),
            );
            $test_name(test_db.db()).await;
        }

        #[gpui::test]
        async fn $sqlite_test_name() {
            let test_db =
                crate::db::TestDb::sqlite(gpui::executor::Deterministic::new(0).build_background());
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

/// The second tuples are (channel_id, parent)
fn graph(channels: &[(ChannelId, &'static str)], edges: &[(ChannelId, ChannelId)]) -> ChannelGraph {
    let mut graph = ChannelGraph {
        channels: vec![],
        edges: vec![],
    };

    for (id, name) in channels {
        graph.channels.push(Channel {
            id: *id,
            name: name.to_string(),
        })
    }

    for (channel, parent) in edges {
        graph.edges.push(ChannelEdge {
            channel_id: channel.to_proto(),
            parent_id: parent.to_proto(),
        })
    }

    graph
}
