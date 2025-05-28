use crate::voice::types::*;
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use gpui::{BackgroundExecutor, Task};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

pub struct VoiceRecorder {
    state: VoiceState,
    event_callback: Option<Box<dyn Fn(VoiceRecorderEvent) + Send + Sync>>,
    executor: Option<BackgroundExecutor>,
    recording_task: Option<Task<()>>,
    audio_buffer: Arc<Mutex<Vec<i16>>>,
    recording_start_time: Option<Instant>,
    stop_sender: Option<std::sync::mpsc::Sender<()>>,
    // Store actual recording configuration
    actual_sample_rate: Option<u32>,
    actual_channels: Option<u32>,
}

impl VoiceRecorder {
    pub fn new() -> Self {
        Self {
            state: VoiceState::Idle,
            event_callback: None,
            executor: None,
            recording_task: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            recording_start_time: None,
            stop_sender: None,
            actual_sample_rate: None,
            actual_channels: None,
        }
    }

    pub fn with_executor(executor: BackgroundExecutor) -> Self {
        Self {
            state: VoiceState::Idle,
            event_callback: None,
            executor: Some(executor),
            recording_task: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            recording_start_time: None,
            stop_sender: None,
            actual_sample_rate: None,
            actual_channels: None,
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
                log::info!("Starting real voice recording");
                
                // Clear previous audio buffer
                self.audio_buffer.lock().clear();
                
                let start_time = Instant::now();
                self.recording_start_time = Some(start_time);
                self.state = VoiceState::Recording { start_time };
                
                // Start real audio capture
                if let Some(executor) = &self.executor {
                    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
                    self.stop_sender = Some(stop_tx);
                    
                    // Get the actual device configuration before starting capture
                    let host = cpal::default_host();
                    if let Some(device) = host.default_input_device() {
                        if let Ok(config) = device.default_input_config() {
                            self.actual_sample_rate = Some(config.sample_rate().0);
                            self.actual_channels = Some(config.channels() as u32);
                            log::info!("Recording with actual config: {}Hz, {} channels", 
                                config.sample_rate().0, config.channels());
                        }
                    }
                    
                    self.recording_task = Some(self.start_audio_capture(executor.clone(), stop_rx)?);
                }
                
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
                
                // Stop the recording task
                if let Some(stop_sender) = self.stop_sender.take() {
                    let _ = stop_sender.send(());
                }
                if let Some(task) = self.recording_task.take() {
                    drop(task); // This will cancel the task
                }
                
                // Get the recorded audio data
                let audio_data = {
                    let buffer = self.audio_buffer.lock();
                    buffer.clone()
                };
                
                // Convert i16 samples to u8 bytes for storage
                let mut byte_data = Vec::with_capacity(audio_data.len() * 2);
                for sample in audio_data {
                    byte_data.extend_from_slice(&sample.to_le_bytes());
                }
                
                let recording = VoiceRecording {
                    id: format!("recording_{}", std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()),
                    duration,
                    data: byte_data,
                    sample_rate: self.actual_sample_rate.unwrap_or(48000), // Use actual or fallback
                    channels: self.actual_channels.unwrap_or(1), // Use actual or fallback
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
                
                // Stop the recording task
                if let Some(stop_sender) = self.stop_sender.take() {
                    let _ = stop_sender.send(());
                }
                if let Some(task) = self.recording_task.take() {
                    drop(task);
                }
                
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

    fn start_audio_capture(&self, executor: BackgroundExecutor, stop_rx: std::sync::mpsc::Receiver<()>) -> Result<Task<()>> {
        let audio_buffer = self.audio_buffer.clone();
        
        let task = executor.spawn(async move {
            if let Err(e) = Self::capture_audio_loop(audio_buffer, stop_rx).await {
                log::error!("Audio capture failed: {}", e);
            }
        });
        
        Ok(task)
    }

    async fn capture_audio_loop(audio_buffer: Arc<Mutex<Vec<i16>>>, stop_rx: std::sync::mpsc::Receiver<()>) -> Result<()> {
        // Get default input device
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("No audio input device available")?;
        
        if let Ok(name) = device.name() {
            log::info!("Using microphone: {}", name);
        }
        
        let config = device
            .default_input_config()
            .context("Failed to get default input config")?;
        
        log::info!("Input config: {:?}", config);
        
        // Create a channel for audio data
        let (tx, rx): (UnboundedSender<Vec<i16>>, UnboundedReceiver<Vec<i16>>) = 
            futures::channel::mpsc::unbounded();
        
        // Spawn a thread to handle the audio stream (following livekit pattern)
        let (end_on_drop_tx, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
        
        thread::spawn(move || {
            let result = (|| -> Result<()> {
                // Build the input stream
                let stream = device
                    .build_input_stream(
                        &config.into(),
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            // Convert f32 samples to i16
                            let samples: Vec<i16> = data
                                .iter()
                                .map(|&sample| (sample * i16::MAX as f32) as i16)
                                .collect();
                            
                            if let Err(e) = tx.unbounded_send(samples) {
                                log::error!("Failed to send audio data: {}", e);
                            }
                        },
                        |err| {
                            log::error!("Audio stream error: {}", err);
                        },
                        None,
                    )
                    .context("Failed to build input stream")?;
                
                // Start the stream
                stream.play().context("Failed to start audio stream")?;
                
                // Keep the thread alive and holding onto the stream
                end_on_drop_rx.recv().ok();
                
                Ok(())
            })();
            
            if let Err(e) = result {
                log::error!("Audio capture thread error: {}", e);
            }
        });
        
        // Process incoming audio data until stop signal
        let mut rx = rx;
        loop {
            // Check for stop signal (non-blocking)
            if stop_rx.try_recv().is_ok() {
                log::info!("Audio capture stop signal received");
                break;
            }
            
            // Try to receive audio samples (with timeout)
            match smol::future::or(
                async {
                    futures::StreamExt::next(&mut rx).await
                },
                async {
                    smol::Timer::after(Duration::from_millis(100)).await;
                    None
                }
            ).await {
                Some(samples) => {
                    let mut buffer = audio_buffer.lock();
                    buffer.extend_from_slice(&samples);
                    
                    // Limit buffer size to prevent excessive memory usage (e.g., 10 minutes at 48kHz)
                    const MAX_SAMPLES: usize = 48000 * 60 * 10; // 10 minutes
                    let buffer_len = buffer.len();
                    if buffer_len > MAX_SAMPLES {
                        buffer.drain(0..buffer_len - MAX_SAMPLES);
                    }
                }
                None => {
                    // Timeout or channel closed - continue loop to check stop signal
                    continue;
                }
            }
        }
        
        // Signal the audio thread to stop
        drop(end_on_drop_tx);
        log::info!("Audio capture stopped");
        
        Ok(())
    }
}

impl Default for VoiceRecorder {
    fn default() -> Self {
        Self::new()
    }
} 