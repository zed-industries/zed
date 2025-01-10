//! Provides a [calloop] event source from [XDG Desktop Portal] events
//!
//! This module uses the [ashpd] crate

use ashpd::desktop::settings::{ColorScheme, Settings};
use calloop::channel::Channel;
use calloop::{EventSource, Poll, PostAction, Readiness, Token, TokenFactory};
use smol::stream::StreamExt;

use crate::{BackgroundExecutor, WindowAppearance};

pub enum Event {
    WindowAppearance(WindowAppearance),
    #[cfg_attr(feature = "x11", allow(dead_code))]
    CursorTheme(String),
    #[cfg_attr(feature = "x11", allow(dead_code))]
    CursorSize(u32),
}

pub struct XDPEventSource {
    channel: Channel<Event>,
}

impl XDPEventSource {
    pub fn new(executor: &BackgroundExecutor) -> Self {
        let (sender, channel) = calloop::channel::channel();

        let background = executor.clone();

        executor
            .spawn(async move {
                let settings = Settings::new().await?;

                if let Ok(initial_appearance) = settings.color_scheme().await {
                    sender.send(Event::WindowAppearance(WindowAppearance::from_native(
                        initial_appearance,
                    )))?;
                }
                if let Ok(initial_theme) = settings
                    .read::<String>("org.gnome.desktop.interface", "cursor-theme")
                    .await
                {
                    sender.send(Event::CursorTheme(initial_theme))?;
                }

                // If u32 is used here, it throws invalid type error
                if let Ok(initial_size) = settings
                    .read::<i32>("org.gnome.desktop.interface", "cursor-size")
                    .await
                {
                    sender.send(Event::CursorSize(initial_size as u32))?;
                }

                if let Ok(mut cursor_theme_changed) = settings
                    .receive_setting_changed_with_args(
                        "org.gnome.desktop.interface",
                        "cursor-theme",
                    )
                    .await
                {
                    let sender = sender.clone();
                    background
                        .spawn(async move {
                            while let Some(theme) = cursor_theme_changed.next().await {
                                let theme = theme?;
                                sender.send(Event::CursorTheme(theme))?;
                            }
                            anyhow::Ok(())
                        })
                        .detach();
                }

                if let Ok(mut cursor_size_changed) = settings
                    .receive_setting_changed_with_args::<i32>(
                        "org.gnome.desktop.interface",
                        "cursor-size",
                    )
                    .await
                {
                    let sender = sender.clone();
                    background
                        .spawn(async move {
                            while let Some(size) = cursor_size_changed.next().await {
                                let size = size?;
                                sender.send(Event::CursorSize(size as u32))?;
                            }
                            anyhow::Ok(())
                        })
                        .detach();
                }

                let mut appearance_changed = settings.receive_color_scheme_changed().await?;
                while let Some(scheme) = appearance_changed.next().await {
                    sender.send(Event::WindowAppearance(WindowAppearance::from_native(
                        scheme,
                    )))?;
                }

                anyhow::Ok(())
            })
            .detach();

        Self { channel }
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

    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
    fn set_native(&mut self, cs: ColorScheme) {
        *self = Self::from_native(cs);
    }
}
