use std::collections::hash_map;
use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use sqlx_core::connection::Connection;
use sqlx_core::database::Database;
use sqlx_core::describe::Describe;
use sqlx_core::executor::Executor;
use sqlx_core::type_checking::TypeChecking;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "_sqlite"))]
mod impls;

pub trait DatabaseExt: Database + TypeChecking {
    const DATABASE_PATH: &'static str;
    const ROW_PATH: &'static str;

    fn db_path() -> syn::Path {
        syn::parse_str(Self::DATABASE_PATH).unwrap()
    }

    fn row_path() -> syn::Path {
        syn::parse_str(Self::ROW_PATH).unwrap()
    }

    fn describe_blocking(query: &str, database_url: &str) -> sqlx_core::Result<Describe<Self>>;
}

#[allow(dead_code)]
pub struct CachingDescribeBlocking<DB: DatabaseExt> {
    connections: Lazy<Mutex<HashMap<String, DB::Connection>>>,
}

#[allow(dead_code)]
impl<DB: DatabaseExt> CachingDescribeBlocking<DB> {
    pub const fn new() -> Self {
        CachingDescribeBlocking {
            connections: Lazy::new(|| Mutex::new(HashMap::new())),
        }
    }

    pub fn describe(&self, query: &str, database_url: &str) -> sqlx_core::Result<Describe<DB>>
    where
        for<'a> &'a mut DB::Connection: Executor<'a, Database = DB>,
    {
        let mut cache = self
            .connections
            .lock()
            .expect("previous panic in describe call");

        crate::block_on(async {
            let conn = match cache.entry(database_url.to_string()) {
                hash_map::Entry::Occupied(hit) => hit.into_mut(),
                hash_map::Entry::Vacant(miss) => {
                    miss.insert(DB::Connection::connect(database_url).await?)
                }
            };

            conn.describe(query).await
        })
    }
}
