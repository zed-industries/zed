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

pub(crate) async fn perform_update(app_dir: &Path, hwnd: Option<isize>) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    #[cfg(not(test))]
    let work = async {
        let mut index = 0;
        loop {
            let Some(job) = JOBS.get(index) else {
                break;
            };
            let ret = match job {
                Jobs::Remove(relative_path) => {
                    let path = app_dir.join(relative_path);
                    if path.exists() {
                        log::info!("Removing old file: {}", path.display());
                        log_err!(
                            smol::fs::remove_file(path).await,
                            "Failed to remove old file"
                        )
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
                            smol::fs::copy(from_path, to_path).await,
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
                        log_err!(
                            smol::fs::remove_dir_all(path).await,
                            "Failed to remove directory"
                        )
                    } else {
                        log::warn!("Directory not found: {}", path.display());
                        None
                    }
                }
            };

            if ret.is_some() {
                index += 1;
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        Ok::<(), anyhow::Error>(())
    }
    .fuse();
    #[cfg(test)]
    let work = async {
        let mut index = 0;
        loop {
            if JOBS.get(index).is_none() {
                break;
            }
            let ret = {
                smol::Timer::after(std::time::Duration::from_secs(1)).await;
                if let Ok(setting) = std::env::var("ZED_AUTO_UPDATE") {
                    match setting.as_str() {
                        "inf" => None,
                        "err" => Err(anyhow::anyhow!("Test error"))?,
                        _ => {
                            panic!("ZED_AUTO_UPDATE is set to {}, aborting test", setting);
                        }
                    }
                } else {
                    Some(())
                }
            };
            if ret.is_some() {
                index += 1;
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        Ok::<(), anyhow::Error>(())
    }
    .fuse();
    futures::pin_mut!(work);

    futures::select_biased! {
        result = work => {
            result?;
            let _ = smol::process::Command::new(app_dir.join("Zed.exe")).spawn();
            log::info!("Update completed successfully");
            Ok(())
        }
        _ = FutureExt::fuse(smol::Timer::after(std::time::Duration::from_secs(10))) => {
            Err(anyhow::anyhow!("Update timed out"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::perform_update;

    #[test]
    fn test_perform_update() {
        let app_dir = std::path::Path::new("C:/");
        assert!(smol::block_on(perform_update(app_dir, None)).is_ok());

        // Simulate a timeout
        std::env::set_var("ZED_AUTO_UPDATE", "inf");
        let ret = smol::block_on(perform_update(app_dir, None));
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Update timed out"));

        // Simulate a test error
        std::env::set_var("ZED_AUTO_UPDATE", "err");
        let ret = smol::block_on(perform_update(app_dir, None));
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Test error"));
    }
}
