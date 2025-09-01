use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::channel::mpsc::UnboundedSender;
use gpui::{App, AsyncApp, BackgroundExecutor, BorrowAppContext, Global};
use libwebrtc::{native::apm, prelude::AudioFrame};
use log::info;
use parking_lot::Mutex;
use rodio::{
    Decoder, OutputStream, OutputStreamBuilder, Source,
    cpal::Sample,
    mixer::Mixer,
    nz,
    source::{Buffered, LimitSettings, UniformSourceIterator},
};
use settings::Settings;
use std::{
    borrow::Cow,
    io::Cursor,
    num::NonZero,
    path::PathBuf,
    sync::{
        Arc,
        mpsc::{TryRecvError, channel},
    },
    thread,
    time::Duration,
};
use util::{ResultExt, debug_panic};

mod audio_settings;
mod replays;
mod rodio_ext;
pub use audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

// NOTE: We used to use WebRTC's mixer which only supported
// 16kHz, 32kHz and 48kHz. As 48 is the most common "next step up"
// for audio output devices like speakers/bluetooth, we just hard-code
// this; and downsample when we need to.
//
// Since most noise cancelling requires 16kHz we will move to
// that in the future.
pub const SAMPLE_RATE: NonZero<u32> = nz!(48000);
pub const CHANNEL_COUNT: NonZero<u16> = nz!(2);
const BUFFER_SIZE: usize = // echo canceller and livekit want 10ms of audio
    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;

pub const REPLAY_DURATION: Duration = Duration::from_secs(30);

pub fn init(cx: &mut App) {
    AudioSettings::register(cx);
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    Joined,
    Leave,
    Mute,
    Unmute,
    StartScreenshare,
    StopScreenshare,
    AgentDone,
}

impl Sound {
    fn file(&self) -> &'static str {
        match self {
            Self::Joined => "joined_call",
            Self::Leave => "leave_call",
            Self::Mute => "mute",
            Self::Unmute => "unmute",
            Self::StartScreenshare => "start_screenshare",
            Self::StopScreenshare => "stop_screenshare",
            Self::AgentDone => "agent_done",
        }
    }
}

pub struct Audio {
    output_handle: Option<OutputStream>,
    output_mixer: Option<Mixer>,
    pub echo_canceller: Arc<Mutex<apm::AudioProcessingModule>>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    replays: replays::Replays,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            output_handle: Default::default(),
            output_mixer: Default::default(),
            echo_canceller: Arc::new(Mutex::new(apm::AudioProcessingModule::new(
                true, false, false, false,
            ))),
            source_cache: Default::default(),
            replays: Default::default(),
        }
    }
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self) -> Result<&Mixer> {
        if self.output_handle.is_none() {
            self.output_handle = Some(
                OutputStreamBuilder::open_default_stream()
                    .context("Could not open default output stream")?,
            );
            if let Some(output_handle) = &self.output_handle {
                let (mixer, source) = rodio::mixer::mixer(CHANNEL_COUNT, SAMPLE_RATE);
                // or the mixer will end immediately as its empty.
                mixer.add(rodio::source::Zero::new(CHANNEL_COUNT, SAMPLE_RATE));
                self.output_mixer = Some(mixer);

                let echo_canceller = Arc::clone(&self.echo_canceller);
                let source = source.inspect_buffer::<BUFFER_SIZE, _>(move |buffer| {
                    let mut buf: [i16; _] = buffer.map(|s| s.to_sample());
                    echo_canceller
                        .lock()
                        .process_reverse_stream(
                            &mut buf,
                            SAMPLE_RATE.get() as i32,
                            CHANNEL_COUNT.get().into(),
                        )
                        .expect("Audio input and output threads should not panic");
                });
                output_handle.mixer().add(source);
            }
        }

        Ok(self
            .output_mixer
            .as_ref()
            .expect("we only get here if opening the outputstream succeeded"))
    }

    pub fn save_replays(
        &self,
        executor: BackgroundExecutor,
    ) -> gpui::Task<anyhow::Result<(PathBuf, Duration)>> {
        self.replays.replays_to_tar(executor)
    }

    pub fn open_microphone(
        cx: AsyncApp,
        frame_tx: UnboundedSender<AudioFrame<'static>>,
    ) -> anyhow::Result<()> {
        let (apm, mut replays) = cx.try_read_default_global::<Audio, _>(|audio, _| {
            (Arc::clone(&audio.echo_canceller), audio.replays.clone())
        })?;

        let (stream_error_tx, stream_error_rx) = channel();
        thread::spawn(move || {
            let stream = rodio::microphone::MicrophoneBuilder::new()
                .default_device()?
                .default_config()?
                .prefer_sample_rates([SAMPLE_RATE, SAMPLE_RATE.saturating_mul(nz!(2))])
                .prefer_channel_counts([nz!(1), nz!(2)])
                .prefer_buffer_sizes(512..)
                .open_stream()?;
            info!("Opened microphone: {:?}", stream.config());

            let stream = UniformSourceIterator::new(stream, CHANNEL_COUNT, SAMPLE_RATE)
                .limit(LimitSettings::live_performance())
                .process_buffer::<BUFFER_SIZE, _>(|buffer| {
                    let mut int_buffer: [i16; _] = buffer.map(|s| s.to_sample());
                    if let Err(e) = apm
                        .lock()
                        .process_stream(
                            &mut int_buffer,
                            SAMPLE_RATE.get() as i32,
                            CHANNEL_COUNT.get() as i32,
                        )
                        .context("livekit audio processor error")
                    {
                        let _ = stream_error_tx.send(e);
                    } else {
                        for (sample, processed) in buffer.iter_mut().zip(&int_buffer) {
                            *sample = (*processed).to_sample();
                        }
                    }
                })
                .automatic_gain_control(1.0, 4.0, 0.0, 5.0)
                .periodic_access(Duration::from_millis(100), move |agc_source| {
                    agc_source.set_enabled(true); // todo dvdsk how to get settings in here?
                });

            // todo dvdsk keep the above here, move the rest back to livekit?
            let (replay, mut stream) = stream.replayable(REPLAY_DURATION);
            replays.add_voip_stream("local microphone".to_string(), replay);

            loop {
                let sampled: Vec<_> = stream
                    .by_ref()
                    .take(BUFFER_SIZE)
                    .map(|s| s.to_sample())
                    .collect();

                match stream_error_rx.try_recv() {
                    Ok(apm_error) => return Err::<(), _>(apm_error),
                    Err(TryRecvError::Disconnected) => {
                        debug_panic!("Stream should end on its own without sending an error")
                    }
                    Err(TryRecvError::Empty) => (),
                }

                frame_tx
                    .unbounded_send(AudioFrame {
                        sample_rate: SAMPLE_RATE.get(),
                        num_channels: CHANNEL_COUNT.get() as u32,
                        samples_per_channel: sampled.len() as u32 / CHANNEL_COUNT.get() as u32,
                        data: Cow::Owned(sampled),
                    })
                    .context("Failed to send audio frame")?
            }
        });

        Ok(())
    }

    pub fn play_voip_stream(
        stream_source: impl rodio::Source + Send + 'static,
        stream_name: String,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let (replay_source, source) = stream_source.replayable(REPLAY_DURATION);

        cx.update_default_global(|this: &mut Self, _cx| {
            let output_mixer = this
                .ensure_output_exists()
                .context("Could not get output mixer")?;
            output_mixer.add(source);
            this.replays.add_voip_stream(stream_name, replay_source);
            Ok(())
        })
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_mixer = this
                .ensure_output_exists()
                .context("Could not get output mixer")
                .log_err()?;

            output_mixer.add(source);
            Some(())
        });
    }

    pub fn end_call(cx: &mut App) {
        cx.update_default_global(|this: &mut Self, _cx| {
            this.output_handle.take();
        });
    }

    fn sound_source(&mut self, sound: Sound, cx: &App) -> Result<impl Source + use<>> {
        if let Some(wav) = self.source_cache.get(&sound) {
            return Ok(wav.clone());
        }

        let path = format!("sounds/{}.wav", sound.file());
        let bytes = cx
            .asset_source()
            .load(&path)?
            .map(anyhow::Ok)
            .with_context(|| format!("No asset available for path {path}"))??
            .into_owned();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(cursor)?.buffered();

        self.source_cache.insert(sound, source.clone());

        Ok(source)
    }
}
