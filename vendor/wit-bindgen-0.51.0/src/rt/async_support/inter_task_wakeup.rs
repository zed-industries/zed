use super::FutureState;
use crate::rt::async_support::{BLOCKED, COMPLETED};
use crate::{RawStreamReader, RawStreamWriter, StreamOps, UnitStreamOps};
use std::ptr;
use std::sync::Mutex;

#[derive(Default)]
pub struct State {
    /// A lazily-initialized stream used to signal inter-task notifications.
    ///
    /// This stream is used when one component-model task is used to wake up
    /// another component-model task. This can happen when the async event being
    /// waited on is defined purely in Rust, for example, and doesn't rely on
    /// any component-model primitives.
    stream: Option<RawStreamReader<UnitStreamOps>>,
    /// Boolean if there's an active read of `inter_task_stream`. Used to handle
    /// cancellation/deduplication of the read.
    stream_reading: bool,
}

impl FutureState<'_> {
    pub(super) fn read_inter_task_stream(&mut self) {
        // Lazily allocate the inter-task stream now that we're actually going
        // to sleep. We don't know where the wakeup notification will come from
        // so it's required to allocate one here.
        if self.inter_task_wakeup.stream.is_none() {
            assert!(!self.inter_task_wakeup.stream_reading);
            let (writer, reader) = UnitStreamOps::new();
            self.inter_task_wakeup.stream = Some(reader);
            let mut waker_stream = self.waker.inter_task_stream.lock.lock().unwrap();
            assert!(waker_stream.is_none());
            *waker_stream = Some(writer);
        }

        // If there's not already a pending read then schedule a new read here.
        //
        // Note that this should always return `BLOCKED` since as the only task
        // running it's not possible for a read to be anywhere else in the
        // system. Additionally we keep the read end alive, so this shouldn't
        // ever returned dropped/closed either.
        if !self.inter_task_wakeup.stream_reading {
            let stream = self.inter_task_wakeup.stream.as_mut().unwrap();
            let handle = stream.handle();
            let rc = unsafe { UnitStreamOps.start_read(handle, ptr::null_mut(), 1) };
            assert_eq!(rc, BLOCKED);
            self.inter_task_wakeup.stream_reading = true;
            self.add_waitable(handle);
        }
    }

    /// Cancels the active read of the inter-task stream, if any.
    ///
    /// Has no effect if there is no active read.
    pub(super) fn cancel_inter_task_stream_read(&mut self) {
        if !self.inter_task_wakeup.stream_reading {
            return;
        }
        self.inter_task_wakeup.stream_reading = false;
        let handle = self.inter_task_wakeup.stream.as_mut().unwrap().handle();
        // Note that the return code here is discarded. No matter what the read
        // is cancelled, and whether we actually read something or whether we
        // cancelled doesn't matter.
        unsafe {
            UnitStreamOps.cancel_read(handle);
        }
        self.remove_waitable(handle);
    }
}

impl State {
    pub fn consume_waitable_event(&mut self, waitable: u32, _code: u32) -> bool {
        if let Some(reader) = self.stream.as_mut() {
            if reader.handle() == waitable {
                self.stream_reading = false;
                return true;
            }
        }
        false
    }
}

#[derive(Default)]
pub struct WakerState {
    lock: Mutex<Option<RawStreamWriter<UnitStreamOps>>>,
}

impl WakerState {
    pub fn wake(&self) {
        // Here the wakeup stream should already have been filled in by the
        // original future itself. The stream should also have an active read
        // while the future is sleeping. This means that this write should
        // succeed immediately.
        let mut inter_task_stream = self.lock.lock().unwrap();
        let stream = inter_task_stream.as_mut().unwrap();
        let rc = unsafe { UnitStreamOps.start_write(stream.handle(), ptr::null_mut(), 1) };
        assert_eq!(rc, COMPLETED | (1 << 4));
    }
}
