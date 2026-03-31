#ifndef ZedApp_Bridging_Header_h
#define ZedApp_Bridging_Header_h

#include <stdint.h>

/// Initialize GPUI and Zed. Called once from AppDelegate after UIApplicationMain.
void zed_ios_main(void);

/// Create a new GPUI window for the given UIWindowScene. Called by SceneDelegate.
void zed_ios_open_window(const char *scene_id);

/// Tear down the GPUI window for the given UIWindowScene. Called by SceneDelegate.
void zed_ios_close_window(const char *scene_id);

/// Install Zed's menus into the iPadOS menu bar. Called from buildMenu(with:).
/// builder is a UIMenuBuilder* passed as void* to avoid ObjC in the C header.
void zed_ios_build_menus(void *builder);

/// Persist active SSH sessions before the app enters the background.
/// Called from applicationWillResignActive or sceneDidEnterBackground.
void zed_ios_will_resign_active(void);

#endif /* ZedApp_Bridging_Header_h */
