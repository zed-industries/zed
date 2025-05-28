use crate::voice::types::*;
use anyhow::Result;
use std::time::Duration;

pub struct VoiceRecorder {
    state: VoiceState,
    event_callback: Option<Box<dyn Fn(VoiceRecorderEvent) + Send + Sync>>,
}

impl VoiceRecorder {
    pub fn new() -> Self {
        Self {
            state: VoiceState::Idle,
            event_callback: None,
        }
    }

    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(VoiceRecorderEvent) + Send + Sync + 'static,
    {
        self.event_callback = Some(Box::new(callback));
    }

    fn emit_event(&self, event: VoiceRecorderEvent) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    pub fn get_state(&self) -> &VoiceState {
        &self.state
    }

    pub fn is_recording(&self) -> bool {
        matches!(self.state, VoiceState::Recording { .. })
    }

    pub fn is_processing(&self) -> bool {
        matches!(self.state, VoiceState::Processing)
    }

    pub fn get_recording_duration(&self) -> Option<Duration> {
        if let VoiceState::Recording { start_time } = &self.state {
            Some(start_time.elapsed())
        } else {
            None
        }
    }

    pub fn start_recording(&mut self) -> Result<()> {
        match self.state {
            VoiceState::Idle => {
                log::info!("Starting voice recording");
                self.state = VoiceState::Recording {
                    start_time: std::time::Instant::now(),
                };
                self.emit_event(VoiceRecorderEvent::RecordingStarted);
                Ok(())
            }
            VoiceState::Recording { .. } => {
                Err(anyhow::anyhow!("Already recording"))
            }
            VoiceState::Processing => {
                Err(anyhow::anyhow!("Currently processing previous recording"))
            }
        }
    }

    pub fn stop_recording(&mut self) -> Result<VoiceRecording> {
        match &self.state {
            VoiceState::Recording { start_time } => {
                let duration = start_time.elapsed();
                log::info!("Stopping voice recording after {:.2}s", duration.as_secs_f32());
                
                self.state = VoiceState::Processing;
                
                // In a real implementation, this would:
                // 1. Stop the audio recording
                // 2. Process the audio data
                // 3. Create a VoiceRecording struct
                // 4. Emit RecordingCompleted event
                
                // For now, we'll simulate this with a placeholder
                let recording = VoiceRecording {
                    id: format!("recording_{}", std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()),
                    duration,
                    data: vec![], // Placeholder for actual audio data
                    sample_rate: 44100,
                    channels: 1,
                };
                
                self.state = VoiceState::Idle;
                self.emit_event(VoiceRecorderEvent::RecordingCompleted { recording: recording.clone() });
                
                Ok(recording)
            }
            VoiceState::Idle => {
                Err(anyhow::anyhow!("Not currently recording"))
            }
            VoiceState::Processing => {
                Err(anyhow::anyhow!("Currently processing previous recording"))
            }
        }
    }

    pub fn cancel_recording(&mut self) -> Result<()> {
        match self.state {
            VoiceState::Recording { .. } => {
                log::info!("Cancelling voice recording");
                self.state = VoiceState::Idle;
                Ok(())
            }
            VoiceState::Idle => {
                Err(anyhow::anyhow!("Not currently recording"))
            }
            VoiceState::Processing => {
                Err(anyhow::anyhow!("Cannot cancel while processing"))
            }
        }
    }

    pub fn toggle_recording(&mut self) -> Result<Option<VoiceRecording>> {
        match self.state {
            VoiceState::Idle => {
                self.start_recording()?;
                Ok(None)
            }
            VoiceState::Recording { .. } => {
                let recording = self.stop_recording()?;
                Ok(Some(recording))
            }
            VoiceState::Processing => Err(anyhow::anyhow!("Currently processing previous recording")),
        }
    }
}

impl Default for VoiceRecorder {
    fn default() -> Self {
        Self::new()
    }
} 