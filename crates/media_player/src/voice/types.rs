use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceRecording {
    pub id: String,
    pub duration: Duration,
    pub data: Vec<u8>, // Raw audio data
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Clone, Debug)]
pub enum VoiceState {
    Idle,
    Recording { start_time: std::time::Instant },
    Processing,
}

#[derive(Clone, Debug)]
pub struct PlaybackState {
    pub recording_id: String,
    pub start_time: std::time::Instant,
    pub duration: Duration,
    pub original_duration: Duration,
    pub is_playing: bool,
}

#[derive(Clone, Debug)]
pub struct SeekingState {
    pub recording_id: String,
    pub was_playing_before_seek: bool,
    pub seek_position: f32, // 0.0 to 1.0
}

#[derive(Clone, Debug)]
pub enum VoicePlayerEvent {
    PlaybackStarted { recording_id: String },
    PlaybackPaused { recording_id: String },
    PlaybackResumed { recording_id: String },
    PlaybackStopped { recording_id: String },
    PlaybackCompleted { recording_id: String },
    SeekStarted { recording_id: String, position: f32 },
    SeekUpdated { recording_id: String, position: f32 },
    SeekEnded { recording_id: String, position: f32 },
}

#[derive(Clone, Debug)]
pub enum VoiceRecorderEvent {
    RecordingStarted,
    RecordingCompleted { recording: VoiceRecording },
    RecordingFailed { error: String },
} 