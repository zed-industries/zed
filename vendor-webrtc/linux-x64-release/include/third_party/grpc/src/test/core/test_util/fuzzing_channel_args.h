// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_FUZZING_CHANNEL_ARGS_H
#define GRPC_TEST_CORE_TEST_UTIL_FUZZING_CHANNEL_ARGS_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/util/ref_counted_ptr.h"
#include "test/core/test_util/fuzzing_channel_args.pb.h"

namespace grpc_core {
namespace testing {

struct FuzzingEnvironment {
  // This resource quota is only added to ChannelArgs if the fuzzing
  // configuration requests it.
  RefCountedPtr<ResourceQuota> resource_quota =
      MakeResourceQuota("fuzzing_quota");
};

// Create ChannelArgs from a fuzzer configuration.
ChannelArgs CreateChannelArgsFromFuzzingConfiguration(
    const grpc::testing::FuzzingChannelArgs& fuzzing_channel_args,
    const FuzzingEnvironment& fuzzing_environment);

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_FUZZING_CHANNEL_ARGS_H
