use super::window::IosWindowState;
use gpui::{AppLifecyclePhase, WindowVisibility};
use objc2::{
    ClassType, DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, extern_class,
    extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::AnyObject,
    sel,
};
use objc2_foundation::{
    NSDictionary, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes, NSSet, NSString,
};
use objc2_quartz_core::CADisplayLink;
use objc2_ui_kit::{
    UIApplication, UIApplicationDelegate, UIApplicationLaunchOptionsKey, UIOpenURLContext, UIScene,
    UISceneConfiguration, UISceneConnectionOptions, UISceneDelegate, UISceneSession, UIWindowScene,
    UIWindowSceneDelegate,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Default)]
struct IosPlatformState {
    running: bool,
    finish_launching: Option<Box<dyn FnOnce()>>,
    quit: Option<Box<dyn FnMut()>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    app_lifecycle: Option<Box<dyn FnMut(AppLifecyclePhase)>>,
    memory_warning: Option<Box<dyn FnMut()>>,
    scene: Option<Retained<UIWindowScene>>,
    windows: Vec<Weak<IosWindowState>>,
}

thread_local! {
    static APP_STATE: RefCell<IosPlatformState> = RefCell::default();
}

pub(super) fn run(on_finish_launching: Box<dyn FnOnce()>) {
    let main_thread = MainThreadMarker::new().expect("UIKit requires the main thread");
    APP_STATE.with_borrow_mut(|state| {
        assert!(!state.running, "the iOS application is already running");
        state.running = true;
        state.finish_launching = Some(on_finish_launching);
    });
    // UIKit can restore a scene delegate by class name without asking for a new configuration.
    SceneDelegate::class();
    let delegate_class = NSString::from_class(AppDelegate::class());
    UIApplication::main(None, Some(&delegate_class), main_thread);
}

pub(super) fn window_scene() -> Option<Retained<UIWindowScene>> {
    APP_STATE.with_borrow(|state| state.scene.clone())
}

pub(super) fn register_window(window: &Rc<IosWindowState>) {
    APP_STATE.with_borrow_mut(|state| state.windows.push(Rc::downgrade(window)));
}

pub(super) fn unregister_window(window: &Rc<IosWindowState>) {
    let window = Rc::downgrade(window);
    APP_STATE.with_borrow_mut(|state| state.windows.retain(|entry| !entry.ptr_eq(&window)));
}

pub(super) fn with_windows(mut callback: impl FnMut(&IosWindowState)) {
    // Native callbacks may open or close windows. Do not hold a registry borrow
    // across them, and skip snapshot entries that have since been unregistered.
    let windows = APP_STATE.with_borrow(|state| state.windows.clone());
    for window in windows {
        if APP_STATE.with_borrow(|state| state.windows.iter().any(|entry| entry.ptr_eq(&window))) {
            // The snapshot's Weak keeps its allocation identity from being reused.
            // The strong guard survives synchronous closure of this window.
            if let Some(window) = window.upgrade() {
                callback(&window);
            }
        }
    }
}

pub(super) fn set_quit_callback(callback: Box<dyn FnMut()>) {
    APP_STATE.with_borrow_mut(|state| state.quit = Some(callback));
}

pub(super) fn set_open_urls_callback(callback: Box<dyn FnMut(Vec<String>)>) {
    APP_STATE.with_borrow_mut(|state| state.open_urls = Some(callback));
}

pub(super) fn set_app_lifecycle_callback(callback: Box<dyn FnMut(AppLifecyclePhase)>) {
    APP_STATE.with_borrow_mut(|state| state.app_lifecycle = Some(callback));
}

pub(super) fn set_memory_warning_callback(callback: Box<dyn FnMut()>) {
    APP_STATE.with_borrow_mut(|state| state.memory_warning = Some(callback));
}

fn finish_launching() {
    let callback = APP_STATE.with_borrow_mut(|state| state.finish_launching.take());
    if let Some(callback) = callback {
        callback();
    }
}

fn notify_memory_warning() {
    let callback = APP_STATE.with_borrow_mut(|state| state.memory_warning.take());
    if let Some(mut callback) = callback {
        callback();
        APP_STATE.with_borrow_mut(|state| {
            if state.memory_warning.is_none() {
                state.memory_warning = Some(callback);
            }
        });
    }
}

fn notify_app_lifecycle(phase: AppLifecyclePhase) {
    match phase {
        AppLifecyclePhase::Foreground => with_windows(|window| {
            window.notify_visibility_change(WindowVisibility::Visible);
        }),
        AppLifecyclePhase::Active => {
            with_windows(|window| window.notify_active_status_change(true));
        }
        AppLifecyclePhase::Inactive => {
            with_windows(|window| window.notify_active_status_change(false));
        }
        AppLifecyclePhase::Background => {
            with_windows(|window| window.notify_active_status_change(false));
            // An activation observer may have closed a window; revisit the registry.
            with_windows(|window| window.notify_visibility_change(WindowVisibility::Hidden));
        }
    }
    let callback = APP_STATE.with_borrow_mut(|state| state.app_lifecycle.take());
    if let Some(mut callback) = callback {
        callback(phase);
        APP_STATE.with_borrow_mut(|state| {
            if state.app_lifecycle.is_none() {
                state.app_lifecycle = Some(callback);
            }
        });
    }
}

fn open_urls(contexts: &NSSet<UIOpenURLContext>) {
    if contexts.is_empty() {
        return;
    }
    let urls = contexts
        .iter()
        .filter_map(|context| context.URL().absoluteString().map(|url| url.to_string()))
        .collect();
    let callback = APP_STATE.with_borrow_mut(|state| state.open_urls.take());
    if let Some(mut callback) = callback {
        callback(urls);
        APP_STATE.with_borrow_mut(|state| {
            if state.open_urls.is_none() {
                state.open_urls = Some(callback);
            }
        });
    }
}

extern_class!(
    #[unsafe(super = NSObject)]
    #[name = "UISceneConnectionOptions"]
    struct SceneConnectionOptions;
);

impl SceneConnectionOptions {
    extern_methods!(
        // iOS 18 can return nil on an ordinary launch despite the SDK's nonnull annotation.
        #[unsafe(method(URLContexts))]
        #[unsafe(method_family = none)]
        fn url_contexts(&self) -> Option<Retained<NSSet<UIOpenURLContext>>>;
    );
}

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUIIosAppDelegate"]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl UIApplicationDelegate for AppDelegate {
        #[unsafe(method(application:didFinishLaunchingWithOptions:))]
        unsafe fn did_finish_launching(
            &self,
            _application: &UIApplication,
            _options: Option<&NSDictionary<UIApplicationLaunchOptionsKey, AnyObject>>,
        ) -> bool {
            // GPUI's launch callback runs when a scene is ready to host its window.
            true
        }

        #[unsafe(method_id(application:configurationForConnectingSceneSession:options:))]
        fn configuration_for_scene(
            &self,
            _application: &UIApplication,
            session: &UISceneSession,
            _options: &UISceneConnectionOptions,
        ) -> Retained<UISceneConfiguration> {
            let configuration = UISceneConfiguration::initWithName_sessionRole(
                UISceneConfiguration::alloc(self.mtm()),
                Some(&NSString::from_str("Default Configuration")),
                &session.role(),
            );
            unsafe {
                configuration.setDelegateClass(Some(SceneDelegate::class()));
            }
            configuration
        }

        #[unsafe(method(applicationDidReceiveMemoryWarning:))]
        fn did_receive_memory_warning(&self, _application: &UIApplication) {
            notify_memory_warning();
        }

        #[unsafe(method(applicationWillTerminate:))]
        fn will_terminate(&self, _application: &UIApplication) {
            let callback = APP_STATE.with_borrow_mut(|state| state.quit.take());
            if let Some(mut callback) = callback {
                callback();
            }
        }
    }
);

#[derive(Default)]
struct SceneDelegateIvars {
    display_link: RefCell<Option<Retained<CADisplayLink>>>,
}

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUIIosSceneDelegate"]
    #[ivars = SceneDelegateIvars]
    struct SceneDelegate;

    unsafe impl NSObjectProtocol for SceneDelegate {}
    unsafe impl UIWindowSceneDelegate for SceneDelegate {}

    impl SceneDelegate {
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>) -> Retained<Self> {
            let this = this.set_ivars(SceneDelegateIvars::default());
            unsafe { msg_send![super(this), init] }
        }

        #[unsafe(method(renderFrame:))]
        fn render_frame(&self, _display_link: &CADisplayLink) {
            with_windows(IosWindowState::request_frame);
        }
    }

    unsafe impl UISceneDelegate for SceneDelegate {
        #[unsafe(method(scene:willConnectToSession:options:))]
        fn will_connect(
            &self,
            scene: &UIScene,
            _session: &UISceneSession,
            options: &SceneConnectionOptions,
        ) {
            let Some(scene) = scene.downcast_ref::<UIWindowScene>() else {
                return;
            };
            if window_scene().is_some() {
                log::error!("GPUI iOS currently supports one connected window scene");
                return;
            }
            APP_STATE.with_borrow_mut(|state| state.scene = Some(scene.retain()));
            finish_launching();
            // UIKit can reconnect a scene without restarting the Rust application.
            with_windows(|window| window.attach_to_scene(scene));
            if let Some(contexts) = options.url_contexts() {
                open_urls(&contexts);
            }

            self.stop_display_link();
            let display_link = unsafe {
                let display_link =
                    CADisplayLink::displayLinkWithTarget_selector(self, sel!(renderFrame:));
                display_link.addToRunLoop_forMode(
                    &NSRunLoop::mainRunLoop(),
                    NSRunLoopCommonModes,
                );
                display_link
            };
            *self.ivars().display_link.borrow_mut() = Some(display_link);
        }

        #[unsafe(method(sceneDidDisconnect:))]
        fn did_disconnect(&self, scene: &UIScene) {
            self.stop_display_link();
            if self.owns_scene(scene) {
                notify_app_lifecycle(AppLifecyclePhase::Background);
                APP_STATE.with_borrow_mut(|state| state.scene = None);
            }
        }

        #[unsafe(method(sceneWillEnterForeground:))]
        fn will_enter_foreground(&self, scene: &UIScene) {
            if self.owns_scene(scene) {
                notify_app_lifecycle(AppLifecyclePhase::Foreground);
            }
        }

        #[unsafe(method(sceneDidBecomeActive:))]
        fn did_become_active(&self, scene: &UIScene) {
            if self.owns_scene(scene) {
                notify_app_lifecycle(AppLifecyclePhase::Active);
            }
        }

        #[unsafe(method(sceneWillResignActive:))]
        fn will_resign_active(&self, scene: &UIScene) {
            if self.owns_scene(scene) {
                notify_app_lifecycle(AppLifecyclePhase::Inactive);
            }
        }

        #[unsafe(method(sceneDidEnterBackground:))]
        fn did_enter_background(&self, scene: &UIScene) {
            if self.owns_scene(scene) {
                notify_app_lifecycle(AppLifecyclePhase::Background);
            }
        }

        #[unsafe(method(scene:openURLContexts:))]
        fn open_url_contexts(&self, scene: &UIScene, contexts: &NSSet<UIOpenURLContext>) {
            if self.owns_scene(scene) {
                open_urls(contexts);
            }
        }
    }
);

impl SceneDelegate {
    fn owns_scene(&self, scene: &UIScene) -> bool {
        window_scene().is_some_and(|current| std::ptr::eq::<UIScene>(&**current, scene))
    }

    fn stop_display_link(&self) {
        if let Some(display_link) = self.ivars().display_link.borrow_mut().take() {
            display_link.invalidate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn launch_callback_runs_once_and_can_register_callbacks() {
        APP_STATE.with_borrow_mut(|state| *state = IosPlatformState::default());
        let launches = Rc::new(Cell::new(0));
        APP_STATE.with_borrow_mut(|state| {
            let launches = launches.clone();
            state.finish_launching = Some(Box::new(move || {
                launches.set(launches.get() + 1);
                set_memory_warning_callback(Box::new(|| {}));
            }));
        });
        finish_launching();
        finish_launching();
        assert_eq!(launches.get(), 1);
        assert!(APP_STATE.with_borrow(|state| state.memory_warning.is_some()));
    }

    #[test]
    fn lifecycle_callback_replacement_survives_dispatch() {
        APP_STATE.with_borrow_mut(|state| *state = IosPlatformState::default());
        let events = Rc::new(RefCell::new(Vec::new()));
        set_app_lifecycle_callback(Box::new({
            let events = events.clone();
            move |phase| {
                events.borrow_mut().push((0, phase));
                set_app_lifecycle_callback(Box::new({
                    let events = events.clone();
                    move |phase| events.borrow_mut().push((1, phase))
                }));
            }
        }));
        notify_app_lifecycle(AppLifecyclePhase::Foreground);
        notify_app_lifecycle(AppLifecyclePhase::Active);
        assert_eq!(
            *events.borrow(),
            [
                (0, AppLifecyclePhase::Foreground),
                (1, AppLifecyclePhase::Active)
            ]
        );
    }

    #[test]
    fn memory_warning_callback_is_restored_after_dispatch() {
        APP_STATE.with_borrow_mut(|state| *state = IosPlatformState::default());
        let warnings = Rc::new(Cell::new(0));
        set_memory_warning_callback(Box::new({
            let warnings = warnings.clone();
            move || {
                warnings.set(warnings.get() + 1);
                set_quit_callback(Box::new(|| {}));
            }
        }));
        notify_memory_warning();
        notify_memory_warning();
        assert_eq!(warnings.get(), 2);
        assert!(APP_STATE.with_borrow(|state| state.quit.is_some()));
    }
}
