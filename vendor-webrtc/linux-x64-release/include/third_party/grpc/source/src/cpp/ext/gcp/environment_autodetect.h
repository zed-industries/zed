//
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
//

#ifndef GRPC_SRC_CPP_EXT_GCP_ENVIRONMENT_AUTODETECT_H
#define GRPC_SRC_CPP_EXT_GCP_ENVIRONMENT_AUTODETECT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "src/core/util/sync.h"

namespace grpc {

namespace internal {

absl::Status GcpObservabilityInit();

class EnvironmentAutoDetect {
 public:
  struct ResourceType {
    // For example, "gce_instance", "gke_container", etc.
    std::string resource_type;
    // Values for all the labels listed in the associated resource type.
    std::map<std::string, std::string> labels;
  };

  static EnvironmentAutoDetect& Get();

  // Exposed for testing purposes only
  explicit EnvironmentAutoDetect(std::string project_id);

  // \a callback will be invoked once the environment is done being detected.
  void NotifyOnDone(absl::AnyInvocable<void()> callback);

  const ResourceType* resource() {
    grpc_core::MutexLock lock(&mu_);
    return resource_.get();
  }

 private:
  friend absl::Status grpc::internal::GcpObservabilityInit();

  // GcpObservabilityInit() is responsible for setting up the singleton with the
  // project_id.
  static void Create(std::string project_id);

  const std::string project_id_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
  grpc_core::Mutex mu_;
  std::unique_ptr<ResourceType> resource_ ABSL_GUARDED_BY(mu_);
  std::vector<absl::AnyInvocable<void()>> callbacks_ ABSL_GUARDED_BY(mu_);
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_GCP_ENVIRONMENT_AUTODETECT_H
