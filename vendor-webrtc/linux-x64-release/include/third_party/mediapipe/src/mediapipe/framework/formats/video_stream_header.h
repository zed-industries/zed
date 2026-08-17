// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// The Video stream header.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_VIDEO_STREAM_HEADER_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_VIDEO_STREAM_HEADER_H_

#include "mediapipe/framework/formats/image_format.pb.h"

namespace mediapipe {

// This defines the format of a video stream header.
struct VideoHeader {
  // Video frame format.
  ImageFormat::Format format = ImageFormat::UNKNOWN;

  // Dimensions of the video in pixels.
  int width = 0;
  int height = 0;

  // Video duration in seconds.
  // NOTE: This field was introduced after the others, so it is not widely
  // supported. If you use it, make sure that all intermediate calculators pass
  // it through.
  float duration = 0.0f;

  // The frame rate in Hz at which the video frames are output.
  double frame_rate = 0.0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_VIDEO_STREAM_HEADER_H_
