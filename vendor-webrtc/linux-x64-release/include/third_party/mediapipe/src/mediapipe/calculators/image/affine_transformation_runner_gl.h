// Copyright 2021 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_RUNNER_GL_H_
#define MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_RUNNER_GL_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/calculators/image/affine_transformation.h"
#include "mediapipe/gpu/gl_calculator_helper.h"
#include "mediapipe/gpu/gpu_buffer.h"
#include "mediapipe/gpu/gpu_origin.pb.h"

namespace mediapipe {

absl::StatusOr<std::unique_ptr<AffineTransformation::Runner<
    mediapipe::GpuBuffer, std::unique_ptr<mediapipe::GpuBuffer>>>>
CreateAffineTransformationGlRunner(
    std::shared_ptr<mediapipe::GlCalculatorHelper> gl_helper,
    mediapipe::GpuOrigin::Mode gpu_origin,
    AffineTransformation::Interpolation interpolation);

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_RUNNER_GL_H_
