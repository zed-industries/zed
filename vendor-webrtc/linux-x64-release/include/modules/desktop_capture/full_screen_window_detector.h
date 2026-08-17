/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_FULL_SCREEN_WINDOW_DETECTOR_H_
#define MODULES_DESKTOP_CAPTURE_FULL_SCREEN_WINDOW_DETECTOR_H_

#include <memory>

#include "api/function_view.h"
#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/full_screen_application_handler.h"

namespace webrtc {

// This is a way to handle switch to full-screen mode for application in some
// specific cases:
// - Chrome on MacOS creates a new window in full-screen mode to
//   show a tab full-screen and minimizes the old window.
// - PowerPoint creates new windows in full-screen mode when user goes to
//   presentation mode (Slide Show Window, Presentation Window).
//
// To continue capturing in these cases, we try to find the new full-screen
// window using criteria provided by application specific
// FullScreenApplicationHandler.

class FullScreenWindowDetector
    : public RefCountedNonVirtual<FullScreenWindowDetector> {
 public:
  using ApplicationHandlerFactory =
      std::function<std::unique_ptr<FullScreenApplicationHandler>(
          DesktopCapturer::SourceId sourceId)>;

  FullScreenWindowDetector(
      ApplicationHandlerFactory application_handler_factory);

  FullScreenWindowDetector(const FullScreenWindowDetector&) = delete;
  FullScreenWindowDetector& operator=(const FullScreenWindowDetector&) = delete;

  // Returns the full-screen window in place of the original window if all the
  // criteria provided by FullScreenApplicationHandler are met, or 0 if no such
  // window found.
  DesktopCapturer::SourceId FindFullScreenWindow(
      DesktopCapturer::SourceId original_source_id);

  // The caller should call this function periodically, implementation will
  // update internal state no often than twice per second
  void UpdateWindowListIfNeeded(
      DesktopCapturer::SourceId original_source_id,
      FunctionView<bool(DesktopCapturer::SourceList*)> get_sources);

  static scoped_refptr<FullScreenWindowDetector>
  CreateFullScreenWindowDetector();
  void SetUseHeuristicFullscreenPowerPointWindows(
      bool use_heuristic_fullscreen_powerpoint_windows) {
    use_heuristic_fullscreen_powerpoint_windows_ =
        use_heuristic_fullscreen_powerpoint_windows;
    if (app_handler_) {
      app_handler_->SetUseHeuristicFullscreenPowerPointWindows(
          use_heuristic_fullscreen_powerpoint_windows);
    }
  }

 protected:
  std::unique_ptr<FullScreenApplicationHandler> app_handler_;

 private:
  void CreateApplicationHandlerIfNeeded(DesktopCapturer::SourceId source_id);

  ApplicationHandlerFactory application_handler_factory_;
  // `use_heuristic_fullscreen_powerpoint_windows_` controls if we create the
  // FullScreenPowerPointHandler class or not.
  // TODO(crbug.com/409473386): Remove
  // `use_heuristic_fullscreen_powerpoint_windows_` once the feature is
  // available in stable for some milestones.
  bool use_heuristic_fullscreen_powerpoint_windows_ = true;

  int64_t last_update_time_ms_;
  DesktopCapturer::SourceId previous_source_id_;

  // Save the source id when we fail to create an instance of
  // CreateApplicationHandlerIfNeeded to avoid redundant attempt to do it again.
  DesktopCapturer::SourceId no_handler_source_id_;

  DesktopCapturer::SourceList window_list_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_FULL_SCREEN_WINDOW_DETECTOR_H_
