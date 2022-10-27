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

    db.write_kvp("test", "1")?;
    db.write_kvp("test-2", "2")?;

    let workspace_1 = db.make_new_workspace::<String>(&[]);
    let workspace_2 = db.make_new_workspace::<String>(&[]);
    let workspace_3 = db.make_new_workspace::<String>(&[]);
    let workspace_4 = db.make_new_workspace::<String>(&[]);
    let workspace_5 = db.make_new_workspace::<String>(&[]);
    let workspace_6 = db.make_new_workspace::<String>(&[]);
    let workspace_7 = db.make_new_workspace::<String>(&[]);

    // Order scrambled + sleeps added because sqlite only has 1 second resolution on
    // their timestamps
    db.update_worktrees(&workspace_7.workspace_id, &["/tmp2"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_1.workspace_id, &["/tmp1"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_2.workspace_id, &["/tmp1", "/tmp2"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_3.workspace_id, &["/tmp1", "/tmp2", "/tmp3"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_4.workspace_id, &["/tmp2", "/tmp3"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_5.workspace_id, &["/tmp2", "/tmp3", "/tmp4"]);
    sleep(Duration::from_secs(1));
    db.update_worktrees(&workspace_6.workspace_id, &["/tmp2", "/tmp4"]);

    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
