use std::{
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use gpui_util::ResultExt;
use smallvec::SmallVec;
use windows::Win32::{
    Foundation::HWND,
    Graphics::Dwm::{DWM_TIMING_INFO, DwmFlush, DwmGetCompositionTimingInfo},
    System::Performance::QueryPerformanceFrequency,
};

use crate::SafeHwnd;

#[derive(Clone)]
pub(crate) struct FrameRequestSender(mpsc::Sender<FrameRequest>);

pub(crate) struct FrameRequestReceiver {
    receiver: mpsc::Receiver<FrameRequest>,
    first_request: Option<FrameRequest>,
}

#[derive(Clone)]
pub(crate) struct FrameRequester {
    hwnd: SafeHwnd,
    state: Arc<FrameRequestState>,
    sender: FrameRequestSender,
}

struct FrameRequestState {
    queued: AtomicBool,
    closed: AtomicBool,
}

pub(crate) struct FrameRequest {
    hwnd: SafeHwnd,
    state: Arc<FrameRequestState>,
}

pub(crate) fn frame_request_channel() -> (FrameRequestSender, FrameRequestReceiver) {
    let (sender, receiver) = mpsc::channel();
    (
        FrameRequestSender(sender),
        FrameRequestReceiver {
            receiver,
            first_request: None,
        },
    )
}

impl FrameRequestSender {
    pub(crate) fn requester_for(&self, hwnd: SafeHwnd) -> FrameRequester {
        FrameRequester {
            hwnd,
            state: Arc::new(FrameRequestState {
                queued: AtomicBool::new(false),
                closed: AtomicBool::new(false),
            }),
            sender: self.clone(),
        }
    }
}

impl FrameRequester {
    pub(crate) fn request(&self) {
        if self.state.closed.load(Ordering::Acquire)
            || self.state.queued.swap(true, Ordering::AcqRel)
        {
            return;
        }

        let request = FrameRequest {
            hwnd: self.hwnd,
            state: self.state.clone(),
        };
        if self.sender.0.send(request).is_err() {
            self.state.queued.store(false, Ordering::Release);
        }
    }

    pub(crate) fn close(&self) {
        self.state.closed.store(true, Ordering::Release);
    }
}

impl FrameRequest {
    pub(crate) fn hwnd_if_open(&self) -> Option<SafeHwnd> {
        (!self.state.closed.load(Ordering::Acquire)).then_some(self.hwnd)
    }
}

impl FrameRequestReceiver {
    pub(crate) fn wait(&mut self) -> bool {
        match self.receiver.recv() {
            Ok(request) => {
                self.first_request = Some(request);
                true
            }
            Err(_) => false,
        }
    }

    pub(crate) fn take_requested_windows(&mut self) -> SmallVec<[FrameRequest; 4]> {
        let requests = self
            .first_request
            .take()
            .into_iter()
            .chain(self.receiver.try_iter());
        requests
            .filter_map(|request| {
                request.state.queued.store(false, Ordering::Release);
                if request.state.closed.load(Ordering::Acquire) {
                    return None;
                }
                Some(request)
            })
            .collect()
    }
}

static QPC_TICKS_PER_SECOND: LazyLock<u64> = LazyLock::new(|| {
    let mut frequency = 0;
    // On systems that run Windows XP or later, the function will always succeed and
    // will thus never return zero.
    unsafe { QueryPerformanceFrequency(&mut frequency).unwrap() };
    frequency as u64
});

const VSYNC_INTERVAL_THRESHOLD: Duration = Duration::from_millis(1);
const DEFAULT_VSYNC_INTERVAL: Duration = Duration::from_micros(16_666); // ~60Hz

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool>,
}

impl VSyncProvider {
    pub(crate) fn new() -> Self {
        let interval = get_dwm_interval()
            .context("Failed to get DWM interval")
            .log_err()
            .unwrap_or(DEFAULT_VSYNC_INTERVAL);
        let f = Box::new(|| unsafe { DwmFlush().is_ok() });
        Self { interval, f }
    }

    pub(crate) fn wait_for_vsync(&self) {
        let vsync_start = Instant::now();
        let wait_succeeded = (self.f)();
        let elapsed = vsync_start.elapsed();
        // DwmFlush and DCompositionWaitForCompositorClock returns very early
        // instead of waiting until vblank when the monitor goes to sleep or is
        // unplugged (nothing to present due to desktop occlusion). We use 1ms as
        // a threshold for the duration of the wait functions and fallback to
        // Sleep() if it returns before that. This could happen during normal
        // operation for the first call after the vsync thread becomes non-idle,
        // but it shouldn't happen often.
        if !wait_succeeded || elapsed < VSYNC_INTERVAL_THRESHOLD {
            log::trace!("VSyncProvider::wait_for_vsync() took less time than expected");
            std::thread::sleep(self.interval);
        }
    }
}

fn get_dwm_interval() -> Result<Duration> {
    let mut timing_info = DWM_TIMING_INFO {
        cbSize: std::mem::size_of::<DWM_TIMING_INFO>() as u32,
        ..Default::default()
    };
    unsafe { DwmGetCompositionTimingInfo(HWND::default(), &mut timing_info) }?;
    let interval = retrieve_duration(timing_info.qpcRefreshPeriod, *QPC_TICKS_PER_SECOND);
    // Check for interval values that are impossibly low. A 29 microsecond
    // interval was seen (from a qpcRefreshPeriod of 60).
    if interval < VSYNC_INTERVAL_THRESHOLD {
        Ok(retrieve_duration(
            timing_info.rateRefresh.uiDenominator as u64,
            timing_info.rateRefresh.uiNumerator as u64,
        ))
    } else {
        Ok(interval)
    }
}

#[inline]
fn retrieve_duration(counts: u64, ticks_per_second: u64) -> Duration {
    let ticks_per_microsecond = ticks_per_second / 1_000_000;
    Duration::from_micros(counts / ticks_per_microsecond)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hwnd(value: isize) -> SafeHwnd {
        HWND(value as _).into()
    }

    #[test]
    fn frame_request_receiver_waits_for_demand() {
        let (sender, mut receiver) = frame_request_channel();
        let requester = sender.requester_for(test_hwnd(1));
        let (completed_sender, completed_receiver) = mpsc::sync_channel(1);

        std::thread::spawn(move || {
            let received = receiver.wait();
            completed_sender
                .send((received, receiver.take_requested_windows().len()))
                .unwrap();
        });

        assert!(matches!(
            completed_receiver.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));
        requester.request();
        assert_eq!(
            completed_receiver.recv_timeout(Duration::from_secs(1)),
            Ok((true, 1))
        );
    }

    #[test]
    fn frame_requester_coalesces_until_the_request_is_taken() {
        let (sender, mut receiver) = frame_request_channel();
        let requester = sender.requester_for(test_hwnd(1));

        requester.request();
        requester.request();

        assert!(receiver.wait());
        let requested_windows = receiver.take_requested_windows();
        assert_eq!(requested_windows.len(), 1);
        assert_eq!(
            requested_windows[0]
                .hwnd_if_open()
                .map(|hwnd| hwnd.as_raw()),
            Some(test_hwnd(1).as_raw())
        );

        requester.request();
        assert!(receiver.wait());
        assert_eq!(receiver.take_requested_windows().len(), 1);
    }

    #[test]
    fn frame_request_receiver_batches_distinct_windows_before_vsync() {
        let (sender, mut receiver) = frame_request_channel();
        let first = sender.requester_for(test_hwnd(1));
        let second = sender.requester_for(test_hwnd(2));

        first.request();
        assert!(receiver.wait());
        second.request();

        let requested_windows = receiver.take_requested_windows();
        assert_eq!(requested_windows.len(), 2);
        assert_eq!(
            requested_windows[0]
                .hwnd_if_open()
                .map(|hwnd| hwnd.as_raw()),
            Some(test_hwnd(1).as_raw())
        );
        assert_eq!(
            requested_windows[1]
                .hwnd_if_open()
                .map(|hwnd| hwnd.as_raw()),
            Some(test_hwnd(2).as_raw())
        );
    }

    #[test]
    fn closed_frame_requester_discards_its_pending_request() {
        let (sender, mut receiver) = frame_request_channel();
        let requester = sender.requester_for(test_hwnd(1));

        requester.request();
        assert!(receiver.wait());
        requester.close();

        assert!(receiver.take_requested_windows().is_empty());
    }

    #[test]
    fn closed_frame_requester_ignores_new_requests() {
        let (sender, mut receiver) = frame_request_channel();
        let requester = sender.requester_for(test_hwnd(1));

        requester.close();
        requester.request();
        drop(requester);
        drop(sender);

        assert!(!receiver.wait());
    }

    #[test]
    fn closing_after_take_cancels_dispatch() {
        let (sender, mut receiver) = frame_request_channel();
        let requester = sender.requester_for(test_hwnd(1));

        requester.request();
        assert!(receiver.wait());
        let requested_windows = receiver.take_requested_windows();
        assert_eq!(requested_windows.len(), 1);

        requester.close();

        assert!(requested_windows[0].hwnd_if_open().is_none());
    }

    #[test]
    fn closed_request_does_not_target_a_new_window_with_the_same_hwnd() {
        let (sender, mut receiver) = frame_request_channel();
        let closed_window = sender.requester_for(test_hwnd(1));
        let new_window = sender.requester_for(test_hwnd(1));

        closed_window.request();
        assert!(receiver.wait());
        closed_window.close();
        new_window.request();

        let requested_windows = receiver.take_requested_windows();
        assert_eq!(requested_windows.len(), 1);
        assert!(Arc::ptr_eq(&requested_windows[0].state, &new_window.state));
        assert_eq!(
            requested_windows[0]
                .hwnd_if_open()
                .map(|hwnd| hwnd.as_raw()),
            Some(test_hwnd(1).as_raw())
        );
    }
}
