use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        mpsc::{SendError, Sender},
        Arc,
    },
    thread::JoinHandle,
};

use anyhow::Context as _;
use xcb::{x, Xid};

use crate::{capturer::Options, frame::Frame, targets::linux::get_default_x_display, Target};

use super::{error::LinCapError, LinuxCapturerImpl};

pub struct X11Capturer {
    capturer_join_handle: Option<JoinHandle<()>>,
    capturer_state: Arc<AtomicU8>,
    target: Target,
}

fn draw_cursor(
    conn: &xcb::Connection,
    img: &mut [u8],
    win_x: i16,
    win_y: i16,
    win_width: i16,
    win_height: i16,
    is_win: bool,
    win: &xcb::x::Window,
) -> Result<(), xcb::Error> {
    let cursor_image_cookie = conn.send_request(&xcb::xfixes::GetCursorImage {});
    let cursor_image = conn.wait_for_reply(cursor_image_cookie)?;

    let win_x = win_x as i32;
    let win_y = win_y as i32;

    let win_width = win_width as i32;
    let win_height = win_height as i32;

    let mut cursor_x = cursor_image.x() as i32 - cursor_image.xhot() as i32;
    let mut cursor_y = cursor_image.y() as i32 - cursor_image.yhot() as i32;
    if is_win {
        let disp = conn.get_raw_dpy();
        let mut ncursor_x = 0;
        let mut ncursor_y = 0;
        let mut child_return = 0;
        if unsafe {
            x11::xlib::XTranslateCoordinates(
                disp,
                x11::xlib::XDefaultRootWindow(disp),
                win.resource_id() as u64,
                cursor_image.x() as i32,
                cursor_image.y() as i32,
                &mut ncursor_x,
                &mut ncursor_y,
                &mut child_return,
            )
        } == 0
        {
            return Ok(());
        }
        cursor_x = ncursor_x as i32 - cursor_image.xhot() as i32;
        cursor_y = ncursor_y as i32 - cursor_image.yhot() as i32;
    }

    if cursor_x >= win_width + win_x
        || cursor_y >= win_height + win_y
        || cursor_x < win_x
        || cursor_y < win_y
    {
        return Ok(());
    }

    let x = cursor_x.max(win_x);
    let y = cursor_y.max(win_y);

    let w = ((cursor_x + cursor_image.width() as i32).min(win_x + win_width) - x) as u32;
    let h = ((cursor_y + cursor_image.height() as i32).min(win_y + win_height) - y) as u32;

    let c_off = (x - cursor_x) as u32;
    let i_off: i32 = x - win_x;

    let stride: u32 = 4;
    let mut cursor_idx: u32 = ((y - cursor_y) * cursor_image.width() as i32) as u32;
    let mut image_idx: u32 = ((y - win_y) * win_width * stride as i32) as u32;

    for y in 0..h {
        cursor_idx += c_off;
        image_idx += i_off as u32 * stride;
        for x in 0..w {
            let cursor_pix = cursor_image.cursor_image()[cursor_idx as usize];
            let r = (cursor_pix & 0xFF) as u8;
            let g = ((cursor_pix >> 8) & 0xFF) as u8;
            let b = ((cursor_pix >> 16) & 0xFF) as u8;
            let a = ((cursor_pix >> 24) & 0xFF);

            let i = image_idx as usize;
            if a == 0xFF {
                img[i] = r;
                img[i + 1] = g;
                img[i + 2] = b;
            } else if a > 0 {
                let a = 255 - a;
                img[i] = r + ((img[i] as u32 * a + 255 / 2) / 255) as u8;
                img[i + 1] = g + ((img[i + 1] as u32 * a + 255 / 2) / 255) as u8;
                img[i + 2] = b + ((img[i + 2] as u32 * a + 255 / 2) / 255) as u8;
            }

            cursor_idx += 1;
            image_idx += stride;
        }
        cursor_idx += cursor_image.width() as u32 - w as u32 - c_off as u32;
        image_idx += (win_width - w as i32 - i_off) as u32 * stride;
    }

    Ok(())
}

fn grab(conn: &xcb::Connection, target: &Target, show_cursor: bool) -> anyhow::Result<Frame> {
    let (x, y, width, height, window, is_win) = match &target {
        Target::Window(win) => {
            let geom_cookie = conn.send_request(&x::GetGeometry {
                drawable: x::Drawable::Window(win.raw_handle),
            });
            let geom = conn.wait_for_reply(geom_cookie)?;
            (0, 0, geom.width(), geom.height(), win.raw_handle, true)
        }
        Target::Display(disp) => (
            disp.x_offset,
            disp.y_offset,
            disp.width,
            disp.height,
            disp.raw_handle,
            false,
        ),
    };

    let img_cookie = conn.send_request(&x::GetImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(window),
        x,
        y,
        width,
        height,
        plane_mask: u32::MAX,
    });
    let img = conn.wait_for_reply(img_cookie)?;

    let mut img_data = img.data().to_vec();

    if show_cursor {
        draw_cursor(
            &conn,
            &mut img_data,
            x,
            y,
            width as i16,
            height as i16,
            is_win,
            &window,
        )?;
    }

    Ok(Frame::BGRx(crate::frame::BGRxFrame {
        display_time: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("Unix epoch is in the past")?
            .as_nanos() as u64,
        width: width as i32,
        height: height as i32,
        data: img_data,
    }))
}

fn query_xfixes_version(conn: &xcb::Connection) -> Result<(), xcb::Error> {
    let cookie = conn.send_request(&xcb::xfixes::QueryVersion {
        client_major_version: xcb::xfixes::MAJOR_VERSION,
        client_minor_version: xcb::xfixes::MINOR_VERSION,
    });
    let _ = conn.wait_for_reply(cookie)?;
    Ok(())
}

impl X11Capturer {
    pub fn new(options: &Options, tx: Sender<anyhow::Result<Frame>>) -> Result<Self, LinCapError> {
        let (conn, screen_num) = xcb::Connection::connect_with_xlib_display_and_extensions(
            &[xcb::Extension::RandR, xcb::Extension::XFixes],
            &[],
        )
        .map_err(|e| LinCapError::new(e.to_string()))?;
        query_xfixes_version(&conn).map_err(|e| LinCapError::new(e.to_string()))?;
        let setup = conn.get_setup();
        let Some(screen) = setup.roots().nth(screen_num as usize) else {
            return Err(LinCapError::new(String::from("Failed to get setup root")));
        };

        let target = match &options.target {
            Some(t) => t.clone(),
            None => Target::Display(
                get_default_x_display(&conn, screen)
                    .map_err(|e| LinCapError::new(e.to_string()))?,
            ),
        };

        let framerate = options.fps as f32;
        let show_cursor = options.show_cursor;
        let capturer_state = Arc::new(AtomicU8::new(0));
        let capturer_state_clone = Arc::clone(&capturer_state);

        let target_clone = target.clone();
        let jh = std::thread::spawn(move || {
            while capturer_state_clone.load(Ordering::Acquire) == 0 {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            let frame_time = std::time::Duration::from_secs_f32(1.0 / framerate);
            while capturer_state_clone.load(Ordering::Acquire) == 1 {
                let start = std::time::Instant::now();

                match tx.send(grab(&conn, &target_clone, show_cursor)) {
                    Ok(()) => {}
                    Err(SendError(_)) => {
                        log::debug!("Frame receiver was dropped.")
                    }
                }

                let elapsed = start.elapsed();
                if elapsed < frame_time {
                    std::thread::sleep(frame_time - elapsed);
                }
            }
        });

        Ok(Self {
            capturer_state,
            capturer_join_handle: Some(jh),
            target,
        })
    }
}

impl LinuxCapturerImpl for X11Capturer {
    fn start_capture(&mut self) {
        self.capturer_state.store(1, Ordering::Release);
    }

    fn stop_capture(&mut self) {
        self.capturer_state.store(2, Ordering::Release);
        if let Some(handle) = self.capturer_join_handle.take() {
            match handle.join() {
                Ok(()) => {}
                Err(err) => log::error!("Failed to join X11 screen capture thread: {:?}", err),
            }
        }
    }
    fn target(&self) -> Option<&Target> {
        Some(&self.target)
    }
}
