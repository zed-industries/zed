use std::{fs::File, path::Path, thread::sleep, time::Duration};

use db::pane::SerializedDockPane;
use settings::DockAnchor;

const TEST_FILE: &'static str = "test-db.db";

fn main() -> anyhow::Result<()> {
    let db = db::Db::open_in_memory();
    if db.real().is_none() {
        return Err(anyhow::anyhow!("Migrations failed"));
    }
    let file = Path::new(TEST_FILE);

    let f = File::create(file)?;
    drop(f);

    let workspace = db.make_new_workspace::<String>(&[]);

    db.update_worktrees(&workspace.workspace_id, &["/tmp"]);

    db.save_dock_pane(SerializedDockPane {
        workspace: workspace.workspace_id,
        anchor_position: DockAnchor::Expanded,
        shown: true,
    });

    let new_workspace = db.workspace_for_roots(&["/tmp"]);

    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
