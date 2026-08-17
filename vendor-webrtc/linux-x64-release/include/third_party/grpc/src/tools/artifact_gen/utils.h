// Copyright 2025 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_TOOLS_ARTIFACT_GEN_UTILS_H
#define GRPC_TOOLS_ARTIFACT_GEN_UTILS_H

#include "include/nlohmann/json.hpp"

nlohmann::json LoadYaml(const std::string& filename);
std::vector<std::string> AllFilesInDir(const std::string& dir);
std::string LoadString(const std::string& filename);

#endif  // GRPC_TOOLS_ARTIFACT_GEN_UTILS_H
