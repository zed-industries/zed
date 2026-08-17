/*
 * Copyright 2022 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_GET_HWY_MODE_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_GET_HWY_MODE_H_

#include "absl/strings/string_view.h"

namespace distributed_point_functions {
namespace dpf_internal {

// Utility function for printing the mode selected by Highway. Used for
// debugging.
const absl::string_view GetHwyModeAsString();

}  // namespace dpf_internal
}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_GET_HWY_MODE_H_
