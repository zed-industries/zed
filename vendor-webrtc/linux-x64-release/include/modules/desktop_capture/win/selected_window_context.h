/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_SELECTED_WINDOW_CONTEXT_H_
#define MODULES_DESKTOP_CAPTURE_WIN_SELECTED_WINDOW_CONTEXT_H_

#include <windows.h>

#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/win/window_capture_utils.h"

namespace webrtc {

class SelectedWindowContext {
 public:
  SelectedWindowContext(HWND selected_window,
                        DesktopRect selected_window_rect,
                        WindowCaptureHelperWin* window_capture_helper);

  bool IsSelectedWindowValid() const;

  bool IsWindowOwnedBySelectedWindow(HWND hwnd) const;
  bool IsWindowOverlappingSelectedWindow(HWND hwnd) const;

  HWND selected_window() const;
  WindowCaptureHelperWin* window_capture_helper() const;

 private:
  const HWND selected_window_;
  const DesktopRect selected_window_rect_;
  WindowCaptureHelperWin* const window_capture_helper_;
  DWORD selected_window_thread_id_;
  DWORD selected_window_process_id_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_SELECTED_WINDOW_CONTEXT_H_
