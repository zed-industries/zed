use futures::{channel::oneshot, Future, FutureExt};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::{collections::HashMap, marker::PhantomData, ops::Deref, sync::Arc, thread};
use thread_local::ThreadLocal;

use crate::{
    connection::Connection,
    domain::{Domain, Migrator},
    util::UnboundedSyncSender,
};

const MIGRATION_RETRIES: usize = 10;

type QueuedWrite = Box<dyn 'static + Send + FnOnce(&Connection)>;
lazy_static! {
    /// List of queues of tasks by database uri. This lets us serialize writes to the database
    /// and have a single worker thread per db file. This means many thread safe connections
    /// (possibly with different migrations) could all be communicating with the same background
    /// thread.
    static ref QUEUES: RwLock<HashMap<Arc<str>, UnboundedSyncSender<QueuedWrite>>> =
        Default::default();
}

/// Thread safe connection to a given database file or in memory db. This can be cloned, shared, static,
/// whatever. It derefs to a synchronous connection by thread that is read only. A write capable connection
/// may be accessed by passing a callback to the `write` function which will queue the callback
pub struct ThreadSafeConnection<M: Migrator = ()> {
    uri: Arc<str>,
    persistent: bool,
    connection_initialize_query: Option<&'static str>,
    connections: Arc<ThreadLocal<Connection>>,
    _migrator: PhantomData<M>,
}

unsafe impl<T: Migrator> Send for ThreadSafeConnection<T> {}
unsafe impl<T: Migrator> Sync for ThreadSafeConnection<T> {}

pub struct ThreadSafeConnectionBuilder<M: Migrator = ()> {
    db_initialize_query: Option<&'static str>,
    connection: ThreadSafeConnection<M>,
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

    pub async fn build(self) -> ThreadSafeConnection<M> {
        let db_initialize_query = self.db_initialize_query;

        self.connection
            .write(move |connection| {
                if let Some(db_initialize_query) = db_initialize_query {
                    connection.exec(db_initialize_query).expect(&format!(
                        "Db initialize query failed to execute: {}",
                        db_initialize_query
                    ))()
                    .unwrap();
                }

                let mut failure_result = None;
                for _ in 0..MIGRATION_RETRIES {
                    failure_result = Some(M::migrate(connection));
                    if failure_result.as_ref().unwrap().is_ok() {
                        break;
                    }
                }

                failure_result.unwrap().expect("Migration failed");
            })
            .await;

        self.connection
    }
}

impl<M: Migrator> ThreadSafeConnection<M> {
    pub fn builder(uri: &str, persistent: bool) -> ThreadSafeConnectionBuilder<M> {
        ThreadSafeConnectionBuilder::<M> {
            db_initialize_query: None,
            connection: Self {
                uri: Arc::from(uri),
                persistent,
                connection_initialize_query: None,
                connections: Default::default(),
                _migrator: PhantomData,
            },
        }
    }

    /// Opens a new db connection with the initialized file path. This is internal and only
    /// called from the deref function.
    fn open_file(&self) -> Connection {
        Connection::open_file(self.uri.as_ref())
    }

    /// Opens a shared memory connection using the file path as the identifier. This is internal
    /// and only called from the deref function.
    fn open_shared_memory(&self) -> Connection {
        Connection::open_memory(Some(self.uri.as_ref()))
    }

    fn queue_write_task(&self, callback: QueuedWrite) {
        // Startup write thread for this database if one hasn't already
        // been started and insert a channel to queue work for it
        if !QUEUES.read().contains_key(&self.uri) {
            let mut queues = QUEUES.write();
            if !queues.contains_key(&self.uri) {
                use std::sync::mpsc::channel;

                let (sender, reciever) = channel::<QueuedWrite>();
                let mut write_connection = self.create_connection();
                // Enable writes for this connection
                write_connection.write = true;
                thread::spawn(move || {
                    while let Ok(write) = reciever.recv() {
                        write(&write_connection)
                    }
                });

                queues.insert(self.uri.clone(), UnboundedSyncSender::new(sender));
            }
        }

        // Grab the queue for this database
        let queues = QUEUES.read();
        let write_channel = queues.get(&self.uri).unwrap();

        write_channel
            .send(callback)
            .expect("Could not send write action to backgorund thread");
    }

    pub fn write<T: 'static + Send + Sync>(
        &self,
        callback: impl 'static + Send + FnOnce(&Connection) -> T,
    ) -> impl Future<Output = T> {
        // Create a one shot channel for the result of the queued write
        // so we can await on the result
        let (sender, reciever) = oneshot::channel();
        self.queue_write_task(Box::new(move |connection| {
            sender.send(callback(connection)).ok();
        }));

        reciever.map(|response| response.expect("Background writer thread unexpectedly closed"))
    }

    pub(crate) fn create_connection(&self) -> Connection {
        let mut connection = if self.persistent {
            self.open_file()
        } else {
            self.open_shared_memory()
        };

        // Disallow writes on the connection. The only writes allowed for thread safe connections
        // are from the background thread that can serialize them.
        connection.write = false;

        if let Some(initialize_query) = self.connection_initialize_query {
            connection.exec(initialize_query).expect(&format!(
                "Initialize query failed to execute: {}",
                initialize_query
            ))()
            .unwrap()
        }

        connection
    }
}

impl ThreadSafeConnection<()> {
    /// Special constructor for ThreadSafeConnection which disallows db initialization and migrations.
    /// This allows construction to be infallible and not write to the db.
    pub fn new(
        uri: &str,
        persistent: bool,
        connection_initialize_query: Option<&'static str>,
    ) -> Self {
        Self {
            uri: Arc::from(uri),
            persistent,
            connection_initialize_query,
            connections: Default::default(),
            _migrator: PhantomData,
        }
    }
}

impl<D: Domain> Clone for ThreadSafeConnection<D> {
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            persistent: self.persistent,
            connection_initialize_query: self.connection_initialize_query.clone(),
            connections: self.connections.clone(),
            _migrator: PhantomData,
        }
    }
}

// TODO:
//  1. When migration or initialization fails, move the corrupted db to a holding place and create a new one
//  2. If the new db also fails, downgrade to a shared in memory db
//  3. In either case notify the user about what went wrong
impl<M: Migrator> Deref for ThreadSafeConnection<M> {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connections.get_or(|| self.create_connection())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use lazy_static::__Deref;
    use std::thread;

    use crate::{domain::Domain, thread_safe_connection::ThreadSafeConnection};

    #[test]
    fn many_initialize_and_migrate_queries_at_once() {
        let mut handles = vec![];

        enum TestDomain {}
        impl Domain for TestDomain {
            fn name() -> &'static str {
                "test"
            }
            fn migrations() -> &'static [&'static str] {
                &["CREATE TABLE test(col1 TEXT, col2 TEXT) STRICT;"]
            }
        }

        for _ in 0..100 {
            handles.push(thread::spawn(|| {
                let builder =
                    ThreadSafeConnection::<TestDomain>::builder("annoying-test.db", false)
                        .with_db_initialization_query("PRAGMA journal_mode=WAL")
                        .with_connection_initialize_query(indoc! {"
                                PRAGMA synchronous=NORMAL;
                                PRAGMA busy_timeout=1;
                                PRAGMA foreign_keys=TRUE;
                                PRAGMA case_sensitive_like=TRUE;
                            "});
                let _ = smol::block_on(builder.build()).deref();
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    #[test]
    #[should_panic]
    fn wild_zed_lost_failure() {
        enum TestWorkspace {}
        impl Domain for TestWorkspace {
            fn name() -> &'static str {
                "workspace"
            }

            fn migrations() -> &'static [&'static str] {
                &["
                    CREATE TABLE workspaces(
                        workspace_id INTEGER PRIMARY KEY,
                        dock_visible INTEGER, -- Boolean
                        dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
                        dock_pane INTEGER, -- NULL indicates that we don't have a dock pane yet
                        timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
                        FOREIGN KEY(dock_pane) REFERENCES panes(pane_id),
                        FOREIGN KEY(active_pane) REFERENCES panes(pane_id)
                    ) STRICT;
                    
                    CREATE TABLE panes(
                        pane_id INTEGER PRIMARY KEY,
                        workspace_id INTEGER NOT NULL,
                        active INTEGER NOT NULL, -- Boolean
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) 
                            ON DELETE CASCADE 
                            ON UPDATE CASCADE
                    ) STRICT;
                "]
            }
        }

        let builder =
            ThreadSafeConnection::<TestWorkspace>::builder("wild_zed_lost_failure", false)
                .with_connection_initialize_query("PRAGMA FOREIGN_KEYS=true");

        smol::block_on(builder.build());
    }
}
