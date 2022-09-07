use anyhow::{anyhow, Context, Result};
use core_foundation::{
    array::CFArray,
    base::{TCFType, TCFTypeRef},
    dictionary::CFDictionary,
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use core_graphics::window::{
    kCGNullWindowID, kCGWindowListOptionExcludeDesktopElements, kCGWindowListOptionOnScreenOnly,
    kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID, CGWindowListCopyWindowInfo,
};
use futures::{channel::oneshot, Future};
use std::ffi::c_void;

extern "C" {
    fn LKRelease(object: *const c_void);

    fn LKRoomCreate() -> *const c_void;
    fn LKRoomConnect(
        room: *const c_void,
        url: CFStringRef,
        token: CFStringRef,
        callback: extern "C" fn(*mut c_void, CFStringRef) -> (),
        callback_data: *mut c_void,
    );
    fn LKRoomPublishVideoTrack(
        room: *const c_void,
        track: *const c_void,
        callback: extern "C" fn(*mut c_void, CFStringRef) -> (),
        callback_data: *mut c_void,
    );

    fn LKCreateScreenShareTrackForWindow(windowId: u32) -> *const c_void;
}

pub struct Room(*const c_void);

impl Room {
    pub fn new() -> Self {
        Self(unsafe { LKRoomCreate() })
    }

    pub fn connect(&self, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        let url = CFString::new(url);
        let token = CFString::new(token);
        let (did_connect, tx, rx) = Self::build_done_callback();
        unsafe {
            LKRoomConnect(
                self.0,
                url.as_concrete_TypeRef(),
                token.as_concrete_TypeRef(),
                did_connect,
                tx,
            )
        }

        async { rx.await.unwrap().context("error connecting to room") }
    }

    pub fn publish_video_track(&self, track: &LocalVideoTrack) -> impl Future<Output = Result<()>> {
        let (did_publish, tx, rx) = Self::build_done_callback();
        unsafe {
            LKRoomPublishVideoTrack(self.0, track.0, did_publish, tx);
        }
        async { rx.await.unwrap().context("error publishing video track") }
    }

    fn build_done_callback() -> (
        extern "C" fn(*mut c_void, CFStringRef),
        *mut c_void,
        oneshot::Receiver<Result<()>>,
    ) {
        let (tx, rx) = oneshot::channel();
        extern "C" fn done_callback(tx: *mut c_void, error: CFStringRef) {
            let tx = unsafe { Box::from_raw(tx as *mut oneshot::Sender<Result<()>>) };
            if error.is_null() {
                let _ = tx.send(Ok(()));
            } else {
                let error = unsafe { CFString::wrap_under_get_rule(error).to_string() };
                let _ = tx.send(Err(anyhow!(error)));
            }
        }
        (
            done_callback,
            Box::into_raw(Box::new(tx)) as *mut c_void,
            rx,
        )
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        unsafe { LKRelease(self.0) }
    }
}

pub struct LocalVideoTrack(*const c_void);

impl LocalVideoTrack {
    pub fn screen_share_for_window(window_id: u32) -> Self {
        Self(unsafe { LKCreateScreenShareTrackForWindow(window_id) })
    }
}

impl Drop for LocalVideoTrack {
    fn drop(&mut self) {
        unsafe { LKRelease(self.0) }
    }
}

#[derive(Debug)]
pub struct WindowInfo {
    pub id: u32,
    pub owner_pid: i32,
    pub owner_name: Option<String>,
}

pub fn list_windows() -> Vec<WindowInfo> {
    unsafe {
        let dicts = CFArray::<CFDictionary>::wrap_under_get_rule(CGWindowListCopyWindowInfo(
            kCGWindowListOptionOnScreenOnly | kCGWindowListOptionExcludeDesktopElements,
            kCGNullWindowID,
        ));

        dicts
            .iter()
            .map(|dict| {
                let id =
                    CFNumber::wrap_under_get_rule(*dict.get(kCGWindowNumber.as_void_ptr()) as _)
                        .to_i64()
                        .unwrap() as u32;

                let owner_pid =
                    CFNumber::wrap_under_get_rule(*dict.get(kCGWindowOwnerPID.as_void_ptr()) as _)
                        .to_i32()
                        .unwrap();

                let owner_name = dict
                    .find(kCGWindowOwnerName.as_void_ptr())
                    .map(|name| CFString::wrap_under_get_rule(*name as _).to_string());
                WindowInfo {
                    id,
                    owner_pid,
                    owner_name,
                }
            })
            .collect()
    }
}
