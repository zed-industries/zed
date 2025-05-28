use crate::voice::types::*;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use gpui::{BackgroundExecutor, Task};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

pub struct VoicePlayer {
    recordings: HashMap<String, VoiceRecording>,
    playback_state: Option<PlaybackState>,
    seeking_state: Option<SeekingState>,
    // Store paused recordings' progress separately
    paused_progress: HashMap<String, (f32, Duration)>, // recording_id -> (progress, current_time)
    event_callback: Option<Box<dyn Fn(VoicePlayerEvent) + Send + Sync>>,
    executor: Option<BackgroundExecutor>,
    playback_task: Option<Task<()>>,
    audio_stop_sender: Option<std::sync::mpsc::Sender<()>>,
    playback_speed: f32, // 1.0 = normal speed, 0.5 = half speed, 2.0 = double speed
}

impl VoicePlayer {
    pub fn new() -> Self {
        Self {
            recordings: HashMap::new(),
            playback_state: None,
            seeking_state: None,
            paused_progress: HashMap::new(),
            event_callback: None,
            executor: None,
            playback_task: None,
            audio_stop_sender: None,
            playback_speed: 1.0,
        }
    }

    pub fn with_executor(executor: BackgroundExecutor) -> Self {
        Self {
            recordings: HashMap::new(),
            playback_state: None,
            seeking_state: None,
            paused_progress: HashMap::new(),
            event_callback: None,
            executor: Some(executor),
            playback_task: None,
            audio_stop_sender: None,
            playback_speed: 1.0,
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

        // Start real audio playback
        if let Some(executor) = &self.executor {
            let (stop_tx, stop_rx) = std::sync::mpsc::channel();
            self.audio_stop_sender = Some(stop_tx);
            self.playback_task = Some(self.start_audio_playback(executor.clone(), recording, stop_rx)?);
        }

        self.emit_event(VoicePlayerEvent::PlaybackStarted { recording_id });
        Ok(())
    }

    pub fn stop_playback(&mut self) {
        if let Some(playback_state) = &self.playback_state {
            log::info!("Stopping playback of voice recording: {}", playback_state.recording_id);
            let recording_id = playback_state.recording_id.clone();
            
            // Stop audio playback
            if let Some(stop_sender) = self.audio_stop_sender.take() {
                let _ = stop_sender.send(());
            }
            if let Some(task) = self.playback_task.take() {
                drop(task);
            }
            
            self.emit_event(VoicePlayerEvent::PlaybackStopped { recording_id });
        }
        
        self.playback_state = None;
        self.seeking_state = None;
    }

    pub fn pause_playback(&mut self) -> Result<()> {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.is_playing {
                log::info!("Pausing playback of voice recording: {}", playback_state.recording_id);
                
                // Update the remaining duration based on elapsed time
                let elapsed = playback_state.start_time.elapsed();
                playback_state.duration = playback_state.duration.saturating_sub(elapsed);
                playback_state.is_playing = false;
                
                // Stop audio playback
                if let Some(stop_sender) = self.audio_stop_sender.take() {
                    let _ = stop_sender.send(());
                }
                if let Some(task) = self.playback_task.take() {
                    drop(task);
                }
                
                let recording_id = playback_state.recording_id.clone();
                self.emit_event(VoicePlayerEvent::PlaybackPaused { recording_id });
                Ok(())
            } else {
                Err(anyhow::anyhow!("Playback is already paused"))
            }
        } else {
            Err(anyhow::anyhow!("No active playback to pause"))
        }
    }

    pub fn resume_playback(&mut self, recording_id: String) -> Result<()> {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.recording_id == recording_id && !playback_state.is_playing {
                log::info!("Resuming playback of voice recording: {}", recording_id);
                
                playback_state.start_time = std::time::Instant::now();
                playback_state.is_playing = true;
                
                // Resume real audio playback from current position
                if let Some(executor) = &self.executor {
                    let recording = self.recordings.get(&recording_id)
                        .ok_or_else(|| anyhow::anyhow!("Recording not found: {}", recording_id))?
                        .clone();
                    
                    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
                    self.audio_stop_sender = Some(stop_tx);
                    
                    // Calculate start position for resume
                    let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration);
                    let start_position = played_duration.as_secs_f32() / recording.duration.as_secs_f32();
                    
                    self.playback_task = Some(self.start_audio_playback_from_position(
                        executor.clone(), 
                        recording, 
                        start_position,
                        stop_rx
                    )?);
                }
                
                self.emit_event(VoicePlayerEvent::PlaybackResumed { recording_id });
                Ok(())
            } else {
                Err(anyhow::anyhow!("Cannot resume: recording not paused or different recording"))
            }
        } else {
            Err(anyhow::anyhow!("No playback state to resume"))
        }
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

            // Start real audio playback from saved position
            if let Some(executor) = &self.executor {
                let (stop_tx, stop_rx) = std::sync::mpsc::channel();
                self.audio_stop_sender = Some(stop_tx);
                
                let start_position = progress;
                self.playback_task = Some(self.start_audio_playback_from_position(
                    executor.clone(), 
                    recording, 
                    start_position,
                    stop_rx
                )?);
            }

            self.emit_event(VoicePlayerEvent::PlaybackResumed { recording_id });
            Ok(())
        } else {
            // No saved progress, start from beginning
            self.start_playback(recording_id)
        }
    }

    fn start_audio_playback(&self, executor: BackgroundExecutor, recording: VoiceRecording, stop_rx: std::sync::mpsc::Receiver<()>) -> Result<Task<()>> {
        self.start_audio_playback_from_position(executor, recording, 0.0, stop_rx)
    }

    fn start_audio_playback_from_position(
        &self, 
        executor: BackgroundExecutor, 
        recording: VoiceRecording, 
        start_position: f32,
        stop_rx: std::sync::mpsc::Receiver<()>
    ) -> Result<Task<()>> {
        let playback_speed = self.playback_speed;
        let task = executor.spawn(async move {
            if let Err(e) = Self::audio_playback_loop(recording, start_position, playback_speed, stop_rx).await {
                log::error!("Audio playback failed: {}", e);
            }
        });
        
        Ok(task)
    }

    async fn audio_playback_loop(
        recording: VoiceRecording, 
        start_position: f32,
        playback_speed: f32,
        stop_rx: std::sync::mpsc::Receiver<()>
    ) -> Result<()> {
        // Get default output device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No audio output device available")?;
        
        if let Ok(name) = device.name() {
            log::info!("Using audio output device: {}", name);
        }
        
        let config = device
            .default_output_config()
            .context("Failed to get default output config")?;
        
        log::info!("Output config: {:?}, playback speed: {:.2}x", config, playback_speed);
        
        // Log recording vs output configuration
        log::info!("Recording config: {}Hz, {} channels vs Output config: {}Hz, {} channels", 
            recording.sample_rate, recording.channels, config.sample_rate().0, config.channels());
        
        // Convert recorded data back to i16 samples
        let mut audio_samples: Vec<i16> = Vec::new();
        for chunk in recording.data.chunks_exact(2) {
            if let Ok(bytes) = chunk.try_into() {
                audio_samples.push(i16::from_le_bytes(bytes));
            }
        }
        
        // Calculate start sample based on position
        let start_sample = (audio_samples.len() as f32 * start_position) as usize;
        let samples_to_play = &audio_samples[start_sample..];
        
        // Handle sample rate conversion if needed
        let resampled_samples = if recording.sample_rate != config.sample_rate().0 {
            let ratio = config.sample_rate().0 as f32 / recording.sample_rate as f32;
            log::info!("Resampling from {}Hz to {}Hz (ratio: {:.3})", 
                recording.sample_rate, config.sample_rate().0, ratio);
            
            let mut resampled = Vec::new();
            for i in 0..((samples_to_play.len() as f32 * ratio) as usize) {
                let source_index = (i as f32 / ratio) as usize;
                if source_index < samples_to_play.len() {
                    resampled.push(samples_to_play[source_index]);
                }
            }
            resampled
        } else {
            samples_to_play.to_vec()
        };
        
        // Handle channel conversion if needed (mono to stereo)
        let final_samples = if recording.channels == 1 && config.channels() == 2 {
            log::info!("Converting mono to stereo");
            let mut stereo_samples = Vec::with_capacity(resampled_samples.len() * 2);
            for sample in resampled_samples {
                stereo_samples.push(sample); // Left channel
                stereo_samples.push(sample); // Right channel (duplicate)
            }
            stereo_samples
        } else if recording.channels == 2 && config.channels() == 1 {
            log::info!("Converting stereo to mono");
            let mut mono_samples = Vec::with_capacity(resampled_samples.len() / 2);
            for chunk in resampled_samples.chunks_exact(2) {
                // Average left and right channels
                let mono_sample = ((chunk[0] as i32 + chunk[1] as i32) / 2) as i16;
                mono_samples.push(mono_sample);
            }
            mono_samples
        } else {
            resampled_samples
        };
        
        log::info!("Playing {} samples starting from position {:.1}% (sample {}) at {:.2}x speed", 
            final_samples.len(), start_position * 100.0, start_sample, playback_speed);
        
        // Create a shared buffer for audio data with speed control
        let audio_buffer = Arc::new(Mutex::new(final_samples));
        let sample_index = Arc::new(Mutex::new(0.0f32)); // Use float for fractional indexing
        
        // Create a channel for completion notification
        let (completion_tx, completion_rx) = std::sync::mpsc::channel::<()>();
        
        // Spawn a thread to handle the audio stream
        let (end_on_drop_tx, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
        
        thread::spawn(move || {
            let result = (|| -> Result<()> {
                // Build the output stream
                let stream = device
                    .build_output_stream(
                        &config.into(),
                        {
                            let audio_buffer = audio_buffer.clone();
                            let sample_index = sample_index.clone();
                            let completion_tx = completion_tx.clone();
                            let mut completion_sent = false;
                            
                            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                                let buffer = audio_buffer.lock();
                                let mut index = sample_index.lock();
                                
                                for sample in data.iter_mut() {
                                    let current_index = *index as usize;
                                    if current_index < buffer.len() {
                                        // Convert i16 to f32 and write to output
                                        *sample = buffer[current_index] as f32 / i16::MAX as f32;
                                        // Advance index by playback speed (faster = skip samples, slower = repeat samples)
                                        *index += playback_speed;
                                    } else {
                                        // End of audio data
                                        *sample = 0.0;
                                        if !completion_sent {
                                            let _ = completion_tx.send(());
                                            completion_sent = true;
                                        }
                                    }
                                }
                            }
                        },
                        |err| {
                            log::error!("Audio output stream error: {}", err);
                        },
                        None,
                    )
                    .context("Failed to build output stream")?;
                
                // Start the stream
                stream.play().context("Failed to start audio output stream")?;
                
                // Keep the thread alive and holding onto the stream
                end_on_drop_rx.recv().ok();
                
                Ok(())
            })();
            
            if let Err(e) = result {
                log::error!("Audio playback thread error: {}", e);
            }
        });
        
        // Wait for either stop signal or completion
        loop {
            // Check for stop signal (non-blocking)
            if stop_rx.try_recv().is_ok() {
                log::info!("Audio playback stop signal received");
                break;
            }
            
            // Check for completion (non-blocking)
            if completion_rx.try_recv().is_ok() {
                log::info!("Audio playback completed naturally");
                break;
            }
            
            // Small delay to prevent busy waiting
            smol::Timer::after(Duration::from_millis(10)).await;
        }
        
        // Signal the audio thread to stop
        drop(end_on_drop_tx);
        log::info!("Audio playback stopped");
        
        Ok(())
    }

    pub fn get_playback_speed(&self) -> f32 {
        self.playback_speed
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        // Clamp speed to reasonable range
        self.playback_speed = speed.clamp(0.25, 4.0);
        log::info!("Set playback speed to {:.2}x", self.playback_speed);
    }
}

impl Default for VoicePlayer {
    fn default() -> Self {
        Self::new()
    }
} 