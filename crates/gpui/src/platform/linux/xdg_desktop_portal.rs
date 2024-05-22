//! Provides a [calloop] event source from [XDG Desktop Portal] events
//!
//! This module uses the [ashpd] crate and handles many async loop
use std::rc::{Weak, Rc};
use ashpd::desktop::settings::{ColorScheme, Settings};
use parking_lot::Mutex;
use smol::stream::StreamExt;

use crate::{BackgroundExecutor, ForegroundExecutor, WindowAppearance};

pub enum Event {
    WindowAppearance(WindowAppearance),
}

pub fn init_portal_listener(executor: &ForegroundExecutor, appearance: Weak<Mutex<WindowAppearance>>, appearance_changed_cb : Box<dyn FnMut()>) {
    executor
        .spawn(async move {
            if let Err(e) = observe_appearance(appearance, appearance_changed_cb).await {
                log::error!("{e}");
            }
        })
        .detach();
}

async fn observe_appearance(appearance: Weak<Mutex<WindowAppearance>>, mut appearance_changed_cb : Box<dyn FnMut()>) -> Result<(), anyhow::Error> {
    let settings = Settings::new().await?;

    // We get the color set before the initialization of the application
    let scheme = settings.color_scheme().await?;

    weak_try(&appearance)?.lock().set_native(scheme);
    (appearance_changed_cb)();

    // We observe the color change during the execution of the application
    let mut stream = settings.receive_color_scheme_changed().await?;
    while let Some(scheme) = stream.next().await {
        weak_try(&appearance)?.lock().set_native(scheme);
        (appearance_changed_cb)();
    }

    Ok(())
}

fn weak_try<T>(from: &Weak<T>) -> Result<Rc<T>, anyhow::Error> {
    if let Some(ret) = from.upgrade() {
        Ok(ret)
    } else {
        Err(anyhow::format_err!(
            "Trying to get a value from a drop reference"
        ))
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
