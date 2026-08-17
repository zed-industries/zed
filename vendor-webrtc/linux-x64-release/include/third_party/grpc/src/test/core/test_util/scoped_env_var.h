//
// Copyright 2022 gRPC authors.
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
//

#ifndef GRPC_TEST_CORE_TEST_UTIL_SCOPED_ENV_VAR_H
#define GRPC_TEST_CORE_TEST_UTIL_SCOPED_ENV_VAR_H

#include <grpc/support/port_platform.h>

#include "src/core/util/env.h"

namespace grpc_core {
namespace testing {

class ScopedEnvVar {
 public:
  ScopedEnvVar(const char* env_var, const char* value) : env_var_(env_var) {
    SetEnv(env_var_, value);
  }

  virtual ~ScopedEnvVar() { UnsetEnv(env_var_); }

 private:
  const char* env_var_;
};

class ScopedExperimentalEnvVar : public ScopedEnvVar {
 public:
  explicit ScopedExperimentalEnvVar(const char* env_var)
      : ScopedEnvVar(env_var, "true") {}
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_SCOPED_ENV_VAR_H
