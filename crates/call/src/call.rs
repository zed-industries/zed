pub mod call_settings;

mod call_impl;

pub use call_impl::*;

/// Parameters for audio channels registration
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeadlessAudioChannelParams {
    /// Channel ID
    pub channel_id: Option<String>,
    /// Participant ID
    pub participant_id: Option<String>,
}

/// Headless audio bridge for transmitting synthesized agent audio and speech streams
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AgentVoiceChannel {
    pub channel_id: String,
    pub participant_id: String,
    pub is_muted: bool,
    pub sample_rate: u32,
}

/// Headless audio manager for routing programmatic agent speech without physical audio hardware
#[derive(Clone, Debug, Default)]
pub struct HeadlessAudioBridge {
    pub active_channels: Vec<AgentVoiceChannel>,
}

impl HeadlessAudioBridge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_channel(&mut self, channel: AgentVoiceChannel) {
        self.active_channels.push(channel);
    }
}
