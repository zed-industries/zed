use ashpd::zbus::{
    Connection, Proxy,
    fdo::PropertiesProxy,
    proxy::{Builder, CacheProperties},
};
use futures::{StreamExt as _, stream};
use gpui::PowerMode;

// The UPower namespace replaced net.hadess in v0.20; prefer it while retaining
// compatibility with older daemons.
// https://upower.pages.freedesktop.org/power-profiles-daemon/gdbus-org.freedesktop.UPower.PowerProfiles.html
// https://gitlab.freedesktop.org/upower/power-profiles-daemon/-/releases/0.20
const POWER_PROFILES_SERVICES: [(&str, &str); 2] = [
    (
        "org.freedesktop.UPower.PowerProfiles",
        "/org/freedesktop/UPower/PowerProfiles",
    ),
    ("net.hadess.PowerProfiles", "/net/hadess/PowerProfiles"),
];

fn power_mode(profile: &str) -> PowerMode {
    match profile {
        "power-saver" => PowerMode::LowPower,
        "balanced" => PowerMode::Balanced,
        "performance" => PowerMode::HighPower,
        _ => PowerMode::Unknown,
    }
}

pub(super) async fn monitor_power_mode(
    connection: &Connection,
    mut on_change: impl FnMut(PowerMode) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let mut proxies = Vec::new();
    let mut changes = Vec::new();
    for (service, path) in POWER_PROFILES_SERVICES {
        let proxy = Builder::<Proxy<'_>>::new(connection)
            .destination(service)?
            .path(path)?
            .interface(service)?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;
        let properties = PropertiesProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;
        changes.push(
            properties
                .receive_properties_changed()
                .await?
                .map(|_| ())
                .boxed(),
        );
        changes.push(proxy.receive_owner_changed().await?.map(|_| ()).boxed());
        proxies.push(proxy);
    }
    let mut changes = stream::select_all(changes);
    let mut previous_mode = None;
    loop {
        // Subscribe before reading, and bypass property caches so service loss
        // cannot leave us reporting the previous owner's profile.
        let mut mode = PowerMode::Unknown;
        for proxy in &proxies {
            match proxy.get_property::<String>("ActiveProfile").await {
                Ok(profile) => {
                    mode = power_mode(&profile);
                    break;
                }
                Err(error) => log::debug!(
                    "could not query power profile from {}: {error}",
                    proxy.destination()
                ),
            }
        }
        if previous_mode != Some(mode) {
            on_change(mode)?;
            previous_mode = Some(mode);
        }
        if changes.next().await.is_none() {
            on_change(PowerMode::Unknown)?;
            anyhow::bail!("power profile notification streams closed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context as _;
    use futures::channel::mpsc;
    use std::{
        collections::HashMap,
        io::{BufRead as _, BufReader},
        process::{Child, Stdio},
        sync::Arc,
        time::Duration,
    };

    struct TestBus(Child);

    impl TestBus {
        #[expect(
            clippy::disallowed_methods,
            reason = "this integration test owns and reaps a real D-Bus daemon"
        )]
        fn start() -> anyhow::Result<(Self, String)> {
            let child = gpui_util::new_std_command("dbus-daemon")
                .args(["--session", "--nofork", "--print-address=1"])
                .stdout(Stdio::piped())
                .spawn()?;
            let mut bus = Self(child);
            let stdout = bus
                .0
                .stdout
                .take()
                .context("missing D-Bus address output")?;
            let mut address = String::new();
            BufReader::new(stdout).read_line(&mut address)?;
            Ok((bus, address.trim().to_owned()))
        }
    }

    impl Drop for TestBus {
        fn drop(&mut self) {
            if let Err(error) = self.0.kill() {
                log::warn!("failed to stop test D-Bus daemon: {error}");
            }
            if let Err(error) = self.0.wait() {
                log::warn!("failed to reap test D-Bus daemon: {error}");
            }
        }
    }

    macro_rules! mock_profiles {
        ($name:ident, $interface:literal) => {
            struct $name(Arc<parking_lot::Mutex<String>>);

            #[ashpd::zbus::interface(name = $interface, crate = "ashpd::zbus")]
            impl $name {
                #[zbus(property)]
                fn active_profile(&self) -> String {
                    self.0.lock().clone()
                }
            }
        };
    }

    mock_profiles!(ModernProfiles, "org.freedesktop.UPower.PowerProfiles");
    mock_profiles!(LegacyProfiles, "net.hadess.PowerProfiles");

    #[test]
    fn maps_profiles_without_guessing_unknown_values() {
        for (profile, expected) in [
            ("power-saver", PowerMode::LowPower),
            ("balanced", PowerMode::Balanced),
            ("performance", PowerMode::HighPower),
            ("custom", PowerMode::Unknown),
            ("", PowerMode::Unknown),
        ] {
            assert_eq!(power_mode(profile), expected);
        }
    }

    #[test]
    #[expect(
        clippy::disallowed_methods,
        reason = "real D-Bus I/O requires a wall-clock timeout, not a GPUI scheduler timer"
    )]
    fn monitors_profiles_and_service_restarts() -> anyhow::Result<()> {
        smol::block_on(futures::future::select(
            Box::pin(check_service_lifecycle()),
            Box::pin(async {
                smol::Timer::after(Duration::from_secs(15)).await;
                anyhow::bail!("timed out monitoring mock power profiles")
            }),
        ))
        .factor_first()
        .0
    }

    async fn check_service_lifecycle() -> anyhow::Result<()> {
        let (_bus, address) = TestBus::start()?;
        let modern_profile = Arc::new(parking_lot::Mutex::new("balanced".to_owned()));
        let legacy_profile = Arc::new(parking_lot::Mutex::new("balanced".to_owned()));
        let server = ashpd::zbus::connection::Builder::address(address.as_str())?
            .serve_at(
                POWER_PROFILES_SERVICES[0].1,
                ModernProfiles(modern_profile.clone()),
            )?
            .serve_at(
                POWER_PROFILES_SERVICES[1].1,
                LegacyProfiles(legacy_profile.clone()),
            )?
            .build()
            .await?;
        let connection = ashpd::zbus::connection::Builder::address(address.as_str())?
            .build()
            .await?;
        let (sender, mut receiver) = mpsc::unbounded();
        let monitor = smol::spawn({
            let connection = connection.clone();
            async move {
                monitor_power_mode(&connection, |mode| {
                    sender.unbounded_send(mode)?;
                    Ok(())
                })
                .await
            }
        });
        assert_eq!(receiver.next().await, Some(PowerMode::Unknown));

        for ((service, path), active_profile) in POWER_PROFILES_SERVICES
            .into_iter()
            .zip([&modern_profile, &legacy_profile])
        {
            server.request_name(service).await?;
            assert_eq!(receiver.next().await, Some(PowerMode::Balanced));
            for (profile, expected) in [
                ("power-saver", PowerMode::LowPower),
                ("performance", PowerMode::HighPower),
                ("custom", PowerMode::Unknown),
                ("balanced", PowerMode::Balanced),
            ] {
                *active_profile.lock() = profile.to_owned();
                server
                    .emit_signal(
                        None::<&str>,
                        path,
                        "org.freedesktop.DBus.Properties",
                        "PropertiesChanged",
                        &(
                            service,
                            HashMap::from([(
                                "ActiveProfile",
                                ashpd::zbus::zvariant::Value::from(profile),
                            )]),
                            Vec::<&str>::new(),
                        ),
                    )
                    .await?;
                assert_eq!(receiver.next().await, Some(expected));
            }
            server.release_name(service).await?;
            assert_eq!(receiver.next().await, Some(PowerMode::Unknown));
            server.request_name(service).await?;
            assert_eq!(receiver.next().await, Some(PowerMode::Balanced));
            server.release_name(service).await?;
            assert_eq!(receiver.next().await, Some(PowerMode::Unknown));
        }
        *legacy_profile.lock() = "power-saver".to_owned();
        *modern_profile.lock() = "performance".to_owned();
        server.request_name(POWER_PROFILES_SERVICES[1].0).await?;
        assert_eq!(receiver.next().await, Some(PowerMode::LowPower));
        server.request_name(POWER_PROFILES_SERVICES[0].0).await?;
        assert_eq!(receiver.next().await, Some(PowerMode::HighPower));
        server.release_name(POWER_PROFILES_SERVICES[0].0).await?;
        assert_eq!(receiver.next().await, Some(PowerMode::LowPower));
        server.release_name(POWER_PROFILES_SERVICES[1].0).await?;
        assert_eq!(receiver.next().await, Some(PowerMode::Unknown));
        drop(monitor);
        Ok(())
    }
}
