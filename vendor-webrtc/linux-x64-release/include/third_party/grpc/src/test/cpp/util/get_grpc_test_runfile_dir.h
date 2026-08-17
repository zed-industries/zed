// Copyright 2023 The gRPC Authors.
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

#ifndef GRPC_TEST_CPP_UTIL_GET_GRPC_TEST_RUNFILE_DIR_H
#define GRPC_TEST_CPP_UTIL_GET_GRPC_TEST_RUNFILE_DIR_H

#include <optional>
#include <string>

namespace grpc {

// Gets the absolute path of the runfile directory (a bazel/blaze concept) for a
// gRPC test. The path to the data files can be referred by joining the runfile
// directory with the workspace-relative path (e.g.
// "test/cpp/util/get_grpc_test_runfile_dir.h").
std::optional<std::string> GetGrpcTestRunFileDir();

}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_GET_GRPC_TEST_RUNFILE_DIR_H
