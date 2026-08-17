use std::sync::mpsc::{channel, Receiver, RecvError};

use crate::{
    macros::get_string,
    os_types::{
        base::{PidT, UInt32, BOOL},
        geometry::CGRect,
    },
};
use block::{ConcreteBlock, RcBlock};
use objc::{
    msg_send,
    runtime::{Class, Object},
    Message, *,
};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSString};
use objc_id::*;

#[derive(Debug)]
pub struct UnsafeSCRunningApplication;
unsafe impl Message for UnsafeSCRunningApplication {}
impl UnsafeSCRunningApplication {
    pub fn get_process_id(&self) -> PidT {
        unsafe { msg_send![self, processID] }
    }
    pub fn get_application_name(&self) -> Option<String> {
        unsafe { get_string!(self, applicationName) }
    }
    pub fn get_bundle_identifier(&self) -> Option<String> {
        unsafe { get_string!(self, bundleIdentifier) }
    }
}

impl INSObject for UnsafeSCRunningApplication {
    fn class() -> &'static Class {
        Class::get("SCRunningApplication")
                .expect("Missing SCRunningApplication class, check that the binary is linked with ScreenCaptureKit")
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UnsafeSCWindow;
unsafe impl Message for UnsafeSCWindow {}

impl UnsafeSCWindow {
    pub fn get_owning_application(&self) -> Option<ShareId<UnsafeSCRunningApplication>> {
        unsafe {
            let ptr: *mut UnsafeSCRunningApplication = msg_send![self, owningApplication];
            if ptr.is_null() {
                None
            } else {
                Some(Id::from_ptr(ptr))
            }
        }
    }
    pub fn get_window_layer(&self) -> UInt32 {
        unsafe { msg_send![self, windowLayer] }
    }
    pub fn get_window_id(&self) -> UInt32 {
        unsafe { msg_send![self, windowID] }
    }
    pub fn get_frame(&self) -> CGRect {
        unsafe { msg_send![self, frame] }
    }
    pub fn get_title(&self) -> Option<String> {
        unsafe { get_string!(self, title) }
    }
    pub fn get_is_on_screen(&self) -> BOOL {
        unsafe { msg_send![self, isOnScreen] }
    }
    pub fn get_is_active(&self) -> BOOL {
        unsafe { msg_send![self, isActive] }
    }
}

impl INSObject for UnsafeSCWindow {
    fn class() -> &'static runtime::Class {
        Class::get("SCWindow")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Debug)]
pub struct UnsafeSCDisplay;
unsafe impl Message for UnsafeSCDisplay {}

impl UnsafeSCDisplay {
    pub fn get_display_id(&self) -> UInt32 {
        unsafe { msg_send![self, displayID] }
    }
    pub fn get_frame(&self) -> CGRect {
        unsafe { msg_send![self, frame] }
    }
    pub fn get_height(&self) -> UInt32 {
        unsafe { msg_send![self, height] }
    }
    pub fn get_width(&self) -> UInt32 {
        unsafe { msg_send![self, width] }
    }
}

impl INSObject for UnsafeSCDisplay {
    fn class() -> &'static runtime::Class {
        Class::get("SCDisplay")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Default)]
pub enum OnScreenOnlySettings<'a> {
    EveryWindow,
    #[default]
    OnlyOnScreen,
    AboveWindow(&'a UnsafeSCWindow),
    BelowWindow(&'a UnsafeSCWindow),
}
#[derive(Default)]
pub struct ExcludingDesktopWindowsConfig<'a> {
    exclude_desktop_windows: bool,
    on_screen_windows_only: OnScreenOnlySettings<'a>,
}

#[derive(Debug)]
pub struct UnsafeSCShareableContent;
unsafe impl Message for UnsafeSCShareableContent {}

type CompletionHandlerBlock = RcBlock<(*mut UnsafeSCShareableContent, *mut Object), ()>;
impl UnsafeSCShareableContent {
    unsafe fn new_completion_handler(
    ) -> (CompletionHandlerBlock, Receiver<Result<Id<Self>, String>>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |sc: *mut Self, error: *mut Object| {
            if error.is_null() {
                tx.send(Ok(Id::from_ptr(sc)))
                    .expect("could create owned pointer for UnsafeSCShareableContent");
            } else {
                let code: *mut NSString = msg_send![error, localizedDescription];
                tx.send(Err((*code).as_str().to_string()));
            }
        });
        (handler.copy(), rx)
    }

    pub fn get_with_config(config: &ExcludingDesktopWindowsConfig) -> Result<Id<Self>, String> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            match config.on_screen_windows_only {
                OnScreenOnlySettings::EveryWindow => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnly: 0
                    completionHandler: handler
                ],

                OnScreenOnlySettings::AboveWindow(ref w) => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnlyAboveWindow: &w
                    completionHandler: handler
                ],
                OnScreenOnlySettings::BelowWindow(ref w) => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnlyBelowWindow: &w
                    completionHandler: handler
                ],
                OnScreenOnlySettings::OnlyOnScreen => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnly: 1
                    completionHandler: handler
                ],
            }
            rx.recv().unwrap_or(Err("Failed to recv".to_string()))
        }
    }
    pub fn get() -> Result<Id<Self>, String> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: handler
            ];

            rx.recv().unwrap_or(Err("Failed to recv".to_string()))
        }
    }

    pub fn displays(&self) -> Vec<ShareId<UnsafeSCDisplay>> {
        let display_ptr: ShareId<NSArray<UnsafeSCDisplay, Shared>> =
            unsafe { Id::from_ptr(msg_send![self, displays]) };

        display_ptr.to_shared_vec()
    }
    pub fn applications(&self) -> Vec<ShareId<UnsafeSCRunningApplication>> {
        let applications_ptr: ShareId<NSArray<UnsafeSCRunningApplication, Shared>> =
            unsafe { Id::from_ptr(msg_send![self, applications]) };

        applications_ptr.to_shared_vec()
    }
    pub fn windows(&self) -> Vec<ShareId<UnsafeSCWindow>> {
        let windows_ptr: ShareId<NSArray<UnsafeSCWindow, Shared>> =
            unsafe { Id::from_ptr(msg_send![self, windows]) };

        windows_ptr.to_shared_vec()
    }
}

#[cfg(test)]
mod get_shareable_content_with_config {
    use super::*;
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn get_exclude_desktop_windows() {
        let mut config = ExcludingDesktopWindowsConfig::default();

        let _ = UnsafeSCShareableContent::get_with_config(&config);

        config.exclude_desktop_windows = true;
        let _ = UnsafeSCShareableContent::get_with_config(&config);

        config.exclude_desktop_windows = true;
        config.on_screen_windows_only = OnScreenOnlySettings::EveryWindow;
        let _ = UnsafeSCShareableContent::get_with_config(&config);
    }
}
#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_windows() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for w in sc.windows().iter() {
            assert!(
                w.get_title().is_some() || w.get_title().is_none(),
                "Can get title"
            );
        }
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_displays() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for d in sc.displays().iter() {
            println!("frame: {:?}", d.get_frame());
            assert!(d.get_frame().size.width > 0f64, "Can get application_name");
        }
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_applications() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for a in sc.applications().iter() {
            assert!(
                a.get_application_name().is_some() || a.get_application_name().is_none(),
                "Can get application_name"
            );
        }
    }
}
