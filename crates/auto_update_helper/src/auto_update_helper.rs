use std::{
    fmt::Debug,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

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

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    RemoveOld(Vec<PathBuf>),
    CopyNew(Vec<(PathBuf, PathBuf)>),
    DeleteInstall,
    DeleteUpdates,
    Done,
}

fn update(log: &mut File, app_dir: &Path) -> Result<()> {
    let install_dir = app_dir.join("install");
    let update_dir = app_dir.join("updates");

    let start = std::time::Instant::now();
    let mut status =
        UpdateStatus::RemoveOld(vec![app_dir.join("Zed.exe"), app_dir.join("bin\\zed.exe")]);
    while start.elapsed().as_secs() < 10 {
        match status {
            UpdateStatus::RemoveOld(old_files) => {
                let mut sccess = Vec::with_capacity(old_files.len());
                for old_file in old_files.iter() {
                    if old_file.exists() {
                        let result = std::fs::remove_file(&old_file);
                        if let Err(error) = result {
                            write_to_log_file(
                                log,
                                format!(
                                    "Failed to remove old file {}: {:?}",
                                    old_file.display(),
                                    error
                                ),
                            );
                        } else {
                            sccess.push(old_file);
                        }
                    } else {
                        sccess.push(old_file);
                    }
                }
                let left_old_files = old_files
                    .iter()
                    .filter(|old_file| !sccess.contains(old_file))
                    .map(|old_file| old_file.clone())
                    .collect::<Vec<_>>();
                if left_old_files.is_empty() {
                    status = UpdateStatus::CopyNew(vec![
                        (install_dir.join("Zed.exe"), app_dir.join("Zed.exe")),
                        (
                            install_dir.join("bin\\zed.exe"),
                            app_dir.join("bin\\zed.exe"),
                        ),
                    ]);
                } else {
                    status = UpdateStatus::RemoveOld(left_old_files);
                }
            }
            UpdateStatus::CopyNew(new_files) => {
                let mut sccess = Vec::with_capacity(new_files.len());
                for (new_file, old_file) in new_files.iter() {
                    if new_file.exists() {
                        let result = std::fs::copy(&new_file, &old_file);
                        if let Err(error) = result {
                            write_to_log_file(
                                log,
                                format!(
                                    "Failed to copy new file {} to {}: {:?}",
                                    new_file.display(),
                                    old_file.display(),
                                    error
                                ),
                            );
                        } else {
                            sccess.push((new_file, old_file));
                        }
                    } else {
                        sccess.push((new_file, old_file));
                    }
                }
                let left_new_files = new_files
                    .iter()
                    .filter(|(new_file, _)| !sccess.iter().any(|(n, _)| *n == new_file))
                    .map(|(new_file, old_file)| (new_file.clone(), old_file.clone()))
                    .collect::<Vec<_>>();

                if left_new_files.is_empty() {
                    status = UpdateStatus::DeleteInstall;
                } else {
                    status = UpdateStatus::CopyNew(left_new_files);
                }
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
            UpdateStatus::Done => {
                let ret = std::process::Command::new(app_dir.join("Zed.exe")).spawn();
                write_to_log_file(log, format!("Starting Zed: {:?}", ret));
                break;
            }
        }
    }
    if status != UpdateStatus::Done {
        return Err(anyhow::anyhow!("Failed to update Zed"));
    }
    write_to_log_file(log, format!("Update takes: {:?}", start.elapsed()));

    Ok(())
}

fn run(log: &mut File) -> Result<()> {
    let app_dir = std::env::current_exe()?
        .parent()
        .context("No parent directory")?
        .parent()
        .context("No parent directory")?
        .to_path_buf();
    update(log, app_dir.as_path())?;
    Ok(())
}

fn main() {
    let mut log = generate_log_file().unwrap();
    let ret = run(&mut log);
    write_to_log_file(&mut log, ret);
}
