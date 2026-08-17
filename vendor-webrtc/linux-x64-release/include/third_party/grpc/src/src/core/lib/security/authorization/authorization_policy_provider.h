// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_AUTHORIZATION_POLICY_PROVIDER_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_AUTHORIZATION_POLICY_PROVIDER_H

#include <grpc/grpc_security.h>
#include <grpc/impl/channel_arg_names.h>
#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/lib/security/authorization/authorization_engine.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/useful.h"

struct grpc_authorization_policy_provider
    : public grpc_core::DualRefCounted<grpc_authorization_policy_provider> {
 public:
  static absl::string_view ChannelArgName() {
    return GRPC_ARG_AUTHORIZATION_POLICY_PROVIDER;
  }
  static int ChannelArgsCompare(const grpc_authorization_policy_provider* a,
                                const grpc_authorization_policy_provider* b) {
    return QsortCompare(a, b);
  }
  struct AuthorizationEngines {
    grpc_core::RefCountedPtr<grpc_core::AuthorizationEngine> allow_engine;
    grpc_core::RefCountedPtr<grpc_core::AuthorizationEngine> deny_engine;
  };
  virtual AuthorizationEngines engines() = 0;
};

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_AUTHORIZATION_POLICY_PROVIDER_H
