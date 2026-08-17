/* Copyright 2023 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_CONVERTER_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_CONVERTER_H_

#include "mediapipe/tasks/c/components/containers/detection_result.h"
#include "mediapipe/tasks/cc/components/containers/detection_result.h"

namespace mediapipe::tasks::c::components::containers {

void CppConvertToDetection(
    const mediapipe::tasks::components::containers::Detection& in,
    Detection* out);

void CppConvertToDetectionResult(
    const mediapipe::tasks::components::containers::DetectionResult& in,
    DetectionResult* out);

void CppCloseDetection(Detection* in);

void CppCloseDetectionResult(DetectionResult* in);

}  // namespace mediapipe::tasks::c::components::containers

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_CONVERTER_H_
