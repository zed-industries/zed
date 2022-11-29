use futures::{Future, FutureExt};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::{collections::HashMap, marker::PhantomData, ops::Deref, sync::Arc, thread};
use thread_local::ThreadLocal;

use crate::{
    connection::Connection,
    domain::{Domain, Migrator},
    util::UnboundedSyncSender,
};

type QueuedWrite = Box<dyn 'static + Send + FnOnce(&Connection)>;

lazy_static! {
    static ref QUEUES: RwLock<HashMap<Arc<str>, UnboundedSyncSender<QueuedWrite>>> =
        Default::default();
}

pub struct ThreadSafeConnection<M: Migrator = ()> {
    uri: Arc<str>,
    persistent: bool,
    initialize_query: Option<&'static str>,
    connections: Arc<ThreadLocal<Connection>>,
    _migrator: PhantomData<M>,
}

unsafe impl<T: Migrator> Send for ThreadSafeConnection<T> {}
unsafe impl<T: Migrator> Sync for ThreadSafeConnection<T> {}

impl<M: Migrator> ThreadSafeConnection<M> {
    pub fn new(uri: &str, persistent: bool) -> Self {
        Self {
            uri: Arc::from(uri),
            persistent,
            initialize_query: None,
            connections: Default::default(),
            _migrator: PhantomData,
        }
    }

    /// Sets the query to run every time a connection is opened. This must
    /// be infallible (EG only use pragma statements)
    pub fn with_initialize_query(mut self, initialize_query: &'static str) -> Self {
        self.initialize_query = Some(initialize_query);
        self
    }

    /// Opens a new db connection with the initialized file path. This is internal and only
    /// called from the deref function.
    /// If opening fails, the connection falls back to a shared memory connection
    fn open_file(&self) -> Connection {
        // This unwrap is secured by a panic in the constructor. Be careful if you remove it!
        Connection::open_file(self.uri.as_ref())
    }

    /// Opens a shared memory connection using the file path as the identifier. This unwraps
    /// as we expect it always to succeed
    fn open_shared_memory(&self) -> Connection {
        Connection::open_memory(Some(self.uri.as_ref()))
    }

    // Open a new connection for the given domain, leaving this
    // connection intact.
    pub fn for_domain<D2: Domain>(&self) -> ThreadSafeConnection<D2> {
        ThreadSafeConnection {
            uri: self.uri.clone(),
            persistent: self.persistent,
            initialize_query: self.initialize_query,
            connections: Default::default(),
            _migrator: PhantomData,
        }
    }

    pub fn write<T: 'static + Send + Sync>(
        &self,
        callback: impl 'static + Send + FnOnce(&Connection) -> T,
    ) -> impl Future<Output = T> {
        // Startup write thread for this database if one hasn't already
        // been started and insert a channel to queue work for it
        if !QUEUES.read().contains_key(&self.uri) {
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

            let mut queues = QUEUES.write();
            queues.insert(self.uri.clone(), UnboundedSyncSender::new(sender));
        }

        // Grab the queue for this database
        let queues = QUEUES.read();
        let write_channel = queues.get(&self.uri).unwrap();

        // Create a one shot channel for the result of the queued write
        // so we can await on the result
        let (sender, reciever) = futures::channel::oneshot::channel();
        write_channel
            .send(Box::new(move |connection| {
                sender.send(callback(connection)).ok();
            }))
            .expect("Could not send write action to background thread");

        reciever.map(|response| response.expect("Background thread unexpectedly closed"))
    }

    pub(crate) fn create_connection(&self) -> Connection {
        let mut connection = if self.persistent {
            self.open_file()
        } else {
            self.open_shared_memory()
        };

        // Enable writes for the migrations and initialization queries
        connection.write = true;

        if let Some(initialize_query) = self.initialize_query {
            connection.exec(initialize_query).expect(&format!(
                "Initialize query failed to execute: {}",
                initialize_query
            ))()
            .unwrap()
        }

        M::migrate(&connection).expect("Migrations failed");

        // Disable db writes for normal thread local connection
        connection.write = false;
        connection
    }
}

impl<D: Domain> Clone for ThreadSafeConnection<D> {
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            persistent: self.persistent,
            initialize_query: self.initialize_query.clone(),
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
    use std::{fs, ops::Deref, thread};

    use crate::domain::Domain;

    use super::ThreadSafeConnection;

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
                let _ = ThreadSafeConnection::<TestDomain>::new("annoying-test.db", false)
                    .with_initialize_query(
                        "
                        PRAGMA journal_mode=WAL;
                        PRAGMA synchronous=NORMAL;
                        PRAGMA busy_timeout=1;
                        PRAGMA foreign_keys=TRUE;
                        PRAGMA case_sensitive_like=TRUE;
                    ",
                    )
                    .deref();
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }

        // fs::remove_file("annoying-test.db").unwrap();
        // fs::remove_file("annoying-test.db-shm").unwrap();
        // fs::remove_file("annoying-test.db-wal").unwrap();
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

        let _ = ThreadSafeConnection::<TestWorkspace>::new("wild_zed_lost_failure", false)
            .with_initialize_query("PRAGMA FOREIGN_KEYS=true")
            .deref();
    }
}
