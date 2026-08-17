/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_DXGI_CONTEXT_H_
#define MODULES_DESKTOP_CAPTURE_WIN_DXGI_CONTEXT_H_

#include <vector>

#include "modules/desktop_capture/desktop_region.h"

namespace webrtc {

// A DxgiOutputContext stores the status of a single DxgiFrame of
// DxgiOutputDuplicator.
struct DxgiOutputContext final {
  // The updated region DxgiOutputDuplicator::DetectUpdatedRegion() output
  // during last Duplicate() function call. It's always relative to the (0, 0).
  DesktopRegion updated_region;
};

// A DxgiAdapterContext stores the status of a single DxgiFrame of
// DxgiAdapterDuplicator.
struct DxgiAdapterContext final {
  DxgiAdapterContext();
  DxgiAdapterContext(const DxgiAdapterContext& other);
  ~DxgiAdapterContext();

  // Child DxgiOutputContext belongs to this AdapterContext.
  std::vector<DxgiOutputContext> contexts;
};

// A DxgiFrameContext stores the status of a single DxgiFrame of
// DxgiDuplicatorController.
struct DxgiFrameContext final {
 public:
  DxgiFrameContext();
  // Unregister this Context instance from DxgiDuplicatorController during
  // destructing.
  ~DxgiFrameContext();

  // Reset current Context, so it will be reinitialized next time.
  void Reset();

  // A Context will have an exactly same `controller_id` as
  // DxgiDuplicatorController, to ensure it has been correctly setted up after
  // each DxgiDuplicatorController::Initialize().
  int controller_id = 0;

  // Child DxgiAdapterContext belongs to this DxgiFrameContext.
  std::vector<DxgiAdapterContext> contexts;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_DXGI_CONTEXT_H_
