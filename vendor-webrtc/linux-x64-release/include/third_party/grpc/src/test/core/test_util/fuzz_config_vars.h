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

#ifndef GRPC_TEST_CORE_TEST_UTIL_FUZZ_CONFIG_VARS_H
#define GRPC_TEST_CORE_TEST_UTIL_FUZZ_CONFIG_VARS_H

// See fuzz_config_vars_helpers.h for a fuzztest domain that quickly
// finds interesting and legal configs.

#include <grpc/support/port_platform.h>

#include "src/core/config/config_vars.h"
#include "test/core/test_util/fuzz_config_vars.pb.h"

namespace grpc_core {

ConfigVars::Overrides OverridesFromFuzzConfigVars(
    const grpc::testing::FuzzConfigVars& vars);
void ApplyFuzzConfigVars(const grpc::testing::FuzzConfigVars& vars);

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_FUZZ_CONFIG_VARS_H
