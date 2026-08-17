/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_WGC_DESKTOP_FRAME_H_
#define MODULES_DESKTOP_CAPTURE_WIN_WGC_DESKTOP_FRAME_H_

#include <d3d11.h>
#include <wrl/client.h>

#include <memory>
#include <vector>

#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

// DesktopFrame implementation used by capturers that use the
// Windows.Graphics.Capture API.
class WgcDesktopFrame final : public DesktopFrame {
 public:
  // WgcDesktopFrame receives an rvalue reference to the `image_data` vector
  // so that it can take ownership of it (and avoid a copy).
  WgcDesktopFrame(DesktopSize size,
                  int stride,
                  std::vector<uint8_t>&& image_data);

  WgcDesktopFrame(const WgcDesktopFrame&) = delete;
  WgcDesktopFrame& operator=(const WgcDesktopFrame&) = delete;

  ~WgcDesktopFrame() override;

 private:
  std::vector<uint8_t> image_data_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_WGC_DESKTOP_FRAME_H_
