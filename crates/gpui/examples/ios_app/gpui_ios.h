//
//  gpui_ios.h
//  GPUI iOS FFI Header
//
//  This header declares the C-compatible functions exported by the GPUI Rust library
//  for use in iOS Objective-C code.
//

#ifndef GPUI_IOS_H
#define GPUI_IOS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Initialize the GPUI iOS application.
///
/// This should be called from `application:didFinishLaunchingWithOptions:`
/// in the iOS app delegate, before any other GPUI functions.
///
/// Returns a pointer to the app state that should be passed to other FFI functions.
/// Returns NULL if initialization fails.
void* gpui_ios_initialize(void);

/// Called when the iOS app has finished launching.
///
/// This should be called from `application:didFinishLaunchingWithOptions:`
/// in the iOS app delegate, after `gpui_ios_initialize()` returns.
///
/// This invokes the callback passed to Application::run().
void gpui_ios_did_finish_launching(void* app_ptr);

/// Called when the iOS app will enter the foreground.
///
/// This should be called from `applicationWillEnterForeground:` in the app delegate.
/// This notifies all GPUI windows that the app is becoming active.
void gpui_ios_will_enter_foreground(void* app_ptr);

/// Called when the iOS app did become active.
///
/// This should be called from `applicationDidBecomeActive:` in the app delegate.
/// This indicates the app is now in the foreground and receiving events.
void gpui_ios_did_become_active(void* app_ptr);

/// Called when the iOS app will resign active.
///
/// This should be called from `applicationWillResignActive:` in the app delegate.
/// This indicates the app is about to become inactive (e.g., incoming call, switching apps).
void gpui_ios_will_resign_active(void* app_ptr);

/// Called when the iOS app did enter the background.
///
/// This should be called from `applicationDidEnterBackground:` in the app delegate.
/// At this point, the app should save user data and release shared resources.
void gpui_ios_did_enter_background(void* app_ptr);

/// Called when the iOS app will terminate.
///
/// This should be called from `applicationWillTerminate:` in the app delegate.
/// This is a good place to save any unsaved data.
void gpui_ios_will_terminate(void* app_ptr);

/// Called when a touch event occurs.
///
/// This bridges UIKit touch events to GPUI's input system.
/// Parameters:
/// - window_ptr: Pointer to the IosWindow
/// - touch_ptr: Pointer to the UITouch object
/// - event_ptr: Pointer to the UIEvent object
void gpui_ios_handle_touch(void* window_ptr, void* touch_ptr, void* event_ptr);

/// Request a frame to be rendered.
///
/// This should be called from CADisplayLink callback.
/// The window_ptr should be the value returned by gpui_ios_get_window().
void gpui_ios_request_frame(void* window_ptr);

/// Get the most recently created GPUI window pointer.
///
/// Returns the pointer to the IosWindow that was most recently registered,
/// or NULL if no windows have been created.
/// This should be called after gpui_ios_did_finish_launching() to get the
/// window pointer needed for gpui_ios_request_frame().
void* gpui_ios_get_window(void);

/// Run a demo GPUI application.
///
/// This creates a GPUI Application and opens a test window.
/// Call this from application:didFinishLaunchingWithOptions: to start the demo.
/// This is an alternative to using gpui_ios_initialize/gpui_ios_did_finish_launching
/// when you want a self-contained demo.
void gpui_ios_run_demo(void);

/// Show the software keyboard.
///
/// Call this when a text input field gains focus.
/// The window_ptr should be the value returned by gpui_ios_get_window().
void gpui_ios_show_keyboard(void* window_ptr);

/// Hide the software keyboard.
///
/// Call this when a text input field loses focus.
/// The window_ptr should be the value returned by gpui_ios_get_window().
void gpui_ios_hide_keyboard(void* window_ptr);

/// Handle text input from the software keyboard.
///
/// This is called when the user types on the keyboard.
/// Parameters:
/// - window_ptr: Pointer to the IosWindow
/// - text_ptr: Pointer to NSString with the entered text
void gpui_ios_handle_text_input(void* window_ptr, void* text_ptr);

/// Handle a key event from an external keyboard.
///
/// Parameters:
/// - window_ptr: Pointer to the IosWindow
/// - key_code: The key code from UIKeyboardHIDUsage
/// - modifiers: Modifier flags from UIKeyModifierFlags
/// - is_key_down: true for key down, false for key up
void gpui_ios_handle_key_event(void* window_ptr, uint32_t key_code, uint32_t modifiers, _Bool is_key_down);

#ifdef __cplusplus
}
#endif

#endif /* GPUI_IOS_H */
