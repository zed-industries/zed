#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod dialog;
#[cfg(target_os = "windows")]
mod updater;

#[cfg(target_os = "windows")]
fn main() {
    if let Err(e) = windows_impl::run() {
        log::error!("Error: Zed update failed, {:?}", e);
        windows_impl::show_error(format!("Error: {:?}", e));
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
mod windows_impl {
    use std::{
        ffi::{OsStr, OsString},
        path::Path,
    };

    use super::dialog::create_dialog_window;
    use super::updater::perform_update;
    use anyhow::{Context as _, Result};
    use windows::{
        Win32::{
            Foundation::{HWND, LPARAM, WPARAM},
            UI::WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, MB_ICONERROR, MB_SYSTEMMODAL, MSG, MessageBoxW,
                PostMessageW, WM_USER,
            },
        },
        core::HSTRING,
    };

    pub(crate) const WM_JOB_UPDATED: u32 = WM_USER + 1;
    pub(crate) const WM_TERMINATE: u32 = WM_USER + 2;

    #[derive(Debug, PartialEq, Eq)]
    struct Args {
        launch: bool,
        launch_arguments: Vec<OsString>,
    }

    pub(crate) fn run() -> Result<()> {
        let helper_dir = std::env::current_exe()?
            .parent()
            .context("No parent directory")?
            .to_path_buf();
        init_log(&helper_dir)?;
        let app_dir = helper_dir
            .parent()
            .context("No parent directory")?
            .to_path_buf();

        log::info!("======= Starting Zed update =======");
        let (tx, rx) = std::sync::mpsc::channel();
        let hwnd = create_dialog_window(rx)?.0 as isize;
        let args = parse_args(std::env::args_os().skip(1));
        std::thread::spawn(move || {
            let result = perform_update(
                app_dir.as_path(),
                Some(hwnd),
                args.launch,
                &args.launch_arguments,
            );
            tx.send(result).ok();
            unsafe { PostMessageW(Some(HWND(hwnd as _)), WM_TERMINATE, WPARAM(0), LPARAM(0)) }.ok();
        });
        unsafe {
            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).as_bool() {
                DispatchMessageW(&message);
            }
        }
        Ok(())
    }

    fn init_log(helper_dir: &Path) -> Result<()> {
        simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            std::fs::File::options()
                .append(true)
                .create(true)
                .open(helper_dir.join("auto_update_helper.log"))?,
        )?;
        Ok(())
    }

    fn parse_args(input: impl IntoIterator<Item = OsString>) -> Args {
        let mut args = Args {
            launch: true,
            launch_arguments: Vec::new(),
        };

        let mut input = input.into_iter();
        if let Some(arg) = input.next() {
            if arg == OsStr::new("--launch") {
                args.launch = input.next().as_deref() != Some(OsStr::new("false"));
            } else if let Some(launch) = arg.to_str().and_then(|arg| arg.strip_prefix("--launch="))
            {
                args.launch = launch != "false";
            } else {
                args.launch_arguments.push(arg);
            }
            args.launch_arguments.extend(input);
        }

        args
    }

    pub(crate) fn show_error(mut content: String) {
        if content.len() > 600 {
            content.truncate(600);
            content.push_str("…\n");
        }
        let _ = unsafe {
            MessageBoxW(
                None,
                &HSTRING::from(content),
                windows::core::w!("Error: Zed update failed."),
                MB_ICONERROR | MB_SYSTEMMODAL,
            )
        };
    }

    #[cfg(test)]
    mod tests {
        use std::ffi::OsString;

        use crate::windows_impl::parse_args;

        #[test]
        fn test_parse_args() {
            // launch can be specified via two separate arguments
            assert!(parse_args(["--launch".into(), "true".into()]).launch);
            assert!(!parse_args(["--launch".into(), "false".into()]).launch);

            // launch can be specified via one single argument
            assert!(parse_args(["--launch=true".into()]).launch);
            assert!(!parse_args(["--launch=false".into()]).launch);

            // launch defaults to true on no arguments
            assert!(parse_args([]).launch);

            // launch defaults to true on invalid arguments
            assert!(parse_args(["--launch".into()]).launch);
            assert!(parse_args(["--launch=".into()]).launch);
            assert!(parse_args(["--launch=invalid".into()]).launch);
        }

        #[test]
        fn test_parse_args_preserves_launch_arguments() {
            let launch_arguments = vec![
                OsString::from("--user-data-dir"),
                OsString::from(r"C:\Zed Data"),
            ];
            assert_eq!(
                parse_args(launch_arguments.clone()).launch_arguments,
                launch_arguments
            );
        }
    }
}
