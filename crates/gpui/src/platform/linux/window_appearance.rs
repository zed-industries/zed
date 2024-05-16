//! Manages the window appearance state in linux platforms
//!
//! The more common way to get the window appearance in linux platforms
//! consists of using xdg_portal made available in this project by the [ashpd]
//! crate.
//!
//! This module manages tasks used for automatically update the appearance from
//! the system and provide a fancy way to access it
use crate::{BackgroundExecutor, WindowAppearance};
use ashpd::desktop::settings::{ColorScheme, Settings};
use parking_lot::Mutex;
use smol::stream::StreamExt;
use std::{borrow::BorrowMut, sync::Arc};

pub(super) fn spawn_window_appearance_monitor(
    executor: &BackgroundExecutor,
) -> Arc<Mutex<WindowAppearance>> {
    let ret = Arc::new(Mutex::new(WindowAppearance::Light));
    let to_update = ret.clone();

    executor
        .spawn(async move {
            if let Err(e) = spawn_window_appearance_async(to_update).await {
                log::error!("{e}");
            }
        })
        .detach();

    ret
}

async fn spawn_window_appearance_async(
    to_update: Arc<Mutex<WindowAppearance>>,
) -> Result<(), ashpd::Error> {
    let settings = Settings::new().await?;

    log::info!("Initializing the window appearance monitor");

    // Get the initial color scheme value from the settings
    let color_scheme = settings.color_scheme().await?;
    to_update
        .lock()
        .borrow_mut()
        .replace_by_native(color_scheme);

    // Monitor the color scheme from settings
    let mut cs_stream = settings.receive_color_scheme_changed().await?;

    while let Some(color_scheme) = cs_stream.next().await {
        to_update
            .lock()
            .borrow_mut()
            .replace_by_native(color_scheme);

        log::info!(
            "Received a new appearance: {}",
            match color_scheme {
                ColorScheme::PreferDark => "dark",
                ColorScheme::PreferLight => "light",
                ColorScheme::NoPreference => "no preference",
            }
        );
    }

    Ok(())
}

impl WindowAppearance {
    fn from_native(cs: ColorScheme) -> WindowAppearance {
        match cs {
            ColorScheme::PreferDark => WindowAppearance::Dark,
            ColorScheme::PreferLight => WindowAppearance::Light,
            ColorScheme::NoPreference => WindowAppearance::Light,
        }
    }

    fn replace_by_native(&mut self, cs: ColorScheme) {
        *self = Self::from_native(cs);
    }
}
