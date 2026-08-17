use std::sync::{Arc, Mutex};

use enumflags2::BitFlags;
use futures_util::{StreamExt, task::SpawnExt};
use zbus::names::{OwnedWellKnownName, WellKnownName};

use crate::backend::{Result, session::SessionManager};

pub struct Builder {
    name: OwnedWellKnownName,
    flags: BitFlags<zbus::fdo::RequestNameFlags>,
    #[cfg(feature = "account")]
    account_impl: Option<Arc<dyn crate::backend::account::AccountImpl>>,
    access_impl: Option<Arc<dyn crate::backend::access::AccessImpl>>,
    app_chooser_impl: Option<Arc<dyn crate::backend::app_chooser::AppChooserImpl>>,
    #[cfg(feature = "background")]
    background_impl: Option<Arc<dyn crate::backend::background::BackgroundImpl>>,
    #[cfg(feature = "email")]
    email_impl: Option<Arc<dyn crate::backend::email::EmailImpl>>,
    #[cfg(feature = "file_chooser")]
    file_chooser_impl: Option<Arc<dyn crate::backend::file_chooser::FileChooserImpl>>,
    lockdown_impl: Option<Arc<dyn crate::backend::lockdown::LockdownImpl>>,
    #[cfg(feature = "documents")]
    permission_store_impl: Option<Arc<dyn crate::backend::permission_store::PermissionStoreImpl>>,
    #[cfg(feature = "print")]
    print_impl: Option<Arc<dyn crate::backend::print::PrintImpl>>,
    #[cfg(feature = "screencast")]
    screencast_impl: Option<Arc<dyn crate::backend::screencast::ScreencastImpl>>,
    #[cfg(feature = "screenshot")]
    screenshot_impl: Option<Arc<dyn crate::backend::screenshot::ScreenshotImpl>>,
    #[cfg(feature = "secret")]
    secret_impl: Option<Arc<dyn crate::backend::secret::SecretImpl>>,
    #[cfg(feature = "settings")]
    settings_impl: Option<Arc<dyn crate::backend::settings::SettingsImpl>>,
    #[cfg(feature = "wallpaper")]
    wallpaper_impl: Option<Arc<dyn crate::backend::wallpaper::WallpaperImpl>>,
    #[cfg(feature = "usb")]
    usb_impl: Option<Arc<dyn crate::backend::usb::UsbImpl>>,
    spawn: Option<Arc<dyn futures_util::task::Spawn + Send + Sync + 'static>>,
    name_lost: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    sessions: Arc<Mutex<SessionManager>>,
}

impl Builder {
    pub fn new<'a, W>(well_known_name: W) -> zbus::Result<Self>
    where
        W: TryInto<WellKnownName<'a>>,
        <W as TryInto<WellKnownName<'a>>>::Error: Into<zbus::Error>,
    {
        let well_known_name = well_known_name.try_into().map_err(Into::into)?;
        Ok(Self {
            name: well_known_name.into(),
            // same flags as zbus::Connection::request_name
            flags: zbus::fdo::RequestNameFlags::ReplaceExisting
                | zbus::fdo::RequestNameFlags::DoNotQueue,
            #[cfg(feature = "account")]
            account_impl: None,
            access_impl: None,
            app_chooser_impl: None,
            #[cfg(feature = "background")]
            background_impl: None,
            #[cfg(feature = "email")]
            email_impl: None,
            #[cfg(feature = "file_chooser")]
            file_chooser_impl: None,
            lockdown_impl: None,

            #[cfg(feature = "documents")]
            permission_store_impl: None,
            #[cfg(feature = "print")]
            print_impl: None,
            #[cfg(feature = "screencast")]
            screencast_impl: None,
            #[cfg(feature = "screenshot")]
            screenshot_impl: None,
            #[cfg(feature = "secret")]
            secret_impl: None,
            #[cfg(feature = "settings")]
            settings_impl: None,
            #[cfg(feature = "wallpaper")]
            wallpaper_impl: None,
            #[cfg(feature = "usb")]
            usb_impl: None,
            spawn: None,
            name_lost: None,
            sessions: Arc::new(Mutex::new(SessionManager::default())),
        })
    }

    pub fn with_flags(mut self, flags: BitFlags<zbus::fdo::RequestNameFlags>) -> Self {
        self.flags = flags;
        self
    }

    #[cfg(not(any(feature = "tokio")))]
    pub fn with_spawn(
        mut self,
        spawn: impl futures_util::task::Spawn + Send + Sync + 'static,
    ) -> Self {
        self.spawn = Some(Arc::new(spawn));
        self
    }

    pub fn with_name_lost(mut self, name_lost: impl Fn() + Send + Sync + 'static) -> Self {
        self.name_lost = Some(Arc::new(name_lost));
        self
    }

    #[cfg(feature = "account")]
    pub fn account(mut self, imp: impl crate::backend::account::AccountImpl + 'static) -> Self {
        self.account_impl = Some(Arc::new(imp));
        self
    }

    pub fn access(mut self, imp: impl crate::backend::access::AccessImpl + 'static) -> Self {
        self.access_impl = Some(Arc::new(imp));
        self
    }

    pub fn app_chooser(
        mut self,
        imp: impl crate::backend::app_chooser::AppChooserImpl + 'static,
    ) -> Self {
        self.app_chooser_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "background")]
    pub fn background(
        mut self,
        imp: impl crate::backend::background::BackgroundImpl + 'static,
    ) -> Self {
        self.background_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "email")]
    pub fn email(mut self, imp: impl crate::backend::email::EmailImpl + 'static) -> Self {
        self.email_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "file_chooser")]
    pub fn file_chooser(
        mut self,
        imp: impl crate::backend::file_chooser::FileChooserImpl + 'static,
    ) -> Self {
        self.file_chooser_impl = Some(Arc::new(imp));
        self
    }

    pub fn lockdown(mut self, imp: impl crate::backend::lockdown::LockdownImpl + 'static) -> Self {
        self.lockdown_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "documents")]
    pub fn permission_store(
        mut self,
        imp: impl crate::backend::permission_store::PermissionStoreImpl + 'static,
    ) -> Self {
        self.permission_store_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "print")]
    pub fn print(mut self, imp: impl crate::backend::print::PrintImpl + 'static) -> Self {
        self.print_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "screencast")]
    pub fn screencast(
        mut self,
        imp: impl crate::backend::screencast::ScreencastImpl + 'static,
    ) -> Self {
        self.screencast_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "screenshot")]
    pub fn screenshot(
        mut self,
        imp: impl crate::backend::screenshot::ScreenshotImpl + 'static,
    ) -> Self {
        self.screenshot_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "secret")]
    pub fn secret(mut self, imp: impl crate::backend::secret::SecretImpl + 'static) -> Self {
        self.secret_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "settings")]
    pub fn settings(mut self, imp: impl crate::backend::settings::SettingsImpl + 'static) -> Self {
        self.settings_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "wallpaper")]
    pub fn wallpaper(
        mut self,
        imp: impl crate::backend::wallpaper::WallpaperImpl + 'static,
    ) -> Self {
        self.wallpaper_impl = Some(Arc::new(imp));
        self
    }

    #[cfg(feature = "usb")]
    pub fn usb(mut self, imp: impl crate::backend::usb::UsbImpl + 'static) -> Self {
        self.usb_impl = Some(Arc::new(imp));
        self
    }

    pub async fn build(self) -> Result<()> {
        let connection = crate::proxy::Proxy::connection().await?;
        self.build_with_connection(connection).await
    }

    pub async fn build_with_connection(self, connection: zbus::Connection) -> Result<()> {
        #[cfg(feature = "tokio")]
        let spawn = self.spawn.unwrap_or(Arc::new(super::spawn::TokioSpawner));

        #[cfg(not(feature = "tokio"))]
        let spawn = self
            .spawn
            .expect("Must provide a spawner when not using tokio");

        if let Some(name_lost) = self.name_lost {
            let proxy = zbus::fdo::DBusProxy::new(&connection).await?;
            let mut name_lost_stream = proxy.receive_name_lost().await?;
            if let Err(error) = spawn.spawn(async move {
                while (name_lost_stream.next().await).is_some() {
                    name_lost();
                }
            }) {
                return Err(crate::PortalError::Failed(error.to_string()));
            }
        }

        let object_server = connection.object_server();
        #[cfg(feature = "account")]
        if let Some(imp) = self.account_impl {
            let portal = crate::backend::account::AccountInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Account`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        if let Some(imp) = self.access_impl {
            let portal = crate::backend::access::AccessInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Access`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        if let Some(imp) = self.app_chooser_impl {
            let portal = crate::backend::app_chooser::AppChooserInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.AppChooser`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "background")]
        if let Some(imp) = self.background_impl {
            let portal = crate::backend::background::BackgroundInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Background`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "email")]
        if let Some(imp) = self.email_impl {
            let portal = crate::backend::email::EmailInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Email`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "file_chooser")]
        if let Some(imp) = self.file_chooser_impl {
            let portal = crate::backend::file_chooser::FileChooserInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.FileChooser`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        if let Some(imp) = self.lockdown_impl {
            let portal = crate::backend::lockdown::LockdownInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Lockdown`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "documents")]
        if let Some(imp) = self.permission_store_impl {
            let portal = crate::backend::permission_store::PermissionStoreInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.PermissionStore`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "print")]
        if let Some(imp) = self.print_impl {
            let portal = crate::backend::print::PrintInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Print`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "screencast")]
        if let Some(imp) = self.screencast_impl {
            let portal = crate::backend::screencast::ScreencastInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
                Arc::clone(&self.sessions),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.ScreenCast`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "screenshot")]
        if let Some(imp) = self.screenshot_impl {
            let portal = crate::backend::screenshot::ScreenshotInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Screenshot`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "secret")]
        if let Some(imp) = self.secret_impl {
            let portal = crate::backend::secret::SecretInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Secret`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "settings")]
        if let Some(imp) = self.settings_impl {
            let portal = crate::backend::settings::SettingsInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Settings`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "wallpaper")]
        if let Some(imp) = self.wallpaper_impl {
            let portal = crate::backend::wallpaper::WallpaperInterface::new(
                imp,
                connection.clone(),
                Arc::clone(&spawn),
            );
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Wallpaper`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        #[cfg(feature = "usb")]
        if let Some(imp) = self.usb_impl {
            let portal =
                crate::backend::usb::UsbInterface::new(imp, connection.clone(), Arc::clone(&spawn));
            #[cfg(feature = "tracing")]
            tracing::debug!("Serving interface `org.freedesktop.impl.portal.Usb`");
            object_server
                .at("/org/freedesktop/portal/desktop", portal)
                .await?;
        }

        connection
            .request_name_with_flags(self.name, self.flags)
            .await?;

        Ok(())
    }
}
