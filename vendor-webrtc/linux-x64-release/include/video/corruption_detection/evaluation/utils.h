/*
 * Copyright 2025 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_EVALUATION_UTILS_H_
#define VIDEO_CORRUPTION_DETECTION_EVALUATION_UTILS_H_

#include <cstdint>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/system/file_wrapper.h"

namespace webrtc {

// Creates a temporary Y4M file with the given `width`, `height` and
// `framerate`. The temporary file is removed when the class is destroyed.
class TempY4mFileCreator {
 public:
  TempY4mFileCreator(int width, int height, int framerate);

  // Removes the temporary created file.
  ~TempY4mFileCreator();

  // Creates a temporary Y4M video file with the content given by
  // `file_content`. `file_content` should have YUV420p format, where each frame
  // is of size `width_ * height_ * 3 / 2` and stack one after another in YYYYUV
  // format.
  //
  // The number of frames depends on the size of `file_content`.
  void CreateTempY4mFile(ArrayView<const uint8_t> file_content);

  absl::string_view y4m_filepath() const { return y4m_filepath_; }

 private:
  // Writes the file header. It populates file header with the width, height and
  // framerate information given by the class constructor.
  void WriteFileHeader(FileWrapper& video_file) const;

  const int width_;
  const int height_;
  const int framerate_;
  const int frame_size_;

  const std::string y4m_filepath_;
};

struct Y4mMetadata {
  int width = 0;
  int height = 0;
  int framerate = 0;
};

Y4mMetadata ReadMetadataFromY4mHeader(absl::string_view clip_path);

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_EVALUATION_UTILS_H_
