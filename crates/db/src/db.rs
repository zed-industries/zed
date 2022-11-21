pub mod kvp;

// Re-export
pub use anyhow;
pub use indoc::indoc;
pub use lazy_static;
pub use sqlez;

use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use util::channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME};
use util::paths::DB_DIR;

const INITIALIZE_QUERY: &'static str = indoc! {"
    PRAGMA journal_mode=WAL;
    PRAGMA synchronous=NORMAL;
    PRAGMA busy_timeout=1;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
"};

lazy_static::lazy_static! {
    static ref DB_WIPED: AtomicBool = AtomicBool::new(false);
}

/// Open or create a database at the given directory path.
pub fn open_file_db<M: Migrator>() -> ThreadSafeConnection<M> {
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

    ThreadSafeConnection::new(Some(db_path.to_string_lossy().as_ref()), true)
        .with_initialize_query(INITIALIZE_QUERY)
}

pub fn open_memory_db<M: Migrator>(db_name: Option<&str>) -> ThreadSafeConnection<M> {
    ThreadSafeConnection::new(db_name, false).with_initialize_query(INITIALIZE_QUERY)
}

/// Implements a basic DB wrapper for a given domain
#[macro_export]
macro_rules! connection {
    ($id:ident: $t:ident<$d:ty>) => {
        pub struct $t(::db::sqlez::thread_safe_connection::ThreadSafeConnection<$d>);

        impl ::std::ops::Deref for $t {
            type Target = ::db::sqlez::thread_safe_connection::ThreadSafeConnection<$d>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        ::db::lazy_static::lazy_static! {
            pub static ref $id: $t = $t(if cfg!(any(test, feature = "test-support")) {
                ::db::open_memory_db(None)
            } else {
                ::db::open_file_db()
            });
        }
    };
}

#[macro_export]
macro_rules! sql_method {
    ($id:ident() ->  Result<()>: $sql:expr) => {
        pub fn $id(&self) -> $crate::sqlez::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.exec($sql)?().context(::std::format!(
                "Error in {}, exec failed to execute or parse for: {}",
                ::std::stringify!($id),
                ::std::stringify!($sql),
            ))
        }
    };
    ($id:ident($($arg:ident: $arg_type:ty),+) -> Result<()>: $sql:expr) => {
        pub fn $id(&self, $($arg: $arg_type),+) -> $crate::sqlez::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.exec_bound::<($($arg_type),+)>($sql)?(($($arg),+))
                .context(::std::format!(
                    "Error in {}, exec_bound failed to execute or parse for: {}",
                    ::std::stringify!($id),
                    ::std::stringify!($sql),
                ))
        }
    };
    ($id:ident() ->  Result<Vec<$return_type:ty>>: $sql:expr) => {
         pub fn $id(&self) -> $crate::sqlez::anyhow::Result<Vec<$return_type>> {
             use $crate::anyhow::Context;

             self.select::<$return_type>($sql)?(())
                 .context(::std::format!(
                     "Error in {}, select_row failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))
         }
    };
    ($id:ident($($arg:ident: $arg_type:ty),+) -> Result<Vec<$return_type:ty>>: $sql:expr) => {
         pub fn $id(&self, $($arg: $arg_type),+) -> $crate::sqlez::anyhow::Result<Vec<$return_type>> {
             use $crate::anyhow::Context;

             self.select_bound::<($($arg_type),+), $return_type>($sql)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, exec_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))
         }
    };
    ($id:ident() ->  Result<Option<$return_type:ty>>: $sql:expr) => {
         pub fn $id(&self) -> $crate::sqlez::anyhow::Result<Option<$return_type>> {
             use $crate::anyhow::Context;

             self.select_row::<$return_type>($sql)?()
                 .context(::std::format!(
                     "Error in {}, select_row failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))
         }
    };
    ($id:ident($($arg:ident: $arg_type:ty),+) ->  Result<Option<$return_type:ty>>: $sql:expr) => {
         pub fn $id(&self, $($arg: $arg_type),+) -> $crate::sqlez::anyhow::Result<Option<$return_type>>  {
             use $crate::anyhow::Context;

             self.select_row_bound::<($($arg_type),+), $return_type>($sql)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))

         }
    };
    ($id:ident() ->  Result<$return_type:ty>: $sql:expr) => {
         pub fn $id(&self) ->  $crate::sqlez::anyhow::Result<$return_type>  {
             use $crate::anyhow::Context;

             self.select_row::<$return_type>($sql)?()
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))?
                 .context(::std::format!(
                     "Error in {}, select_row_bound expected single row result but found none for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))
         }
    };
    ($id:ident($($arg:ident: $arg_type:ty),+) ->  Result<$return_type:ty>: $sql:expr) => {
         pub fn $id(&self, $($arg: $arg_type),+) ->  $crate::sqlez::anyhow::Result<$return_type>  {
             use $crate::anyhow::Context;

             self.select_row_bound::<($($arg_type),+), $return_type>($sql)?(($($arg),+))
                 .context(::std::format!(
                     "Error in {}, select_row_bound failed to execute or parse for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))?
                 .context(::std::format!(
                     "Error in {}, select_row_bound expected single row result but found none for: {}",
                     ::std::stringify!($id),
                     ::std::stringify!($sql),
                 ))
         }
    };
}
