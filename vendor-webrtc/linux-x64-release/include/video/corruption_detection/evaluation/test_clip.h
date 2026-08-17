/*
 * Copyright 2025 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_EVALUATION_TEST_CLIP_H_
#define VIDEO_CORRUPTION_DETECTION_EVALUATION_TEST_CLIP_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/video_codecs/video_codec.h"
#include "test/testsupport/file_utils.h"

namespace webrtc {

// Identifies a test clip.
// If the clip is a YUV video, the user should use the `CreateYuvClip` method.
// Hence, the user should also give information about the resolution and
// framerate of the video. If the clip is a Y4M video, the resolution and
// framerate are derived from the file header, by using the `CreateY4mClip`
// method.
class TestClip {
 public:
  static TestClip CreateYuvClip(absl::string_view filename,
                                int width,
                                int height,
                                int framerate,
                                VideoCodecMode codec_mode);

  static TestClip CreateY4mClip(absl::string_view filename,
                                VideoCodecMode codec_mode);

  // Returns the path to the video with the '.yuv' or '.y4m' extension.
  // Observe that this path can only be reached as long as the `TestClip`
  // instance is alive.
  absl::string_view clip_path() const { return clip_path_with_extension_; }
  VideoCodecMode codec_mode() const { return codec_mode_; }
  int width() const { return width_; }
  int height() const { return height_; }
  int framerate() const { return framerate_; }
  bool is_yuv() const { return is_yuv_; }

 private:
  TestClip(absl::string_view clip_path_with_extension,
           int width,
           int height,
           int framerate,
           VideoCodecMode codec_mode,
           bool is_yuv)
      : clip_path_with_extension_(std::string(clip_path_with_extension)),
        codec_mode_(codec_mode),
        width_(width),
        height_(height),
        framerate_(framerate),
        is_yuv_(is_yuv) {}

  // The path to the video with the '.yuv' or '.y4m' extension.
  const std::string clip_path_with_extension_;
  // Specifies whether the video is a real time or a screensharing video. It
  // is used to initialize the encoder properly.
  const VideoCodecMode codec_mode_;
  const int width_ = 0;
  const int height_ = 0;
  const int framerate_ = 0;
  const bool is_yuv_ = false;
};

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_EVALUATION_TEST_CLIP_H_
