use assets::SoundRegistry;
use gpui::{AppContext, AssetSource};
use rodio::{OutputStream, OutputStreamHandle};
use util::ResultExt;

mod assets;

pub fn init(source: impl AssetSource, cx: &mut AppContext) {
    cx.set_global(SoundRegistry::new(source));
    cx.set_global(Audio::new());
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

impl Audio {
    pub fn new() -> Self {
        let (_output_stream, output_handle) = OutputStream::try_default().log_err().unzip();

        Self {
            _output_stream,
            output_handle,
        }
    }

    pub fn play_sound(sound: Sound, cx: &AppContext) {
        if !cx.has_global::<Self>() {
            return;
        }

        let this = cx.global::<Self>();

        let Some(output_handle) = this.output_handle.as_ref() else {
            return;
        };

        let Some(source) = SoundRegistry::global(cx).get(sound.file()).log_err() else {
        return;
    };

        output_handle.play_raw(source).log_err();
    }
}
