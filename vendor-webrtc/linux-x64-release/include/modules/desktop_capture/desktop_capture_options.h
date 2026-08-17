/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_OPTIONS_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_OPTIONS_H_

#include "api/scoped_refptr.h"
#include "rtc_base/system/rtc_export.h"

#if defined(WEBRTC_USE_X11)
#include "modules/desktop_capture/linux/x11/shared_x_display.h"
#endif

#if defined(WEBRTC_USE_PIPEWIRE)
#include "modules/desktop_capture/linux/wayland/shared_screencast_stream.h"
#endif

#if defined(WEBRTC_MAC) && !defined(WEBRTC_IOS)
#include "modules/desktop_capture/mac/desktop_configuration_monitor.h"
#endif

#include "modules/desktop_capture/full_screen_window_detector.h"

namespace webrtc {

// An object that stores initialization parameters for screen and window
// capturers.
class RTC_EXPORT DesktopCaptureOptions {
 public:
  // Returns instance of DesktopCaptureOptions with default parameters. On Linux
  // also initializes X window connection. x_display() will be set to null if
  // X11 connection failed (e.g. DISPLAY isn't set).
  static DesktopCaptureOptions CreateDefault();

  DesktopCaptureOptions();
  DesktopCaptureOptions(const DesktopCaptureOptions& options);
  DesktopCaptureOptions(DesktopCaptureOptions&& options);
  ~DesktopCaptureOptions();

  DesktopCaptureOptions& operator=(const DesktopCaptureOptions& options);
  DesktopCaptureOptions& operator=(DesktopCaptureOptions&& options);

#if defined(WEBRTC_USE_X11)
  const scoped_refptr<SharedXDisplay>& x_display() const { return x_display_; }
  void set_x_display(scoped_refptr<SharedXDisplay> x_display) {
    x_display_ = x_display;
  }
#endif

#if defined(WEBRTC_MAC) && !defined(WEBRTC_IOS)
  // TODO(zijiehe): Remove both DesktopConfigurationMonitor and
  // FullScreenChromeWindowDetector out of DesktopCaptureOptions. It's not
  // reasonable for external consumers to set these two parameters.
  const webrtc::scoped_refptr<DesktopConfigurationMonitor>&
  configuration_monitor() const {
    return configuration_monitor_;
  }
  // If nullptr is set, ScreenCapturer won't work and WindowCapturer may return
  // inaccurate result from IsOccluded() function.
  void set_configuration_monitor(
      webrtc::scoped_refptr<DesktopConfigurationMonitor> m) {
    configuration_monitor_ = m;
  }

  bool allow_iosurface() const { return allow_iosurface_; }
  void set_allow_iosurface(bool allow) { allow_iosurface_ = allow; }

  // If this flag is set, and the system supports it, ScreenCaptureKit will be
  // used for desktop capture.
  // TODO: crbug.com/327458809 - Force the use of SCK and ignore this flag in
  // new versions of macOS that remove support for the CGDisplay-based APIs.
  bool allow_sck_capturer() const { return allow_sck_capturer_; }
  void set_allow_sck_capturer(bool allow) { allow_sck_capturer_ = allow; }

  // If ScreenCaptureKit is used for desktop capture and this flag is
  // set, the ScreenCaptureKit backend will use SCContentSharingPicker for
  // picking source.
  bool allow_sck_system_picker() const { return allow_sck_system_picker_; }
  void set_allow_sck_system_picker(bool allow) {
    allow_sck_system_picker_ = allow;
  }
#endif

  const scoped_refptr<FullScreenWindowDetector>& full_screen_window_detector()
      const {
    return full_screen_window_detector_;
  }
  void set_full_screen_window_detector(
      scoped_refptr<FullScreenWindowDetector> detector) {
    full_screen_window_detector_ = detector;
  }

  // Flag indicating that the capturer should use screen change notifications.
  // Enables/disables use of XDAMAGE in the X11 capturer.
  bool use_update_notifications() const { return use_update_notifications_; }
  void set_use_update_notifications(bool use_update_notifications) {
    use_update_notifications_ = use_update_notifications;
  }

  // Flag indicating if desktop effects (e.g. Aero) should be disabled when the
  // capturer is active. Currently used only on Windows.
  bool disable_effects() const { return disable_effects_; }
  void set_disable_effects(bool disable_effects) {
    disable_effects_ = disable_effects;
  }

  // Flag that should be set if the consumer uses updated_region() and the
  // capturer should try to provide correct updated_region() for the frames it
  // generates (e.g. by comparing each frame with the previous one).
  bool detect_updated_region() const { return detect_updated_region_; }
  void set_detect_updated_region(bool detect_updated_region) {
    detect_updated_region_ = detect_updated_region;
  }

  // Indicates that the capturer should try to include the cursor in the frame.
  // If it is able to do so it will set `DesktopFrame::may_contain_cursor()`.
  // Not all capturers will support including the cursor. If this value is false
  // or the cursor otherwise cannot be included in the frame, then cursor
  // metadata will be sent, though the capturer may choose to always send cursor
  // metadata.
  bool prefer_cursor_embedded() const { return prefer_cursor_embedded_; }
  void set_prefer_cursor_embedded(bool prefer_cursor_embedded) {
    prefer_cursor_embedded_ = prefer_cursor_embedded;
  }

#if defined(WEBRTC_WIN)
  // Enumerating windows owned by the current process on Windows has some
  // complications due to |GetWindowText*()| APIs potentially causing a
  // deadlock (see the comments in the `GetWindowListHandler()` function in
  // window_capture_utils.cc for more details on the deadlock).
  // To avoid this issue, consumers can either ensure that the thread that runs
  // their message loop never waits on `GetSourceList()`, or they can set this
  // flag to false which will prevent windows running in the current process
  // from being enumerated and included in the results. Consumers can still
  // provide the WindowId for their own windows to `SelectSource()` and capture
  // them.
  bool enumerate_current_process_windows() const {
    return enumerate_current_process_windows_;
  }
  void set_enumerate_current_process_windows(
      bool enumerate_current_process_windows) {
    enumerate_current_process_windows_ = enumerate_current_process_windows;
  }

  // Allowing directx based capturer or not, this capturer works on windows 7
  // with platform update / windows 8 or upper.
  bool allow_directx_capturer() const { return allow_directx_capturer_; }
  void set_allow_directx_capturer(bool enabled) {
    allow_directx_capturer_ = enabled;
  }

  // Flag that may be set to allow use of the cropping window capturer (which
  // captures the screen & crops that to the window region in some cases). An
  // advantage of using this is significantly higher capture frame rates than
  // capturing the window directly. A disadvantage of using this is the
  // possibility of capturing unrelated content (e.g. overlapping windows that
  // aren't detected properly, or neighboring regions when moving/resizing the
  // captured window). Note: this flag influences the behavior of calls to
  // DesktopCapturer::CreateWindowCapturer; calls to
  // CroppingWindowCapturer::CreateCapturer ignore the flag (treat it as true).
  bool allow_cropping_window_capturer() const {
    return allow_cropping_window_capturer_;
  }
  void set_allow_cropping_window_capturer(bool allow) {
    allow_cropping_window_capturer_ = allow;
  }

#if defined(RTC_ENABLE_WIN_WGC)
  // This flag enables the WGC capturer for capturing the screen.
  // This capturer should offer similar or better performance than the cropping
  // capturer without the disadvantages listed above. However, the WGC capturer
  // is only available on Windows 10 version 1809 (Redstone 5) and up. This flag
  // will have no affect on older versions.
  // If set, and running a supported version of Win10, this flag will take
  // precedence over the cropping, directx, and magnification flags.
  bool allow_wgc_screen_capturer() const { return allow_wgc_screen_capturer_; }
  void set_allow_wgc_screen_capturer(bool allow) {
    allow_wgc_screen_capturer_ = allow;
  }

  // This flag has the same effect as allow_wgc_screen_capturer but it only
  // enables or disables WGC for window capturing (not screen).
  bool allow_wgc_window_capturer() const { return allow_wgc_window_capturer_; }
  void set_allow_wgc_window_capturer(bool allow) {
    allow_wgc_window_capturer_ = allow;
  }

  // This flag enables the WGC capturer for fallback capturer.
  // The flag is useful when the first capturer (eg. WindowCapturerWinGdi) is
  // unreliable in certain devices where WGC is supported, but not used by
  // default.
  bool allow_wgc_capturer_fallback() const {
    return allow_wgc_capturer_fallback_;
  }
  void set_allow_wgc_capturer_fallback(bool allow) {
    allow_wgc_capturer_fallback_ = allow;
  }

  // This flag enables 0Hz mode in combination with the WGC capturer.
  // The flag has no effect if the allow_wgc_capturer flag is false.
  bool allow_wgc_zero_hertz() const { return allow_wgc_zero_hertz_; }
  void set_allow_wgc_zero_hertz(bool allow) { allow_wgc_zero_hertz_ = allow; }

  // This flag controls whether the WGC capturer is required to draw a border
  // around the captured window/screen.
  // The flag has no effect if the allow_wgc_capturer flag is false.
  bool wgc_require_border() const { return wgc_require_border_; }
  void set_wgc_require_border(bool require) { wgc_require_border_ = require; }
#endif  // defined(RTC_ENABLE_WIN_WGC)
#endif  // defined(WEBRTC_WIN)

#if defined(WEBRTC_USE_PIPEWIRE)
  bool allow_pipewire() const { return allow_pipewire_; }
  void set_allow_pipewire(bool allow) { allow_pipewire_ = allow; }

  const webrtc::scoped_refptr<SharedScreenCastStream>& screencast_stream()
      const {
    return screencast_stream_;
  }
  void set_screencast_stream(
      webrtc::scoped_refptr<SharedScreenCastStream> stream) {
    screencast_stream_ = stream;
  }

  void set_width(uint32_t width) { width_ = width; }
  uint32_t get_width() const { return width_; }

  void set_height(uint32_t height) { height_ = height; }
  uint32_t get_height() const { return height_; }

  void set_pipewire_use_damage_region(bool use_damage_regions) {
    pipewire_use_damage_region_ = use_damage_regions;
  }
  bool pipewire_use_damage_region() const {
    return pipewire_use_damage_region_;
  }
#endif

 private:
#if defined(WEBRTC_USE_X11)
  scoped_refptr<SharedXDisplay> x_display_;
#endif
#if defined(WEBRTC_USE_PIPEWIRE)
  // An instance of shared PipeWire ScreenCast stream we share between
  // BaseCapturerPipeWire and MouseCursorMonitorPipeWire as cursor information
  // is sent together with screen content.
  webrtc::scoped_refptr<SharedScreenCastStream> screencast_stream_;
#endif
#if defined(WEBRTC_MAC) && !defined(WEBRTC_IOS)
  webrtc::scoped_refptr<DesktopConfigurationMonitor> configuration_monitor_;
  bool allow_iosurface_ = false;
  bool allow_sck_capturer_ = false;
  bool allow_sck_system_picker_ = false;
#endif

  scoped_refptr<FullScreenWindowDetector> full_screen_window_detector_;

#if defined(WEBRTC_WIN)
  bool enumerate_current_process_windows_ = true;
  bool allow_directx_capturer_ = false;
  bool allow_cropping_window_capturer_ = false;
#if defined(RTC_ENABLE_WIN_WGC)
  bool allow_wgc_screen_capturer_ = false;
  bool allow_wgc_window_capturer_ = false;
  bool allow_wgc_capturer_fallback_ = false;
  bool allow_wgc_zero_hertz_ = false;
  bool wgc_require_border_ = false;
#endif
#endif
#if defined(WEBRTC_USE_X11)
  bool use_update_notifications_ = false;
#else
  bool use_update_notifications_ = true;
#endif
  bool disable_effects_ = true;
  bool detect_updated_region_ = false;
  bool prefer_cursor_embedded_ = false;
#if defined(WEBRTC_USE_PIPEWIRE)
  bool allow_pipewire_ = false;
  bool pipewire_use_damage_region_ = true;
  uint32_t width_ = 0;
  uint32_t height_ = 0;
#endif
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_OPTIONS_H_
