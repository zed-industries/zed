//! The [`Application`] type: building an app, configuring it before launch, and
//! starting the platform run loop that drives it.

use anyhow::Result;
use gpui_authoring::{
    App, AppCell, AssetSource, AsyncApp, BackgroundExecutor, DefaultTextSystem, ForegroundExecutor,
    FramePipeline, LayoutEngine, Platform, QuitMode, WindowId, http_client::HttpClient,
};
use std::{ffi::OsString, path::PathBuf, rc::Rc, sync::Arc};

/// A reference to a GPUI application, typically constructed in the `main` function of your app.
/// You won't interact with this type much outside of initial configuration and startup.
pub struct Application(Rc<AppCell>);

/// A strong handle to an [`Application`] started with [`Application::run_embedded`].
///
/// Dropping this handle releases the app, so an embedder must hold it for as long as the
/// app should run. While held, it is the embedder's entry point back into GPUI each time
/// the external run loop gives it control.
pub struct ApplicationHandle {
    app: Rc<AppCell>,
}

impl ApplicationHandle {
    /// Invoke `f` with the app context. Must not be called re-entrantly from code that
    /// is already inside an update; the app state is a `RefCell` and will panic on a
    /// double borrow.
    pub fn update<R>(&self, f: impl FnOnce(&mut App) -> R) -> R {
        let cx = &mut *self.app.borrow_mut();
        f(cx)
    }

    /// An [`AsyncApp`] for use across await points. It holds the app weakly; keeping the
    /// app alive remains this handle's job.
    pub fn to_async(&self) -> AsyncApp {
        self.update(|cx| cx.to_async())
    }
}

/// Represents an application before it is fully launched. Once your app is
/// configured, you'll start the app with `App::run`.
impl Application {
    /// Builds an app with a caller-provided platform implementation.
    pub fn with_platform(platform: Rc<dyn Platform>) -> Self {
        Self(App::new_app_for_platform(platform))
    }

    /// Builds an app with accessibility (AccessKit) integration forcibly
    /// disabled.
    ///
    /// In this mode, accessibility APIs (e.g.
    /// [`div().role()`][gpui_authoring::StatefulInteractiveElement::role])
    /// silently no-op.
    ///
    /// See the [accessibility guide](gpui_authoring::_accessibility) for an
    /// overview of the features this disables.
    pub fn new_inaccessible(platform: Rc<dyn Platform>) -> Self {
        let this = Self::with_platform(platform);
        this.0.borrow_mut().set_accessibility_force_disabled(true);
        this
    }

    /// Assigns the source of assets for the application.
    pub fn with_assets(self, asset_source: impl AssetSource) -> Self {
        self.0.borrow_mut().set_asset_source(Arc::new(asset_source));
        self
    }

    /// Configures arguments to pass when restarting the application.
    pub fn with_restart_arguments(self, arguments: Vec<OsString>) -> Self {
        self.0.borrow_mut().set_restart_arguments(arguments);
        self
    }

    /// Sets the HTTP client for the application.
    pub fn with_http_client(self, http_client: Arc<dyn HttpClient>) -> Self {
        self.0.borrow_mut().set_http_client(http_client);
        self
    }

    /// Configures when the application should automatically quit.
    /// By default, [`QuitMode::Default`] is used.
    pub fn with_quit_mode(self, mode: QuitMode) -> Self {
        self.0.borrow_mut().set_quit_mode(mode);
        self
    }

    /// Sets the factory that creates each window's layout engine.
    ///
    /// Defaults to the engine bundled with GPUI. Supply a different
    /// implementation to swap the layout engine without touching windows.
    pub fn with_layout_engine(
        self,
        layout_engine: impl Fn() -> Box<dyn LayoutEngine> + 'static,
    ) -> Self {
        self.0
            .borrow_mut()
            .set_layout_engine_factory(Rc::new(layout_engine));
        self
    }

    /// Sets the factory that creates each window's frame pipeline.
    ///
    /// Defaults to the immediate-mode pipeline bundled with GPUI, which
    /// re-evaluates the view tree every frame. Supply a different implementation
    /// to change how a window draws its frames.
    pub fn with_frame_pipeline(
        self,
        frame_pipeline: impl Fn(WindowId) -> Box<dyn FramePipeline> + 'static,
    ) -> Self {
        self.0
            .borrow_mut()
            .set_frame_pipeline_factory(Rc::new(frame_pipeline));
        self
    }

    /// Start the application. The provided callback will be called once the
    /// app is fully launched.
    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut App),
    {
        let this = self.0.clone();
        let platform = self.0.borrow().platform();
        platform.run(Box::new(move || {
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
    }

    /// Start the application for an embedder that drives the run loop itself.
    ///
    /// On ordinary platforms `Platform::run` blocks for the lifetime of the app, and the
    /// app state is kept alive by [`Application::run`]'s stack frame. Embedded platforms —
    /// where the run loop belongs to someone else, e.g. GPUI compiled into a Wasm guest,
    /// or a GPUI view hosted inside a foreign native application — implement
    /// `Platform::run` to invoke the launch callback and return immediately. This method
    /// supports that shape: it returns an [`ApplicationHandle`] that keeps the app alive
    /// and lets the embedder re-enter it whenever the external run loop yields control.
    pub fn run_embedded<F>(self, on_finish_launching: F) -> ApplicationHandle
    where
        F: 'static + FnOnce(&mut App),
    {
        let this = self.0.clone();
        let platform = self.0.borrow().platform();
        platform.run(Box::new(move || {
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
        ApplicationHandle { app: self.0 }
    }

    /// Register a handler to be invoked when the platform instructs the application
    /// to open one or more URLs.
    pub fn on_open_urls<F>(&self, callback: F) -> &Self
    where
        F: 'static + FnMut(Vec<String>),
    {
        let platform = self.0.borrow().platform();
        platform.on_open_urls(Box::new(callback));
        self
    }

    /// Invokes a handler when an already-running application is launched.
    /// On macOS, this can occur when the application icon is double-clicked or the app is launched via the dock.
    pub fn on_reopen<F>(&self, mut callback: F) -> &Self
    where
        F: 'static + FnMut(&mut App),
    {
        let this = Rc::downgrade(&self.0);
        let platform = self.0.borrow().platform();
        platform.on_reopen(Box::new(move || {
            if let Some(app) = this.upgrade() {
                callback(&mut app.borrow_mut());
            }
        }));
        self
    }

    /// Returns a handle to the [`BackgroundExecutor`] associated with this app, which can be used to spawn futures in the background.
    pub fn background_executor(&self) -> BackgroundExecutor {
        self.0.borrow().background_executor().clone()
    }

    /// Returns a handle to the [`ForegroundExecutor`] associated with this app, which can be used to spawn futures in the foreground.
    pub fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.borrow().foreground_executor().clone()
    }

    /// Returns a reference to the [`DefaultTextSystem`] associated with this app.
    pub fn text_system(&self) -> Arc<DefaultTextSystem> {
        self.0.borrow().text_system().clone()
    }

    /// Returns the file URL of the executable with the specified name in the application bundle
    pub fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.0.borrow().path_for_auxiliary_executable(name)
    }
}
