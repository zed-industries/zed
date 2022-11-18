use std::{marker::PhantomData, ops::Deref, sync::Arc};

use connection::Connection;
use thread_local::ThreadLocal;

use crate::{
    connection,
    domain::{Domain, Migrator},
};

pub struct ThreadSafeConnection<M: Migrator> {
    uri: Option<Arc<str>>,
    persistent: bool,
    initialize_query: Option<&'static str>,
    connection: Arc<ThreadLocal<Connection>>,
    _pd: PhantomData<M>,
}

unsafe impl<T: Migrator> Send for ThreadSafeConnection<T> {}
unsafe impl<T: Migrator> Sync for ThreadSafeConnection<T> {}

impl<M: Migrator> ThreadSafeConnection<M> {
    pub fn new(uri: Option<&str>, persistent: bool) -> Self {
        if persistent == true && uri == None {
            // This panic is securing the unwrap in open_file(), don't remove it!
            panic!("Cannot create a persistent connection without a URI")
        }
        Self {
            uri: uri.map(|str| Arc::from(str)),
            persistent,
            initialize_query: None,
            connection: Default::default(),
            _pd: PhantomData,
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
        Connection::open_file(self.uri.as_ref().unwrap())
    }

    /// Opens a shared memory connection using the file path as the identifier. This unwraps
    /// as we expect it always to succeed
    fn open_shared_memory(&self) -> Connection {
        Connection::open_memory(self.uri.as_ref().map(|str| str.deref()))
    }

    // Open a new connection for the given domain, leaving this
    // connection intact.
    pub fn for_domain<D2: Domain>(&self) -> ThreadSafeConnection<D2> {
        ThreadSafeConnection {
            uri: self.uri.clone(),
            persistent: self.persistent,
            initialize_query: self.initialize_query,
            connection: Default::default(),
            _pd: PhantomData,
        }
    }
}

impl<D: Domain> Clone for ThreadSafeConnection<D> {
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            persistent: self.persistent,
            initialize_query: self.initialize_query.clone(),
            connection: self.connection.clone(),
            _pd: PhantomData,
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
                ))()
                .unwrap();
            }

            M::migrate(&connection).expect("Migrations failed");

            connection
        })
    }
}
