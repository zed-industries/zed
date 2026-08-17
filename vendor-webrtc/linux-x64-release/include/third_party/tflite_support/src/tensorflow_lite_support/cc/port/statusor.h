/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_PORT_STATUSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_PORT_STATUSOR_H_

// This header file is used to manage the depended StatusOr library. It creates
// an extra layer that makes it easier to switch between the desired version of
// StatusOr.
#include "absl/status/statusor.h"  // from @com_google_absl

namespace tflite {
namespace support {

template <typename T>
using StatusOr = absl::StatusOr<T>;

}  // namespace support
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_CC_PORT_STATUSOR_H_
