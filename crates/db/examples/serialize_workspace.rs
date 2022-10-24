use std::{fs::File, path::Path};

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
    db.write_kvp("test", "2")?;
    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
