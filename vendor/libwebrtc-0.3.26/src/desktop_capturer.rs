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

use crate::imp::desktop_capturer as imp_dc;

/// Configuration options for creating a desktop capturer.
///
/// It contains a subset of libwebrtc's DesktopCaptureOptions.
///
/// By default, it captures the entire screen and does not include the cursor.
///
/// # Example
/// ```no_run
/// use libwebrtc::desktop_capturer::{DesktopCapturerOptions, DesktopCaptureSourceType};
///
/// let mut options = DesktopCapturerOptions::new(DesktopCaptureSourceType::Screen);
/// options.set_include_cursor(true);
/// ```
pub struct DesktopCapturerOptions {
    sys_handle: imp_dc::DesktopCapturerOptions,
}

/// Specifies the type of source that a desktop capturer should capture.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DesktopCaptureSourceType {
    Screen,
    Window,
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    Generic,
}

impl DesktopCapturerOptions {
    /// Creates a new `DesktopCapturerOptions` with default values.
    ///
    /// # Arguments
    ///
    /// * `source_type` - The type of source to capture (screen or window).
    ///
    /// # Defaults
    ///
    /// - Cursor is not included in captured frames (use [`set_include_cursor`](Self::set_include_cursor) to change)
    /// - On macOS, the ScreenCaptureKit system picker is enabled (use [`set_sck_system_picker`](Self::set_sck_system_picker) to change)
    pub fn new(source_type: DesktopCaptureSourceType) -> Self {
        let source_type = match source_type {
            DesktopCaptureSourceType::Screen => imp_dc::SourceType::Screen,
            DesktopCaptureSourceType::Window => imp_dc::SourceType::Window,
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            DesktopCaptureSourceType::Generic => imp_dc::SourceType::Generic,
        };
        Self { sys_handle: imp_dc::DesktopCapturerOptions::new(source_type) }
    }

    /// Sets whether to include the cursor in captured frames.
    pub fn set_include_cursor(&mut self, include: bool) {
        self.sys_handle = self.sys_handle.with_cursor(include);
    }

    /// Sets whether to allow the ScreenCaptureKit system picker on macOS.
    ///
    /// This is enabled by default.
    ///
    /// When disabled, for capturing displays the client should get the source id
    /// via a different way as [`DesktopCapturer::get_source_list`] returns an empty vector.
    #[cfg(target_os = "macos")]
    pub fn set_sck_system_picker(&mut self, allow_sck_system_picker: bool) {
        self.sys_handle = self.sys_handle.with_sck_system_picker(allow_sck_system_picker);
    }
}

/// A desktop capturer for capturing screens or windows.
pub struct DesktopCapturer {
    handle: imp_dc::DesktopCapturer,
}

impl DesktopCapturer {
    /// Creates a new `DesktopCapturer` with the specified callback and options.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the capturer
    ///
    /// # Returns
    ///
    /// Returns `Some(DesktopCapturer)` if the capturer was created successfully,
    /// or `None` if creation failed (e.g., due to platform limitations or permissions).
    pub fn new(options: DesktopCapturerOptions) -> Option<Self> {
        let desktop_capturer = imp_dc::DesktopCapturer::new(options.sys_handle);
        if desktop_capturer.is_none() {
            return None;
        }
        Some(Self { handle: desktop_capturer.unwrap() })
    }

    /// Starts capturing from the specified source.
    ///
    /// # Arguments
    ///
    /// * `source` - The capture source to use. It should be None when the capturer
    ///   is configured to use the system picker (on platforms that support it).
    /// * `callback` - A function that will be called for each captured frame. The callback
    ///   receives a [`CaptureResult`] indicating success or error, and a [`DesktopFrame`]
    ///   containing the captured image data.
    ///
    /// # Note
    ///
    /// After calling this method, you must call [`capture_frame`](Self::capture_frame)
    /// to actually capture frames. This method only initializes the capture session.
    pub fn start_capture<T>(&mut self, source: Option<CaptureSource>, mut callback: T)
    where
        T: FnMut(Result<DesktopFrame, CaptureError>) + Send + 'static,
    {
        if let Some(source) = source {
            self.handle.select_source(source.sys_handle.id());
        }
        let inner_callback = move |result: Result<imp_dc::DesktopFrame, imp_dc::CaptureError>| {
            callback(capture_result_from_sys(result));
        };
        self.handle.start(inner_callback);
    }

    /// Captures a single frame.
    ///
    /// You must call [`start_capture`](Self::start_capture) before calling this method.
    pub fn capture_frame(&mut self) {
        self.handle.capture_frame();
    }

    /// Retrieves a list of available capture sources.
    ///
    /// Returns a list of screens or windows that can be captured, depending
    /// on whether the capturer was configured for window or screen capture.
    ///
    /// # Returns
    ///
    /// A vector of [`CaptureSource`] objects representing available capture sources.
    pub fn get_source_list(&self) -> Vec<CaptureSource> {
        let source_list = self.handle.get_source_list();
        source_list.into_iter().map(|source| CaptureSource { sys_handle: source }).collect()
    }
}

pub struct DesktopFrame {
    sys_handle: imp_dc::DesktopFrame,
}

impl DesktopFrame {
    fn new(sys_handle: imp_dc::DesktopFrame) -> Self {
        Self { sys_handle }
    }

    pub fn width(&self) -> i32 {
        self.sys_handle.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.sys_handle.height() as i32
    }

    pub fn stride(&self) -> u32 {
        self.sys_handle.stride() as u32
    }

    pub fn left(&self) -> i32 {
        self.sys_handle.left()
    }

    pub fn top(&self) -> i32 {
        self.sys_handle.top()
    }

    pub fn data(&self) -> &[u8] {
        self.sys_handle.data()
    }
}

#[derive(Clone)]
pub struct CaptureSource {
    sys_handle: imp_dc::CaptureSource,
}

impl CaptureSource {
    pub fn id(&self) -> u64 {
        self.sys_handle.id()
    }
    pub fn title(&self) -> String {
        self.sys_handle.title()
    }
    pub fn display_id(&self) -> i64 {
        self.sys_handle.display_id()
    }
}

impl std::fmt::Display for CaptureSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureSource")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("display_id", &self.display_id())
            .finish()
    }
}

#[derive(Debug, PartialEq)]
pub enum CaptureError {
    Temporary,
    Permanent,
}

fn capture_result_from_sys(
    result: Result<imp_dc::DesktopFrame, imp_dc::CaptureError>,
) -> Result<DesktopFrame, CaptureError> {
    match result {
        Ok(frame) => Ok(DesktopFrame::new(frame)),
        Err(error) => Err(match error {
            imp_dc::CaptureError::Temporary => CaptureError::Temporary,
            imp_dc::CaptureError::Permanent => CaptureError::Permanent,
        }),
    }
}
