//! Provides a [calloop] event source from [XDG Desktop Portal] events
//!
//! This module uses the [ashpd] crate and handles many async loop
use std::future::Future;

use ashpd::desktop::settings::{ColorScheme, Settings};
use calloop::channel::{Channel, Sender};
use calloop::{EventSource, Poll, PostAction, Readiness, Token, TokenFactory};
use parking_lot::Mutex;
use smol::stream::StreamExt;
use util::ResultExt;

use crate::{BackgroundExecutor, WindowAppearance};

pub enum Event {
    WindowAppearance(WindowAppearance),
}

pub struct XDPEventSource {
    channel: Channel<Event>,
}

impl XDPEventSource {
    pub fn new(executor: &BackgroundExecutor) -> Self {
        let (sender, channel) = calloop::channel::channel();

        Self::spawn_observer(executor, Self::appearance_observer(sender.clone()));

        Self { channel }
    }

    fn spawn_observer(
        executor: &BackgroundExecutor,
        to_spawn: impl Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    ) {
        executor
            .spawn(async move {
                to_spawn.await.log_err();
            })
            .detach()
    }

    async fn appearance_observer(sender: Sender<Event>) -> Result<(), anyhow::Error> {
        let settings = Settings::new().await?;

        // We observe the color change during the execution of the application
        let mut stream = settings.receive_color_scheme_changed().await?;
        while let Some(scheme) = stream.next().await {
            sender.send(Event::WindowAppearance(WindowAppearance::from_native(
                scheme,
            )))?;
        }

        Ok(())
    }
}

impl EventSource for XDPEventSource {
    type Event = Event;
    type Metadata = ();
    type Ret = ();
    type Error = anyhow::Error;

    fn process_events<F>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.channel.process_events(readiness, token, |evt, _| {
            if let calloop::channel::Event::Msg(msg) = evt {
                (callback)(msg, &mut ())
            }
        })?;

        Ok(PostAction::Continue)
    }

    fn register(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.channel.register(poll, token_factory)?;

        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.channel.reregister(poll, token_factory)?;

        Ok(())
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.channel.unregister(poll)?;

        Ok(())
    }
}

impl WindowAppearance {
    fn from_native(cs: ColorScheme) -> WindowAppearance {
        match cs {
            ColorScheme::PreferDark => WindowAppearance::Dark,
            ColorScheme::PreferLight => WindowAppearance::Light,
            ColorScheme::NoPreference => WindowAppearance::Light,
        }
    }

    fn set_native(&mut self, cs: ColorScheme) {
        *self = Self::from_native(cs);
    }
}

pub fn window_appearance(executor: &BackgroundExecutor) -> Result<WindowAppearance, anyhow::Error> {
    executor.block(async {
        let settings = Settings::new().await?;

        let scheme = settings.color_scheme().await?;

        let appearance = WindowAppearance::from_native(scheme);

        Ok(appearance)
    })
}

pub fn should_auto_hide_scrollbars(executor: &BackgroundExecutor) -> Result<bool, anyhow::Error> {
    executor.block(async {
        let settings = Settings::new().await?;
        let auto_hide = settings
            .read::<bool>("org.gnome.desktop.interface", "overlay-scrolling")
            .await?;

        Ok(auto_hide)
    })
}
