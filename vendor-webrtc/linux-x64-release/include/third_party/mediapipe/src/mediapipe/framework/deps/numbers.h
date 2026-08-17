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

#ifndef MEDIAPIPE_DEPS_NUMBERS_H_
#define MEDIAPIPE_DEPS_NUMBERS_H_

#include <cstdint>
#include <string>

#include "absl/strings/numbers.h"
#include "absl/strings/str_cat.h"

namespace mediapipe {
ABSL_MUST_USE_RESULT inline std::string SimpleDtoa(double d) {
  if (static_cast<double>(static_cast<int64_t>(d)) == d) {
    return absl::StrCat(static_cast<int64_t>(d));
  } else {
    return absl::StrCat(d);
  }
}
}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_NUMBERS_H_
