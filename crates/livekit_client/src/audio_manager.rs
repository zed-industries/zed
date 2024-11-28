use collections::HashMap;
use cpal::SupportedStreamConfig;
use futures::StreamExt as _;
use gpui::AppContext;
use gpui::BackgroundExecutor;
use gpui::Task;
use livekit::id::TrackSid;
use livekit::track::RemoteAudioTrack;
use parking_lot::Mutex;
use std::sync::OnceLock;
use std::sync::Weak;
use std::{
    collections::{vec_deque, VecDeque},
    sync::Arc,
    thread,
};

use crate::webrtc::audio_stream::native::NativeAudioStream;

struct AudioMixer {
    max_size: usize,
    // Mixed data, ready to render
    storage: VecDeque<i16>,
    next_track_id: usize,
    track_indices: HashMap<usize, usize>,
}

#[derive(Hash, Debug)]
struct InternalTrackId(usize);

impl AudioMixer {
    fn new(max_size: usize) -> Self {
        AudioMixer {
            max_size,
            storage: VecDeque::with_capacity(max_size),
            track_indices: Default::default(),
            next_track_id: 0,
        }
    }

    fn start_track(&mut self) -> InternalTrackId {
        let track_id = self.next_track_id;
        self.next_track_id = track_id.wrapping_add(1);
        self.track_indices.insert(track_id, 0);
        InternalTrackId(track_id)
    }

    fn end_track(&mut self, id: InternalTrackId) {
        self.track_indices.remove(&id.0);
    }

    fn push_frame(&mut self, track_id: &InternalTrackId, frame: impl AsRef<[i16]>) {
        let old_capacity = self.storage.capacity();

        let frame = frame.as_ref();
        // In case the frame is > self.max_size, get the tail of the frame that is within `self.max_size`
        let start_ix = frame.len().saturating_sub(self.max_size);
        let frame = &frame[start_ix..self.max_size];
        debug_assert!(frame.len() <= self.max_size);

        let track_index = self.track_indices[&track_id.0];

        // If the frame causes the buffer to exceed self.max_size, trim from the front.
        let mut new_length = track_index + frame.len();
        if new_length > self.max_size {
            let amount_to_trim = new_length - self.max_size;
            self.pop_frame(amount_to_trim);
            debug_assert!((new_length - amount_to_trim) == self.max_size);
            new_length = self.max_size;
        }

        if new_length > self.storage.len() {
            self.storage.resize(new_length, 0);
        }

        debug_assert!(self.storage.len() >= frame.len());
        debug_assert!(self.storage.len() <= self.max_size);
        debug_assert!(self.storage.capacity() == old_capacity);

        for (entry, data) in self
            .storage
            .range_mut(track_index..new_length)
            .zip(frame.iter())
        {
            *entry = *entry + data
        }
    }

    fn pop_frame(&mut self, frame_size: usize) -> vec_deque::Drain<i16> {
        for index in self.track_indices.values_mut() {
            *index = index.saturating_sub(frame_size);
        }

        self.storage.drain(0..frame_size.min(self.storage.len()))
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn set_buffer_size(&mut self, new_max_size: usize) {
        self.storage.resize(new_max_size, 0);
        self.storage.clear();
        self.max_size = new_max_size;
        for index in self.track_indices.values_mut() {
            *index = 0
        }
    }
}

trait AssertSend: Send {}

// TODO: use cpal types
type TmpConfigType = (i32, i32);

struct DeviceConfiguration {
    config: TmpConfigType,
    mixer: Weak<Mutex<AudioMixer>>,
}

struct DeviceManager {
    device_listener: Option<()>,
    token: Weak<dyn FnOnce() + 'static + Send + Sync>,
    thread: std::sync::mpsc::Sender<()>,
}

impl AssertSend for DeviceConfiguration {}

struct OutputManager {
    executor: BackgroundExecutor,
    // This tracks what exactly is producing audio
    tracks: HashMap<TrackSid, RemoteAudioTrack>,
    // This is dropped and recreated everytime we stop and start producing audio
    device_manager: Option<DeviceManager>,
    // This is dropped and recreated everytime the device changes
    device_output: Option<DeviceConfiguration>,
}

impl OutputManager {
    pub fn add_audio_track(&mut self, track: RemoteAudioTrack) -> anyhow::Result<AudioToken> {
        let sid = track.sid();
        let output = self.start_output()?;

        Ok(AudioToken::new({
            move || {
                with_output_manager(|audio_manager| {
                    audio_manager.tracks.remove(&sid);
                    drop(output)
                });
            }
        }))
    }

    fn start_output_stream(&mut self) -> anyhow::Result<()> {
        // TODO: query the device for teh configuration

        let config = (1 as i32, 2 as i32);
        let mixer = Arc::new(Mutex::new(AudioMixer::new(100)));

        for track in self.tracks.values() {
            // TODO: who should handle these tasks?
            Self::initialize_audio_stream(config, mixer, track, &self.executor);
        }

        // This token get's captured by each input stream, once we're out of input
        // we're done producing output
        self.device_output = Some(DeviceConfiguration {
            device_listener,
            mixer: Arc::downgrade(&mixer),
            token: Arc::downgrade(&token.on_drop.clone().unwrap()),
            thread: _thread,
        });

        Ok(())
    }

    fn start_output(&mut self) -> anyhow::Result<AudioToken> {
        if let Some(token) = self
            .device_manager
            .as_ref()
            .and_then(|output| output.token.upgrade())
        {
            if self.device_output.is_none() {
                self.start_output_stream();
            }

            return Ok(AudioToken {
                on_drop: Some(token),
            });
        }

        // TODO: Initialize stream holding thread
        let (_thread, thread_rx) = std::sync::mpsc::channel();

        // TODO: initialize device listener

        Ok(token)
    }

    fn initialize_audio_stream(
        config: TmpConfigType,
        mixer: &Arc<Mutex<AudioMixer>>,
        track: &RemoteAudioTrack,
        executor: &BackgroundExecutor,
    ) -> Task<()> {
        let mut stream = NativeAudioStream::new(track.rtc_track(), config.0, config.0);
    }
}

static AUDIO_MANAGER: OnceLock<Mutex<OutputManager>> = OnceLock::new();

pub fn init_output_manager(cx: &mut AppContext) {
    AUDIO_MANAGER.get_or_init(|| {
        Mutex::new(OutputManager {
            device_listener: None,
            executor: cx.background_executor().clone(),
            device_output: None,
            tracks: HashMap::default(),
        })
    });
}

pub fn with_output_manager<R>(f: impl FnOnce(&mut OutputManager) -> R) -> R {
    let mut audio_manager = AUDIO_MANAGER.get().unwrap().lock();
    f(&mut audio_manager)
}

struct AudioToken {
    on_drop: Option<Arc<dyn FnOnce() + 'static + Send + Sync>>,
}

impl AudioToken {
    fn new(f: impl FnOnce() + 'static + Send + Sync) -> Self {
        Self {
            on_drop: Some(Arc::new(f)),
        }
    }
}

impl Drop for AudioToken {
    fn drop(&mut self) {
        if let Some(on_drop) = self.on_drop.take() {
            on_drop()
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    use anyhow::Context as _;
    use cpal::traits::{DeviceTrait as _, HostTrait};

    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .context("failed to read default output device")?;
    let output_config = output_device.default_output_config()?;
    Ok((output_device, output_config))
}

#[cfg(not(target_os = "windows"))]
pub fn start_output_stream(
    output_config: cpal::SupportedStreamConfig,
    output_device: cpal::Device,
    track: &crate::RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
    // buffer: Arc<Mutex<AudioBufferMixer>>,
) -> (Task<()>, std::sync::mpsc::Sender<()>) {
    use cpal::traits::{DeviceTrait as _, StreamTrait as _};
    use futures::StreamExt as _;
    use util::ResultExt as _;

    use crate::webrtc::audio_stream::native::NativeAudioStream;

    const MS_OF_BUFFER: usize = 100;
    const MS_IN_SEC: usize = 1000;
    let initial_buffer_size =
        (output_config.sample_rate().0 as usize * output_config.channels() as usize) / MS_IN_SEC
            * MS_OF_BUFFER;

    let buffer = Arc::new(Mutex::new(AudioMixer::new(initial_buffer_size)));

    let sample_rate = output_config.sample_rate();

    let mut stream = NativeAudioStream::new(
        track.rtc_track(),
        sample_rate.0 as i32,
        output_config.channels() as i32,
    );

    let receive_task = background_executor.spawn({
        let buffer = buffer.clone();
        let track_id = {
            let mut buffer = buffer.lock();
            buffer.start_track()
        };
        async move {
            while let Some(frame) = stream.next().await {
                let frame_size = frame.samples_per_channel * frame.num_channels;
                debug_assert!(frame.data.len() == frame_size as usize);

                let mut buffer = buffer.lock();
                buffer.push_frame(&track_id, frame.data);
            }
        }
    });

    // The _output_stream needs to be on it's own thread because it's !Send
    // and we experienced a deadlock when it's created on the main thread.
    let (thread, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
    thread::spawn(move || {
        if cfg!(any(test, feature = "test-support")) {
            // Can't play audio in tests
            return;
        }

        let output_stream = output_device.build_output_stream(
            &output_config.config(),
            {
                let buffer = buffer.clone();
                move |data, _info| {
                    let mut buffer = buffer.lock();
                    if buffer.len() >= data.len() {
                        let frame = buffer.pop_frame(data.len());
                        for (data, entry) in data.iter_mut().zip(frame) {
                            *data = entry;
                        }
                    } else {
                        data.fill(0);
                    }
                }
            },
            |error| log::error!("error playing audio track: {:?}", error),
            None,
        );

        let Some(output_stream) = output_stream.log_err() else {
            return;
        };

        output_stream.play().log_err();
        // Block forever to keep the output stream alive
        end_on_drop_rx.recv().ok();
    });

    (receive_task, thread)
}
