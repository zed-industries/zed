//
//
// Copyright 2015 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_LIB_IOMGR_RESOLVE_ADDRESS_H
#define GRPC_SRC_CORE_LIB_IOMGR_RESOLVE_ADDRESS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/container/flat_hash_set.h"
#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/handle_containers.h"
#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/time.h"

#define GRPC_MAX_SOCKADDR_SIZE 128

namespace grpc_core {
extern const char* kDefaultSecurePort;
constexpr int kDefaultSecurePortInt = 443;
constexpr Duration kDefaultDNSRequestTimeout = Duration::Minutes(2);

// A singleton class used for async and blocking DNS resolution
class DNSResolver {
 public:
  /// Task handle for DNS Resolution requests.
  struct LookupTaskHandle {
    intptr_t keys[2];
    static const LookupTaskHandle kInvalid;
    friend bool operator==(const LookupTaskHandle& lhs,
                           const LookupTaskHandle& rhs);
    friend bool operator!=(const LookupTaskHandle& lhs,
                           const LookupTaskHandle& rhs);
  };
  using TaskHandle = LookupTaskHandle;
  using TaskHandleSet = absl::flat_hash_set<
      TaskHandle,
      grpc_event_engine::experimental::TaskHandleComparator<TaskHandle>::Hash>;

  static const TaskHandle kNullHandle;

  virtual ~DNSResolver() {}

  static std::string HandleToString(TaskHandle handle);

  // Asynchronously resolve name. Use \a default_port if a port isn't designated
  // in \a name, otherwise use the port in \a name. On completion, \a on_done is
  // invoked with the result.
  //
  // Note for implementations: calls may acquire locks in \a on_done which
  // were previously held while starting the request. Therefore,
  // implementations must not invoke \a on_done inline from the call site that
  // starts the request. The DNSCallbackExecCtxScheduler utility may help
  // address this.
  //
  // \a interested_parties may be deleted after a request is cancelled.
  virtual TaskHandle LookupHostname(
      std::function<void(absl::StatusOr<std::vector<grpc_resolved_address>>)>
          on_resolved,
      absl::string_view name, absl::string_view default_port, Duration timeout,
      grpc_pollset_set* interested_parties, absl::string_view name_server) = 0;

  // Resolve name in a blocking fashion. Use \a default_port if a port isn't
  // designated in \a name, otherwise use the port in \a name.
  virtual absl::StatusOr<std::vector<grpc_resolved_address>>
  LookupHostnameBlocking(absl::string_view name,
                         absl::string_view default_port) = 0;

  // Asynchronously resolve an SRV Record to Hostnames.
  // On completion, \a on_done is invoked with the result.
  //
  // The same caveats in \a LookupHostname apply here as well.
  //
  // TODO(hork): return std::vector<SRVRecord> and ask the client to do the
  // subsequent hostname lookups.
  virtual TaskHandle LookupSRV(
      std::function<void(absl::StatusOr<std::vector<grpc_resolved_address>>)>
          on_resolved,
      absl::string_view name, Duration timeout,
      grpc_pollset_set* interested_parties, absl::string_view name_server) = 0;

  // Asynchronously resolve a TXT Record. On completion, \a on_done is invoked
  // with the resulting string.
  //
  // The same caveats in \a LookupHostname apply here.
  virtual TaskHandle LookupTXT(
      std::function<void(absl::StatusOr<std::string>)> on_resolved,
      absl::string_view name, Duration timeout,
      grpc_pollset_set* interested_parties, absl::string_view name_server) = 0;

  // This shares the same semantics with \a EventEngine::Cancel: successfully
  // cancelled lookups will not have their callbacks executed, and this
  // method returns true. If a TaskHandle is unknown, this method should return
  // false.
  virtual bool Cancel(TaskHandle handle) = 0;
};

// Override the active DNS resolver which should be used for all DNS
// resolution in gRPC.
void ResetDNSResolver(std::shared_ptr<DNSResolver> resolver);

// Get the singleton DNS resolver instance which should be used for all
// DNS resolution in gRPC.
std::shared_ptr<DNSResolver> GetDNSResolver();

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_IOMGR_RESOLVE_ADDRESS_H
