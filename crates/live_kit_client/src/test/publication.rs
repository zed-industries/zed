use super::*;

#[derive(Clone, Debug)]
pub enum TrackPublication {
    Local(LocalTrackPublication),
    Remote(RemoteTrackPublication),
}

#[derive(Clone, Debug)]
pub struct LocalTrackPublication {
    pub(crate) sid: TrackSid,
    pub(crate) room: WeakRoom,
}

#[derive(Clone, Debug)]
pub struct RemoteTrackPublication {
    pub(crate) sid: TrackSid,
    pub(crate) room: WeakRoom,
    pub(crate) track: RemoteTrack,
}

impl TrackPublication {
    pub fn sid(&self) -> TrackSid {
        match self {
            TrackPublication::Local(track) => track.sid(),
            TrackPublication::Remote(track) => track.sid(),
        }
    }

    pub fn is_muted(&self) -> bool {
        match self {
            TrackPublication::Local(track) => track.is_muted(),
            TrackPublication::Remote(track) => track.is_muted(),
        }
    }
}

impl LocalTrackPublication {
    pub fn sid(&self) -> TrackSid {
        self.sid.clone()
    }

    pub fn mute(&self) {
        self.set_mute(true)
    }

    pub fn unmute(&self) {
        self.set_mute(false)
    }

    fn set_mute(&self, mute: bool) {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .set_track_muted(&room.token(), &self.sid, mute)
                .ok();
        }
    }

    pub fn is_muted(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .is_track_muted(&room.token(), &self.sid)
                .unwrap_or(false)
        } else {
            false
        }
    }
}

impl RemoteTrackPublication {
    pub fn sid(&self) -> TrackSid {
        self.sid.clone()
    }

    pub fn track(&self) -> Option<RemoteTrack> {
        Some(self.track.clone())
    }

    pub fn is_muted(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .is_track_muted(&room.token(), &self.sid)
                .unwrap_or(false)
        } else {
            false
        }
    }
}
