/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_RENDER_RESOLUTION_H_
#define API_VIDEO_RENDER_RESOLUTION_H_

namespace webrtc {

// TODO(bugs.webrtc.org/12114) : remove in favor of Resolution.
class RenderResolution {
 public:
  constexpr RenderResolution() = default;
  constexpr RenderResolution(int width, int height)
      : width_(width), height_(height) {}
  RenderResolution(const RenderResolution&) = default;
  RenderResolution& operator=(const RenderResolution&) = default;

  friend bool operator==(const RenderResolution& lhs,
                         const RenderResolution& rhs) {
    return lhs.width_ == rhs.width_ && lhs.height_ == rhs.height_;
  }
  friend bool operator!=(const RenderResolution& lhs,
                         const RenderResolution& rhs) {
    return !(lhs == rhs);
  }

  constexpr bool Valid() const { return width_ > 0 && height_ > 0; }

  constexpr int Width() const { return width_; }
  constexpr int Height() const { return height_; }

 private:
  int width_ = 0;
  int height_ = 0;
};

}  // namespace webrtc

#endif  // API_VIDEO_RENDER_RESOLUTION_H_
