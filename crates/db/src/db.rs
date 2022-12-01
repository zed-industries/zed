pub mod kvp;

// Re-export
pub use anyhow;
pub use indoc::indoc;
pub use lazy_static;
pub use smol;
pub use sqlez;
pub use sqlez_macros;

use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez_macros::sql;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use util::channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME};
use util::paths::DB_DIR;

const CONNECTION_INITIALIZE_QUERY: &'static str = sql!(
    PRAGMA synchronous=NORMAL;
    PRAGMA busy_timeout=1;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
);

const DB_INITIALIZE_QUERY: &'static str = sql!(
    PRAGMA journal_mode=WAL;
);

lazy_static::lazy_static! {
    static ref DB_WIPED: AtomicBool = AtomicBool::new(false);
}

/// Open or create a database at the given directory path.
pub async fn open_db<M: Migrator>() -> ThreadSafeConnection<M> {
    // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
    let current_db_dir = (*DB_DIR).join(Path::new(&format!("0-{}", *RELEASE_CHANNEL_NAME)));

    if *RELEASE_CHANNEL == ReleaseChannel::Dev
        && std::env::var("WIPE_DB").is_ok()
        && !DB_WIPED.load(Ordering::Acquire)
    {
        remove_dir_all(&current_db_dir).ok();
        DB_WIPED.store(true, Ordering::Relaxed);
    }

    create_dir_all(&current_db_dir).expect("Should be able to create the database directory");
    let db_path = current_db_dir.join(Path::new("db.sqlite"));

    ThreadSafeConnection::<M>::builder(db_path.to_string_lossy().as_ref(), true)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
}

#[cfg(any(test, feature = "test-support"))]
pub async fn open_test_db<M: Migrator>(db_name: &str) -> ThreadSafeConnection<M> {
    use sqlez::thread_safe_connection::locking_queue;

    ThreadSafeConnection::<M>::builder(db_name, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        // Serialize queued writes via a mutex and run them synchronously
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
}

/// Implements a basic DB wrapper for a given domain
#[macro_export]
macro_rules! connection {
    ($id:ident: $t:ident<$d:ty>) => {
        pub struct $t($crate::sqlez::thread_safe_connection::ThreadSafeConnection<$d>);

        impl ::std::ops::Deref for $t {
            type Target = $crate::sqlez::thread_safe_connection::ThreadSafeConnection<$d>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[cfg(any(test, feature = "test-support"))]
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_test_db(stringify!($id))));
        }

        #[cfg(not(any(test, feature = "test-support")))]
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_db()));
        }
    };
}

#[macro_export]
macro_rules! query {
    ($vis:vis fn $id:ident() -> Result<()> { $($sql:tt)+ }) => {
        $vis fn $id(&self) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.exec(sql_stmt)?().context(::std::format!(
                "Error in {}, exec failed to execute or parse for: {}",
                ::std::stringify!($id),
                sql_stmt,
            ))
        }
    };
    ($vis:vis async fn $id:ident() -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec(sql_stmt)?().context(::std::format!(
                    "Error in {}, exec failed to execute or parse for: {}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<()> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.exec_bound::<($($arg_type),+)>(sql_stmt)?(($($arg),+))
                .context(::std::format!(
                    "Error in {}, exec_bound failed to execute or parse for: {}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident($arg:ident: $arg_type:ty) -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $arg: $arg_type) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(move |connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec_bound::<$arg_type>(sql_stmt)?($arg)
                    .context(::std::format!(
                        "Error in {}, exec_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(move |connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec_bound::<($($arg_type),+)>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "Error in {}, exec_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
         $vis fn $id(&self) -> $crate::anyhow::Result<Vec<$return_type>> {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select::<$return_type>(sql_stmt)?(())
                 .context(::std::format!(
                     "Error in {}, select_row failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))
         }
    };
    ($vis:vis async fn $id:ident() -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        pub async fn $id(&self) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select::<$return_type>(sql_stmt)?(())
                    .context(::std::format!(
                        "Error in {}, select_row failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
         $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Vec<$return_type>> {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, exec_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))
         }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "Error in {}, exec_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
         $vis fn $id(&self) -> $crate::anyhow::Result<Option<$return_type>> {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select_row::<$return_type>(sql_stmt)?()
                 .context(::std::format!(
                     "Error in {}, select_row failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))
         }
    };
    ($vis:vis async fn $id:ident() ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) -> $crate::anyhow::Result<Option<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row::<$return_type>(sql_stmt)?()
                    .context(::std::format!(
                        "Error in {}, select_row failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($arg:ident: $arg_type:ty) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $arg: $arg_type) -> $crate::anyhow::Result<Option<$return_type>>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<$arg_type, $return_type>(sql_stmt)?($arg)
                .context(::std::format!(
                    "Error in {}, select_row_bound failed to execute or parse for: {}",
                    ::std::stringify!($id),
                    sql_stmt
                ))

        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
         $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Option<$return_type>>  {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))

         }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Option<$return_type>>  {
            use $crate::anyhow::Context;


            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row_bound::<($($arg_type),+), $return_type>(indoc! { $sql })?(($($arg),+))
                    .context(::std::format!(
                        "Error in {}, select_row_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
         $vis fn $id(&self) ->  $crate::anyhow::Result<$return_type>  {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select_row::<$return_type>(indoc! { $sql })?()
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))?
                 .context(::std::format!(
                     "Error in {}, select_row_bound expected single row result but found none for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))
         }
    };
    ($vis:vis async fn $id:ident() ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row::<$return_type>(sql_stmt)?()
                    .context(::std::format!(
                        "Error in {}, select_row_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))?
                    .context(::std::format!(
                        "Error in {}, select_row_bound expected single row result but found none for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($arg:ident: $arg_type:ty) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        pub fn $id(&self, $arg: $arg_type) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<$arg_type, $return_type>(sql_stmt)?($arg)
                .context(::std::format!(
                    "Error in {}, select_row_bound failed to execute or parse for: {}",
                    ::std::stringify!($id),
                    sql_stmt
                ))?
                .context(::std::format!(
                    "Error in {}, select_row_bound expected single row result but found none for: {}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
         $vis fn $id(&self, $($arg: $arg_type),+) ->  $crate::anyhow::Result<$return_type>  {
             use $crate::anyhow::Context;

             let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

             self.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))?
                 .context(::std::format!(
                     "Error in {}, select_row_bound expected single row result but found none for: {}",
                     ::std::stringify!($id),
                     sql_stmt
                 ))
         }
    };
    ($vis:vis fn async $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;


            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "Error in {}, select_row_bound failed to execute or parse for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))?
                    .context(::std::format!(
                        "Error in {}, select_row_bound expected single row result but found none for: {}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
}
