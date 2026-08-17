// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cxx::UniquePtr;
use ffi::CaptureResult;

use crate::{desktop_capturer::ffi::DesktopFrame, impl_thread_safety};

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[derive(Clone)]
    struct Source {
        id: u64,
        title: String,
        display_id: i64,
    }

    #[derive(Debug, PartialEq)]
    enum SourceType {
        Screen,
        Window,
        Generic,
    }

    #[derive(Clone, Debug)]
    struct DesktopCapturerOptions {
        source_type: SourceType,
        include_cursor: bool,
        allow_sck_system_picker: bool,
    }

    enum CaptureResult {
        Success,
        ErrorTemporary,
        ErrorPermanent,
    }

    unsafe extern "C++" {
        include!("livekit/desktop_capturer.h");

        type DesktopCapturer;
        type DesktopFrame;

        fn new_desktop_capturer(options: DesktopCapturerOptions) -> UniquePtr<DesktopCapturer>;
        fn capture_frame(self: &DesktopCapturer);
        fn get_source_list(self: &DesktopCapturer) -> Vec<Source>;
        fn select_source(self: &DesktopCapturer, id: u64) -> bool;
        fn start(self: Pin<&mut DesktopCapturer>, callback: Box<DesktopCapturerCallbackWrapper>);

        fn width(self: &DesktopFrame) -> i32;
        fn height(self: &DesktopFrame) -> i32;
        fn stride(self: &DesktopFrame) -> i32;
        fn left(self: &DesktopFrame) -> i32;
        fn top(self: &DesktopFrame) -> i32;
        fn data(self: &DesktopFrame) -> *const u8;
    }

    extern "Rust" {
        type DesktopCapturerCallbackWrapper;

        fn on_capture_result(
            self: &mut DesktopCapturerCallbackWrapper,
            result: CaptureResult,
            frame: UniquePtr<DesktopFrame>,
        );
    }
}

impl_thread_safety!(ffi::DesktopCapturer, Send + Sync);

#[derive(Debug, PartialEq)]
pub enum CaptureError {
    Temporary,
    Permanent,
}

pub trait DesktopCapturerCallback: Send {
    fn on_capture_result(&mut self, result: Result<UniquePtr<DesktopFrame>, CaptureError>);
}

pub struct DesktopCapturerCallbackWrapper {
    callback: Box<dyn DesktopCapturerCallback>,
}

impl DesktopCapturerCallbackWrapper {
    pub fn new(callback: Box<dyn DesktopCapturerCallback>) -> Self {
        Self { callback }
    }

    fn on_capture_result(&mut self, result: CaptureResult, frame: UniquePtr<DesktopFrame>) {
        match result {
            CaptureResult::Success => self.callback.on_capture_result(Ok(frame)),
            CaptureResult::ErrorTemporary => {
                self.callback.on_capture_result(Err(CaptureError::Temporary))
            }
            CaptureResult::ErrorPermanent => {
                self.callback.on_capture_result(Err(CaptureError::Permanent))
            }
            _ => self.callback.on_capture_result(Err(CaptureError::Permanent)),
        }
    }
}
