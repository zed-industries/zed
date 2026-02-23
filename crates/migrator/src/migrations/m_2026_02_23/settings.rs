use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn remove_file_finder_git_status(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(file_finder) = obj.get_mut("file_finder") else {
        return Ok(());
    };

    let Some(file_finder_obj) = file_finder.as_object_mut() else {
        return Ok(());
    };

    file_finder_obj.remove("git_status");

    if file_finder_obj.is_empty() {
        obj.remove("file_finder");
    }

    Ok(())
}
