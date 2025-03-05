use std::{
    fmt::{Debug, Display},
    fs::File,
    path::Path,
};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    running_app_pid: u32,
}

fn generate_log_file() -> Result<File> {
    let file_path = std::env::current_exe()?
        .parent()
        .context("No parent directory")?
        .parent()
        .context("No parent directory")?
        .join("auto_update_helper.log");

    if file_path.exists() {
        std::fs::remove_file(&file_path).context("Failed to remove existing log file")?;
    }

    Ok(std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .context("Failed to create log file")?)
}

fn write_to_log_file(log: &mut File, message: impl Debug) {
    use std::io::Write;
    let _ = writeln!(log, "{:?}", message);
}

// fn wait_for_app_to_exit(app_dir: &Path) -> Result<()> {
//     let start = std::time::Instant::now();
//     while start.elapsed().as_secs() < 10 {
//         if !nix::unistd::Pid::from_raw(pid as i32)
//             .is_alive()
//             .context("Failed to check if process is alive")?
//         {
//             return Ok(());
//         }
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }

//     Ok(())
// }

fn update(app_dir: &Path) -> Result<()> {
    let install_dir = app_dir.join("install");

    Ok(())
}

fn run(log: &mut File) -> Result<()> {
    let args = Args::parse();
    let app_dir = std::env::current_exe()?
        .parent()
        .context("No parent directory")?
        .parent()
        .context("No parent directory")?
        .to_path_buf();
    // wait_for_app_to_exit(app_dir.as_path())?;
    update(app_dir.as_path())?;
    Ok(())
}

fn main() {
    let mut log = generate_log_file().unwrap();
    let ret = run(&mut log);
    write_to_log_file(&mut log, ret);
}
