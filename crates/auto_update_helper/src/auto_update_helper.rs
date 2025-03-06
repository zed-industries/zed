#![cfg(target_os = "windows")]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use anyhow::{Context, Result};
use dialog::create_dialog_window;
use updater::perform_update;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MessageBoxW, PostMessageW, MB_ICONERROR, MB_SYSTEMMODAL,
            MSG, WM_USER,
        },
    },
};

mod dialog;
mod updater;

const TOTAL_JOBS: usize = 6;
const WM_JOB_UPDATED: u32 = WM_USER + 1;
const WM_TERMINATE: u32 = WM_USER + 2;

fn main() {
    if let Err(e) = run() {
        log::error!("Error: Zed update failed, {:?}", e);
        show_error(format!("Error: {:?}", e));
    }
}

fn run() -> Result<()> {
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
    std::thread::spawn(move || {
        tx.send(perform_update(app_dir.as_path(), hwnd)).ok();
        unsafe { PostMessageW(HWND(hwnd as _), WM_TERMINATE, WPARAM(0), LPARAM(0)) }.ok();
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

fn show_error(mut content: String) {
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
