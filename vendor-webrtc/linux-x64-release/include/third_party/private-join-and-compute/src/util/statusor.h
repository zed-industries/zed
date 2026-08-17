/*
 * Copyright 2019 Google Inc.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef UTIL_STATUSOR_H_
#define UTIL_STATUSOR_H_

#include "third_party/abseil-cpp/absl/status/statusor.h"

namespace private_join_and_compute {

template <typename T>
using StatusOr = absl::StatusOr<T>;

}  // namespace private_join_and_compute

#endif  // UTIL_STATUSOR_H_
