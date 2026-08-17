// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_UTILS_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_UTILS_H_

#include <array>
#include <optional>

#include "absl/types/optional.h"
#include "mediapipe/calculators/tensor/image_to_tensor_calculator.pb.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/statusor.h"
#if !MEDIAPIPE_DISABLE_GPU
#include "mediapipe/gpu/gpu_buffer.h"
#endif  // !MEDIAPIPE_DISABLE_GPU
#include "mediapipe/gpu/gpu_origin.pb.h"

namespace mediapipe {

struct RotatedRect {
  float center_x;
  float center_y;
  float width;
  float height;
  float rotation;
};

// Pixel extrapolation method.
// When converting image to tensor it may happen that tensor needs to read
// pixels outside image boundaries. Border mode helps to specify how such pixels
// will be calculated.
// TODO: Consider moving this to a separate border_mode.h file.
enum class BorderMode { kZero, kReplicate };

// Struct that host commonly accessed parameters used in the
// ImageTo[Batch]TensorCalculator.
struct OutputTensorParams {
  std::optional<int> output_height;
  std::optional<int> output_width;
  int output_batch;
  bool is_float_output;
  float range_min;
  float range_max;
};

// Generates a new ROI or converts it from normalized rect.
RotatedRect GetRoi(int input_width, int input_height,
                   absl::optional<mediapipe::NormalizedRect> norm_rect);

// Pads ROI, so extraction happens correctly if aspect ratio is to be kept.
// Returns letterbox padding applied.
absl::StatusOr<std::array<float, 4>> PadRoi(int input_tensor_width,
                                            int input_tensor_height,
                                            bool keep_aspect_ratio,
                                            RotatedRect* roi);

// Represents a transformation of value which involves scaling and offsetting.
// To apply transformation:
// ValueTransformation transform = ...
// float transformed_value = transform.scale * value + transform.offset;
struct ValueTransformation {
  float scale;
  float offset;
};

// Returns value transformation to apply to a value in order to convert it from
// [from_range_min, from_range_max] into [to_range_min, to_range_max] range.
// from_range_min must be less than from_range_max
// to_range_min must be less than to_range_max
absl::StatusOr<ValueTransformation> GetValueRangeTransformation(
    float from_range_min, float from_range_max, float to_range_min,
    float to_range_max);

// Populates 4x4 "matrix" with row major order transformation matrix which
// maps (x, y) in range [0, 1] (describing points of @sub_rect)
// to (x', y') in range [0, 1]*** (describing points of a rect:
// [0, @rect_width] x [0, @rect_height] = RECT).
//
// *** (x', y') will go out of the range for points from @sub_rect
//     which are not contained by RECT and it's expected behavior
//
// @sub_rect - rotated sub rect in absolute coordinates
// @rect_width - rect width
// @rect_height - rect height
// @flip_horizontally - we need to flip the output buffer.
// @matrix - 4x4 matrix (array of 16 elements) to populate
void GetRotatedSubRectToRectTransformMatrix(const RotatedRect& sub_rect,
                                            int rect_width, int rect_height,
                                            bool flip_horizontally,
                                            std::array<float, 16>* matrix);

// Returns the transpose of the matrix found with
// "GetRotatedSubRectToRectTransformMatrix".  That is to say, this populates a
// 4x4 "matrix" with col major order transformation matrix which maps (x, y) in
// range [0, 1] (describing points of @sub_rect) to (x', y') in range [0, 1]***
// (describing points of a rect: [0, @rect_width] x [0, @rect_height] = RECT).
//
// *** (x', y') will go out of the range for points from @sub_rect
//     which are not contained by RECT and it's expected behavior
//
// @sub_rect - rotated sub rect in absolute coordinates
// @rect_width - rect width
// @rect_height - rect height
// @flip_horizontally - we need to flip the output buffer.
// @matrix - 4x4 matrix (array of 16 elements) to populate
void GetTransposedRotatedSubRectToRectTransformMatrix(
    const RotatedRect& sub_rect, int rect_width, int rect_height,
    bool flip_horizontally, std::array<float, 16>* matrix);

// Validates the output dimensions set in the option proto. The input option
// proto is expected to have to following fields:
//  output_tensor_float_range, output_tensor_int_range, output_tensor_uint_range
//  output_tensor_width, output_tensor_height.
// See ImageToTensorCalculatorOptions for the description of each field.
template <typename T>
absl::Status ValidateOptionOutputDims(const T& options) {
  RET_CHECK(options.has_output_tensor_float_range() ||
            options.has_output_tensor_int_range() ||
            options.has_output_tensor_uint_range())
      << "Output tensor range is required.";
  if (options.has_output_tensor_float_range()) {
    RET_CHECK_LT(options.output_tensor_float_range().min(),
                 options.output_tensor_float_range().max())
        << "Valid output float tensor range is required.";
  }
  if (options.has_output_tensor_uint_range()) {
    RET_CHECK_LT(options.output_tensor_uint_range().min(),
                 options.output_tensor_uint_range().max())
        << "Valid output uint tensor range is required.";
    RET_CHECK_GE(options.output_tensor_uint_range().min(), 0)
        << "The minimum of the output uint tensor range must be "
           "non-negative.";
    RET_CHECK_LE(options.output_tensor_uint_range().max(), 255)
        << "The maximum of the output uint tensor range must be less than or "
           "equal to 255.";
  }
  if (options.has_output_tensor_int_range()) {
    RET_CHECK_LT(options.output_tensor_int_range().min(),
                 options.output_tensor_int_range().max())
        << "Valid output int tensor range is required.";
    RET_CHECK_GE(options.output_tensor_int_range().min(), -128)
        << "The minimum of the output int tensor range must be greater than "
           "or equal to -128.";
    RET_CHECK_LE(options.output_tensor_int_range().max(), 127)
        << "The maximum of the output int tensor range must be less than or "
           "equal to 127.";
  }
  if (options.has_output_tensor_width()) {
    RET_CHECK_GT(options.output_tensor_width(), 0)
        << "Valid output tensor width is required.";
  }
  if (options.has_output_tensor_height()) {
    RET_CHECK_GT(options.output_tensor_height(), 0)
        << "Valid output tensor height is required.";
  }
  return absl::OkStatus();
}

template <typename T>
OutputTensorParams GetOutputTensorParams(const T& options) {
  OutputTensorParams params;
  if (options.has_output_tensor_uint_range()) {
    params.range_min =
        static_cast<float>(options.output_tensor_uint_range().min());
    params.range_max =
        static_cast<float>(options.output_tensor_uint_range().max());
  } else if (options.has_output_tensor_int_range()) {
    params.range_min =
        static_cast<float>(options.output_tensor_int_range().min());
    params.range_max =
        static_cast<float>(options.output_tensor_int_range().max());
  } else {
    params.range_min = options.output_tensor_float_range().min();
    params.range_max = options.output_tensor_float_range().max();
  }
  if (options.has_output_tensor_width()) {
    params.output_width = options.output_tensor_width();
  }
  if (options.has_output_tensor_height()) {
    params.output_height = options.output_tensor_height();
  }
  params.is_float_output = options.has_output_tensor_float_range();
  params.output_batch = 1;
  return params;
}

// Converts the BorderMode proto into struct.
BorderMode GetBorderMode(
    const mediapipe::ImageToTensorCalculatorOptions::BorderMode& mode);

// Gets the output tensor type.
Tensor::ElementType GetOutputTensorType(bool uses_gpu,
                                        const OutputTensorParams& params);

// Gets the number of output channels from the input Image format.
int GetNumOutputChannels(const mediapipe::Image& image);

// Converts the packet that hosts different format (Image, ImageFrame,
// GpuBuffer) into the mediapipe::Image format.
absl::StatusOr<std::shared_ptr<const mediapipe::Image>> GetInputImage(
    const api2::Packet<api2::OneOf<Image, mediapipe::ImageFrame>>&
        image_packet);

#if !MEDIAPIPE_DISABLE_GPU
absl::StatusOr<std::shared_ptr<const mediapipe::Image>> GetInputImage(
    const api2::Packet<mediapipe::GpuBuffer>& image_gpu_packet);
#endif  // !MEDIAPIPE_DISABLE_GPU

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_UTILS_H_
