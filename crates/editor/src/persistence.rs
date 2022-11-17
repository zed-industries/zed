use std::path::PathBuf;

use db::connection;
use indoc::indoc;
use lazy_static::lazy_static;
use project::WorktreeId;
use sqlez::domain::Domain;
use workspace::{ItemId, Workspace};

use crate::Editor;

connection!(DB: EditorDb<(Workspace, Editor)>);

impl Domain for Editor {
    fn name() -> &'static str {
        "editor"
    }

    fn migrations() -> &'static [&'static str] {
        &[indoc! {"
                
        "}]
    }
}

impl EditorDb {
    fn get_path(_item_id: ItemId, _workspace_id: WorktreeId) -> PathBuf {
        unimplemented!();
    }
}
