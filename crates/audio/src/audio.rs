use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, BackgroundExecutor, BorrowAppContext, Global};

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
mod non_windows_and_freebsd_deps {
    pub(super) use gpui::AsyncApp;
    pub(super) use libwebrtc::native::apm;
    pub(super) use log::info;
    pub(super) use parking_lot::Mutex;
    pub(super) use rodio::cpal::Sample;
    pub(super) use rodio::source::LimitSettings;
    pub(super) use std::sync::Arc;
}

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
use non_windows_and_freebsd_deps::*;

use rodio::{
    Decoder, OutputStream, OutputStreamBuilder, Source,
    mixer::Mixer,
    nz,
    source::{AutomaticGainControlSettings, Buffered},
};
use settings::Settings;
use std::{io::Cursor, num::NonZero, path::PathBuf, sync::atomic::Ordering, time::Duration};
use util::ResultExt;

mod audio_settings;
mod replays;
mod rodio_ext;
#[cfg(test)]
mod test;
pub use audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

use crate::audio_settings::LIVE_SETTINGS;

// We are migrating to 44100Hz mono from 48kHz stereo. In the future once we are
// reasonably sure most users have upgraded we will remove the LEGACY
// parameters.
// We migrate to 44100 because if its good for cd's its good enough for us
pub const SAMPLE_RATE: NonZero<u32> = nz!(44100);
pub const CHANNEL_COUNT: NonZero<u16> = nz!(1);
pub const BUFFER_SIZE: usize = // echo canceller and livekit want 10ms of audio
    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;

pub const LEGACY_SAMPLE_RATE: NonZero<u32> = nz!(48000);
pub const LEGACY_CHANNEL_COUNT: NonZero<u16> = nz!(2);

pub const REPLAY_DURATION: Duration = Duration::from_secs(30);

pub fn init(cx: &mut App) {
    AudioSettings::register(cx);
    LIVE_SETTINGS.initialize(cx);
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
    #[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
    pub echo_canceller: Arc<Mutex<apm::AudioProcessingModule>>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    replays: replays::Replays,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            output_handle: Default::default(),
            output_mixer: Default::default(),
            #[cfg(not(any(
                all(target_os = "windows", target_env = "gnu"),
                target_os = "freebsd"
            )))]
            echo_canceller: Arc::new(Mutex::new(apm::AudioProcessingModule::new(
                true, false, false, false,
            ))),
            source_cache: Default::default(),
            replays: Default::default(),
        }
    }
}

fn automatic_gain_control_settings() -> AutomaticGainControlSettings {
    AutomaticGainControlSettings {
        target_level: 0.9,
        attack_time: Duration::from_secs_f32(1f32),
        release_time: Duration::from_secs_f32(0f32),
        absolute_max_gain: 5.0,
    }
}

impl Global for Audio {}

impl Audio {
    fn setup_mixer(&mut self) -> impl Source + use<> {
        let (mixer, source) = rodio::mixer::mixer(CHANNEL_COUNT, SAMPLE_RATE);
        // or the mixer will end immediately as its empty.
        mixer.add(rodio::source::Zero::new(CHANNEL_COUNT, SAMPLE_RATE));
        // The webrtc apm is not yet compiling for windows & freebsd
        #[cfg(not(any(
            any(all(target_os = "windows", target_env = "gnu")),
            target_os = "freebsd"
        )))]
        let echo_canceller = Arc::clone(&self.echo_canceller);
        #[cfg(not(any(
            any(all(target_os = "windows", target_env = "gnu")),
            target_os = "freebsd"
        )))]
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
        self.output_mixer = Some(mixer);
        source
    }

    fn ensure_output_exists(&mut self) -> Result<&Mixer> {
        #[cfg(debug_assertions)]
        log::warn!(
            "Audio does not sound correct without optimizations. Use a release build to debug audio issues"
        );

        if self.output_mixer.is_none() {
            let mixer_source = self.setup_mixer();
            let output_handle = OutputStreamBuilder::open_default_stream()
                .context("Could not open default output stream")?;
            info!("Output stream: {:?}", output_handle);
            output_handle.mixer().add(mixer_source);
            self.output_handle = Some(output_handle);
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

    #[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
    pub fn input_pipeline(
        voip_parts: VoipParts,
        raw_mic_input: impl Source,
    ) -> anyhow::Result<impl Source> {
        let stream = raw_mic_input
            .possibly_disconnected_channels_to_mono()
            .constant_samplerate(SAMPLE_RATE)
            .limit(LimitSettings::live_performance());
            // .process_buffer::<BUFFER_SIZE, _>(move |buffer| {
            //     let mut int_buffer: [i16; _] = buffer.map(|s| s.to_sample());
            //     if voip_parts
            //         .echo_canceller
            //         .lock()
            //         .process_stream(
            //             &mut int_buffer,
            //             SAMPLE_RATE.get() as i32,
            //             CHANNEL_COUNT.get() as i32,
            //         )
            //         .context("livekit audio processor error")
            //         .log_err()
            //         .is_some()
            //     {
            //         for (sample, processed) in buffer.iter_mut().zip(&int_buffer) {
            //             *sample = (*processed).to_sample();
            //         }
            //     }
            // });
        // .denoise()
        // .context("Could not set up denoiser")?
        // .automatic_gain_control(automatic_gain_control_settings())
        // .periodic_access(Duration::from_millis(100), move |agc_source| {
        //     agc_source
        //         .set_enabled(LIVE_SETTINGS.auto_microphone_volume.load(Ordering::Relaxed));
        //     let denoise = agc_source.inner_mut();
        //     denoise
        //         .set_enabled(LIVE_SETTINGS.denoise.load(Ordering::Relaxed))
        //         .unwrap(); // todo make this log?
        // });

        let stream = if voip_parts.legacy_audio_compatible {
            stream.constant_params(LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE)
        } else {
            stream.constant_params(CHANNEL_COUNT, SAMPLE_RATE)
        };

        // the audio processing module gives us half its buffer duration
        // of empty samples. We can skip those.
        const APM_CHUNK_DURATION: Duration = Duration::from_millis(10);
        let stream = stream.skip_duration(APM_CHUNK_DURATION / 2);

        let (replay, stream) = stream.replayable(REPLAY_DURATION)?;
        voip_parts
            .replays
            .add_voip_stream("local microphone".to_string(), replay);

        Ok(stream)
    }

    #[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
    pub fn open_microphone(voip_parts: VoipParts) -> anyhow::Result<impl Source> {
        let stream = rodio::microphone::MicrophoneBuilder::new()
            .default_device()?
            .default_config()?
            .prefer_sample_rates([
                SAMPLE_RATE, // sample rates trivially resamplable to `SAMPLE_RATE`
                SAMPLE_RATE.saturating_mul(nz!(2)),
            ])
            .prefer_channel_counts([nz!(1), nz!(2), nz!(3), nz!(4)])
            .prefer_buffer_sizes(512..)
            .open_stream()?;
        info!("Opened microphone: {:?}", stream.config());
        Self::input_pipeline(voip_parts, stream)
    }

    pub fn play_voip_stream(
        source: impl rodio::Source + Send + 'static,
        speaker_name: String,
        is_staff: bool,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let (replay_source, source) = source
            .constant_params(CHANNEL_COUNT, SAMPLE_RATE)
            .automatic_gain_control(automatic_gain_control_settings())
            .periodic_access(Duration::from_millis(100), move |agc_source| {
                agc_source.set_enabled(LIVE_SETTINGS.auto_speaker_volume.load(Ordering::Relaxed));
            })
            .replayable(REPLAY_DURATION)
            .expect("REPLAY_DURATION is longer than 100ms");

        cx.update_default_global(|this: &mut Self, _cx| {
            let output_mixer = this
                .ensure_output_exists()
                .context("Could not get output mixer")?;
            output_mixer.add(source);
            if is_staff {
                this.replays.add_voip_stream(speaker_name, replay_source);
            }
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

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
pub struct VoipParts {
    echo_canceller: Arc<Mutex<apm::AudioProcessingModule>>,
    replays: replays::Replays,
    legacy_audio_compatible: bool,
}

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
impl VoipParts {
    pub fn new(cx: &AsyncApp) -> anyhow::Result<Self> {
        let (apm, replays) = cx.try_read_default_global::<Audio, _>(|audio, _| {
            (Arc::clone(&audio.echo_canceller), audio.replays.clone())
        })?;
        let legacy_audio_compatible =
            AudioSettings::try_read_global(cx, |settings| settings.legacy_audio_compatible)
                .unwrap_or(true);

        Ok(Self {
            legacy_audio_compatible,
            echo_canceller: apm,
            replays,
        })
    }
}
