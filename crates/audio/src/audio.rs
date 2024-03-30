use assets::SoundRegistry;
use derive_more::{Deref, DerefMut};
use gpui::{AppContext, AssetSource, BorrowAppContext, Global};
use rodio::{OutputStream, OutputStreamHandle};
use util::ResultExt;

mod assets;

pub fn init(source: impl AssetSource, cx: &mut AppContext) {
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
        }
    }
}

pub struct Audio {
    _output_stream: Option<OutputStream>,
    output_handle: Option<OutputStreamHandle>,
}

#[derive(Deref, DerefMut)]
struct GlobalAudio(Audio);

impl Global for GlobalAudio {}

impl Audio {
    pub fn new() -> Self {
        Self {
            _output_stream: None,
            output_handle: None,
        }
    }

    fn ensure_output_exists(&mut self) -> Option<&OutputStreamHandle> {
        if self.output_handle.is_none() {
            let (_output_stream, output_handle) = OutputStream::try_default().log_err().unzip();
            self.output_handle = output_handle;
            self._output_stream = _output_stream;
        }

        self.output_handle.as_ref()
    }

    pub fn play_sound(sound: Sound, cx: &mut AppContext) {
        if !cx.has_global::<GlobalAudio>() {
            return;
        }

        cx.update_global::<GlobalAudio, _>(|this, cx| {
            let output_handle = this.ensure_output_exists()?;
            let source = SoundRegistry::global(cx).get(sound.file()).log_err()?;
            output_handle.play_raw(source).log_err()?;
            Some(())
        });
    }

    pub fn end_call(cx: &mut AppContext) {
        if !cx.has_global::<GlobalAudio>() {
            return;
        }

        cx.update_global::<GlobalAudio, _>(|this, _| {
            this._output_stream.take();
            this.output_handle.take();
        });
    }
}
