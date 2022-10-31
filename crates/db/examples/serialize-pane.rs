use std::{fs::File, path::Path};

use db::pane::{DockAnchor, SerializedDockPane};

const TEST_FILE: &'static str = "test-db.db";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let db = db::Db::open_in_memory();
    if db.real().is_none() {
        return Err(anyhow::anyhow!("Migrations failed"));
    }
    let file = Path::new(TEST_FILE);

    let f = File::create(file)?;
    drop(f);

    let workspace_1 = db.workspace_for_roots(&["/tmp"]);
    let workspace_2 = db.workspace_for_roots(&["/tmp", "/tmp2"]);
    let workspace_3 = db.workspace_for_roots(&["/tmp3", "/tmp2"]);
    dbg!(&workspace_1, &workspace_2, &workspace_3);
    db.write_to(file).ok();

    db.save_dock_pane(&SerializedDockPane {
        workspace_id: workspace_1.workspace_id,
        anchor_position: DockAnchor::Expanded,
        visible: true,
    });
    db.save_dock_pane(&SerializedDockPane {
        workspace_id: workspace_2.workspace_id,
        anchor_position: DockAnchor::Bottom,
        visible: true,
    });
    db.save_dock_pane(&SerializedDockPane {
        workspace_id: workspace_3.workspace_id,
        anchor_position: DockAnchor::Right,
        visible: false,
    });

    // db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
