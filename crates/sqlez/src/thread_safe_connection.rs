use std::{ops::Deref, sync::Arc};

use connection::Connection;
use thread_local::ThreadLocal;

use crate::connection;

pub struct ThreadSafeConnection {
    uri: Arc<str>,
    persistent: bool,
    initialize_query: Option<&'static str>,
    connection: Arc<ThreadLocal<Connection>>,
}

impl ThreadSafeConnection {
    pub fn new(uri: &str, persistent: bool) -> Self {
        Self {
            uri: Arc::from(uri),
            persistent,
            initialize_query: None,
            connection: Default::default(),
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
        Connection::open_file(self.uri.as_ref())
    }

    /// Opens a shared memory connection using the file path as the identifier. This unwraps
    /// as we expect it always to succeed
    fn open_shared_memory(&self) -> Connection {
        Connection::open_memory(self.uri.as_ref())
    }
}

impl Clone for ThreadSafeConnection {
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            persistent: self.persistent,
            initialize_query: self.initialize_query.clone(),
            connection: self.connection.clone(),
        }
    }
}

impl Deref for ThreadSafeConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection.get_or(|| {
            let connection = if self.persistent {
                self.open_file()
            } else {
                self.open_shared_memory()
            };

            if let Some(initialize_query) = self.initialize_query {
                connection.exec(initialize_query).expect(&format!(
                    "Initialize query failed to execute: {}",
                    initialize_query
                ));
            }

            connection
        })
    }
}
