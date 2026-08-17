// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TFLITE_DELEGATE_PTR_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TFLITE_DELEGATE_PTR_H_

#include <functional>
#include <memory>

#include "tensorflow/lite/c/c_api_types.h"

namespace mediapipe {

// TODO: Consider renaming TfLiteDelegatePtr.
using TfLiteDelegatePtr =
    std::unique_ptr<TfLiteOpaqueDelegate,
                    std::function<void(TfLiteOpaqueDelegate*)>>;

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TFLITE_DELEGATE_PTR_H_
