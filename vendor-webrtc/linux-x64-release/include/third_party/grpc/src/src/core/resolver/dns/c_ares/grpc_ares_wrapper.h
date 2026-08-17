//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_WRAPPER_H
#define GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_WRAPPER_H

#include <ares.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/log/log.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/util/sync.h"

#if GRPC_ARES == 1

#define GRPC_DNS_ARES_DEFAULT_QUERY_TIMEOUT_MS 120000

typedef struct grpc_ares_ev_driver grpc_ares_ev_driver;

struct grpc_ares_request {
  /// synchronizes access to this request, and also to associated
  /// ev_driver and fd_node objects
  grpc_core::Mutex mu;
  /// indicates the DNS server to use, if specified
  struct ares_addr_port_node dns_server_addr ABSL_GUARDED_BY(mu);
  /// following members are set in grpc_resolve_address_ares_impl
  /// closure to call when the request completes
  grpc_closure* on_done ABSL_GUARDED_BY(mu) = nullptr;
  /// the pointer to receive the resolved addresses
  std::unique_ptr<grpc_core::EndpointAddressesList>* addresses_out
      ABSL_GUARDED_BY(mu);
  /// the pointer to receive the resolved balancer addresses
  std::unique_ptr<grpc_core::EndpointAddressesList>* balancer_addresses_out
      ABSL_GUARDED_BY(mu);
  /// the pointer to receive the service config in JSON
  char** service_config_json_out ABSL_GUARDED_BY(mu) = nullptr;
  /// the event driver used by this request
  grpc_ares_ev_driver* ev_driver ABSL_GUARDED_BY(mu) = nullptr;
  /// number of ongoing queries
  size_t pending_queries ABSL_GUARDED_BY(mu) = 0;
  /// the errors explaining query failures, appended to in query callbacks
  grpc_error_handle error ABSL_GUARDED_BY(mu);
};

// Asynchronously resolve \a name (A/AAAA records only).
// It uses \a default_port if a port isn't designated in \a name, otherwise it
// uses the port in \a name. grpc_ares_init() must be called at least once
// before this function. The returned grpc_ares_request object is owned by the
// caller and it is safe to free after on_done is called back.

// Note on synchronization: \a as on_done might be called from another thread
//~immediately, access to the grpc_ares_request* return value must be
// synchronized by the caller. TODO(apolcyn): we should remove this requirement
// by changing this API to use two phase initialization - one API to create
// the grpc_ares_request* and another to start the async work.
extern grpc_ares_request* (*grpc_dns_lookup_hostname_ares)(
    const char* dns_server, const char* name, const char* default_port,
    grpc_pollset_set* interested_parties, grpc_closure* on_done,
    std::unique_ptr<grpc_core::EndpointAddressesList>* addresses,
    int query_timeout_ms);

// Asynchronously resolve a SRV record.
// See \a grpc_dns_lookup_hostname_ares for usage details and caveats.
extern grpc_ares_request* (*grpc_dns_lookup_srv_ares)(
    const char* dns_server, const char* name,
    grpc_pollset_set* interested_parties, grpc_closure* on_done,
    std::unique_ptr<grpc_core::EndpointAddressesList>* balancer_addresses,
    int query_timeout_ms);

// Asynchronously resolve a TXT record.
// See \a grpc_dns_lookup_hostname_ares for usage details and caveats.
extern grpc_ares_request* (*grpc_dns_lookup_txt_ares)(
    const char* dns_server, const char* name,
    grpc_pollset_set* interested_parties, grpc_closure* on_done,
    char** service_config_json, int query_timeout_ms);

// Cancel the pending grpc_ares_request \a request
extern void (*grpc_cancel_ares_request)(grpc_ares_request* request);

// Initialize gRPC ares wrapper. Must be called at least once before
// grpc_resolve_address_ares().
grpc_error_handle grpc_ares_init(void);

// Uninitialized gRPC ares wrapper. If there was more than one previous call to
// grpc_ares_init(), this function uninitializes the gRPC ares wrapper only if
// it has been called the same number of times as grpc_ares_init().
void grpc_ares_cleanup(void);

// Indicates whether or not AAAA queries should be attempted.
// E.g., return false if ipv6 is known to not be available.
bool grpc_ares_query_ipv6();

// Sorts destinations in lb_addrs according to RFC 6724.
void grpc_cares_wrapper_address_sorting_sort(
    const grpc_ares_request* request,
    grpc_core::EndpointAddressesList* addresses);

// Exposed in this header for C-core tests only
extern void (*grpc_ares_test_only_inject_config)(ares_channel* channel);

// Exposed in this header for C-core tests only
extern bool g_grpc_ares_test_only_force_tcp;

#endif

#endif  // GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_WRAPPER_H
