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

// Consider this file an implementation detail. None of this is part of the
// public API.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_POOL_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_POOL_H_

#include <utility>
#include <vector>

#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/formats/image_frame.h"

namespace mediapipe {

using ImageFrameSharedPtr = std::shared_ptr<ImageFrame>;

class ImageFramePool : public std::enable_shared_from_this<ImageFramePool> {
 public:
  // Creates a pool. This pool will manage buffers of the specified dimensions,
  // and will keep keep_count buffers around for reuse.
  // We enforce creation as a shared_ptr so that we can use a weak reference in
  // the buffers' deleters.
  static std::shared_ptr<ImageFramePool> Create(int width, int height,
                                                ImageFormat::Format format,
                                                int keep_count) {
    return std::shared_ptr<ImageFramePool>(
        new ImageFramePool(width, height, format, keep_count));
  }

  // Obtains a buffers. May either be reused or created anew.
  ImageFrameSharedPtr GetBuffer();

  int width() const { return width_; }
  int height() const { return height_; }
  ImageFormat::Format format() const { return format_; }

  // This method is meant for testing.
  std::pair<int, int> GetInUseAndAvailableCounts();

 private:
  ImageFramePool(int width, int height, ImageFormat::Format format,
                 int keep_count);

  // Return a buffer to the pool.
  void Return(ImageFrame* buf);

  // If the total number of buffers is greater than keep_count, destroys any
  // surplus buffers that are no longer in use.
  void TrimAvailable(std::vector<std::unique_ptr<ImageFrame>>* trimmed)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  const int width_;
  const int height_;
  const ImageFormat::Format format_;
  const int keep_count_;

  absl::Mutex mutex_;
  int in_use_count_ ABSL_GUARDED_BY(mutex_) = 0;
  std::vector<std::unique_ptr<ImageFrame>> available_ ABSL_GUARDED_BY(mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_POOL_H_
