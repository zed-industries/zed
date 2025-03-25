use std::path::Path;

use anyhow::Result;
use futures::FutureExt;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

static JOBS: [Jobs; 6] = [
    Jobs::Remove("Zed.exe"),
    Jobs::Remove("bin\\zed.exe"),
    Jobs::Copy(CopyDetails {
        from: "install\\Zed.exe",
        to: "Zed.exe",
    }),
    Jobs::Copy(CopyDetails {
        from: "install\\bin\\zed.exe",
        to: "bin\\zed.exe",
    }),
    Jobs::Cleanup("updates"),
    Jobs::Cleanup("install"),
];

pub(crate) const JOBS_COUNT: usize = JOBS.len();

#[derive(Debug)]
enum Jobs<'a> {
    Remove(&'a str),
    Copy(CopyDetails<'a>),
    Cleanup(&'a str),
}

#[derive(Debug)]
struct CopyDetails<'a> {
    from: &'a str,
    to: &'a str,
}

macro_rules! log_err {
    ($e:expr, $s:literal) => {
        match $e {
            Ok(_) => Some(()),
            Err(err) => {
                log::error!("{}: {:?}", $s, err);
                None
            }
        }
    };
}

pub(crate) async fn perform_update(app_dir: &Path, hwnd: isize) -> Result<()> {
    let hwnd = HWND(hwnd as _);

    let work = async {
        let mut index = 0;
        loop {
            let Some(job) = JOBS.get(index) else {
                break;
            };
            // #[cfg(not(debug_assertions))]
            let ret = match job {
                Jobs::Remove(relativ_path) => {
                    let path = app_dir.join(relativ_path);
                    if path.exists() {
                        log::info!("Removing old file: {}", path.display());
                        log_err!(std::fs::remove_file(path), "Failed to remove old file")
                    } else {
                        log::warn!("Old file not found: {}", path.display());
                        None
                    }
                }
                Jobs::Copy(details) => {
                    let from_path = app_dir.join(details.from);
                    let to_path = app_dir.join(details.to);
                    if from_path.exists() {
                        log::info!(
                            "Copying new file {} to {}",
                            from_path.display(),
                            to_path.display()
                        );
                        log_err!(
                            std::fs::copy(&from_path, &to_path),
                            "Failed to copy new file"
                        )
                    } else {
                        log::warn!("New file not found: {}", from_path.display());
                        None
                    }
                }
                Jobs::Cleanup(relative_path) => {
                    let path = app_dir.join(relative_path);
                    if path.exists() {
                        log::info!("Cleaning up: {}", path.display());
                        log_err!(std::fs::remove_dir_all(path), "Failed to remove directory")
                    } else {
                        log::warn!("Directory not found: {}", path.display());
                        None
                    }
                }
            };
            // #[cfg(debug_assertions)]
            // let ret = {
            //     smol::Timer::after(std::time::Duration::from_secs(1)).await;
            //     Some(())
            // };
            if ret.is_some() {
                index += 1;
                unsafe { PostMessageW(Some(hwnd), WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        Ok(())
    }
    .fuse();
    futures::pin_mut!(work);

    futures::select_biased! {
        result = work => {
            match result {
                Ok(_) => {
                    let _ = smol::process::Command::new(app_dir.join("Zed.exe")).spawn();
                    log::info!("Update completed successfully");
                    Ok(())
                }
                Err(err) => {
                    log::error!("Update failed: {:?}", err);
                    Err(err)
                }
            }
        }
        _ = FutureExt::fuse(smol::Timer::after(std::time::Duration::from_secs(10))) => {
            log::error!("Update timed out");
            Err(anyhow::anyhow!("Update timed out"))
        }
    }
}
