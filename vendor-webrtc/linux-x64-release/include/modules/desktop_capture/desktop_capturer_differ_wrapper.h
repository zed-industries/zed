/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_DIFFER_WRAPPER_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_DIFFER_WRAPPER_H_

#include <memory>
#if defined(WEBRTC_USE_GIO)
#include "modules/desktop_capture/desktop_capture_metadata.h"
#endif  // defined(WEBRTC_USE_GIO)
#include "modules/desktop_capture/desktop_capture_types.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/shared_desktop_frame.h"
#include "modules/desktop_capture/shared_memory.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// DesktopCapturer wrapper that calculates updated_region() by comparing frames
// content. This class always expects the underlying DesktopCapturer
// implementation returns a superset of updated regions in DestkopFrame. If a
// DesktopCapturer implementation does not know the updated region, it should
// set updated_region() to full frame.
//
// This class marks entire frame as updated if the frame size or frame stride
// has been changed.
class RTC_EXPORT DesktopCapturerDifferWrapper
    : public DesktopCapturer,
      public DesktopCapturer::Callback {
 public:
  // Creates a DesktopCapturerDifferWrapper with a DesktopCapturer
  // implementation, and takes its ownership.
  explicit DesktopCapturerDifferWrapper(
      std::unique_ptr<DesktopCapturer> base_capturer);

  ~DesktopCapturerDifferWrapper() override;

  // DesktopCapturer interface.
  void Start(DesktopCapturer::Callback* callback) override;
  void SetSharedMemoryFactory(
      std::unique_ptr<SharedMemoryFactory> shared_memory_factory) override;
  void CaptureFrame() override;
  void SetExcludedWindow(WindowId window) override;
  bool GetSourceList(SourceList* screens) override;
  bool SelectSource(SourceId id) override;
  bool FocusOnSelectedSource() override;
  bool IsOccluded(const DesktopVector& pos) override;
#if defined(WEBRTC_USE_GIO)
  DesktopCaptureMetadata GetMetadata() override;
#endif  // defined(WEBRTC_USE_GIO)
 private:
  // DesktopCapturer::Callback interface.
  void OnCaptureResult(Result result,
                       std::unique_ptr<DesktopFrame> frame) override;

  const std::unique_ptr<DesktopCapturer> base_capturer_;
  DesktopCapturer::Callback* callback_;
  std::unique_ptr<SharedDesktopFrame> last_frame_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_DIFFER_WRAPPER_H_
