use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use gpui::{App, BorrowAppContext, Global};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Source, source::Buffered};
use settings::Settings;
use std::io::Cursor;
use util::ResultExt;

mod audio_settings;
pub use audio_settings::AudioSettings;

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

#[derive(Default)]
pub struct Audio {
    output_handle: Option<OutputStream>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self) -> Option<&OutputStream> {
        if self.output_handle.is_none() {
            self.output_handle = OutputStreamBuilder::open_default_stream().log_err();
        }

        self.output_handle.as_ref()
    }

    pub fn play_source(
        source: impl rodio::Source + Send + 'static,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        cx.update_default_global(|this: &mut Self, _cx| {
            let output_handle = this
                .ensure_output_exists()
                .ok_or_else(|| anyhow!("Could not open audio output"))?;
            output_handle.mixer().add(source);
            Ok(())
        })
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_handle = this.ensure_output_exists()?;
            output_handle.mixer().add(source);
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
