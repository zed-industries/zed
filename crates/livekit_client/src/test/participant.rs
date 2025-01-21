use super::*;

#[derive(Clone, Debug)]
pub enum Participant {
    Local(LocalParticipant),
    Remote(RemoteParticipant),
}

#[derive(Clone, Debug)]
pub struct LocalParticipant {
    pub(super) identity: ParticipantIdentity,
    pub(super) room: Room,
}

#[derive(Clone, Debug)]
pub struct RemoteParticipant {
    pub(super) identity: ParticipantIdentity,
    pub(super) room: WeakRoom,
}

impl Participant {
    pub fn identity(&self) -> ParticipantIdentity {
        match self {
            Participant::Local(participant) => participant.identity.clone(),
            Participant::Remote(participant) => participant.identity.clone(),
        }
    }
}

impl LocalParticipant {
    pub async fn unpublish_track(&self, track: &TrackSid) -> Result<()> {
        self.room
            .test_server()
            .unpublish_track(self.room.token(), track)
            .await
    }

    pub async fn publish_track(
        &self,
        track: LocalTrack,
        _options: TrackPublishOptions,
    ) -> Result<LocalTrackPublication> {
        let this = self.clone();
        let track = track.clone();
        let server = this.room.test_server();
        let sid = match track {
            LocalTrack::Video(track) => {
                server.publish_video_track(this.room.token(), track).await?
            }
            LocalTrack::Audio(track) => {
                server
                    .publish_audio_track(this.room.token(), &track)
                    .await?
            }
        };
        Ok(LocalTrackPublication {
            room: self.room.downgrade(),
            sid,
        })
    }
}

impl RemoteParticipant {
    pub fn track_publications(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
        if let Some(room) = self.room.upgrade() {
            let server = room.test_server();
            let audio = server
                .audio_tracks(room.token())
                .unwrap()
                .into_iter()
                .filter(|track| track.publisher_id() == self.identity)
                .map(|track| {
                    (
                        track.sid(),
                        RemoteTrackPublication {
                            sid: track.sid(),
                            room: self.room.clone(),
                            track: RemoteTrack::Audio(track),
                        },
                    )
                });
            let video = server
                .video_tracks(room.token())
                .unwrap()
                .into_iter()
                .filter(|track| track.publisher_id() == self.identity)
                .map(|track| {
                    (
                        track.sid(),
                        RemoteTrackPublication {
                            sid: track.sid(),
                            room: self.room.clone(),
                            track: RemoteTrack::Video(track),
                        },
                    )
                });
            audio.chain(video).collect()
        } else {
            HashMap::default()
        }
    }

    pub fn identity(&self) -> ParticipantIdentity {
        self.identity.clone()
    }
}
