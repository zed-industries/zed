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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_POLICY_PROVIDER_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_POLICY_PROVIDER_H

#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include <functional>
#include <memory>
#include <string>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/security/authorization/authorization_engine.h"
#include "src/core/lib/security/authorization/authorization_policy_provider.h"
#include "src/core/lib/security/authorization/rbac_translator.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/thd.h"

namespace grpc_core {

// Provider class will get gRPC Authorization policy from string during
// initialization. This policy will be translated to Envoy RBAC policies and
// used to initialize allow and deny AuthorizationEngine objects. This provider
// will return the same authorization engines everytime.
class StaticDataAuthorizationPolicyProvider
    : public grpc_authorization_policy_provider {
 public:
  static absl::StatusOr<RefCountedPtr<grpc_authorization_policy_provider>>
  Create(absl::string_view authz_policy);

  // Use factory method "Create" to create an instance of
  // StaticDataAuthorizationPolicyProvider.
  explicit StaticDataAuthorizationPolicyProvider(RbacPolicies policies);

  AuthorizationEngines engines() override {
    return {allow_engine_, deny_engine_};
  }

  void Orphaned() override {}

 private:
  RefCountedPtr<AuthorizationEngine> allow_engine_;
  RefCountedPtr<AuthorizationEngine> deny_engine_;
};

// Provider class will get gRPC Authorization policy from provided file path.
// This policy will be translated to Envoy RBAC policies and used to initialize
// allow and deny AuthorizationEngine objects. This provider will periodically
// load file contents in specified path, and upon modification update the engine
// instances with new policy configuration. During reload if the file contents
// are invalid or there are I/O errors, we will skip that particular update and
// log error status. The authorization decisions will be made using the latest
// valid policy.
class FileWatcherAuthorizationPolicyProvider
    : public grpc_authorization_policy_provider {
 public:
  static absl::StatusOr<RefCountedPtr<grpc_authorization_policy_provider>>
  Create(absl::string_view authz_policy_path,
         unsigned int refresh_interval_sec);

  // Use factory method "Create" to create an instance of
  // FileWatcherAuthorizationPolicyProvider.
  FileWatcherAuthorizationPolicyProvider(absl::string_view authz_policy_path,
                                         unsigned int refresh_interval_sec,
                                         absl::Status* status);

  void SetCallbackForTesting(
      std::function<void(bool contents_changed, absl::Status Status)> cb);

  void Orphaned() override;

  AuthorizationEngines engines() override {
    MutexLock lock(&mu_);
    return {allow_engine_, deny_engine_};
  }

 private:
  // Force an update from the file system regardless of the interval.
  absl::Status ForceUpdate();

  std::string authz_policy_path_;
  std::string file_contents_;
  unsigned int refresh_interval_sec_;

  std::unique_ptr<Thread> refresh_thread_;
  gpr_event shutdown_event_;

  Mutex mu_;
  // Callback is executed on every reload. This is useful for testing purpose.
  std::function<void(bool contents_changed, absl::Status status)> cb_
      ABSL_GUARDED_BY(mu_) = nullptr;
  // Engines created using authz_policy_.
  RefCountedPtr<AuthorizationEngine> allow_engine_ ABSL_GUARDED_BY(mu_);
  RefCountedPtr<AuthorizationEngine> deny_engine_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_POLICY_PROVIDER_H
