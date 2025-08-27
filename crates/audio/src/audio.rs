use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use gpui::{App, BorrowAppContext, Global};
use libwebrtc::native::apm;
use parking_lot::Mutex;
use rodio::{
    Decoder, OutputStream, OutputStreamBuilder, Source, cpal::Sample, mixer::Mixer,
    source::Buffered,
};
use settings::Settings;
use std::{io::Cursor, num::NonZero, sync::Arc};
use util::ResultExt;

mod audio_settings;
mod rodio_ext;
pub use audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

// NOTE: We use WebRTC's mixer which only supports
// 16kHz, 32kHz and 48kHz. As 48 is the most common "next step up"
// for audio output devices like speakers/bluetooth, we just hard-code
// this; and downsample when we need to.
//
// Since most noise cancelling requires 16kHz we will move to
// that in the future. Same for channel count. That should be input
// channels and fixed to 1.
pub const SAMPLE_RATE: NonZero<u32> = NonZero::new(48000).expect("not zero");
pub const CHANNEL_COUNT: NonZero<u16> = NonZero::new(2).expect("not zero");

pub fn init(cx: &mut App) {
    AudioSettings::register(cx);
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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
        }
    }
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self) -> Option<&Mixer> {
        if self.output_handle.is_none() {
            self.output_handle = OutputStreamBuilder::open_default_stream().log_err();
            if let Some(output_handle) = &self.output_handle {
                let (mixer, source) = rodio::mixer::mixer(CHANNEL_COUNT, SAMPLE_RATE);
                self.output_mixer = Some(mixer);

                let echo_canceller = Arc::clone(&self.echo_canceller);
                const BUFFER_SIZE: usize = // echo canceller wants 10ms of audio
                    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;
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

        self.output_mixer.as_ref()
    }

    pub fn play_source(
        source: impl rodio::Source + Send + 'static,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        cx.update_default_global(|this: &mut Self, _cx| {
            let output_mixer = this
                .ensure_output_exists()
                .ok_or_else(|| anyhow!("Could not open audio output"))?;
            output_mixer.add(source);
            Ok(())
        })
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_mixer = this.ensure_output_exists()?;

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
