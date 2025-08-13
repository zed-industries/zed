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
    use std::path::Path;

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

    #[derive(Debug)]
    struct Args {
        launch: Option<bool>,
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
        let args = parse_args();
        std::thread::spawn(move || {
            let result = perform_update(app_dir.as_path(), Some(hwnd), args.launch.unwrap_or(true));
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

    fn parse_args() -> Args {
        let mut result = Args { launch: None };
        if let Some(candidate) = std::env::args().nth(1) {
            parse_single_arg(&candidate, &mut result);
        }

        result
    }

    fn parse_single_arg(arg: &str, result: &mut Args) {
        let Some((key, value)) = arg.strip_prefix("--").and_then(|arg| arg.split_once('=')) else {
            log::error!(
                "Invalid argument format: '{}'. Expected format: --key=value",
                arg
            );
            return;
        };

        match key {
            "launch" => parse_launch_arg(value, &mut result.launch),
            _ => log::error!("Unknown argument: --{}", key),
        }
    }

    fn parse_launch_arg(value: &str, arg: &mut Option<bool>) {
        match value {
            "true" => *arg = Some(true),
            "false" => *arg = Some(false),
            _ => log::error!(
                "Invalid value for --launch: '{}'. Expected 'true' or 'false'",
                value
            ),
        }
    }

    pub(crate) fn show_error(mut content: String) {
        if content.len() > 600 {
            content.truncate(600);
            content.push_str("...\n");
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
        use crate::windows_impl::{Args, parse_launch_arg, parse_single_arg};

        #[test]
        fn test_parse_launch_arg() {
            let mut arg = None;
            parse_launch_arg("true", &mut arg);
            assert_eq!(arg, Some(true));

            let mut arg = None;
            parse_launch_arg("false", &mut arg);
            assert_eq!(arg, Some(false));

            let mut arg = None;
            parse_launch_arg("invalid", &mut arg);
            assert_eq!(arg, None);
        }

        #[test]
        fn test_parse_single_arg() {
            let mut args = Args { launch: None };
            parse_single_arg("--launch=true", &mut args);
            assert_eq!(args.launch, Some(true));

            let mut args = Args { launch: None };
            parse_single_arg("--launch=false", &mut args);
            assert_eq!(args.launch, Some(false));

            let mut args = Args { launch: None };
            parse_single_arg("--launch=invalid", &mut args);
            assert_eq!(args.launch, None);

            let mut args = Args { launch: None };
            parse_single_arg("--launch", &mut args);
            assert_eq!(args.launch, None);

            let mut args = Args { launch: None };
            parse_single_arg("--unknown", &mut args);
            assert_eq!(args.launch, None);
        }
    }
}
