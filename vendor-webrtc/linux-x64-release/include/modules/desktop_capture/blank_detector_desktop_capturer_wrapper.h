/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_BLANK_DETECTOR_DESKTOP_CAPTURER_WRAPPER_H_
#define MODULES_DESKTOP_CAPTURE_BLANK_DETECTOR_DESKTOP_CAPTURER_WRAPPER_H_

#include <memory>

#include "modules/desktop_capture/desktop_capture_types.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/rgba_color.h"
#include "modules/desktop_capture/shared_memory.h"

namespace webrtc {

// A DesktopCapturer wrapper detects the return value of its owned
// DesktopCapturer implementation. If sampled pixels returned by the
// DesktopCapturer implementation all equal to the blank pixel, this wrapper
// returns ERROR_TEMPORARY. If the DesktopCapturer implementation fails for too
// many times, this wrapper returns ERROR_PERMANENT.
class BlankDetectorDesktopCapturerWrapper final
    : public DesktopCapturer,
      public DesktopCapturer::Callback {
 public:
  // Creates BlankDetectorDesktopCapturerWrapper. BlankDesktopCapturerWrapper
  // takes ownership of `capturer`. The `blank_pixel` is the unmodified color
  // returned by the `capturer`.
  BlankDetectorDesktopCapturerWrapper(std::unique_ptr<DesktopCapturer> capturer,
                                      RgbaColor blank_pixel,
                                      bool check_per_capture = false);
  ~BlankDetectorDesktopCapturerWrapper() override;

  // DesktopCapturer interface.
  void Start(DesktopCapturer::Callback* callback) override;
  void SetSharedMemoryFactory(
      std::unique_ptr<SharedMemoryFactory> shared_memory_factory) override;
  void CaptureFrame() override;
  void SetExcludedWindow(WindowId window) override;
  bool GetSourceList(SourceList* sources) override;
  bool SelectSource(SourceId id) override;
  bool FocusOnSelectedSource() override;
  bool IsOccluded(const DesktopVector& pos) override;

 private:
  // DesktopCapturer::Callback interface.
  void OnCaptureResult(Result result,
                       std::unique_ptr<DesktopFrame> frame) override;

  bool IsBlankFrame(const DesktopFrame& frame) const;

  // Detects whether pixel at (x, y) equals to `blank_pixel_`.
  bool IsBlankPixel(const DesktopFrame& frame, int x, int y) const;

  const std::unique_ptr<DesktopCapturer> capturer_;
  const RgbaColor blank_pixel_;

  // Whether a non-blank frame has been received.
  bool non_blank_frame_received_ = false;

  // Whether the last frame is blank.
  bool last_frame_is_blank_ = false;

  // Whether current frame is the first frame.
  bool is_first_frame_ = true;

  // Blank inspection is made per capture instead of once for all
  // screens or windows.
  bool check_per_capture_ = false;

  DesktopCapturer::Callback* callback_ = nullptr;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_BLANK_DETECTOR_DESKTOP_CAPTURER_WRAPPER_H_
