#[cfg(target_os = "windows")]
mod windows_repro {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        thread,
        time::{Duration, Instant},
    };

    use gpui::*;
    use gpui_platform::application;
    use raw_window_handle::RawWindowHandle;

    const WM_NULL: u32 = 0x0000;
    const WM_KEYDOWN: u32 = 0x0100;
    const WM_KEYUP: u32 = 0x0101;
    const VK_A: usize = 0x41;
    const A_SCAN_CODE: u32 = 0x1e;

    #[link(name = "user32")]
    unsafe extern "system" {
        fn PostMessageW(window: isize, message: u32, wparam: usize, lparam: isize) -> i32;
    }

    fn post_message(window: isize, message: u32, wparam: usize, lparam: u32) -> bool {
        // SAFETY: `window` comes from GPUI's live Win32 platform window, and the message
        // parameters contain integers only. Failure is reported by the return value.
        (unsafe { PostMessageW(window, message, wparam, lparam as i32 as isize) }) != 0
    }

    fn post_key_message(window: isize, message: u32, lparam: u32, cancelled: &AtomicBool) -> bool {
        let deadline = Instant::now() + Duration::from_millis(250);
        while !cancelled.load(Ordering::Acquire) && Instant::now() < deadline {
            if post_message(window, message, VK_A, lparam) {
                return true;
            }
            thread::yield_now();
        }
        false
    }

    fn sleep_while_running(duration: Duration, cancelled: &AtomicBool) -> bool {
        let deadline = Instant::now() + duration;
        while !cancelled.load(Ordering::Acquire) {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return true;
            }
            thread::sleep(remaining.min(Duration::from_millis(10)));
        }
        false
    }

    struct StressController {
        cancelled: Arc<AtomicBool>,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Drop for StressController {
        fn drop(&mut self) {
            self.cancelled.store(true, Ordering::Release);
            if let Some(thread) = self.thread.take()
                && thread.join().is_err()
            {
                eprintln!("stress thread panicked");
            }
        }
    }

    // Posted WM_KEYDOWN reaches GPUI's keyboard handler while WM_NULL keeps the
    // posted-message FIFO continuously busy. Input-class fairness is tested separately.
    fn start_stress(window: isize) -> StressController {
        let cancelled = Arc::new(AtomicBool::new(false));
        let thread_cancelled = cancelled.clone();
        let thread = thread::spawn(move || {
            if !sleep_while_running(Duration::from_secs(1), &thread_cancelled) {
                return;
            }

            let flood_running = Arc::new(AtomicBool::new(true));
            let flood_running_thread = flood_running.clone();
            let flood_cancelled = thread_cancelled.clone();
            let flood = thread::spawn(move || {
                while !flood_cancelled.load(Ordering::Acquire)
                    && flood_running_thread.load(Ordering::Acquire)
                {
                    for _ in 0..16 {
                        if flood_cancelled.load(Ordering::Acquire) {
                            break;
                        }
                        if !post_message(window, WM_NULL, 0, 0) {
                            thread::yield_now();
                            break;
                        }
                    }
                    thread::yield_now();
                }
            });

            let started = Instant::now();
            let mut key_count = 0;
            while !thread_cancelled.load(Ordering::Acquire)
                && started.elapsed() < Duration::from_secs(8)
            {
                let previous_state = if key_count == 0 { 0 } else { 1 << 30 };
                if !post_key_message(
                    window,
                    WM_KEYDOWN,
                    1 | (A_SCAN_CODE << 16) | previous_state,
                    &thread_cancelled,
                ) {
                    break;
                }
                key_count += 1;
                if !sleep_while_running(Duration::from_millis(17), &thread_cancelled) {
                    break;
                }
            }

            flood_running.store(false, Ordering::Release);
            if flood.join().is_err() {
                eprintln!("message-flood thread panicked");
            }

            if !thread_cancelled.load(Ordering::Acquire) {
                post_key_message(
                    window,
                    WM_KEYUP,
                    1 | (A_SCAN_CODE << 16) | (1 << 30) | (1 << 31),
                    &thread_cancelled,
                );
            }
        });

        StressController {
            cancelled,
            thread: Some(thread),
        }
    }

    struct PresentStarvation {
        focus_handle: FocusHandle,
        key_count: usize,
        _stress_controller: Option<StressController>,
    }

    impl PresentStarvation {
        fn new(
            window: &mut Window,
            cx: &mut Context<Self>,
            stress_controller: Option<StressController>,
        ) -> Self {
            let focus_handle = cx.focus_handle();
            window.focus(&focus_handle, cx);
            Self {
                focus_handle,
                key_count: 0,
                _stress_controller: stress_controller,
            }
        }
    }

    impl Render for PresentStarvation {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|this, _, _, cx| {
                    this.key_count += 1;
                    cx.notify();
                }))
                .size_full()
                .flex()
                .flex_col()
                .justify_center()
                .items_center()
                .gap_2()
                .bg(if self.key_count.is_multiple_of(2) {
                    rgb(0x173b57)
                } else {
                    rgb(0x8a2d2d)
                })
                .text_color(rgb(0xffffff))
                .child("Windows present-starvation reproduction")
                .child(format!("Dispatched keys: {}", self.key_count))
                .child("The baseline freezes during the eight-second stress, then catches up.")
        }
    }

    pub fn run() {
        application().run(|cx: &mut App| {
            cx.on_window_closed(|cx, _window_id| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            let result = cx.open_window(WindowOptions::default(), |window, cx| {
                let stress_controller = if let Ok(handle) =
                    raw_window_handle::HasWindowHandle::window_handle(window)
                    && let RawWindowHandle::Win32(handle) = handle.as_raw()
                {
                    let native_window = handle.hwnd.get();
                    let stress_controller = start_stress(native_window);
                    let cancelled = stress_controller.cancelled.clone();
                    window.on_window_should_close(cx, move |_, _| {
                        cancelled.store(true, Ordering::Release);
                        true
                    });
                    Some(stress_controller)
                } else {
                    None
                };
                cx.new(|cx| PresentStarvation::new(window, cx, stress_controller))
            });
            if let Err(error) = result {
                eprintln!("failed to open reproduction window: {error:#}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
    }
}

#[cfg(target_os = "windows")]
fn main() {
    windows_repro::run();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("present_starvation is a Windows-only reproduction");
}
