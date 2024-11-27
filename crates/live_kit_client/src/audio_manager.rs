use collections::HashMap;
use futures::StreamExt as _;
use gpui::BackgroundExecutor;
use gpui::Task;
use parking_lot::Mutex;
use std::sync::OnceLock;
use std::sync::Weak;
use std::{
    collections::{vec_deque, VecDeque},
    sync::Arc,
    thread,
};

struct AudioBufferMixer {
    max_size: usize,
    // Mixed data, ready to render
    storage: VecDeque<i16>,
    next_track_id: usize,
    track_indices: HashMap<usize, usize>,
}

#[derive(Hash, Debug)]
struct InternalTrackId(usize);

impl AudioBufferMixer {
    fn new(max_size: usize) -> Self {
        AudioBufferMixer {
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

struct AudioManager {
    active_output: Weak<Mutex<AudioBufferMixer>>,
    // All of these streams, are owned and RAII'd, by Zed
}

// Call/Room (Strong, retaining)
//  - Arc<>
//  - Task<>
//
// AudioManager (This is not retaining)
//  - Weak<Mutex<AudioBuffer>>
// BUT: we need to retain them, so we can re-create the NativeAudioStream
//  (which holds the resampler)
//  Which means we are retaining RemoteAudioTrack
// We need to retain the RemoteAudioTrack pointer, but we also need to return something
// that can let us know when it's dropped, so we know to drop the RemoteAudioTrack

enum AudioManagerMessage {
    AddRemoteAudioTrack(crate::RemoteAudioTrack),
}

static AUDIO_MANAGER_CHANNEL: OnceLock<futures::channel::mpsc::Sender<()>> = OnceLock::new();

fn init(cx: &BackgroundExecutor) {
    AUDIO_MANAGER_CHANNEL.get_or_init(|| {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        let mixer = Arc::new(Mutex::new(None));
        // let audio_manager = ???
        cx.spawn(async move {
            while let Some(event) = rx.next().await {
                match event {
                    AudioManagerMessage::AddRemoteAudioTrack(track) => {}
                }
            }
        });

        return tx;
    });
}

pub fn get_audio_manager_channel() -> &AudioManager {
    AUDIO_MANAGER_CHANNEL.get().unwrap()
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

    let buffer = Arc::new(Mutex::new(AudioBufferMixer::new(initial_buffer_size)));

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
