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

#ifndef MEDIAPIPE_UTIL_TRACKING_IMAGE_UTIL_H_
#define MEDIAPIPE_UTIL_TRACKING_IMAGE_UTIL_H_

#include <vector>

#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/framework/port/opencv_imgproc_inc.h"
#include "mediapipe/framework/port/vector.h"
#include "mediapipe/util/tracking/motion_models.pb.h"
#include "mediapipe/util/tracking/region_flow.pb.h"

namespace mediapipe {

// Returns median of the L1 color distance between img_1 and img_2.
float FrameDifferenceMedian(const cv::Mat& img_1, const cv::Mat& img_2);

// Matlab's jet color map (returned assuming RGB channel order in [0, 1]
// normalized intensity domain). For details: http://goo.gl/gmHKZ
// Returned as map with num_entries entries.
void JetColoring(int num_entries, std::vector<Vector3_f>* color_map);

// Draws a saliency point frame to a single frame.
// Optionally renders axis aligned bounding box for each SalientPointFrame.
void RenderSaliency(const SalientPointFrame& salient_points,
                    const cv::Scalar& line_color, int line_thickness,
                    bool render_bounding_box, cv::Mat* image);

// Templated CopyBorder methods for increased speed. In-place border copy
// for specified Mat of type T with channels. Passed matrix is assumed to be of
// full size, that is we copy the content at [border, cols - 2 * border] x
// [border, rows - 2 * border] to the full size.
template <typename T, int border, int channels>
void CopyMatBorder(cv::Mat* mat);

// Same as above for copying border only in X or Y
template <typename T, int border, int channels>
void CopyMatBorderX(cv::Mat* mat);
template <typename T, int border, int channels>
void CopyMatBorderY(cv::Mat* mat);

template <typename T, int border, int channels>
void CopyMatBorder(cv::Mat* mat) {
  const int width = mat->cols - 2 * border;
  const int height = mat->rows - 2 * border;

  // Maximum values we clamp at to avoid going out of bound small images.
  const int max_w = width - 1;
  const int max_h = height - 1;

  // Top rows.
  for (int r = 0; r < border; ++r) {
    const T* src_ptr =
        mat->ptr<T>(border + std::min(r, max_h)) + border * channels;
    T* dst_ptr = mat->ptr<T>(border - 1 - r);

    // Top left elems.
    for (int i = 0; i < border; ++i, dst_ptr += channels) {
      for (int j = 0; j < channels; ++j) {
        dst_ptr[j] = src_ptr[std::min(max_w, border - 1 - i) * channels + j];
      }
    }

    // src and dst should point to same column from here.
    ABSL_DCHECK_EQ(0, (src_ptr - dst_ptr) * sizeof(T) % mat->step[0]);

    // Top row copy.
    memcpy(dst_ptr, src_ptr, width * channels * sizeof(dst_ptr[0]));
    src_ptr += width * channels;  // Points one behind end.
    dst_ptr += width * channels;

    // Top right elems.
    for (int i = 0; i < border; ++i, dst_ptr += channels) {
      if (i <= max_w) {
        src_ptr -= channels;
      }
      for (int j = 0; j < channels; ++j) {
        dst_ptr[j] = src_ptr[j];
      }
    }
  }

  // Left and right border.
  for (int r = 0; r < height; ++r) {
    // Get pointers to left most and right most column within image.
    T* left_ptr = mat->ptr<T>(r + border) + border * channels;
    T* right_ptr = left_ptr + (width - 1) * channels;
    for (int i = 0; i < border; ++i) {
      for (int j = 0; j < channels; ++j) {
        left_ptr[-(i + 1) * channels + j] =
            left_ptr[std::min(max_w, i) * channels + j];
        right_ptr[(i + 1) * channels + j] =
            right_ptr[-std::min(max_w, i) * channels + j];
      }
    }
  }

  // Bottom rows.
  for (int r = 0; r < border; ++r) {
    const T* src_ptr = mat->ptr<T>(border + height - 1 - std::min(r, max_h)) +
                       border * channels;
    T* dst_ptr = mat->ptr<T>(border + height + r);

    // First elems.
    for (int i = 0; i < border; ++i, dst_ptr += channels) {
      for (int j = 0; j < channels; ++j) {
        dst_ptr[j] = src_ptr[(border - 1 - std::min(max_w, i)) * channels + j];
      }
    }

    // src and dst should point to same column from here.
    ABSL_DCHECK_EQ(0, (dst_ptr - src_ptr) * sizeof(T) % mat->step[0]);
    memcpy(dst_ptr, src_ptr, width * channels * sizeof(dst_ptr[0]));
    src_ptr += width * channels;  // Points one behind the end.
    dst_ptr += width * channels;

    // Top right elems.
    for (int i = 0; i < border; ++i, dst_ptr += channels) {
      if (i <= max_w) {
        src_ptr -= channels;
      }
      for (int j = 0; j < channels; ++j) {
        dst_ptr[j] = src_ptr[j];
      }
    }
  }
}

template <typename T, int border, int channels>
void CopyMatBorderX(cv::Mat* mat) {
  const int width = mat->cols - 2 * border;
  const int height = mat->rows - 2 * border;

  // Maximum values we clamp at to avoid going out of bound small images.
  const int max_w = width - 1;

  // Left and right border.
  for (int r = 0; r < height; ++r) {
    T* left_ptr = mat->ptr<T>(r + border) + border * channels;
    T* right_ptr = left_ptr + (width - 1) * channels;
    for (int i = 0; i < border; ++i) {
      for (int j = 0; j < channels; ++j) {
        left_ptr[-(i + 1) * channels + j] =
            left_ptr[std::min(i, max_w) * channels + j];
        right_ptr[(i + 1) * channels + j] =
            right_ptr[-std::min(max_w, i) * channels + j];
      }
    }
  }
}

template <typename T, int border, int channels>
void CopyMatBorderY(cv::Mat* mat) {
  const int width = mat->cols - 2 * border;
  const int height = mat->rows - 2 * border;

  // Maximum values we clamp at to avoid going out of bound small images.
  const int max_h = height - 1;

  // Top rows.
  for (int r = 0; r < border; ++r) {
    const T* src_ptr =
        mat->ptr<T>(border + std::min(max_h, r)) + border * channels;
    T* dst_ptr = mat->ptr<T>(border - 1 - r) + border * channels;
    memcpy(dst_ptr, src_ptr, width * channels * sizeof(dst_ptr[0]));
  }

  // Bottom rows.
  for (int r = 0; r < border; ++r) {
    const T* src_ptr = mat->ptr<T>(border + height - 1 - std::min(max_h, r)) +
                       border * channels;
    T* dst_ptr = mat->ptr<T>(border + height + r) + border * channels;
    memcpy(dst_ptr, src_ptr, width * channels * sizeof(dst_ptr[0]));
  }
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_IMAGE_UTIL_H_
