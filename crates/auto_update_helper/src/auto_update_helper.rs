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
    use std::{borrow::Cow, path::Path};

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

    #[derive(Debug, Default)]
    struct Args {
        launch: bool,
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
        let args = parse_args(std::env::args().skip(1));
        std::thread::spawn(move || {
            let result = perform_update(app_dir.as_path(), Some(hwnd), args.launch);
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

    fn parse_args(input: impl IntoIterator<Item = String>) -> Args {
        let mut args: Args = Args { launch: true };

        let mut input = input.into_iter();
        if let Some(arg) = input.next() {
            let launch_arg;

            if arg == "--launch" {
                launch_arg = input.next().map(Cow::Owned);
            } else if let Some(rest) = arg.strip_prefix("--launch=") {
                launch_arg = Some(Cow::Borrowed(rest));
            } else {
                launch_arg = None;
            }

            if launch_arg.as_deref() == Some("false") {
                args.launch = false;
            }
        }

        args
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
    }
}
