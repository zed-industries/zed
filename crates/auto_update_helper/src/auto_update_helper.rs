use std::{
    fmt::Debug,
    fs::File,
    path::{Path, PathBuf},
    sync::mpsc::SyncSender,
};

use anyhow::{Context, Result};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{SetLastError, HWND, LPARAM, LRESULT, RECT, WIN32_ERROR, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateFontW, DeleteObject, EndPaint, ReleaseDC, SelectObject, TextOutW,
            FW_NORMAL, LOGFONTW, PAINTSTRUCT,
        },
        UI::{
            Controls::{PBM_SETRANGE, PBM_SETSTEP, PBM_STEPIT, PROGRESS_CLASS},
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDesktopWindow, GetMessageW,
                GetWindowLongPtrW, GetWindowRect, MessageBoxW, PostMessageW, PostQuitMessage,
                RegisterClassW, SendMessageW, SetWindowLongPtrW, SystemParametersInfoW, CS_HREDRAW,
                CS_VREDRAW, GWLP_USERDATA, MB_ICONERROR, MB_SYSTEMMODAL, MSG,
                SPI_GETICONTITLELOGFONT, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WINDOW_EX_STYLE,
                WM_CREATE, WM_DESTROY, WM_PAINT, WM_USER, WNDCLASSW, WS_CAPTION, WS_CHILD,
                WS_EX_TOPMOST, WS_POPUP, WS_VISIBLE,
            },
        },
    },
};

const TOTAL_JOBS: usize = 6;
const WM_JOB_UPDATED: u32 = WM_USER + 1;

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

fn update(app_dir: &Path, hwnd: isize) -> Result<()> {
    let install_dir = app_dir.join("install");
    let update_dir = app_dir.join("updates");
    let hwnd = HWND(hwnd as _);

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
                            // write_to_log_file(
                            //     log,
                            //     format!(
                            //         "Failed to remove old file {}: {:?}",
                            //         old_file.display(),
                            //         error
                            //     ),
                            // );
                        } else {
                            sccess.push(old_file);
                            unsafe {
                                PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                            }
                        }
                    } else {
                        sccess.push(old_file);
                        unsafe {
                            PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
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
                            // write_to_log_file(
                            //     log,
                            //     format!(
                            //         "Failed to copy new file {} to {}: {:?}",
                            //         new_file.display(),
                            //         old_file.display(),
                            //         error
                            //     ),
                            // );
                        } else {
                            sccess.push((new_file, old_file));
                            unsafe {
                                PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                            }
                        }
                    } else {
                        sccess.push((new_file, old_file));
                        unsafe {
                            PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
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
                    // write_to_log_file(
                    //     log,
                    //     format!("Failed to remove install directory: {:?}", error),
                    // );
                    continue;
                }
                status = UpdateStatus::DeleteUpdates;
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::DeleteUpdates => {
                let result = std::fs::remove_dir_all(&update_dir);
                if let Err(error) = result {
                    // write_to_log_file(
                    //     log,
                    //     format!("Failed to remove updates directory: {:?}", error),
                    // );
                    continue;
                }
                status = UpdateStatus::Done;
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::Done => {
                let _ = std::process::Command::new(app_dir.join("Zed.exe")).spawn();
                break;
            }
        }
    }
    if status != UpdateStatus::Done {
        return Err(anyhow::anyhow!("Failed to update Zed, timeout"));
    }

    Ok(())
}

fn run(log: &mut File) -> Result<()> {
    let app_dir = std::env::current_exe()?
        .parent()
        .context("No parent directory")?
        .parent()
        .context("No parent directory")?
        .to_path_buf();
    let (tx, rx) = std::sync::mpsc::channel();
    let (hwnd_tx, hwnd_rx) = std::sync::mpsc::sync_channel(1);
    write_to_log_file(log, "Running dialog window");
    std::thread::spawn({
        let result_sender = tx.clone();
        move || {
            result_sender.send(run_dialog_window(hwnd_tx)).ok();
        }
    });
    if let Ok(hwnd) = hwnd_rx.recv() {
        std::thread::spawn(move || tx.send(update(app_dir.as_path(), hwnd)).ok());
    }
    for result in rx.iter() {
        if let Err(ref e) = result {
            write_to_log_file(log, e);
            show_result(
                format!("Error: {:?}", e),
                "Error: Zed update failed".to_owned(),
            );
            break;
        }
    }

    Ok(())
}

fn run_dialog_window(hwnd_sender: SyncSender<isize>) -> Result<()> {
    unsafe {
        let class_name = windows::core::w!("ProgressBarJunkui");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };
        RegisterClassW(&wc);
        let mut rect = RECT::default();
        GetWindowRect(GetDesktopWindow(), &mut rect)?;
        let width = 400;
        let height = 150;

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST,
            class_name,
            windows::core::w!("Progress Bar demo"),
            WS_VISIBLE | WS_POPUP | WS_CAPTION,
            rect.right / 2 - width / 2,
            rect.bottom / 2 - height / 2,
            width,
            height,
            None,
            None,
            None,
            None,
        )?;
        hwnd_sender.send(hwnd.0 as isize)?;

        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).as_bool() {
            DispatchMessageW(&message);
        }
        Ok(())
    }
}

fn main() {
    let mut log = generate_log_file().unwrap();
    if let Err(e) = run(&mut log) {
        show_result(
            format!("Error: {:?}", e),
            "Error: Zed update failed".to_owned(),
        );
        write_to_log_file(&mut log, e);
    }
}

macro_rules! return_if_failed {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                return LRESULT(e.code().0 as _);
            }
        }
    };
}

macro_rules! make_lparam {
    ($l:expr, $h:expr) => {
        LPARAM(($l as u32 | ($h as u32) << 16) as isize)
    };
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Create progress bar
            let mut rect = RECT::default();
            return_if_failed!(GetWindowRect(hwnd, &mut rect));
            let progress_bar = return_if_failed!(CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PROGRESS_CLASS,
                None,
                WS_CHILD | WS_VISIBLE,
                20,
                50,
                340,
                35,
                hwnd,
                None,
                None,
                None,
            ));
            SendMessageW(
                progress_bar,
                PBM_SETRANGE,
                WPARAM(0),
                make_lparam!(0, TOTAL_JOBS * 10),
            );
            SendMessageW(progress_bar, PBM_SETSTEP, WPARAM(10), LPARAM(0));
            let data = Box::new(progress_bar.0 as isize);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let font_name = get_system_ui_font_name();
            let font = CreateFontW(
                24,
                0,
                0,
                0,
                FW_NORMAL.0 as _,
                0,
                0,
                0,
                0,
                0,
                0,
                2,
                0,
                &HSTRING::from(font_name),
            );
            let temp = SelectObject(hdc, font);
            let string = HSTRING::from("Zed Editor is updating...");
            return_if_failed!(TextOutW(hdc, 20, 15, string.as_wide()).ok());
            return_if_failed!(DeleteObject(temp).ok());

            return_if_failed!(EndPaint(hwnd, &ps).ok());
            ReleaseDC(hwnd, hdc);

            LRESULT(0)
        }
        WM_JOB_UPDATED => {
            let raw = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut isize };
            let progress_bar = HWND(*raw as _);
            SendMessageW(progress_bar, PBM_STEPIT, WPARAM(0), LPARAM(0))
        }
        WM_DESTROY => {
            PostQuitMessage(-1);

            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn get_system_ui_font_name() -> String {
    unsafe {
        let mut info: LOGFONTW = std::mem::zeroed();
        if SystemParametersInfoW(
            SPI_GETICONTITLELOGFONT,
            std::mem::size_of::<LOGFONTW>() as u32,
            Some(&mut info as *mut _ as _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_ok()
        {
            let font_name = String::from_utf16_lossy(&info.lfFaceName);
            font_name.trim_matches(char::from(0)).to_owned()
        } else {
            "MS Shell Dlg".to_owned()
        }
    }
}

fn show_result(content: String, caption: String) {
    let _ = unsafe {
        MessageBoxW(
            None,
            &HSTRING::from(content),
            &HSTRING::from(caption),
            MB_ICONERROR | MB_SYSTEMMODAL,
        )
    };
}
