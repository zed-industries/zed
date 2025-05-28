use crate::voice::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::time::Duration;

pub struct VoicePlayer {
    recordings: HashMap<String, VoiceRecording>,
    playback_state: Option<PlaybackState>,
    seeking_state: Option<SeekingState>,
    // Store paused recordings' progress separately
    paused_progress: HashMap<String, (f32, Duration)>, // recording_id -> (progress, current_time)
    event_callback: Option<Box<dyn Fn(VoicePlayerEvent) + Send + Sync>>,
}

impl VoicePlayer {
    pub fn new() -> Self {
        Self {
            recordings: HashMap::new(),
            playback_state: None,
            seeking_state: None,
            paused_progress: HashMap::new(),
            event_callback: None,
        }
    }

    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(VoicePlayerEvent) + Send + Sync + 'static,
    {
        self.event_callback = Some(Box::new(callback));
    }

    fn emit_event(&self, event: VoicePlayerEvent) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    pub fn add_recording(&mut self, recording: VoiceRecording) {
        log::info!("Added voice recording: {} ({}s)", recording.id, recording.duration.as_secs_f32());
        self.recordings.insert(recording.id.clone(), recording);
    }

    pub fn remove_recording(&mut self, recording_id: &str) {
        if let Some(playback_state) = &self.playback_state {
            if playback_state.recording_id == recording_id {
                self.stop_playback();
            }
        }
        
        // Also remove any saved progress
        self.paused_progress.remove(recording_id);
        self.recordings.remove(recording_id);
    }

    pub fn get_recording(&self, recording_id: &str) -> Option<&VoiceRecording> {
        self.recordings.get(recording_id)
    }

    pub fn get_recordings(&self) -> &HashMap<String, VoiceRecording> {
        &self.recordings
    }

    pub fn get_playback_state(&self) -> Option<&PlaybackState> {
        self.playback_state.as_ref()
    }

    pub fn get_seeking_state(&self) -> Option<&SeekingState> {
        self.seeking_state.as_ref()
    }

    pub fn is_playing(&self, recording_id: &str) -> bool {
        self.playback_state
            .as_ref()
            .map(|state| state.recording_id == recording_id && state.is_playing)
            .unwrap_or(false)
    }

    pub fn is_paused(&self, recording_id: &str) -> bool {
        // Check if it's the current recording and paused
        if let Some(playback_state) = &self.playback_state {
            if playback_state.recording_id == recording_id && !playback_state.is_playing {
                return true;
            }
        }
        
        // Check if it has saved progress (meaning it was paused when switching)
        self.paused_progress.contains_key(recording_id)
    }

    pub fn is_seeking(&self, recording_id: &str) -> bool {
        self.seeking_state
            .as_ref()
            .map(|state| state.recording_id == recording_id)
            .unwrap_or(false)
    }

    pub fn get_progress(&self, recording_id: &str) -> Option<(f32, Duration)> {
        let recording = self.recordings.get(recording_id)?;
        
        // Check if we're currently seeking this recording
        if let Some(seeking_state) = &self.seeking_state {
            if seeking_state.recording_id == recording_id {
                // Show seeking position
                let seek_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * seeking_state.seek_position);
                return Some((seeking_state.seek_position, seek_time));
            }
        }

        // Show normal playback progress
        if let Some(playback_state) = &self.playback_state {
            if playback_state.recording_id == recording_id {
                if playback_state.is_playing {
                    let elapsed = playback_state.start_time.elapsed();
                    let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration) + elapsed;
                    let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                    Some((progress, played_duration))
                } else {
                    // Paused state - calculate progress based on remaining duration
                    let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration);
                    let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                    Some((progress, played_duration))
                }
            } else {
                // Check if this recording has saved progress from being paused
                if let Some(&(progress, current_time)) = self.paused_progress.get(recording_id) {
                    Some((progress, current_time))
                } else {
                    Some((0.0, Duration::ZERO))
                }
            }
        } else {
            // Check if this recording has saved progress from being paused
            if let Some(&(progress, current_time)) = self.paused_progress.get(recording_id) {
                Some((progress, current_time))
            } else {
                Some((0.0, Duration::ZERO))
            }
        }
    }

    pub fn toggle_playback(&mut self, recording_id: String) -> Result<()> {
        // Check if this recording is currently playing
        if let Some(playback_state) = &self.playback_state {
            if playback_state.recording_id == recording_id {
                if playback_state.is_playing {
                    self.pause_playback()?;
                } else {
                    self.resume_playback(recording_id)?;
                }
                return Ok(());
            } else {
                // Different recording is playing, save its progress and pause it
                self.save_current_progress_and_pause()?;
            }
        }
        
        // Start playback for new recording (or resume if it has saved progress)
        if self.paused_progress.contains_key(&recording_id) {
            self.resume_from_saved_progress(recording_id)
        } else {
            self.start_playback(recording_id)
        }
    }

    pub fn start_playback(&mut self, recording_id: String) -> Result<()> {
        let recording = self.recordings.get(&recording_id)
            .ok_or_else(|| anyhow::anyhow!("Recording not found: {}", recording_id))?
            .clone();

        log::info!("Starting playback of voice recording: {} ({}s)", recording.id, recording.duration.as_secs_f32());
        
        // Stop any existing playback
        self.stop_playback();
        
        // Set up new playback state
        self.playback_state = Some(PlaybackState {
            recording_id: recording_id.clone(),
            start_time: std::time::Instant::now(),
            duration: recording.duration,
            original_duration: recording.duration,
            is_playing: true,
        });

        self.emit_event(VoicePlayerEvent::PlaybackStarted { recording_id });
        Ok(())
    }

    pub fn stop_playback(&mut self) {
        if let Some(playback_state) = &self.playback_state {
            log::info!("Stopping playback of voice recording: {}", playback_state.recording_id);
            let recording_id = playback_state.recording_id.clone();
            self.emit_event(VoicePlayerEvent::PlaybackStopped { recording_id });
        }
        
        self.playback_state = None;
        self.seeking_state = None;
    }

    pub fn pause_playback(&mut self) -> Result<()> {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.is_playing {
                log::info!("Pausing playback of voice recording: {}", playback_state.recording_id);
                playback_state.is_playing = false;
                
                // Calculate how much time has elapsed and adjust the duration
                let elapsed = playback_state.start_time.elapsed();
                playback_state.duration = playback_state.duration.saturating_sub(elapsed);
                
                let recording_id = playback_state.recording_id.clone();
                self.emit_event(VoicePlayerEvent::PlaybackPaused { recording_id });
            }
        }
        Ok(())
    }

    pub fn resume_playback(&mut self, recording_id: String) -> Result<()> {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.recording_id == recording_id && !playback_state.is_playing {
                log::info!("Resuming playback of voice recording: {}", playback_state.recording_id);
                
                // Reset start time for remaining duration
                playback_state.start_time = std::time::Instant::now();
                playback_state.is_playing = true;
                
                self.emit_event(VoicePlayerEvent::PlaybackResumed { recording_id });
            }
        }
        Ok(())
    }

    pub fn start_seeking(&mut self, recording_id: String, relative_position: f32) -> Result<()> {
        let recording = self.recordings.get(&recording_id)
            .ok_or_else(|| anyhow::anyhow!("Recording not found: {}", recording_id))?
            .clone();

        // Guard: Don't start seeking if we're already seeking this recording
        if let Some(existing_seeking_state) = &self.seeking_state {
            if existing_seeking_state.recording_id == recording_id {
                log::debug!("🎯 Already seeking recording: {}, ignoring duplicate start_seek call", recording_id);
                return Ok(());
            }
        }
        
        // Check if this recording is currently playing
        let was_playing = self.playback_state
            .as_ref()
            .map(|state| state.recording_id == recording_id && state.is_playing)
            .unwrap_or(false);
        
        // Pause playback if it was playing
        if was_playing {
            log::info!("🔇 Pausing playback for seeking: {}", recording_id);
            self.pause_playback()?;
        }
        
        // Set seeking state
        self.seeking_state = Some(SeekingState {
            recording_id: recording_id.clone(),
            was_playing_before_seek: was_playing,
            seek_position: relative_position,
        });
        
        // Update playback position
        let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
        
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.recording_id == recording_id {
                // Update existing playback state to new position
                playback_state.duration = recording.duration.saturating_sub(target_time);
                playback_state.start_time = std::time::Instant::now();
                playback_state.is_playing = false; // Paused during seek
            } else {
                // Different recording, create new playback state
                self.stop_playback();
                self.playback_state = Some(PlaybackState {
                    recording_id: recording_id.clone(),
                    start_time: std::time::Instant::now(),
                    duration: recording.duration.saturating_sub(target_time),
                    original_duration: recording.duration,
                    is_playing: false,
                });
            }
        } else {
            // No current playback, create new state at seek position
            self.playback_state = Some(PlaybackState {
                recording_id: recording_id.clone(),
                start_time: std::time::Instant::now(),
                duration: recording.duration.saturating_sub(target_time),
                original_duration: recording.duration,
                is_playing: false,
            });
        }
        
        log::info!("🎯 Started seeking to position {:.1}s in recording: {} (was_playing: {})", 
            target_time.as_secs_f32(), recording_id, was_playing);
        
        self.emit_event(VoicePlayerEvent::SeekStarted { 
            recording_id, 
            position: relative_position 
        });
        
        Ok(())
    }

    pub fn update_seek_position(&mut self, recording_id: String, relative_position: f32) -> Result<()> {
        // Only update if we're currently seeking this recording
        if let Some(seeking_state) = &mut self.seeking_state {
            if seeking_state.recording_id == recording_id {
                // Update the seek position
                seeking_state.seek_position = relative_position;
                
                // Update the playback state to reflect the new position
                if let Some(recording) = self.recordings.get(&recording_id) {
                    let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
                    
                    if let Some(playback_state) = &mut self.playback_state {
                        if playback_state.recording_id == recording_id {
                            // Update playback position
                            playback_state.duration = recording.duration.saturating_sub(target_time);
                            playback_state.start_time = std::time::Instant::now();
                            // Keep is_playing as false during seeking
                        }
                    }
                }
                
                log::debug!("🎯 Continuous seek to {:.1}s ({:.1}%)", 
                    relative_position * self.recordings.get(&recording_id).unwrap().duration.as_secs_f32(), 
                    relative_position * 100.0);

                self.emit_event(VoicePlayerEvent::SeekUpdated { 
                    recording_id, 
                    position: relative_position 
                });
            }
        }
        Ok(())
    }

    pub fn end_seeking(&mut self, recording_id: String) -> Result<()> {
        if let Some(seeking_state) = self.seeking_state.take() {
            if seeking_state.recording_id == recording_id {
                log::info!("🎯 Ended seeking for recording: {} (was_playing: {})", 
                    recording_id, seeking_state.was_playing_before_seek);
                
                let position = seeking_state.seek_position;
                
                // Resume playback if it was playing before seeking
                if seeking_state.was_playing_before_seek {
                    log::info!("▶️ Resuming playback after seeking: {}", recording_id);
                    self.resume_playback(recording_id.clone())?;
                } else {
                    log::info!("⏸️ Staying paused after seeking: {}", recording_id);
                }
                
                self.emit_event(VoicePlayerEvent::SeekEnded { 
                    recording_id, 
                    position 
                });
            } else {
                // Put the seeking state back if it's for a different recording
                self.seeking_state = Some(seeking_state);
            }
        }
        Ok(())
    }

    pub fn seek_to_position(&mut self, recording_id: String, relative_position: f32) -> Result<()> {
        let recording = self.recordings.get(&recording_id)
            .ok_or_else(|| anyhow::anyhow!("Recording not found: {}", recording_id))?
            .clone();

        let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
        
        log::info!("Seeking to position {:.1}s in recording: {}", target_time.as_secs_f32(), recording.id);
        
        // Update playback state to new position
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.recording_id == recording_id {
                playback_state.duration = recording.duration.saturating_sub(target_time);
                playback_state.start_time = std::time::Instant::now();
            }
        } else {
            // No current playback, create new state at seek position
            self.playback_state = Some(PlaybackState {
                recording_id: recording_id.clone(),
                start_time: std::time::Instant::now(),
                duration: recording.duration.saturating_sub(target_time),
                original_duration: recording.duration,
                is_playing: false,
            });
        }
        
        Ok(())
    }

    /// Handle mouse-based seeking with proper position calculation
    pub fn handle_mouse_seek(&mut self, recording_id: String, mouse_x: f32, element_width: f32) -> Result<()> {
        // Calculate relative position from mouse coordinates
        let relative_position = (mouse_x / element_width).clamp(0.0, 1.0);
        
        log::info!("Mouse seek: x={:.1}, width={:.1}, position={:.1}%", 
            mouse_x, element_width, relative_position * 100.0);
        
        // Use the existing seeking system
        self.start_seeking(recording_id, relative_position)
    }

    /// Handle mouse-based seeking during drag
    pub fn handle_mouse_seek_drag(&mut self, recording_id: String, mouse_x: f32, element_width: f32) -> Result<()> {
        // Calculate relative position from mouse coordinates
        let relative_position = (mouse_x / element_width).clamp(0.0, 1.0);
        
        // Use the existing seeking system
        self.update_seek_position(recording_id, relative_position)
    }

    pub fn update_playback(&mut self) -> Option<VoicePlayerEvent> {
        if let Some(playback_state) = &self.playback_state {
            if playback_state.is_playing {
                let elapsed = playback_state.start_time.elapsed();
                
                if elapsed >= playback_state.duration {
                    // Playback finished
                    let recording_id = playback_state.recording_id.clone();
                    log::info!("Playback completed for recording: {}", recording_id);
                    self.stop_playback();
                    return Some(VoicePlayerEvent::PlaybackCompleted { recording_id });
                }
            }
        }
        None
    }

    /// Save the current playback progress and pause it
    fn save_current_progress_and_pause(&mut self) -> Result<()> {
        if let Some(playback_state) = &self.playback_state {
            let recording_id = playback_state.recording_id.clone();
            
            // Calculate current progress
            let played_duration = if playback_state.is_playing {
                let elapsed = playback_state.start_time.elapsed();
                playback_state.original_duration.saturating_sub(playback_state.duration) + elapsed
            } else {
                playback_state.original_duration.saturating_sub(playback_state.duration)
            };
            
            let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
            
            // Save the progress
            self.paused_progress.insert(recording_id.clone(), (progress, played_duration));
            
            log::info!("💾 Saved progress for recording {}: {:.1}% ({:.1}s)", 
                recording_id, progress * 100.0, played_duration.as_secs_f32());
            
            // Emit pause event
            self.emit_event(VoicePlayerEvent::PlaybackPaused { recording_id });
        }
        
        // Clear current playback state
        self.playback_state = None;
        self.seeking_state = None;
        
        Ok(())
    }

    /// Resume playback from saved progress
    fn resume_from_saved_progress(&mut self, recording_id: String) -> Result<()> {
        let recording = self.recordings.get(&recording_id)
            .ok_or_else(|| anyhow::anyhow!("Recording not found: {}", recording_id))?
            .clone();

        if let Some(&(progress, played_duration)) = self.paused_progress.get(&recording_id) {
            log::info!("▶️ Resuming recording {} from saved progress: {:.1}% ({:.1}s)", 
                recording_id, progress * 100.0, played_duration.as_secs_f32());
            
            // Calculate remaining duration
            let remaining_duration = recording.duration.saturating_sub(played_duration);
            
            // Set up playback state from saved progress
            self.playback_state = Some(PlaybackState {
                recording_id: recording_id.clone(),
                start_time: std::time::Instant::now(),
                duration: remaining_duration,
                original_duration: recording.duration,
                is_playing: true,
            });

            // Remove from paused progress since it's now active
            self.paused_progress.remove(&recording_id);

            self.emit_event(VoicePlayerEvent::PlaybackResumed { recording_id });
            Ok(())
        } else {
            // No saved progress, start from beginning
            self.start_playback(recording_id)
        }
    }
}

impl Default for VoicePlayer {
    fn default() -> Self {
        Self::new()
    }
} 