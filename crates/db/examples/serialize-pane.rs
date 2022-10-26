use std::{fs::File, path::Path, thread::sleep, time::Duration};

const TEST_FILE: &'static str = "test-db.db";

fn main() -> anyhow::Result<()> {
    let db = db::Db::open_in_memory();
    if db.real().is_none() {
        return Err(anyhow::anyhow!("Migrations failed"));
    }
    let file = Path::new(TEST_FILE);

    let f = File::create(file)?;
    drop(f);

    let workspace = db.make_new_workspace();

    db.update_worktree_roots(&workspace.workspace_id, &["/tmp"]);

    db.save_pane_splits(center_pane_group);
    db.save_dock_pane();

    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
