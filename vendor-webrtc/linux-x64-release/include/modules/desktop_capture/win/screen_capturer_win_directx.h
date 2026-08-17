/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_SCREEN_CAPTURER_WIN_DIRECTX_H_
#define MODULES_DESKTOP_CAPTURE_WIN_SCREEN_CAPTURER_WIN_DIRECTX_H_

#include <d3dcommon.h>

#include <memory>
#include <unordered_map>
#include <vector>

#include "api/scoped_refptr.h"
#include "modules/desktop_capture/desktop_capture_options.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_region.h"
#include "modules/desktop_capture/screen_capture_frame_queue.h"
#include "modules/desktop_capture/win/dxgi_duplicator_controller.h"
#include "modules/desktop_capture/win/dxgi_frame.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// ScreenCapturerWinDirectx captures 32bit RGBA using DirectX.
class RTC_EXPORT ScreenCapturerWinDirectx : public DesktopCapturer {
 public:
  using D3dInfo = DxgiDuplicatorController::D3dInfo;

  // Whether the system supports DirectX based capturing.
  static bool IsSupported();

  // Returns a most recent D3dInfo composed by
  // DxgiDuplicatorController::Initialize() function. This function implicitly
  // calls DxgiDuplicatorController::Initialize() if it has not been
  // initialized. This function returns false and output parameter is kept
  // unchanged if DxgiDuplicatorController::Initialize() failed.
  // The D3dInfo may change based on hardware configuration even without
  // restarting the hardware and software. Refer to https://goo.gl/OOCppq. So
  // consumers should not cache the result returned by this function.
  static bool RetrieveD3dInfo(D3dInfo* info);

  // Whether current process is running in a Windows session which is supported
  // by ScreenCapturerWinDirectx.
  // Usually using ScreenCapturerWinDirectx in unsupported sessions will fail.
  // But this behavior may vary on different Windows version. So consumers can
  // always try IsSupported() function.
  static bool IsCurrentSessionSupported();

  // Maps `device_names` with the result from GetScreenList() and creates a new
  // SourceList to include only the ones in `device_names`. If this function
  // returns true, consumers can always assume `device_names`.size() equals to
  // `screens`->size(), meanwhile `device_names`[i] and `screens`[i] indicate
  // the same monitor on the system.
  // Public for test only.
  static bool GetScreenListFromDeviceNames(
      const std::vector<std::string>& device_names,
      DesktopCapturer::SourceList* screens);

  // Maps `id` with the result from GetScreenListFromDeviceNames() and returns
  // the index of the entity in `device_names`. This function returns -1 if `id`
  // cannot be found.
  // Public for test only.
  static int GetIndexFromScreenId(ScreenId id,
                                  const std::vector<std::string>& device_names);

  // This constructor is deprecated. Please don't use it in new implementations.
  ScreenCapturerWinDirectx();
  explicit ScreenCapturerWinDirectx(const DesktopCaptureOptions& options);

  ~ScreenCapturerWinDirectx() override;

  ScreenCapturerWinDirectx(const ScreenCapturerWinDirectx&) = delete;
  ScreenCapturerWinDirectx& operator=(const ScreenCapturerWinDirectx&) = delete;

  // DesktopCapturer implementation.
  void Start(Callback* callback) override;
  void SetSharedMemoryFactory(
      std::unique_ptr<SharedMemoryFactory> shared_memory_factory) override;
  void CaptureFrame() override;
  bool GetSourceList(SourceList* sources) override;
  bool SelectSource(SourceId id) override;

 private:
  const webrtc::scoped_refptr<DxgiDuplicatorController> controller_;
  DesktopCaptureOptions options_;

  // The underlying DxgiDuplicators may retain a reference to the frames that
  // we ask them to duplicate so that they can continue returning valid frames
  // in the event that the target has not been updated. Thus, we need to ensure
  // that we have a separate frame queue for each source id, so that these held
  // frames don't get overwritten with the data from another Duplicator/monitor.
  std::unordered_map<SourceId, ScreenCaptureFrameQueue<DxgiFrame>>
      frame_queue_map_;
  std::unique_ptr<SharedMemoryFactory> shared_memory_factory_;
  Callback* callback_ = nullptr;
  SourceId current_screen_id_ = kFullDesktopScreenId;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_SCREEN_CAPTURER_WIN_DIRECTX_H_
