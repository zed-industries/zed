use anyhow::{anyhow, Context, Result};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFRelease, CFRetain, TCFType},
    string::{CFString, CFStringRef},
};
use futures::{
    channel::{mpsc, oneshot},
    Future,
};
pub use media::core_video::CVImageBuffer;
use media::core_video::CVImageBufferRef;
use parking_lot::Mutex;
use postage::watch;
use std::{
    ffi::c_void,
    sync::{Arc, Weak},
};

extern "C" {
    fn LKRoomDelegateCreate(
        callback_data: *mut c_void,
        on_did_disconnect: extern "C" fn(callback_data: *mut c_void),
        on_did_subscribe_to_remote_audio_track: extern "C" fn(
            callback_data: *mut c_void,
            publisher_id: CFStringRef,
            track_id: CFStringRef,
            remote_track: *const c_void,
        ),
        on_did_unsubscribe_from_remote_audio_track: extern "C" fn(
            callback_data: *mut c_void,
            publisher_id: CFStringRef,
            track_id: CFStringRef,
        ),
        on_mute_changed_from_remote_audio_track: extern "C" fn(
            callback_data: *mut c_void,
            track_id: CFStringRef,
            muted: bool,
        ),
        on_active_speakers_changed: extern "C" fn(
            callback_data: *mut c_void,
            participants: CFArrayRef,
        ),
        on_did_subscribe_to_remote_video_track: extern "C" fn(
            callback_data: *mut c_void,
            publisher_id: CFStringRef,
            track_id: CFStringRef,
            remote_track: *const c_void,
        ),
        on_did_unsubscribe_from_remote_video_track: extern "C" fn(
            callback_data: *mut c_void,
            publisher_id: CFStringRef,
            track_id: CFStringRef,
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
    fn LKRoomDisconnect(room: *const c_void);
    fn LKRoomPublishVideoTrack(
        room: *const c_void,
        track: *const c_void,
        callback: extern "C" fn(*mut c_void, *mut c_void, CFStringRef),
        callback_data: *mut c_void,
    );
    fn LKRoomPublishAudioTrack(
        room: *const c_void,
        track: *const c_void,
        callback: extern "C" fn(*mut c_void, *mut c_void, CFStringRef),
        callback_data: *mut c_void,
    );
    fn LKRoomUnpublishTrack(room: *const c_void, publication: *const c_void);
    fn LKRoomAudioTracksForRemoteParticipant(
        room: *const c_void,
        participant_id: CFStringRef,
    ) -> CFArrayRef;

    fn LKRoomAudioTrackPublicationsForRemoteParticipant(
        room: *const c_void,
        participant_id: CFStringRef,
    ) -> CFArrayRef;

    fn LKRoomVideoTracksForRemoteParticipant(
        room: *const c_void,
        participant_id: CFStringRef,
    ) -> CFArrayRef;

    fn LKVideoRendererCreate(
        callback_data: *mut c_void,
        on_frame: extern "C" fn(callback_data: *mut c_void, frame: CVImageBufferRef) -> bool,
        on_drop: extern "C" fn(callback_data: *mut c_void),
    ) -> *const c_void;

    fn LKRemoteAudioTrackGetSid(track: *const c_void) -> CFStringRef;
    fn LKVideoTrackAddRenderer(track: *const c_void, renderer: *const c_void);
    fn LKRemoteVideoTrackGetSid(track: *const c_void) -> CFStringRef;

    fn LKAudioInputSources(
        callback_data: *mut c_void,
        callback: extern "C" fn(
            callback_data: *mut c_void,
            name: CFStringRef,
            is_default: bool,
            native_device: *const c_void,
        ),
    ) -> *mut c_void;

    fn LKDisplaySources(
        callback_data: *mut c_void,
        callback: extern "C" fn(
            callback_data: *mut c_void,
            sources: CFArrayRef,
            error: CFStringRef,
        ),
    );
    fn LKCreateScreenShareTrackForDisplay(display: *const c_void) -> *const c_void;
    fn LKLocalAudioTrackCreateTrack() -> *const c_void;

    fn LKLocalTrackPublicationSetMute(
        publication: *const c_void,
        muted: bool,
        on_complete: extern "C" fn(callback_data: *mut c_void, error: CFStringRef),
        callback_data: *mut c_void,
    );

    fn LKRemoteTrackPublicationSetEnabled(
        publication: *const c_void,
        enabled: bool,
        on_complete: extern "C" fn(callback_data: *mut c_void, error: CFStringRef),
        callback_data: *mut c_void,
    );
}

pub type Sid = String;

#[derive(Clone, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connected { url: String, token: String },
}

pub struct Room {
    native_room: *const c_void,
    connection: Mutex<(
        watch::Sender<ConnectionState>,
        watch::Receiver<ConnectionState>,
    )>,
    remote_audio_track_subscribers: Mutex<Vec<mpsc::UnboundedSender<RemoteAudioTrackUpdate>>>,
    remote_video_track_subscribers: Mutex<Vec<mpsc::UnboundedSender<RemoteVideoTrackUpdate>>>,
    _delegate: RoomDelegate,
}

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new_cyclic(|weak_room| {
            let delegate = RoomDelegate::new(weak_room.clone());
            Self {
                native_room: unsafe { LKRoomCreate(delegate.native_delegate) },
                connection: Mutex::new(watch::channel_with(ConnectionState::Disconnected)),
                remote_audio_track_subscribers: Default::default(),
                remote_video_track_subscribers: Default::default(),
                _delegate: delegate,
            }
        })
    }

    pub fn status(&self) -> watch::Receiver<ConnectionState> {
        self.connection.lock().1.clone()
    }

    pub fn connect(self: &Arc<Self>, url: &str, token: &str) -> impl Future<Output = Result<()>> {
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

        let this = self.clone();
        let url = url.to_string();
        let token = token.to_string();
        async move {
            rx.await.unwrap().context("error connecting to room")?;
            *this.connection.lock().0.borrow_mut() = ConnectionState::Connected { url, token };
            Ok(())
        }
    }

    fn did_disconnect(&self) {
        *self.connection.lock().0.borrow_mut() = ConnectionState::Disconnected;
    }

    pub fn audio_input_sources() -> Vec<AudioDevice> {
        let input_sources = Vec::<AudioDevice>::new();
        unsafe {
            extern "C" fn callback(
                vec: *mut c_void,
                name: CFStringRef,
                is_default: bool,
                native_device: *const c_void,
            ) {
                unsafe {
                    let mut input_sources = Box::from_raw(vec as *mut Vec<AudioDevice>);
                    let name = CFString::wrap_under_get_rule(name).to_string();
                    input_sources.push(AudioDevice::new(name, is_default, native_device));
                    let _ = Box::into_raw(input_sources);
                }
            }

            let input_sources_raw =
                LKAudioInputSources(Box::into_raw(Box::new(input_sources)) as *mut _, callback);

            let input_sources = Box::from_raw(input_sources_raw as *mut Vec<AudioDevice>);

            return input_sources.into_iter().collect();
        }
    }

    pub fn audio_output_sources() -> Result<Vec<AudioDevice>> {
        Ok(vec![])
    }

    pub fn display_sources(self: &Arc<Self>) -> impl Future<Output = Result<Vec<MacOSDisplay>>> {
        extern "C" fn callback(tx: *mut c_void, sources: CFArrayRef, error: CFStringRef) {
            unsafe {
                let tx = Box::from_raw(tx as *mut oneshot::Sender<Result<Vec<MacOSDisplay>>>);

                if sources.is_null() {
                    let _ = tx.send(Err(anyhow!("{}", CFString::wrap_under_get_rule(error))));
                } else {
                    let sources = CFArray::wrap_under_get_rule(sources)
                        .into_iter()
                        .map(|source| MacOSDisplay::new(*source))
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

    pub fn publish_video_track(
        self: &Arc<Self>,
        track: &LocalVideoTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        let (tx, rx) = oneshot::channel::<Result<LocalTrackPublication>>();
        extern "C" fn callback(tx: *mut c_void, publication: *mut c_void, error: CFStringRef) {
            let tx =
                unsafe { Box::from_raw(tx as *mut oneshot::Sender<Result<LocalTrackPublication>>) };
            if error.is_null() {
                let _ = tx.send(Ok(LocalTrackPublication::new(publication)));
            } else {
                let error = unsafe { CFString::wrap_under_get_rule(error).to_string() };
                let _ = tx.send(Err(anyhow!(error)));
            }
        }
        unsafe {
            LKRoomPublishVideoTrack(
                self.native_room,
                track.0,
                callback,
                Box::into_raw(Box::new(tx)) as *mut c_void,
            );
        }
        async { rx.await.unwrap().context("error publishing video track") }
    }

    pub fn publish_audio_track(
        self: &Arc<Self>,
        track: &LocalAudioTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        let (tx, rx) = oneshot::channel::<Result<LocalTrackPublication>>();
        extern "C" fn callback(tx: *mut c_void, publication: *mut c_void, error: CFStringRef) {
            let tx =
                unsafe { Box::from_raw(tx as *mut oneshot::Sender<Result<LocalTrackPublication>>) };
            if error.is_null() {
                let _ = tx.send(Ok(LocalTrackPublication::new(publication)));
            } else {
                let error = unsafe { CFString::wrap_under_get_rule(error).to_string() };
                let _ = tx.send(Err(anyhow!(error)));
            }
        }
        unsafe {
            LKRoomPublishAudioTrack(
                self.native_room,
                track.0,
                callback,
                Box::into_raw(Box::new(tx)) as *mut c_void,
            );
        }
        async { rx.await.unwrap().context("error publishing audio track") }
    }

    pub fn unpublish_track(&self, publication: LocalTrackPublication) {
        unsafe {
            LKRoomUnpublishTrack(self.native_room, publication.0);
        }
    }

    pub fn remote_video_tracks(&self, participant_id: &str) -> Vec<Arc<RemoteVideoTrack>> {
        unsafe {
            let tracks = LKRoomVideoTracksForRemoteParticipant(
                self.native_room,
                CFString::new(participant_id).as_concrete_TypeRef(),
            );

            if tracks.is_null() {
                Vec::new()
            } else {
                let tracks = CFArray::wrap_under_get_rule(tracks);
                tracks
                    .into_iter()
                    .map(|native_track| {
                        let native_track = *native_track;
                        let id =
                            CFString::wrap_under_get_rule(LKRemoteVideoTrackGetSid(native_track))
                                .to_string();
                        Arc::new(RemoteVideoTrack::new(
                            native_track,
                            id,
                            participant_id.into(),
                        ))
                    })
                    .collect()
            }
        }
    }

    pub fn remote_audio_tracks(&self, participant_id: &str) -> Vec<Arc<RemoteAudioTrack>> {
        unsafe {
            let tracks = LKRoomAudioTracksForRemoteParticipant(
                self.native_room,
                CFString::new(participant_id).as_concrete_TypeRef(),
            );

            if tracks.is_null() {
                Vec::new()
            } else {
                let tracks = CFArray::wrap_under_get_rule(tracks);
                tracks
                    .into_iter()
                    .map(|native_track| {
                        let native_track = *native_track;
                        let id =
                            CFString::wrap_under_get_rule(LKRemoteAudioTrackGetSid(native_track))
                                .to_string();
                        Arc::new(RemoteAudioTrack::new(
                            native_track,
                            id,
                            participant_id.into(),
                        ))
                    })
                    .collect()
            }
        }
    }

    pub fn remote_audio_track_publications(
        &self,
        participant_id: &str,
    ) -> Vec<Arc<RemoteTrackPublication>> {
        unsafe {
            let tracks = LKRoomAudioTrackPublicationsForRemoteParticipant(
                self.native_room,
                CFString::new(participant_id).as_concrete_TypeRef(),
            );

            if tracks.is_null() {
                Vec::new()
            } else {
                let tracks = CFArray::wrap_under_get_rule(tracks);
                tracks
                    .into_iter()
                    .map(|native_track_publication| {
                        let native_track_publication = *native_track_publication;
                        Arc::new(RemoteTrackPublication::new(native_track_publication))
                    })
                    .collect()
            }
        }
    }

    pub fn remote_audio_track_updates(&self) -> mpsc::UnboundedReceiver<RemoteAudioTrackUpdate> {
        let (tx, rx) = mpsc::unbounded();
        self.remote_audio_track_subscribers.lock().push(tx);
        rx
    }

    pub fn remote_video_track_updates(&self) -> mpsc::UnboundedReceiver<RemoteVideoTrackUpdate> {
        let (tx, rx) = mpsc::unbounded();
        self.remote_video_track_subscribers.lock().push(tx);
        rx
    }

    fn did_subscribe_to_remote_audio_track(&self, track: RemoteAudioTrack) {
        let track = Arc::new(track);
        self.remote_audio_track_subscribers.lock().retain(|tx| {
            tx.unbounded_send(RemoteAudioTrackUpdate::Subscribed(track.clone()))
                .is_ok()
        });
    }

    fn did_unsubscribe_from_remote_audio_track(&self, publisher_id: String, track_id: String) {
        self.remote_audio_track_subscribers.lock().retain(|tx| {
            tx.unbounded_send(RemoteAudioTrackUpdate::Unsubscribed {
                publisher_id: publisher_id.clone(),
                track_id: track_id.clone(),
            })
            .is_ok()
        });
    }

    fn mute_changed_from_remote_audio_track(&self, track_id: String, muted: bool) {
        self.remote_audio_track_subscribers.lock().retain(|tx| {
            tx.unbounded_send(RemoteAudioTrackUpdate::MuteChanged {
                track_id: track_id.clone(),
                muted,
            })
            .is_ok()
        });
    }

    // A vec of publisher IDs
    fn active_speakers_changed(&self, speakers: Vec<String>) {
        self.remote_audio_track_subscribers
            .lock()
            .retain(move |tx| {
                tx.unbounded_send(RemoteAudioTrackUpdate::ActiveSpeakersChanged {
                    speakers: speakers.clone(),
                })
                .is_ok()
            });
    }

    fn did_subscribe_to_remote_video_track(&self, track: RemoteVideoTrack) {
        let track = Arc::new(track);
        self.remote_video_track_subscribers.lock().retain(|tx| {
            tx.unbounded_send(RemoteVideoTrackUpdate::Subscribed(track.clone()))
                .is_ok()
        });
    }

    fn did_unsubscribe_from_remote_video_track(&self, publisher_id: String, track_id: String) {
        self.remote_video_track_subscribers.lock().retain(|tx| {
            tx.unbounded_send(RemoteVideoTrackUpdate::Unsubscribed {
                publisher_id: publisher_id.clone(),
                track_id: track_id.clone(),
            })
            .is_ok()
        });
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
        unsafe {
            LKRoomDisconnect(self.native_room);
            CFRelease(self.native_room);
        }
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
                Self::on_did_disconnect,
                Self::on_did_subscribe_to_remote_audio_track,
                Self::on_did_unsubscribe_from_remote_audio_track,
                Self::on_mute_change_from_remote_audio_track,
                Self::on_active_speakers_changed,
                Self::on_did_subscribe_to_remote_video_track,
                Self::on_did_unsubscribe_from_remote_video_track,
            )
        };
        Self {
            native_delegate,
            weak_room,
        }
    }

    extern "C" fn on_did_disconnect(room: *mut c_void) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        if let Some(room) = room.upgrade() {
            room.did_disconnect();
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_did_subscribe_to_remote_audio_track(
        room: *mut c_void,
        publisher_id: CFStringRef,
        track_id: CFStringRef,
        track: *const c_void,
    ) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let publisher_id = unsafe { CFString::wrap_under_get_rule(publisher_id).to_string() };
        let track_id = unsafe { CFString::wrap_under_get_rule(track_id).to_string() };
        let track = RemoteAudioTrack::new(track, track_id, publisher_id);
        if let Some(room) = room.upgrade() {
            room.did_subscribe_to_remote_audio_track(track);
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_did_unsubscribe_from_remote_audio_track(
        room: *mut c_void,
        publisher_id: CFStringRef,
        track_id: CFStringRef,
    ) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let publisher_id = unsafe { CFString::wrap_under_get_rule(publisher_id).to_string() };
        let track_id = unsafe { CFString::wrap_under_get_rule(track_id).to_string() };
        if let Some(room) = room.upgrade() {
            room.did_unsubscribe_from_remote_audio_track(publisher_id, track_id);
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_mute_change_from_remote_audio_track(
        room: *mut c_void,
        track_id: CFStringRef,
        muted: bool,
    ) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let track_id = unsafe { CFString::wrap_under_get_rule(track_id).to_string() };
        if let Some(room) = room.upgrade() {
            room.mute_changed_from_remote_audio_track(track_id, muted);
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_active_speakers_changed(room: *mut c_void, participants: CFArrayRef) {
        if participants.is_null() {
            return;
        }

        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let speakers = unsafe {
            CFArray::wrap_under_get_rule(participants)
                .into_iter()
                .map(
                    |speaker: core_foundation::base::ItemRef<'_, *const c_void>| {
                        CFString::wrap_under_get_rule(*speaker as CFStringRef).to_string()
                    },
                )
                .collect()
        };

        if let Some(room) = room.upgrade() {
            room.active_speakers_changed(speakers);
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_did_subscribe_to_remote_video_track(
        room: *mut c_void,
        publisher_id: CFStringRef,
        track_id: CFStringRef,
        track: *const c_void,
    ) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let publisher_id = unsafe { CFString::wrap_under_get_rule(publisher_id).to_string() };
        let track_id = unsafe { CFString::wrap_under_get_rule(track_id).to_string() };
        let track = RemoteVideoTrack::new(track, track_id, publisher_id);
        if let Some(room) = room.upgrade() {
            room.did_subscribe_to_remote_video_track(track);
        }
        let _ = Weak::into_raw(room);
    }

    extern "C" fn on_did_unsubscribe_from_remote_video_track(
        room: *mut c_void,
        publisher_id: CFStringRef,
        track_id: CFStringRef,
    ) {
        let room = unsafe { Weak::from_raw(room as *mut Room) };
        let publisher_id = unsafe { CFString::wrap_under_get_rule(publisher_id).to_string() };
        let track_id = unsafe { CFString::wrap_under_get_rule(track_id).to_string() };
        if let Some(room) = room.upgrade() {
            room.did_unsubscribe_from_remote_video_track(publisher_id, track_id);
        }
        let _ = Weak::into_raw(room);
    }
}

impl Drop for RoomDelegate {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.native_delegate);
            let _ = Weak::from_raw(self.weak_room);
        }
    }
}

pub struct LocalAudioTrack(*const c_void);

impl LocalAudioTrack {
    pub fn create() -> Self {
        Self(unsafe { LKLocalAudioTrackCreateTrack() })
    }
}

impl Drop for LocalAudioTrack {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

pub struct LocalVideoTrack(*const c_void);

impl LocalVideoTrack {
    pub fn screen_share_for_display(display: &MacOSDisplay) -> Self {
        Self(unsafe { LKCreateScreenShareTrackForDisplay(display.0) })
    }
}

impl Drop for LocalVideoTrack {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

pub struct LocalTrackPublication(*const c_void);

impl LocalTrackPublication {
    pub fn new(native_track_publication: *const c_void) -> Self {
        unsafe {
            CFRetain(native_track_publication);
        }
        Self(native_track_publication)
    }

    pub fn set_mute(&self, muted: bool) -> impl Future<Output = Result<()>> {
        let (tx, rx) = futures::channel::oneshot::channel();

        extern "C" fn complete_callback(callback_data: *mut c_void, error: CFStringRef) {
            let tx = unsafe { Box::from_raw(callback_data as *mut oneshot::Sender<Result<()>>) };
            if error.is_null() {
                tx.send(Ok(())).ok();
            } else {
                let error = unsafe { CFString::wrap_under_get_rule(error).to_string() };
                tx.send(Err(anyhow!(error))).ok();
            }
        }

        unsafe {
            LKLocalTrackPublicationSetMute(
                self.0,
                muted,
                complete_callback,
                Box::into_raw(Box::new(tx)) as *mut c_void,
            )
        }

        async move { rx.await.unwrap() }
    }
}

impl Drop for LocalTrackPublication {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

pub struct RemoteTrackPublication(*const c_void);

impl RemoteTrackPublication {
    pub fn new(native_track_publication: *const c_void) -> Self {
        unsafe {
            CFRetain(native_track_publication);
        }
        Self(native_track_publication)
    }

    pub fn set_enabled(&self, enabled: bool) -> impl Future<Output = Result<()>> {
        let (tx, rx) = futures::channel::oneshot::channel();

        extern "C" fn complete_callback(callback_data: *mut c_void, error: CFStringRef) {
            let tx = unsafe { Box::from_raw(callback_data as *mut oneshot::Sender<Result<()>>) };
            if error.is_null() {
                tx.send(Ok(())).ok();
            } else {
                let error = unsafe { CFString::wrap_under_get_rule(error).to_string() };
                tx.send(Err(anyhow!(error))).ok();
            }
        }

        unsafe {
            LKRemoteTrackPublicationSetEnabled(
                self.0,
                enabled,
                complete_callback,
                Box::into_raw(Box::new(tx)) as *mut c_void,
            )
        }

        async move { rx.await.unwrap() }
    }
}

impl Drop for RemoteTrackPublication {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

#[derive(Debug)]
pub struct RemoteAudioTrack {
    _native_track: *const c_void,
    sid: Sid,
    publisher_id: String,
}

impl RemoteAudioTrack {
    fn new(native_track: *const c_void, sid: Sid, publisher_id: String) -> Self {
        unsafe {
            CFRetain(native_track);
        }
        Self {
            _native_track: native_track,
            sid,
            publisher_id,
        }
    }

    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    pub fn enable(&self) -> impl Future<Output = Result<()>> {
        async { Ok(()) }
    }

    pub fn disable(&self) -> impl Future<Output = Result<()>> {
        async { Ok(()) }
    }
}

#[derive(Debug)]
pub struct RemoteVideoTrack {
    native_track: *const c_void,
    sid: Sid,
    publisher_id: String,
}

impl RemoteVideoTrack {
    fn new(native_track: *const c_void, sid: Sid, publisher_id: String) -> Self {
        unsafe {
            CFRetain(native_track);
        }
        Self {
            native_track,
            sid,
            publisher_id,
        }
    }

    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    pub fn frames(&self) -> async_broadcast::Receiver<Frame> {
        extern "C" fn on_frame(callback_data: *mut c_void, frame: CVImageBufferRef) -> bool {
            unsafe {
                let tx = Box::from_raw(callback_data as *mut async_broadcast::Sender<Frame>);
                let buffer = CVImageBuffer::wrap_under_get_rule(frame);
                let result = tx.try_broadcast(Frame(buffer));
                let _ = Box::into_raw(tx);
                match result {
                    Ok(_) => true,
                    Err(async_broadcast::TrySendError::Closed(_))
                    | Err(async_broadcast::TrySendError::Inactive(_)) => {
                        log::warn!("no active receiver for frame");
                        false
                    }
                    Err(async_broadcast::TrySendError::Full(_)) => {
                        log::warn!("skipping frame as receiver is not keeping up");
                        true
                    }
                }
            }
        }

        extern "C" fn on_drop(callback_data: *mut c_void) {
            unsafe {
                let _ = Box::from_raw(callback_data as *mut async_broadcast::Sender<Frame>);
            }
        }

        let (tx, rx) = async_broadcast::broadcast(64);
        unsafe {
            let renderer = LKVideoRendererCreate(
                Box::into_raw(Box::new(tx)) as *mut c_void,
                on_frame,
                on_drop,
            );
            LKVideoTrackAddRenderer(self.native_track, renderer);
            rx
        }
    }
}

impl Drop for RemoteVideoTrack {
    fn drop(&mut self) {
        unsafe { CFRelease(self.native_track) }
    }
}

pub enum RemoteVideoTrackUpdate {
    Subscribed(Arc<RemoteVideoTrack>),
    Unsubscribed { publisher_id: Sid, track_id: Sid },
}

pub enum RemoteAudioTrackUpdate {
    ActiveSpeakersChanged { speakers: Vec<Sid> },
    MuteChanged { track_id: Sid, muted: bool },
    Subscribed(Arc<RemoteAudioTrack>),
    Unsubscribed { publisher_id: Sid, track_id: Sid },
}

pub struct AudioDevice {
    name: String,
    default: bool,
    native_device: *const c_void,
}

impl AudioDevice {
    fn new(name: String, default: bool, ptr: *const c_void) -> Self {
        unsafe {
            CFRetain(ptr);
        };
        Self {
            name,
            default,
            native_device: ptr,
        }
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        unsafe { CFRelease(self.native_device) }
    }
}

impl std::fmt::Debug for AudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioDevice")
            .field("name", &self.name)
            .field("default", &self.default)
            .finish()
    }
}

pub struct MacOSDisplay(*const c_void);

impl MacOSDisplay {
    fn new(ptr: *const c_void) -> Self {
        unsafe {
            CFRetain(ptr);
        }
        Self(ptr)
    }
}

impl Drop for MacOSDisplay {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

#[derive(Clone)]
pub struct Frame(CVImageBuffer);

impl Frame {
    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }

    pub fn image(&self) -> CVImageBuffer {
        self.0.clone()
    }
}
