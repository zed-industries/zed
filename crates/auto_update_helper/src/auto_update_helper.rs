use std::{fmt::Debug, fs::File, path::Path};

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

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    RemoveOld,
    CopyNew,
    DeleteInstall,
    DeleteUpdates,
    Done,
}

fn update(log: &mut File, app_dir: &Path) -> Result<()> {
    let install_dir = app_dir.join("install");
    let update_dir = app_dir.join("updates");
    let zed_exe = app_dir.join("Zed.exe");

    let start = std::time::Instant::now();
    let mut status = UpdateStatus::RemoveOld;
    while start.elapsed().as_secs() < 10 {
        match status {
            UpdateStatus::RemoveOld => {
                if zed_exe.exists() {
                    let result = std::fs::remove_file(&zed_exe);
                    if let Err(error) = result {
                        write_to_log_file(log, format!("Failed to remove Zed.exe: {:?}", error));
                        continue;
                    }
                }
                status = UpdateStatus::CopyNew;
            }
            UpdateStatus::CopyNew => {
                let new_exe = install_dir.join("Zed.exe");
                if !new_exe.exists() {
                    return Err(anyhow::anyhow!("New Zed.exe does not exist"));
                }
                let result = std::fs::copy(new_exe, &zed_exe);
                if let Err(error) = result {
                    write_to_log_file(log, format!("Failed to copy new Zed.exe: {:?}", error));
                    continue;
                }
                status = UpdateStatus::DeleteInstall;
            }
            UpdateStatus::DeleteInstall => {
                let result = std::fs::remove_dir_all(&install_dir);
                if let Err(error) = result {
                    write_to_log_file(
                        log,
                        format!("Failed to remove install directory: {:?}", error),
                    );
                    continue;
                }
                status = UpdateStatus::DeleteUpdates;
            }
            UpdateStatus::DeleteUpdates => {
                let result = std::fs::remove_dir_all(&update_dir);
                if let Err(error) = result {
                    write_to_log_file(
                        log,
                        format!("Failed to remove updates directory: {:?}", error),
                    );
                    continue;
                }
                status = UpdateStatus::Done;
            }
            UpdateStatus::Done => break,
        }
    }
    if status != UpdateStatus::Done {
        return Err(anyhow::anyhow!("Failed to update Zed"));
    }
    write_to_log_file(log, format!("Update takes: {:?}", start.elapsed()));

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
    update(log, app_dir.as_path())?;
    Ok(())
}

fn main() {
    let mut log = generate_log_file().unwrap();
    let ret = run(&mut log);
    write_to_log_file(&mut log, ret);
}
