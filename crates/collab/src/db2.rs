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
use sea_orm::ActiveValue;
use sea_orm::{
    entity::prelude::*, ConnectOptions, DatabaseConnection, DatabaseTransaction, DbErr,
    TransactionTrait,
};
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
    url: String,
    pool: DatabaseConnection,
    rooms: DashMap<RoomId, Arc<Mutex<()>>>,
    #[cfg(test)]
    background: Option<std::sync::Arc<gpui::executor::Background>>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

impl Database {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        let mut options = ConnectOptions::new(url.into());
        options.min_connections(1).max_connections(max_connections);
        Ok(Self {
            url: url.into(),
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

        let mut connection = sqlx::AnyConnection::connect(&self.url).await?;

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

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<user::Model>> {
        let ids = ids.iter().map(|id| id.0).collect::<Vec<_>>();
        self.transact(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&tx)
                .await?)
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
            if participant.room_id != room_id.0 {
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
            self.commit_room_transaction(room_id, tx, (ProjectId(project.id), room))
                .await
        })
        .await
    }

    async fn get_room(&self, room_id: RoomId, tx: &DatabaseTransaction) -> Result<proto::Room> {
        let db_room = room::Entity::find_by_id(room_id.0)
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
                                id: project_id as u64,
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
                        user_id: db_participant.user_id as u64,
                        peer_id: answering_connection_id as u32,
                        projects: Default::default(),
                        location: Some(proto::ParticipantLocation { variant: location }),
                    },
                );
            } else {
                pending_participants.push(proto::PendingParticipant {
                    user_id: db_participant.user_id as u64,
                    calling_user_id: db_participant.calling_user_id as u64,
                    initial_project_id: db_participant.initial_project_id.map(|id| id as u64),
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
                    .find(|project| project.id as i32 == db_project.id)
                {
                    project
                } else {
                    participant.projects.push(proto::ParticipantProject {
                        id: db_project.id as u64,
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
            id: db_room.id as u64,
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
    };
}

id_type!(UserId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
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
    use sqlx::migrate::MigrateDatabase;
    use std::sync::Arc;

    pub struct TestDb {
        pub db: Option<Arc<Database>>,
    }

    impl TestDb {
        pub fn sqlite(background: Arc<Background>) -> Self {
            let mut rng = StdRng::from_entropy();
            let url = format!("sqlite://file:zed-test-{}?mode=memory", rng.gen::<u128>());
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();

            let mut db = runtime.block_on(async {
                let db = Database::new(&url, 5).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations.sqlite");
                db.migrate(migrations_path.as_ref(), false).await.unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
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
                let db = Database::new(&url, 5).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
                db.migrate(Path::new(migrations_path), false).await.unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
            }
        }

        pub fn db(&self) -> &Arc<Database> {
            self.db.as_ref().unwrap()
        }
    }

    // TODO: Implement drop
    // impl Drop for PostgresTestDb {
    //     fn drop(&mut self) {
    //         let db = self.db.take().unwrap();
    //         db.teardown(&self.url);
    //     }
    // }
}
