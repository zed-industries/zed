use assets::SoundRegistry;
use derive_more::{Deref, DerefMut};
use gpui::{App, AssetSource, BorrowAppContext, Global};
use rodio::{OutputStream, OutputStreamBuilder};
use util::ResultExt;

mod assets;

pub fn init(source: impl AssetSource, cx: &mut App) {
    SoundRegistry::set_global(source, cx);
    cx.set_global(GlobalAudio(Audio::new()));
}

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
}

#[derive(Deref, DerefMut)]
struct GlobalAudio(Audio);

impl Global for GlobalAudio {}

impl Audio {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_output_exists(&mut self) -> Option<&OutputStream> {
        if self.output_handle.is_none() {
            self.output_handle = OutputStreamBuilder::open_default_stream().log_err();
        }

        self.output_handle.as_ref()
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        if !cx.has_global::<GlobalAudio>() {
            return;
        }

        cx.update_global::<GlobalAudio, _>(|this, cx| {
            let output_handle = this.ensure_output_exists()?;
            let source = SoundRegistry::global(cx).get(sound.file()).log_err()?;
            output_handle.mixer().add(source);
            Some(())
        });
    }

    pub fn end_call(cx: &mut App) {
        if !cx.has_global::<GlobalAudio>() {
            return;
        }

        cx.update_global::<GlobalAudio, _>(|this, _| {
            this.output_handle.take();
        });
    }
}
