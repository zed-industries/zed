use anyhow::{anyhow, Context, Result};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{TCFType, TCFTypeRef},
    dictionary::CFDictionary,
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use core_graphics::window::{
    kCGNullWindowID, kCGWindowListOptionExcludeDesktopElements, kCGWindowListOptionOnScreenOnly,
    kCGWindowNumber, kCGWindowOwnerName, kCGWindowOwnerPID, CGWindowListCopyWindowInfo,
};
use futures::{
    channel::{mpsc, oneshot},
    Future,
};
use media::core_video::{CVImageBuffer, CVImageBufferRef};
use parking_lot::Mutex;
use std::{
    ffi::c_void,
    sync::{Arc, Weak},
};

extern "C" {
    fn LKRelease(object: *const c_void);

    fn LKRoomDelegateCreate(
        callback_data: *mut c_void,
        on_did_subscribe_to_remote_video_track: extern "C" fn(
            callback_data: *mut c_void,
            remote_track: *const c_void,
        ),
    ) -> *const c_void;

    fn LKRoomCreate(delegate: *const c_void) -> *const c_void;
    fn LKRoomConnect(
        room: *const c_void,
        url: CFStringRef,
        token: CFStringRef,
        callback: extern "C" fn(*mut c_void, CFStringRef),
        callback_data: *mut c_void,
    );
    fn LKRoomPublishVideoTrack(
        room: *const c_void,
        track: *const c_void,
        callback: extern "C" fn(*mut c_void, CFStringRef),
        callback_data: *mut c_void,
    );

    fn LKVideoRendererCreate(
        callback_data: *mut c_void,
        on_frame: extern "C" fn(callback_data: *mut c_void, frame: CVImageBufferRef),
        on_drop: extern "C" fn(callback_data: *mut c_void),
    ) -> *const c_void;

    fn LKVideoTrackAddRenderer(track: *const c_void, renderer: *const c_void);

    fn LKCreateScreenShareTrackForWindow(windowId: u32) -> *const c_void;
    fn LKDisplaySources(
        callback_data: *mut c_void,
        callback: extern "C" fn(
            callback_data: *mut c_void,
            sources: CFArrayRef,
            error: CFStringRef,
        ),
    );
}

pub struct Room {
    native_room: *const c_void,
    remote_video_track_subscribers: Mutex<Vec<mpsc::UnboundedSender<Arc<RemoteVideoTrack>>>>,
    _delegate: RoomDelegate,
}

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new_cyclic(|weak_room| {
            let delegate = RoomDelegate::new(weak_room.clone());
            Self {
                native_room: unsafe { LKRoomCreate(delegate.native_delegate) },
                remote_video_track_subscribers: Default::default(),
                _delegate: delegate,
            }
        })
    }

    pub fn connect(&self, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        let url = CFString::new(url);
        let token = CFString::new(token);
        let (did_connect, tx, rx) = Self::build_done_callback();
        unsafe {
            LKRoomConnect(
                self.native_room,
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
            LKRoomPublishVideoTrack(self.native_room, track.0, did_publish, tx);
        }
        async { rx.await.unwrap().context("error publishing video track") }
    }

    pub fn remote_video_tracks(&self) -> mpsc::UnboundedReceiver<Arc<RemoteVideoTrack>> {
        let (tx, rx) = mpsc::unbounded();
        self.remote_video_track_subscribers.lock().push(tx);
        rx
    }

    fn did_subscribe_to_remote_video_track(&self, track: RemoteVideoTrack) {
        let track = Arc::new(track);
        self.remote_video_track_subscribers
            .lock()
            .retain(|tx| tx.unbounded_send(track.clone()).is_ok());
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
        unsafe { LKRelease(self.native_room) }
    }
}

struct RoomDelegate {
    native_delegate: *const c_void,
    weak_room: *const Room,
}

impl RoomDelegate {
    fn new(weak_room: Weak<Room>) -> Self {
        let weak_room = Weak::into_raw(weak_room);
        let native_delegate = unsafe {
            LKRoomDelegateCreate(
                weak_room as *mut c_void,
                Self::on_did_subscribe_to_remote_video_track,
            )
        };
        Self {
            native_delegate,
            weak_room,
        }
    }

    extern "C" fn on_did_subscribe_to_remote_video_track(room: *mut c_void, track: *const c_void) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let track = RemoteVideoTrack(track);
        if let Some(room) = room.upgrade() {
            room.did_subscribe_to_remote_video_track(track);
        }
        let _ = Weak::into_raw(room);
    }
}

impl Drop for RoomDelegate {
    fn drop(&mut self) {
        unsafe {
            LKRelease(self.native_delegate);
            let _ = Weak::from_raw(self.weak_room);
        }
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

pub struct RemoteVideoTrack(*const c_void);

impl RemoteVideoTrack {
    pub fn add_renderer<F>(&self, callback: F)
    where
        F: 'static + FnMut(CVImageBuffer),
    {
        extern "C" fn on_frame<F>(callback_data: *mut c_void, frame: CVImageBufferRef)
        where
            F: FnMut(CVImageBuffer),
        {
            unsafe {
                let buffer = CVImageBuffer::wrap_under_get_rule(frame);
                let callback = &mut *(callback_data as *mut F);
                callback(buffer);
            }
        }

        extern "C" fn on_drop<F>(callback_data: *mut c_void) {
            unsafe {
                let _ = Box::from_raw(callback_data as *mut F);
            }
        }

        let callback_data = Box::into_raw(Box::new(callback));
        unsafe {
            let renderer =
                LKVideoRendererCreate(callback_data as *mut c_void, on_frame::<F>, on_drop::<F>);
            LKVideoTrackAddRenderer(self.0, renderer);
        }
    }
}

impl Drop for RemoteVideoTrack {
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

pub struct MacOSDisplay(*const c_void);

pub fn display_sources() -> impl Future<Output = Result<Vec<MacOSDisplay>>> {
    extern "C" fn callback(tx: *mut c_void, sources: CFArrayRef, error: CFStringRef) {
        unsafe {
            let tx = Box::from_raw(tx as *mut oneshot::Sender<Result<Vec<MacOSDisplay>>>);

            if sources.is_null() {
                let _ = tx.send(Err(anyhow!("{}", CFString::wrap_under_get_rule(error))));
            } else {
                let sources = CFArray::wrap_under_get_rule(sources);
                let sources = sources
                    .into_iter()
                    .map(|source| MacOSDisplay(*source))
                    .collect();
                let _ = tx.send(Ok(sources));
            }
        }
    }

    let (tx, rx) = oneshot::channel();

    unsafe {
        LKDisplaySources(Box::into_raw(Box::new(tx)) as *mut _, callback);
    }

    async move { rx.await.unwrap() }
}
