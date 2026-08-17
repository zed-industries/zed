/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_H_
#define CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_H_

#include <optional>

#include "api/video/video_codec_type.h"

namespace webrtc {

// The source resolution, frame rate and other properties of a
// VideoStreamEncoder.
class VideoStreamInputState {
 public:
  VideoStreamInputState();

  void set_has_input(bool has_input);
  void set_frame_size_pixels(std::optional<int> frame_size_pixels);
  void set_frames_per_second(int frames_per_second);
  void set_video_codec_type(VideoCodecType video_codec_type);
  void set_min_pixels_per_frame(int min_pixels_per_frame);
  void set_single_active_stream_pixels(
      std::optional<int> single_active_stream_pixels);

  bool has_input() const;
  std::optional<int> frame_size_pixels() const;
  int frames_per_second() const;
  VideoCodecType video_codec_type() const;
  int min_pixels_per_frame() const;
  std::optional<int> single_active_stream_pixels() const;

  bool HasInputFrameSizeAndFramesPerSecond() const;

 private:
  bool has_input_;
  std::optional<int> frame_size_pixels_;
  int frames_per_second_;
  VideoCodecType video_codec_type_;
  int min_pixels_per_frame_;
  std::optional<int> single_active_stream_pixels_;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_H_
