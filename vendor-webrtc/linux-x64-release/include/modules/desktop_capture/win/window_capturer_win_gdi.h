/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURER_WIN_GDI_H_
#define MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURER_WIN_GDI_H_

#include <map>
#include <memory>
#include <vector>

#include "modules/desktop_capture/desktop_capture_options.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/win/window_capture_utils.h"
#include "modules/desktop_capture/window_finder_win.h"

namespace webrtc {

class WindowCapturerWinGdi : public DesktopCapturer {
 public:
  explicit WindowCapturerWinGdi(bool enumerate_current_process_windows);

  // Disallow copy and assign
  WindowCapturerWinGdi(const WindowCapturerWinGdi&) = delete;
  WindowCapturerWinGdi& operator=(const WindowCapturerWinGdi&) = delete;

  ~WindowCapturerWinGdi() override;

  static std::unique_ptr<DesktopCapturer> CreateRawWindowCapturer(
      const DesktopCaptureOptions& options);

  // DesktopCapturer interface.
  void Start(Callback* callback) override;
  void CaptureFrame() override;
  bool GetSourceList(SourceList* sources) override;
  bool SelectSource(SourceId id) override;
  bool FocusOnSelectedSource() override;
  bool IsOccluded(const DesktopVector& pos) override;

 private:
  struct CaptureResults {
    Result result;
    std::unique_ptr<DesktopFrame> frame;
  };

  CaptureResults CaptureFrame(bool capture_owned_windows);

  Callback* callback_ = nullptr;

  // HWND and HDC for the currently selected window or nullptr if window is not
  // selected.
  HWND window_ = nullptr;

  DesktopSize previous_size_;

  WindowCaptureHelperWin window_capture_helper_;

  bool enumerate_current_process_windows_;

  // This map is used to avoid flickering for the case when SelectWindow() calls
  // are interleaved with Capture() calls.
  std::map<HWND, DesktopSize> window_size_map_;

  WindowFinderWin window_finder_;

  std::vector<HWND> owned_windows_;
  std::unique_ptr<WindowCapturerWinGdi> owned_window_capturer_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_WINDOW_CAPTURER_WIN_GDI_H_
