use std::{fs::File, path::Path};

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

    db.write_kvp("test", "1")?;
    db.write_kvp("test-2", "2")?;

    db.workspace_for_roots(&["/tmp1"]);
    db.workspace_for_roots(&["/tmp1", "/tmp2"]);
    db.workspace_for_roots(&["/tmp1", "/tmp2", "/tmp3"]);
    db.workspace_for_roots(&["/tmp2", "/tmp3"]);
    db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"]);
    db.workspace_for_roots(&["/tmp2", "/tmp4"]);
    db.workspace_for_roots(&["/tmp2"]);

    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
