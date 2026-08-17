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

#ifndef GRPC_TEST_CORE_TEST_UTIL_POSTMORTEM_H
#define GRPC_TEST_CORE_TEST_UTIL_POSTMORTEM_H

namespace grpc_core {

// Helper class to dump useful post-mortem analysis in the event of a test
// failure.
class PostMortem {
 public:
  // Used as a scoped object PostMortem will check gtest failure and emit
  // state if it sees a test failure on destruction.
  ~PostMortem();

  // Additionally, the library can be used to directly emit state in the event
  // of a crash, for instance.
  static void Emit();
};

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_POSTMORTEM_H
