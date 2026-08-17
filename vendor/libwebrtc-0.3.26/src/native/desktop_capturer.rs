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
use webrtc_sys::desktop_capturer::{self as sys_dc, ffi::new_desktop_capturer};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum SourceType {
    Screen,
    Window,
    Generic,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct DesktopCapturerOptions {
    source_type: SourceType,
    include_cursor: bool,
    #[cfg(target_os = "macos")]
    allow_sck_system_picker: bool,
}

impl Default for DesktopCapturerOptions {
    fn default() -> Self {
        Self {
            source_type: SourceType::Screen,
            include_cursor: false,
            #[cfg(target_os = "macos")]
            allow_sck_system_picker: true,
        }
    }
}

impl DesktopCapturerOptions {
    pub(crate) fn new(source_type: SourceType) -> Self {
        Self { source_type, ..Default::default() }
    }

    pub(crate) fn with_cursor(mut self, include: bool) -> Self {
        self.include_cursor = include;
        self
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn with_sck_system_picker(mut self, allow_sck_system_picker: bool) -> Self {
        self.allow_sck_system_picker = allow_sck_system_picker;
        self
    }

    pub(crate) fn to_sys_handle(&self) -> sys_dc::ffi::DesktopCapturerOptions {
        let source_type = match self.source_type {
            SourceType::Screen => sys_dc::ffi::SourceType::Screen,
            SourceType::Window => sys_dc::ffi::SourceType::Window,
            SourceType::Generic => sys_dc::ffi::SourceType::Generic,
        };
        let mut sys_handle = sys_dc::ffi::DesktopCapturerOptions {
            source_type,
            include_cursor: self.include_cursor,
            allow_sck_system_picker: false,
        };
        #[cfg(target_os = "macos")]
        {
            sys_handle.allow_sck_system_picker = self.allow_sck_system_picker;
        }
        sys_handle
    }
}

pub(crate) struct DesktopCapturer {
    sys_handle: UniquePtr<sys_dc::ffi::DesktopCapturer>,
    #[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "glib-main-loop"))]
    glib_loop: Option<glib::MainLoop>,
}

impl DesktopCapturer {
    pub(crate) fn new(options: DesktopCapturerOptions) -> Option<Self> {
        let sys_handle = new_desktop_capturer(options.to_sys_handle());
        if sys_handle.is_null() {
            None
        } else {
            Some(Self {
                sys_handle,
                #[cfg(all(
                    any(target_os = "linux", target_os = "freebsd"),
                    feature = "glib-main-loop"
                ))]
                glib_loop: None,
            })
        }
    }

    pub(crate) fn capture_frame(&self) {
        self.sys_handle.capture_frame();
    }

    pub(crate) fn start<T>(&mut self, callback: T)
    where
        T: FnMut(Result<DesktopFrame, CaptureError>) + Send + 'static,
    {
        #[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "glib-main-loop"))]
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            let main_loop = glib::MainLoop::new(None, false);
            self.glib_loop = Some(main_loop.clone());
            let _handle = std::thread::spawn(move || {
                main_loop.run();
            });
        }
        let pin_handle = self.sys_handle.pin_mut();
        let callback = DesktopCallback::new(callback);
        let callback_wrapper = sys_dc::DesktopCapturerCallbackWrapper::new(Box::new(callback));
        pin_handle.start(Box::new(callback_wrapper));
    }

    pub(crate) fn select_source(&self, id: u64) -> bool {
        self.sys_handle.select_source(id)
    }

    pub(crate) fn get_source_list(&self) -> Vec<CaptureSource> {
        let mut sources = Vec::new();
        let source_list = self.sys_handle.get_source_list();
        for source in source_list.iter() {
            sources.push(CaptureSource { sys_handle: source.clone() });
        }
        sources
    }
}

#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "glib-main-loop"))]
impl Drop for DesktopCapturer {
    fn drop(&mut self) {
        if let Some(glib_loop) = &self.glib_loop {
            glib_loop.quit();
        }
    }
}

pub(crate) struct DesktopFrame {
    sys_handle: UniquePtr<sys_dc::ffi::DesktopFrame>,
}

impl DesktopFrame {
    fn new(sys_handle: UniquePtr<sys_dc::ffi::DesktopFrame>) -> Self {
        Self { sys_handle }
    }

    pub(crate) fn width(&self) -> i32 {
        self.sys_handle.width()
    }

    pub(crate) fn height(&self) -> i32 {
        self.sys_handle.height()
    }

    pub(crate) fn stride(&self) -> u32 {
        self.sys_handle.stride() as u32
    }

    pub(crate) fn left(&self) -> i32 {
        self.sys_handle.left()
    }

    pub(crate) fn top(&self) -> i32 {
        self.sys_handle.top()
    }

    pub(crate) fn data(&self) -> &[u8] {
        let data = self.sys_handle.data();
        unsafe { std::slice::from_raw_parts(data, self.stride() as usize * self.height() as usize) }
    }
}

struct DesktopCallback<T: FnMut(Result<DesktopFrame, CaptureError>) + Send> {
    callback: T,
}

impl<T> DesktopCallback<T>
where
    T: FnMut(Result<DesktopFrame, CaptureError>) + Send,
{
    fn new(callback: T) -> Self {
        Self { callback }
    }

    fn capture_result_from_sys(
        result: Result<UniquePtr<sys_dc::ffi::DesktopFrame>, sys_dc::CaptureError>,
    ) -> Result<DesktopFrame, CaptureError> {
        match result {
            Ok(frame) => Ok(DesktopFrame::new(frame)),
            Err(error) => Err(match error {
                sys_dc::CaptureError::Temporary => CaptureError::Temporary,
                sys_dc::CaptureError::Permanent => CaptureError::Permanent,
            }),
        }
    }
}

impl<T> sys_dc::DesktopCapturerCallback for DesktopCallback<T>
where
    T: FnMut(Result<DesktopFrame, CaptureError>) + Send,
{
    fn on_capture_result(
        &mut self,
        result: Result<UniquePtr<sys_dc::ffi::DesktopFrame>, sys_dc::CaptureError>,
    ) {
        (self.callback)(DesktopCallback::<T>::capture_result_from_sys(result));
    }
}

#[derive(Clone)]
pub(crate) struct CaptureSource {
    sys_handle: sys_dc::ffi::Source,
}

impl CaptureSource {
    pub(crate) fn id(&self) -> u64 {
        self.sys_handle.id
    }

    pub(crate) fn title(&self) -> String {
        self.sys_handle.title.clone()
    }

    pub(crate) fn display_id(&self) -> i64 {
        self.sys_handle.display_id
    }
}

pub(crate) enum CaptureError {
    Temporary,
    Permanent,
}
