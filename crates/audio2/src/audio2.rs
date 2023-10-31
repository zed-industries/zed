use assets::SoundRegistry;
use futures::{channel::mpsc, StreamExt};
use gpui2::{AppContext, AssetSource, Executor};
use rodio::{OutputStream, OutputStreamHandle};
use util::ResultExt;

mod assets;

pub fn init(source: impl AssetSource, cx: &mut AppContext) {
    cx.set_global(Audio::new(cx.executor()));
    cx.set_global(SoundRegistry::new(source));
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
    tx: mpsc::UnboundedSender<Box<dyn FnOnce(&mut AudioState) + Send>>,
}

struct AudioState {
    _output_stream: Option<OutputStream>,
    output_handle: Option<OutputStreamHandle>,
}

impl AudioState {
    fn ensure_output_exists(&mut self) -> Option<&OutputStreamHandle> {
        if self.output_handle.is_none() {
            let (_output_stream, output_handle) = OutputStream::try_default().log_err().unzip();
            self.output_handle = output_handle;
            self._output_stream = _output_stream;
        }

        self.output_handle.as_ref()
    }

    fn take(&mut self) {
        self._output_stream.take();
        self.output_handle.take();
    }
}

impl Audio {
    pub fn new(executor: &Executor) -> Self {
        let (tx, mut rx) = mpsc::unbounded::<Box<dyn FnOnce(&mut AudioState) + Send>>();
        executor
            .spawn_on_main(|| async move {
                let mut audio = AudioState {
                    _output_stream: None,
                    output_handle: None,
                };

                while let Some(f) = rx.next().await {
                    (f)(&mut audio);
                }
            })
            .detach();

        Self { tx }
    }

    pub fn play_sound(sound: Sound, cx: &mut AppContext) {
        if !cx.has_global::<Self>() {
            return;
        }

        let Some(source) = SoundRegistry::global(cx).get(sound.file()).log_err() else {
            return;
        };

        let this = cx.global::<Self>();
        this.tx
            .unbounded_send(Box::new(move |state| {
                if let Some(output_handle) = state.ensure_output_exists() {
                    output_handle.play_raw(source).log_err();
                }
            }))
            .ok();
    }

    pub fn end_call(cx: &AppContext) {
        if !cx.has_global::<Self>() {
            return;
        }

        let this = cx.global::<Self>();

        this.tx
            .unbounded_send(Box::new(move |state| state.take()))
            .ok();
    }
}
