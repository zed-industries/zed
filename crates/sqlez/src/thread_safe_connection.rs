use anyhow::Context as _;
use collections::HashMap;
use futures::{Future, FutureExt, channel::oneshot};
use parking_lot::{Mutex, RwLock};
use std::{
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, LazyLock},
    thread,
    time::Duration,
};
use thread_local::ThreadLocal;

use crate::{connection::Connection, domain::Migrator, util::UnboundedSyncSender};

const MIGRATION_RETRIES: usize = 10;
const CONNECTION_INITIALIZE_RETRIES: usize = 50;
const CONNECTION_INITIALIZE_RETRY_DELAY: Duration = Duration::from_millis(1);

type QueuedWrite = Box<dyn 'static + Send + FnOnce()>;
type WriteQueue = Box<dyn 'static + Send + Sync + Fn(QueuedWrite)>;
type WriteQueueConstructor = Box<dyn 'static + Send + FnMut() -> WriteQueue>;

/// List of queues of tasks by database uri. This lets us serialize writes to the database
/// and have a single worker thread per db file. This means many thread safe connections
/// (possibly with different migrations) could all be communicating with the same background
/// thread.
static QUEUES: LazyLock<RwLock<HashMap<Arc<str>, WriteQueue>>> = LazyLock::new(Default::default);

/// Thread safe connection to a given database file or in memory db. This can be cloned, shared, static,
/// whatever. It derefs to a synchronous connection by thread that is read only. A write capable connection
/// may be accessed by passing a callback to the `write` function which will queue the callback
#[derive(Clone)]
pub struct ThreadSafeConnection {
    uri: Arc<str>,
    persistent: bool,
    connection_initialize_query: Option<&'static str>,
    connections: Arc<ThreadLocal<Connection>>,
}

unsafe impl Send for ThreadSafeConnection {}
unsafe impl Sync for ThreadSafeConnection {}

pub struct ThreadSafeConnectionBuilder<M: Migrator + 'static = ()> {
    db_initialize_query: Option<&'static str>,
    write_queue_constructor: Option<WriteQueueConstructor>,
    connection: ThreadSafeConnection,
    _migrator: PhantomData<*mut M>,
}

impl<M: Migrator> ThreadSafeConnectionBuilder<M> {
    /// Sets the query to run every time a connection is opened. This must
    /// be infallible (EG only use pragma statements) and not cause writes.
    /// to the db or it will panic.
    pub fn with_connection_initialize_query(mut self, initialize_query: &'static str) -> Self {
        self.connection.connection_initialize_query = Some(initialize_query);
        self
    }

    /// Queues an initialization query for the database file. This must be infallible
    /// but may cause changes to the database file such as with `PRAGMA journal_mode`
    pub fn with_db_initialization_query(mut self, initialize_query: &'static str) -> Self {
        self.db_initialize_query = Some(initialize_query);
        self
    }

    /// Specifies how the thread safe connection should serialize writes. If provided
    /// the connection will call the write_queue_constructor for each database file in
    /// this process. The constructor is responsible for setting up a background thread or
    /// async task which handles queued writes with the provided connection.
    pub fn with_write_queue_constructor(
        mut self,
        write_queue_constructor: WriteQueueConstructor,
    ) -> Self {
        self.write_queue_constructor = Some(write_queue_constructor);
        self
    }

    pub async fn build(self) -> anyhow::Result<ThreadSafeConnection> {
        self.connection
            .initialize_queues(self.write_queue_constructor);

        let db_initialize_query = self.db_initialize_query;

        self.connection
            .write(move |connection| {
                if let Some(db_initialize_query) = db_initialize_query {
                    connection.exec(db_initialize_query).with_context(|| {
                        format!(
                            "Db initialize query failed to execute: {}",
                            db_initialize_query
                        )
                    })?()?;
                }

                // Retry failed migrations in case they were run in parallel from different
                // processes. This gives a best attempt at migrating before bailing
                let mut migration_result =
                    anyhow::Result::<()>::Err(anyhow::anyhow!("Migration never run"));

                let foreign_keys_enabled: bool =
                    connection.select_row::<i32>("PRAGMA foreign_keys")?()
                        .unwrap_or(None)
                        .map(|enabled| enabled != 0)
                        .unwrap_or(false);

                connection.exec("PRAGMA foreign_keys = OFF;")?()?;

                for _ in 0..MIGRATION_RETRIES {
                    migration_result = connection
                        .with_savepoint("thread_safe_multi_migration", || M::migrate(connection));

                    if migration_result.is_ok() {
                        break;
                    }
                }

                if foreign_keys_enabled {
                    connection.exec("PRAGMA foreign_keys = ON;")?()?;
                }
                migration_result
            })
            .await?;

        Ok(self.connection)
    }
}

impl ThreadSafeConnection {
    fn initialize_queues(&self, write_queue_constructor: Option<WriteQueueConstructor>) -> bool {
        if !QUEUES.read().contains_key(&self.uri) {
            let mut queues = QUEUES.write();
            if !queues.contains_key(&self.uri) {
                let mut write_queue_constructor =
                    write_queue_constructor.unwrap_or_else(background_thread_queue);
                queues.insert(self.uri.clone(), write_queue_constructor());
                return true;
            }
        }
        false
    }

    pub fn builder<M: Migrator>(uri: &str, persistent: bool) -> ThreadSafeConnectionBuilder<M> {
        ThreadSafeConnectionBuilder::<M> {
            db_initialize_query: None,
            write_queue_constructor: None,
            connection: Self {
                uri: Arc::from(uri),
                persistent,
                connection_initialize_query: None,
                connections: Default::default(),
            },
            _migrator: PhantomData,
        }
    }

    /// Opens a new db connection with the initialized file path. This is internal and only
    /// called from the deref function.
    fn open_file(uri: &str) -> Connection {
        Connection::open_file(uri)
    }

    /// Opens a shared memory connection using the file path as the identifier. This is internal
    /// and only called from the deref function.
    fn open_shared_memory(uri: &str) -> Connection {
        Connection::open_memory(Some(uri))
    }

    pub fn write<T: 'static + Send + Sync>(
        &self,
        callback: impl 'static + Send + FnOnce(&Connection) -> T,
    ) -> impl Future<Output = T> {
        // Check and invalidate queue and maybe recreate queue
        let queues = QUEUES.read();
        let write_channel = queues
            .get(&self.uri)
            .expect("Queues are inserted when build is called. This should always succeed");

        // Create a one shot channel for the result of the queued write
        // so we can await on the result
        let (sender, receiver) = oneshot::channel();

        let thread_safe_connection = (*self).clone();
        write_channel(Box::new(move || {
            let connection = thread_safe_connection.deref();
            let result = connection.with_write(|connection| callback(connection));
            sender.send(result).ok();
        }));
        receiver.map(|response| response.expect("Write queue unexpectedly closed"))
    }

    pub(crate) fn create_connection(
        persistent: bool,
        uri: &str,
        connection_initialize_query: Option<&'static str>,
    ) -> Connection {
        let mut connection = if persistent {
            Self::open_file(uri)
        } else {
            Self::open_shared_memory(uri)
        };

        if let Some(initialize_query) = connection_initialize_query {
            let mut last_error = None;
            let initialized = (0..CONNECTION_INITIALIZE_RETRIES).any(|attempt| {
                match connection
                    .exec(initialize_query)
                    .and_then(|mut statement| statement())
                {
                    Ok(()) => true,
                    Err(err)
                        if is_schema_lock_error(&err)
                            && attempt + 1 < CONNECTION_INITIALIZE_RETRIES =>
                    {
                        last_error = Some(err);
                        thread::sleep(CONNECTION_INITIALIZE_RETRY_DELAY);
                        false
                    }
                    Err(err) => {
                        panic!(
                            "Initialize query failed to execute: {}\n\nCaused by:\n{err:#}",
                            initialize_query
                        )
                    }
                }
            });

            if !initialized {
                let err = last_error
                    .expect("connection initialization retries should record the last error");
                panic!(
                    "Initialize query failed to execute after retries: {}\n\nCaused by:\n{err:#}",
                    initialize_query
                );
            }
        }

        // Disallow writes on the connection. The only writes allowed for thread safe connections
        // are from the background thread that can serialize them.
        *connection.write.get_mut() = false;

        connection
    }
}

fn is_schema_lock_error(err: &anyhow::Error) -> bool {
    let message = format!("{err:#}");
    message.contains("database schema is locked") || message.contains("database is locked")
}

impl ThreadSafeConnection {
    /// Special constructor for ThreadSafeConnection which disallows db initialization and migrations.
    /// This allows construction to be infallible and not write to the db.
    pub fn new(
        uri: &str,
        persistent: bool,
        connection_initialize_query: Option<&'static str>,
        write_queue_constructor: Option<WriteQueueConstructor>,
    ) -> Self {
        let connection = Self {
            uri: Arc::from(uri),
            persistent,
            connection_initialize_query,
            connections: Default::default(),
        };

        connection.initialize_queues(write_queue_constructor);
        connection
    }
}

impl Deref for ThreadSafeConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connections.get_or(|| {
            Self::create_connection(self.persistent, &self.uri, self.connection_initialize_query)
        })
    }
}

pub fn background_thread_queue() -> WriteQueueConstructor {
    use std::sync::mpsc::channel;

    Box::new(|| {
        let (sender, receiver) = channel::<QueuedWrite>();

        thread::Builder::new()
            .name("sqlezWorker".to_string())
            .spawn(move || {
                while let Ok(write) = receiver.recv() {
                    write()
                }
            })
            .unwrap();

        let sender = UnboundedSyncSender::new(sender);
        Box::new(move |queued_write| {
            sender
                .send(queued_write)
                .expect("Could not send write action to background thread");
        })
    })
}

pub fn locking_queue() -> WriteQueueConstructor {
    Box::new(|| {
        let write_mutex = Mutex::new(());
        Box::new(move |queued_write| {
            let _lock = write_mutex.lock();
            queued_write();
        })
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use std::ops::Deref;

    use std::{thread, time::Duration};

    use crate::{domain::Domain, thread_safe_connection::ThreadSafeConnection};

    #[test]
    fn many_initialize_and_migrate_queries_at_once() {
        let mut handles = vec![];

        enum TestDomain {}
        impl Domain for TestDomain {
            const NAME: &str = "test";
            const MIGRATIONS: &[&str] = &["CREATE TABLE test(col1 TEXT, col2 TEXT) STRICT;"];
        }

        for _ in 0..100 {
            handles.push(thread::spawn(|| {
                let builder =
                    ThreadSafeConnection::builder::<TestDomain>("annoying-test.db", false)
                        .with_db_initialization_query("PRAGMA journal_mode=WAL")
                        .with_connection_initialize_query(indoc! {"
                                PRAGMA synchronous=NORMAL;
                                PRAGMA busy_timeout=1;
                                PRAGMA foreign_keys=TRUE;
                                PRAGMA case_sensitive_like=TRUE;
                            "});

                let _ = pollster::block_on(builder.build()).unwrap().deref();
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    #[test]
    fn connection_initialize_query_retries_transient_schema_lock() {
        let name = "connection_initialize_query_retries_transient_schema_lock";
        let locking_connection = crate::connection::Connection::open_memory(Some(name));
        locking_connection.exec("BEGIN IMMEDIATE").unwrap()().unwrap();
        locking_connection
            .exec("CREATE TABLE test(col TEXT)")
            .unwrap()()
        .unwrap();

        let releaser = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            locking_connection.exec("ROLLBACK").unwrap()().unwrap();
        });

        ThreadSafeConnection::create_connection(false, name, Some("PRAGMA FOREIGN_KEYS=true"));
        releaser.join().unwrap();
    }
}
