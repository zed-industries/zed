#ifndef ZedApp_Bridging_Header_h
#define ZedApp_Bridging_Header_h

#include <stdint.h>

/// Initialize GPUI and Zed. Called once from AppDelegate after UIApplicationMain.
void zed_ios_main(void);

/// Create a new GPUI window for the given UIWindowScene. Called by SceneDelegate.
void zed_ios_open_window(const char *scene_id);

/// Tear down the GPUI window for the given UIWindowScene. Called by SceneDelegate.
void zed_ios_close_window(const char *scene_id);

#endif /* ZedApp_Bridging_Header_h */
