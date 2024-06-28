//! Provides a [calloop] event source from [XDG Desktop Portal] events
//!
//! This module uses the [ashpd] crate

use std::sync::Arc;

use anyhow::anyhow;
use ashpd::desktop::settings::{ColorScheme, Settings};
use calloop::channel::{Channel, Sender};
use calloop::{EventSource, Poll, PostAction, Readiness, Token, TokenFactory};
use mio::Waker;
use smol::stream::StreamExt;

use crate::{BackgroundExecutor, WindowAppearance};

pub enum Event {
    WindowAppearance(WindowAppearance),
    CursorTheme(String),
    CursorSize(u32),
}

pub struct XDPEventSource {
    channel: Channel<Event>,
}

impl XDPEventSource {
    pub fn new(executor: &BackgroundExecutor, waker: Option<Arc<Waker>>) -> Self {
        let (sender, channel) = calloop::channel::channel();

        let background = executor.clone();

        executor
            .spawn(async move {
                fn send_event<T>(
                    sender: &Sender<T>,
                    waker: &Option<Arc<Waker>>,
                    event: T,
                ) -> Result<(), std::sync::mpsc::SendError<T>> {
                    sender.send(event)?;
                    if let Some(waker) = waker {
                        waker.wake().ok();
                    };
                    Ok(())
                }

                let settings = Settings::new().await?;

                if let Ok(initial_appearance) = settings.color_scheme().await {
                    send_event(
                        &sender,
                        &waker,
                        Event::WindowAppearance(WindowAppearance::from_native(initial_appearance)),
                    )?;
                }
                if let Ok(initial_theme) = settings
                    .read::<String>("org.gnome.desktop.interface", "cursor-theme")
                    .await
                {
                    send_event(&sender, &waker, Event::CursorTheme(initial_theme))?;
                }
                if let Ok(initial_size) = settings
                    .read::<u32>("org.gnome.desktop.interface", "cursor-size")
                    .await
                {
                    send_event(&sender, &waker, Event::CursorSize(initial_size))?;
                }

                if let Ok(mut cursor_theme_changed) = settings
                    .receive_setting_changed_with_args(
                        "org.gnome.desktop.interface",
                        "cursor-theme",
                    )
                    .await
                {
                    let sender = sender.clone();
                    let waker = waker.clone();
                    background
                        .spawn(async move {
                            while let Some(theme) = cursor_theme_changed.next().await {
                                let theme = theme?;
                                send_event(&sender, &waker, Event::CursorTheme(theme))?;
                            }
                            anyhow::Ok(())
                        })
                        .detach();
                }

                if let Ok(mut cursor_size_changed) = settings
                    .receive_setting_changed_with_args::<u32>(
                        "org.gnome.desktop.interface",
                        "cursor-size",
                    )
                    .await
                {
                    let sender = sender.clone();
                    let waker = waker.clone();
                    background
                        .spawn(async move {
                            while let Some(size) = cursor_size_changed.next().await {
                                let size = size?;
                                send_event(&sender, &waker, Event::CursorSize(size))?;
                            }
                            anyhow::Ok(())
                        })
                        .detach();
                }

                let mut appearance_changed = settings.receive_color_scheme_changed().await?;
                while let Some(scheme) = appearance_changed.next().await {
                    send_event(
                        &sender,
                        &waker,
                        Event::WindowAppearance(WindowAppearance::from_native(scheme)),
                    )?;
                }

                anyhow::Ok(())
            })
            .detach();

        Self { channel }
    }

    pub fn try_recv(&self) -> anyhow::Result<Event> {
        self.channel
            .try_recv()
            .map_err(|error| anyhow!("{}", error))
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

    #[cfg_attr(target_os = "linux", allow(dead_code))]
    fn set_native(&mut self, cs: ColorScheme) {
        *self = Self::from_native(cs);
    }
}
