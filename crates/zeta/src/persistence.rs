use anyhow::Result;
use collections::HashMap;
use std::path::{Path, PathBuf};
use workspace::WorkspaceDb;

use db::sqlez_macros::sql;
use db::{define_connection, query};

define_connection!(
    pub static ref DB: ZetaDb<WorkspaceDb> = &[
        sql! (
            CREATE TABLE zeta_preferences(
                worktree_path BLOB NOT NULL PRIMARY KEY,
                accepted_data_collection INTEGER
            ) STRICT;
        ),
    ];
);

impl ZetaDb {
    pub fn get_all_zeta_preferences(&self) -> Result<HashMap<PathBuf, bool>> {
        Ok(self.get_all_zeta_preferences_query()?.into_iter().collect())
    }

    query! {
        fn get_all_zeta_preferences_query() -> Result<Vec<(PathBuf, bool)>> {
            SELECT worktree_path, accepted_data_collection FROM zeta_preferences
        }
    }

    query! {
        pub fn get_accepted_data_collection(worktree_path: &Path) -> Result<Option<bool>> {
            SELECT accepted_data_collection FROM zeta_preferences
            WHERE worktree_path = ?
        }
    }

    query! {
        pub async fn save_accepted_data_collection(worktree_path: PathBuf, accepted_data_collection: bool) -> Result<()> {
            INSERT INTO zeta_preferences
                (worktree_path, accepted_data_collection)
            VALUES
                (?1, ?2)
            ON CONFLICT (worktree_path) DO UPDATE SET
                accepted_data_collection = ?2
        }
    }

    query! {
        pub async fn clear_all_zeta_preferences() -> Result<()> {
            DELETE FROM zeta_preferences
        }
    }
}
