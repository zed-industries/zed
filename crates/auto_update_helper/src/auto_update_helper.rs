use std::{
    cell::RefCell,
    fmt::Debug,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::AtomicBool,
        mpsc::{Receiver, Sender, SyncSender},
        Arc,
    },
};

use anyhow::{Context, Result};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateFontW, DeleteObject, EndPaint, ReleaseDC, SelectObject, TextOutW,
            FW_NORMAL, LOGFONTW, PAINTSTRUCT,
        },
        UI::{
            Controls::{PBM_SETRANGE, PBM_SETSTEP, PBM_STEPIT, PROGRESS_CLASS},
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDesktopWindow, GetMessageW,
                GetWindowLongPtrW, GetWindowRect, MessageBoxW, PostMessageW, PostQuitMessage,
                RegisterClassW, SendMessageW, SetWindowLongPtrW, SetWindowTextW,
                SystemParametersInfoW, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA,
                MB_ICONERROR, MB_SYSTEMMODAL, MSG, SPI_GETICONTITLELOGFONT,
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WINDOW_EX_STYLE, WM_CREATE, WM_DESTROY,
                WM_NCCREATE, WM_PAINT, WM_USER, WNDCLASSW, WS_CAPTION, WS_CHILD, WS_EX_TOPMOST,
                WS_POPUP, WS_VISIBLE,
            },
        },
    },
};

const TOTAL_JOBS: usize = 6;
const WM_JOB_UPDATED: u32 = WM_USER + 1;
const WM_TERMINATE: u32 = WM_USER + 2;

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    RemoveOld(Vec<PathBuf>),
    CopyNew(Vec<(PathBuf, PathBuf)>),
    DeleteInstall,
    DeleteUpdates,
    Done,
}

#[derive(Debug)]
enum JobResult {
    Finished,
    OtherJobFailed,
}

macro_rules! return_if_other_failed {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => {
                return Ok(JobResult::OtherJobFailed);
            }
        }
    };
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
                for _ in 0..2 {
                    unsafe {
                        PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                status = UpdateStatus::CopyNew(vec![]);
                // let mut sccess = Vec::with_capacity(old_files.len());
                // for old_file in old_files.iter() {
                //     if old_file.exists() {
                //         let result = std::fs::remove_file(&old_file);
                //         if let Err(error) = result {
                //             log::error!(
                //                 "Failed to remove old file {}: {:?}",
                //                 old_file.display(),
                //                 error
                //             );
                //         } else {
                //             sccess.push(old_file);
                //             return_if_other_failed!(unsafe {
                //                 PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))
                //             });
                //         }
                //     } else {
                //         sccess.push(old_file);
                //         unsafe {
                //             PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                //         }
                //     }
                //     std::thread::sleep(std::time::Duration::from_secs(1));
                // }
                // let left_old_files = old_files
                //     .iter()
                //     .filter(|old_file| !sccess.contains(old_file))
                //     .map(|old_file| old_file.clone())
                //     .collect::<Vec<_>>();
                // if left_old_files.is_empty() {
                //     status = UpdateStatus::CopyNew(vec![
                //         (install_dir.join("Zed.exe"), app_dir.join("Zed.exe")),
                //         (
                //             install_dir.join("bin\\zed.exe"),
                //             app_dir.join("bin\\zed.exe"),
                //         ),
                //     ]);
                // } else {
                //     status = UpdateStatus::RemoveOld(left_old_files);
                // }
            }
            UpdateStatus::CopyNew(new_files) => {
                for _ in 0..2 {
                    unsafe {
                        PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                status = UpdateStatus::DeleteInstall;
                // let mut sccess = Vec::with_capacity(new_files.len());
                // for (new_file, old_file) in new_files.iter() {
                //     if new_file.exists() {
                //         let result = std::fs::copy(&new_file, &old_file);
                //         if let Err(error) = result {
                //             log::error!(
                //                 "Failed to copy new file {} to {}: {:?}",
                //                 new_file.display(),
                //                 old_file.display(),
                //                 error
                //             );
                //         } else {
                //             sccess.push((new_file, old_file));
                //             unsafe {
                //                 PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                //             }
                //         }
                //     } else {
                //         sccess.push((new_file, old_file));
                //         unsafe {
                //             PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                //         }
                //     }
                //     std::thread::sleep(std::time::Duration::from_secs(1));
                // }
                // let left_new_files = new_files
                //     .iter()
                //     .filter(|(new_file, _)| !sccess.iter().any(|(n, _)| *n == new_file))
                //     .map(|(new_file, old_file)| (new_file.clone(), old_file.clone()))
                //     .collect::<Vec<_>>();

                // if left_new_files.is_empty() {
                //     status = UpdateStatus::DeleteInstall;
                // } else {
                //     status = UpdateStatus::CopyNew(left_new_files);
                // }
            }
            UpdateStatus::DeleteInstall => {
                // let result = std::fs::remove_dir_all(&install_dir);
                // if let Err(error) = result {
                //     log::error!("Failed to remove install directory: {:?}", error);
                //     continue;
                // }
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                status = UpdateStatus::DeleteUpdates;
                // unsafe {
                //     PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                // }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::DeleteUpdates => {
                // let result = std::fs::remove_dir_all(&update_dir);
                // if let Err(error) = result {
                //     log::error!("Failed to remove updates directory: {:?}", error);
                //     continue;
                // }
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                status = UpdateStatus::Done;
                // unsafe {
                //     PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                // }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::Done => {
                // let _ = std::process::Command::new(app_dir.join("Zed.exe")).spawn();
                break;
            }
        }
    }
    if status != UpdateStatus::Done {
        return Err(anyhow::anyhow!("Failed to update Zed, timeout"));
    }

    println!("update finished");
    // Ok(())
    Err(anyhow::anyhow!("Failed to update Zed, timeout"))
}

enum WorkResult {
    Finished,
    Failed(String),
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
        tx.send(update(app_dir.as_path(), hwnd));
        unsafe { PostMessageW(HWND(hwnd as _), WM_TERMINATE, WPARAM(0), LPARAM(0)) };
    });
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).as_bool() {
            DispatchMessageW(&message);
        }
    }
    Ok(())
    // let (tx, rx) = std::sync::mpsc::channel();
    // let (hwnd_tx, hwnd_rx) = std::sync::mpsc::sync_channel(1);

    // std::thread::spawn({
    //     let result_sender = tx.clone();
    //     move || {
    //         result_sender.send(run_dialog_window(hwnd_tx)).ok();
    //     }
    // });
    // println!("1");
    // let stop_flag = Arc::new(AtomicBool::new(false));
    // let hwnd = hwnd_rx.recv().unwrap()?;
    // println!("2");
    // std::thread::spawn({
    //     let flag = stop_flag.clone();
    //     move || tx.send(update(app_dir.as_path(), hwnd, flag)).ok()
    // });
    // println!("3");
    // for result in rx.iter() {
    //     println!("result: {:?}", result);
    //     if let Err(ref e) = result {
    //         log::error!("Error: Zed update failed, {:?}", e);
    //         stop_all_threads(hwnd, &stop_flag);
    //         show_result(
    //             format!("Error: {:?}", e),
    //             "Error: Zed update failed".to_owned(),
    //         );
    //         break;
    //     }
    // }

    // Ok(())
}

fn stop_all_threads(hwnd: isize, stop_flag: &Arc<AtomicBool>) {
    unsafe {
        PostMessageW(HWND(hwnd as _), WM_TERMINATE, WPARAM(0), LPARAM(0)).ok();
    }
    stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
}

#[repr(C)]
#[derive(Debug)]
struct DialogInfo {
    rx: Receiver<Result<()>>,
    progress_bar: isize,
}

fn create_dialog_window(receiver: Receiver<Result<()>>) -> Result<HWND> {
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
        let info = Box::new(RefCell::new(DialogInfo {
            rx: receiver,
            progress_bar: 0,
        }));

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
            Some(Box::into_raw(info) as _),
        )?;
        Ok(hwnd)
    }
}

// fn run_dialog_window(hwnd_sender: SyncSender<Result<isize>>) -> Result<JobResult> {
//     let hwnd = create_dialog_window();
//     match hwnd {
//         Ok(hwnd) => hwnd_sender.send(Ok(hwnd.0 as isize))?,
//         Err(e) => {
//             hwnd_sender.send(Err(e))?;
//             return Ok(JobResult::OtherJobFailed);
//         }
//     }
//     unsafe {
//         let mut message = MSG::default();
//         while GetMessageW(&mut message, None, 0, 0).as_bool() {
//             DispatchMessageW(&message);
//         }
//     }
//     Ok(JobResult::Finished)
// }

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

fn main() {
    if let Err(e) = run() {
        log::error!("Error: Zed update failed, {:?}", e);
        show_result(
            format!("Error: {:?}", e),
            "Error: Zed update failed".to_owned(),
        );
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
        WM_NCCREATE => {
            println!("WM_NCCREATE");
            log::info!("WM_NCCREATE");
            let create_struct = lparam.0 as *const CREATESTRUCTW;
            let info = (*create_struct).lpCreateParams as *mut RefCell<DialogInfo>;
            let info = Box::from_raw(info);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(info) as _);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_CREATE => {
            println!("WM_CREATE");
            log::info!("WM_CREATE");
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
            let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<DialogInfo>;
            let data = Box::from_raw(raw);
            data.borrow_mut().progress_bar = progress_bar.0 as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            LRESULT(0)
        }
        WM_PAINT => {
            println!("WM_PAINT");
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
            log::info!("WM_JOB_UPDATED");
            println!("WM_JOB_UPDATED");
            // let raw = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut isize };
            // let progress_bar = HWND(*raw as _);
            let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<DialogInfo>;
            let data = Box::from_raw(raw);
            let progress_bar = data.borrow().progress_bar;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            SendMessageW(HWND(progress_bar as _), PBM_STEPIT, WPARAM(0), LPARAM(0))
        }
        WM_TERMINATE => {
            let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<DialogInfo>;
            let data = Box::from_raw(raw);
            if let Ok(x) = data.borrow_mut().rx.recv() {
                println!("recv: {:?}<-------", x);
            }
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);

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
