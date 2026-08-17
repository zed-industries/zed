/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_FRAME_GENERATOR_INTERFACE_H_
#define API_TEST_FRAME_GENERATOR_INTERFACE_H_

#include <cstddef>
#include <optional>
#include <utility>

#include "api/scoped_refptr.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"

namespace webrtc {
namespace test {

class FrameGeneratorInterface {
 public:
  struct Resolution {
    size_t width;
    size_t height;
  };
  struct VideoFrameData {
    VideoFrameData(scoped_refptr<VideoFrameBuffer> buffer,
                   std::optional<VideoFrame::UpdateRect> update_rect)
        : buffer(std::move(buffer)), update_rect(update_rect) {}

    scoped_refptr<VideoFrameBuffer> buffer;
    std::optional<VideoFrame::UpdateRect> update_rect;
  };

  enum class OutputType { kI420, kI420A, kI010, kNV12 };
  static const char* OutputTypeToString(OutputType type);

  virtual ~FrameGeneratorInterface() = default;

  // Returns VideoFrameBuffer and area where most of update was done to set them
  // on the VideoFrame object.
  virtual VideoFrameData NextFrame() = 0;
  // Skips the next frame in case it doesn't need to be encoded.
  // Default implementation is to call NextFrame and ignore the returned value.
  virtual void SkipNextFrame() { NextFrame(); }

  // Change the capture resolution.
  virtual void ChangeResolution(size_t width, size_t height) = 0;

  virtual Resolution GetResolution() const = 0;

  // Returns the frames per second this generator is supposed to provide
  // according to its data source. Not all frame generators know the frames per
  // second of the data source, in such case this method returns std::nullopt.
  virtual std::optional<int> fps() const = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_FRAME_GENERATOR_INTERFACE_H_
