/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_FUCHSIA_H_
#define MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_FUCHSIA_H_

#include <fuchsia/sysmem2/cpp/fidl.h>
#include <fuchsia/ui/composition/cpp/fidl.h>
#include <lib/sys/cpp/component_context.h>

#include <memory>
#include <unordered_map>

#include "modules/desktop_capture/desktop_capture_options.h"
#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_frame.h"

namespace webrtc {

class ScreenCapturerFuchsia final : public DesktopCapturer {
 public:
  ScreenCapturerFuchsia();
  ~ScreenCapturerFuchsia() override;

  // DesktopCapturer interface.
  void Start(Callback* callback) override;
  void CaptureFrame() override;
  bool GetSourceList(SourceList* screens) override;
  bool SelectSource(SourceId id) override;

 private:
  fuchsia::sysmem2::BufferCollectionConstraints GetBufferConstraints();
  void SetupBuffers();
  uint32_t GetPixelsPerRow(
      const fuchsia::sysmem2::ImageFormatConstraints& constraints);

  Callback* callback_ = nullptr;

  std::unique_ptr<sys::ComponentContext> component_context_;
  fuchsia::sysmem2::AllocatorSyncPtr sysmem_allocator_;
  fuchsia::ui::composition::AllocatorSyncPtr flatland_allocator_;
  fuchsia::ui::composition::ScreenCaptureSyncPtr screen_capture_;
  fuchsia::sysmem2::BufferCollectionSyncPtr collection_;
  fuchsia::sysmem2::BufferCollectionInfo buffer_collection_info_;
  std::unordered_map<uint32_t, uint8_t*> virtual_memory_mapped_addrs_;

  bool fatal_error_;

  // Dimensions of the screen we are capturing
  uint32_t width_;
  uint32_t height_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURER_FUCHSIA_H_
