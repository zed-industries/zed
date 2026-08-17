/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_IOSURFACE_H_
#define MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_IOSURFACE_H_

#include <CoreGraphics/CoreGraphics.h>
#include <IOSurface/IOSurface.h>

#include <memory>

#include "modules/desktop_capture/desktop_frame.h"
#include "sdk/objc/helpers/scoped_cftyperef.h"

namespace webrtc {

class DesktopFrameIOSurface final : public DesktopFrame {
 public:
  // Lock an IOSurfaceRef containing a snapshot of a display. Return NULL if
  // failed to lock. `rect` specifies the portion of the surface that the
  // DesktopFrame should be cropped to.
  static std::unique_ptr<DesktopFrameIOSurface> Wrap(
      webrtc::ScopedCFTypeRef<IOSurfaceRef> io_surface, CGRect rect = {});

  ~DesktopFrameIOSurface() override;

  DesktopFrameIOSurface(const DesktopFrameIOSurface&) = delete;
  DesktopFrameIOSurface& operator=(const DesktopFrameIOSurface&) = delete;

 private:
  // `io_surface` must hold a non-null IOSurfaceRef that is already locked.
  // `data` is the address of the first byte of data in `io_surface`'s locked
  // buffer.
  // `width` and `height` make up the dimensions of `io_surface` in pixels.
  // `stride` is the number of bytes of a single row of pixels in `data`.
  DesktopFrameIOSurface(webrtc::ScopedCFTypeRef<IOSurfaceRef> io_surface,
                        uint8_t* data,
                        int32_t width,
                        int32_t height,
                        int32_t stride);

  const webrtc::ScopedCFTypeRef<IOSurfaceRef> io_surface_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_IOSURFACE_H_
