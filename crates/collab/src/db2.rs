mod project;
mod project_collaborator;
mod room;
mod room_participant;
#[cfg(test)]
mod tests;
mod user;
mod worktree;

use crate::{Error, Result};
use anyhow::anyhow;
use collections::HashMap;
use dashmap::DashMap;
use futures::StreamExt;
use rpc::{proto, ConnectionId};
use sea_orm::{
    entity::prelude::*, ConnectOptions, DatabaseConnection, DatabaseTransaction, DbErr,
    TransactionTrait,
};
use sea_orm::{ActiveValue, IntoActiveModel};
use sea_query::OnConflict;
use serde::{Deserialize, Serialize};
use sqlx::migrate::{Migrate, Migration, MigrationSource};
use sqlx::Connection;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::time::Duration;
use std::{future::Future, marker::PhantomData, rc::Rc, sync::Arc};
use tokio::sync::{Mutex, OwnedMutexGuard};

pub use user::Model as User;

pub struct Database {
    options: ConnectOptions,
    pool: DatabaseConnection,
    rooms: DashMap<RoomId, Arc<Mutex<()>>>,
    #[cfg(test)]
    background: Option<std::sync::Arc<gpui::executor::Background>>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

impl Database {
    pub async fn new(options: ConnectOptions) -> Result<Self> {
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
            rooms: DashMap::with_capacity(16384),
            #[cfg(test)]
            background: None,
            #[cfg(test)]
            runtime: None,
        })
    }

    pub async fn migrate(
        &self,
        migrations_path: &Path,
        ignore_checksum_mismatch: bool,
    ) -> anyhow::Result<Vec<(Migration, Duration)>> {
        let migrations = MigrationSource::resolve(migrations_path)
            .await
            .map_err(|err| anyhow!("failed to load migrations: {err:?}"))?;

        let mut connection = sqlx::AnyConnection::connect(self.options.get_url()).await?;

        connection.ensure_migrations_table().await?;
        let applied_migrations: HashMap<_, _> = connection
            .list_applied_migrations()
            .await?
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        let mut new_migrations = Vec::new();
        for migration in migrations {
            match applied_migrations.get(&migration.version) {
                Some(applied_migration) => {
                    if migration.checksum != applied_migration.checksum && !ignore_checksum_mismatch
                    {
                        Err(anyhow!(
                            "checksum mismatch for applied migration {}",
                            migration.description
                        ))?;
                    }
                }
                None => {
                    let elapsed = connection.apply(&migration).await?;
                    new_migrations.push((migration, elapsed));
                }
            }
        }

        Ok(new_migrations)
    }

    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        self.transact(|tx| async {
            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(email_address.into())),
                github_login: ActiveValue::set(params.github_login.clone()),
                github_user_id: ActiveValue::set(Some(params.github_user_id)),
                admin: ActiveValue::set(admin),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubLogin)
                    .update_column(user::Column::GithubLogin)
                    .to_owned(),
            )
            .exec_with_returning(&tx)
            .await?;

            tx.commit().await?;

            Ok(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                signup_device_id: None,
                inviting_user_id: None,
            })
        })
        .await
    }

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<user::Model>> {
        self.transact(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&tx)
                .await?)
        })
        .await
    }

    pub async fn get_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
    ) -> Result<Option<User>> {
        self.transact(|tx| async {
            let tx = tx;
            if let Some(github_user_id) = github_user_id {
                if let Some(user_by_github_user_id) = user::Entity::find()
                    .filter(user::Column::GithubUserId.eq(github_user_id))
                    .one(&tx)
                    .await?
                {
                    let mut user_by_github_user_id = user_by_github_user_id.into_active_model();
                    user_by_github_user_id.github_login = ActiveValue::set(github_login.into());
                    Ok(Some(user_by_github_user_id.update(&tx).await?))
                } else if let Some(user_by_github_login) = user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(&tx)
                    .await?
                {
                    let mut user_by_github_login = user_by_github_login.into_active_model();
                    user_by_github_login.github_user_id = ActiveValue::set(Some(github_user_id));
                    Ok(Some(user_by_github_login.update(&tx).await?))
                } else {
                    Ok(None)
                }
            } else {
                Ok(user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(&tx)
                    .await?)
            }
        })
        .await
    }

    pub async fn share_project(
        &self,
        room_id: RoomId,
        connection_id: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(ProjectId, proto::Room)>> {
        self.transact(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.0))
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("could not find participant"))?;
            if participant.room_id != room_id {
                return Err(anyhow!("shared project on unexpected room"))?;
            }

            let project = project::ActiveModel {
                room_id: ActiveValue::set(participant.room_id),
                host_user_id: ActiveValue::set(participant.user_id),
                host_connection_id: ActiveValue::set(connection_id.0 as i32),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            worktree::Entity::insert_many(worktrees.iter().map(|worktree| worktree::ActiveModel {
                id: ActiveValue::set(worktree.id as i32),
                project_id: ActiveValue::set(project.id),
                abs_path: ActiveValue::set(worktree.abs_path.clone()),
                root_name: ActiveValue::set(worktree.root_name.clone()),
                visible: ActiveValue::set(worktree.visible),
                scan_id: ActiveValue::set(0),
                is_complete: ActiveValue::set(false),
            }))
            .exec(&tx)
            .await?;

            project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project.id),
                connection_id: ActiveValue::set(connection_id.0 as i32),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(0),
                is_host: ActiveValue::set(true),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, (project.id, room))
                .await
        })
        .await
    }

    async fn get_room(&self, room_id: RoomId, tx: &DatabaseTransaction) -> Result<proto::Room> {
        let db_room = room::Entity::find_by_id(room_id)
            .one(tx)
            .await?
            .ok_or_else(|| anyhow!("could not find room"))?;

        let mut db_participants = db_room
            .find_related(room_participant::Entity)
            .stream(tx)
            .await?;
        let mut participants = HashMap::default();
        let mut pending_participants = Vec::new();
        while let Some(db_participant) = db_participants.next().await {
            let db_participant = db_participant?;
            if let Some(answering_connection_id) = db_participant.answering_connection_id {
                let location = match (
                    db_participant.location_kind,
                    db_participant.location_project_id,
                ) {
                    (Some(0), Some(project_id)) => {
                        Some(proto::participant_location::Variant::SharedProject(
                            proto::participant_location::SharedProject {
                                id: project_id.to_proto(),
                            },
                        ))
                    }
                    (Some(1), _) => Some(proto::participant_location::Variant::UnsharedProject(
                        Default::default(),
                    )),
                    _ => Some(proto::participant_location::Variant::External(
                        Default::default(),
                    )),
                };
                participants.insert(
                    answering_connection_id,
                    proto::Participant {
                        user_id: db_participant.user_id.to_proto(),
                        peer_id: answering_connection_id as u32,
                        projects: Default::default(),
                        location: Some(proto::ParticipantLocation { variant: location }),
                    },
                );
            } else {
                pending_participants.push(proto::PendingParticipant {
                    user_id: db_participant.user_id.to_proto(),
                    calling_user_id: db_participant.calling_user_id.to_proto(),
                    initial_project_id: db_participant.initial_project_id.map(|id| id.to_proto()),
                });
            }
        }

        let mut db_projects = db_room
            .find_related(project::Entity)
            .find_with_related(worktree::Entity)
            .stream(tx)
            .await?;

        while let Some(row) = db_projects.next().await {
            let (db_project, db_worktree) = row?;
            if let Some(participant) = participants.get_mut(&db_project.host_connection_id) {
                let project = if let Some(project) = participant
                    .projects
                    .iter_mut()
                    .find(|project| project.id == db_project.id.to_proto())
                {
                    project
                } else {
                    participant.projects.push(proto::ParticipantProject {
                        id: db_project.id.to_proto(),
                        worktree_root_names: Default::default(),
                    });
                    participant.projects.last_mut().unwrap()
                };

                if let Some(db_worktree) = db_worktree {
                    project.worktree_root_names.push(db_worktree.root_name);
                }
            }
        }

        Ok(proto::Room {
            id: db_room.id.to_proto(),
            live_kit_room: db_room.live_kit_room,
            participants: participants.into_values().collect(),
            pending_participants,
        })
    }

    async fn commit_room_transaction<T>(
        &self,
        room_id: RoomId,
        tx: DatabaseTransaction,
        data: T,
    ) -> Result<RoomGuard<T>> {
        let lock = self.rooms.entry(room_id).or_default().clone();
        let _guard = lock.lock_owned().await;
        tx.commit().await?;
        Ok(RoomGuard {
            data,
            _guard,
            _not_send: PhantomData,
        })
    }

    async fn transact<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: Send + Fn(DatabaseTransaction) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            loop {
                let tx = self.pool.begin().await?;
                match f(tx).await {
                    Ok(result) => return Ok(result),
                    Err(error) => match error {
                        Error::Database2(
                            DbErr::Exec(sea_orm::RuntimeErr::SqlxError(error))
                            | DbErr::Query(sea_orm::RuntimeErr::SqlxError(error)),
                        ) if error
                            .as_database_error()
                            .and_then(|error| error.code())
                            .as_deref()
                            == Some("40001") =>
                        {
                            // Retry (don't break the loop)
                        }
                        error @ _ => return Err(error),
                    },
                }
            }
        };

        #[cfg(test)]
        {
            if let Some(background) = self.background.as_ref() {
                background.simulate_random_delay().await;
            }

            self.runtime.as_ref().unwrap().block_on(body)
        }

        #[cfg(not(test))]
        {
            body.await
        }
    }
}

pub struct RoomGuard<T> {
    data: T,
    _guard: OwnedMutexGuard<()>,
    _not_send: PhantomData<Rc<()>>,
}

impl<T> Deref for RoomGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for RoomGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserParams {
    pub github_login: String,
    pub github_user_id: i32,
    pub invite_count: i32,
}

#[derive(Debug)]
pub struct NewUserResult {
    pub user_id: UserId,
    pub metrics_id: String,
    pub inviting_user_id: Option<UserId>,
    pub signup_device_id: Option<String>,
}

fn random_invite_code() -> String {
    nanoid::nanoid!(16)
}

fn random_email_confirmation_code() -> String {
    nanoid::nanoid!(64)
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            sqlx::Type,
            Serialize,
            Deserialize,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
            pub fn to_proto(self) -> u64 {
                self.0 as u64
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$name> for sea_query::Value {
            fn from(value: $name) -> Self {
                sea_query::Value::Int(Some(value.0))
            }
        }

        impl sea_orm::TryGetable for $name {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                Ok(Self(i32::try_get(res, pre, col)?))
            }
        }

        impl sea_query::ValueType for $name {
            fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
                match v {
                    Value::TinyInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::SmallInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::Int(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::BigInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::TinyUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::SmallUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::Unsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::BigUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    _ => Err(sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name).into()
            }

            fn array_type() -> sea_query::ArrayType {
                sea_query::ArrayType::Int
            }

            fn column_type() -> sea_query::ColumnType {
                sea_query::ColumnType::Integer(None)
            }
        }

        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(Self(n.try_into().map_err(|_| {
                    DbErr::ConvertFromU64(concat!(
                        "error converting ",
                        stringify!($name),
                        " to u64"
                    ))
                })?))
            }
        }

        impl sea_query::Nullable for $name {
            fn null() -> Value {
                Value::Int(None)
            }
        }
    };
}

id_type!(UserId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
id_type!(ProjectCollaboratorId);
id_type!(WorktreeId);

#[cfg(test)]
pub use test::*;

#[cfg(test)]
mod test {
    use super::*;
    use gpui::executor::Background;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rand::prelude::*;
    use sea_orm::ConnectionTrait;
    use sqlx::migrate::MigrateDatabase;
    use std::sync::Arc;

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
                let db = Database::new(options).await.unwrap();
                let sql = include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/migrations.sqlite/20221109000000_test_schema.sql"
                ));
                db.pool
                    .execute(sea_orm::Statement::from_string(
                        db.pool.get_database_backend(),
                        sql.into(),
                    ))
                    .await
                    .unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
                connection: None,
            }
        }

        pub fn postgres(background: Arc<Background>) -> Self {
            lazy_static! {
                static ref LOCK: Mutex<()> = Mutex::new(());
            }

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
                let db = Database::new(options).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
                db.migrate(Path::new(migrations_path), false).await.unwrap();
                db
            });

            db.background = Some(background);
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
                            query.into(),
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
}
