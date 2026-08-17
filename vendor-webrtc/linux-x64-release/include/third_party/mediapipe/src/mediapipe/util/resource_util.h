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

#ifndef MEDIAPIPE_UTIL_RESOURCE_UTIL_H_
#define MEDIAPIPE_UTIL_RESOURCE_UTIL_H_

#include <string>

#include "absl/base/attributes.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"

namespace mediapipe {

// Given a path to a resource, this function attempts to provide an absolute
// path with which it can be accessed as a file.
// - If the input path is an absolute path, it is returned as-is.
// - If the input path is relative, it is searched in a platform-specific
//   location:
//   - On Android with `shadow_copy`, we look for an asset with the given
//     relative path; if it exists, it is copied to the file system (using
//     the AssetCache), and a path to that file is returned.
//   - On iOS, we look for a resource with the given relative path in the
//     application bundle.
//
// Note: The exact search algorithm is subject to change.
// Note: This function should be used by code that needs a resource to be
// accessible as a normal file, usually to call an existing API that only
// accepts file paths. Code that can access data as a stream or as a buffer
// should use the Resources API (see below).
absl::StatusOr<std::string> PathToResourceAsFile(const std::string& path,
                                                 bool shadow_copy = true);

// DEPRECATED: use `CalculatorContext::GetResources` and
// `SubgraphContext::GetResources` which allow for fine grained per graph
// resource loading configuration.
//
// Reads the entire contents of a resource. The search path is as in
// PathToResourceAsFile.
ABSL_DEPRECATED(
    "Use `CalculatorContext::GetResources` and "
    "`SubgraphContext::GetResources` which allow for fine grained per graph "
    "resource loading configuration.")
absl::Status GetResourceContents(const std::string& path, std::string* output,
                                 bool read_as_binary = true);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_RESOURCE_UTIL_H_
