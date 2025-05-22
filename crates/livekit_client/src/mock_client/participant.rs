use crate::{
    AudioStream, LocalAudioTrack, LocalTrackPublication, LocalVideoTrack, Participant,
    ParticipantIdentity, RemoteTrack, RemoteTrackPublication, TrackSid,
    test::{Room, WeakRoom},
};
use anyhow::Result;
use collections::HashMap;
use gpui::{AsyncApp, ScreenCaptureSource, ScreenCaptureStream};

#[derive(Clone, Debug)]
pub struct LocalParticipant {
    pub(crate) identity: ParticipantIdentity,
    pub(crate) room: Room,
}

#[derive(Clone, Debug)]
pub struct RemoteParticipant {
    pub(crate) identity: ParticipantIdentity,
    pub(crate) room: WeakRoom,
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
    pub async fn unpublish_track(&self, track: TrackSid, _cx: &AsyncApp) -> Result<()> {
        self.room
            .test_server()
            .unpublish_track(self.room.token(), &track)
            .await
    }

    pub(crate) async fn publish_microphone_track(
        &self,
        _cx: &AsyncApp,
    ) -> Result<(LocalTrackPublication, AudioStream)> {
        let this = self.clone();
        let server = this.room.test_server();
        let sid = server
            .publish_audio_track(this.room.token(), &LocalAudioTrack {})
            .await?;

        Ok((
            LocalTrackPublication {
                room: self.room.downgrade(),
                sid,
            },
            AudioStream {},
        ))
    }

    pub async fn publish_screenshare_track(
        &self,
        _source: &dyn ScreenCaptureSource,
        _cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, Box<dyn ScreenCaptureStream>)> {
        let this = self.clone();
        let server = this.room.test_server();
        let sid = server
            .publish_video_track(this.room.token(), LocalVideoTrack {})
            .await?;
        Ok((
            LocalTrackPublication {
                room: self.room.downgrade(),
                sid,
            },
            Box::new(TestScreenCaptureStream {}),
        ))
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

struct TestScreenCaptureStream;

impl gpui::ScreenCaptureStream for TestScreenCaptureStream {}
