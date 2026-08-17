use fontconfig_parser::{FontConfig, Result};
use std::path::PathBuf;

fn visit_dir(dir: PathBuf, fonts: &mut Vec<PathBuf>) -> Result<()> {
    let dir = std::fs::read_dir(dir)?;

    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(ty) = entry.file_type() {
                if ty.is_dir() {
                    visit_dir(entry.path(), fonts).ok();
                } else if ty.is_file() || ty.is_symlink() {
                    fonts.push(entry.path());
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut config = FontConfig::default();
    config.merge_config("/etc/fonts/fonts.conf").unwrap();

    println!("dirs: {:#?}", config.dirs);

    let mut fonts = Vec::new();

    for dir in config.dirs {
        visit_dir(dir.path, &mut fonts).ok();
    }

    println!("Find all {} fonts!", fonts.len());

    println!("fonts: {:#?}", fonts);

    Ok(())
}
