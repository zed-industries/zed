// Copyright 2023 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_DNS_SERVICE_RESOLVER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_DNS_SERVICE_RESOLVER_H
#include <grpc/support/port_platform.h>

#ifdef GPR_APPLE
#include <AvailabilityMacros.h>
#ifdef AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER

#include <CoreFoundation/CoreFoundation.h>
#include <dns_sd.h>
#include <grpc/event_engine/event_engine.h>

#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "src/core/lib/event_engine/cf_engine/cf_engine.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_event_engine::experimental {

class DNSServiceResolverImpl
    : public grpc_core::RefCounted<DNSServiceResolverImpl> {
  struct DNSServiceRequest {
    EventEngine::DNSResolver::LookupHostnameCallback on_resolve;
    uint16_t port;
    std::vector<EventEngine::ResolvedAddress> result;
    bool has_ipv4_response = false;
    bool has_ipv6_response = false;
  };

 public:
  explicit DNSServiceResolverImpl(std::shared_ptr<CFEventEngine> engine)
      : engine_(std::move((engine))) {}
  ~DNSServiceResolverImpl() override {
    CHECK(requests_.empty());
    dispatch_release(queue_);
  }

  void Shutdown();

  void LookupHostname(
      EventEngine::DNSResolver::LookupHostnameCallback on_resolve,
      absl::string_view name, absl::string_view default_port);

 private:
  static void ResolveCallback(DNSServiceRef sdRef, DNSServiceFlags flags,
                              uint32_t interfaceIndex,
                              DNSServiceErrorType errorCode,
                              const char* hostname,
                              const struct sockaddr* address, uint32_t ttl,
                              void* context);

 private:
  std::shared_ptr<CFEventEngine> engine_;
  // DNSServiceSetDispatchQueue requires a serial dispatch queue
  dispatch_queue_t queue_ =
      dispatch_queue_create("dns_service_resolver", nullptr);
  grpc_core::Mutex request_mu_;
  absl::flat_hash_map<DNSServiceRef, DNSServiceRequest> requests_
      ABSL_GUARDED_BY(request_mu_);
};

class DNSServiceResolver : public EventEngine::DNSResolver {
 public:
  explicit DNSServiceResolver(std::shared_ptr<CFEventEngine> engine)
      : engine_(std::move(engine)),
        impl_(grpc_core::MakeRefCounted<DNSServiceResolverImpl>(
            std::move((engine_)))) {}

  ~DNSServiceResolver() override { impl_->Shutdown(); }

  void LookupHostname(
      EventEngine::DNSResolver::LookupHostnameCallback on_resolve,
      absl::string_view name, absl::string_view default_port) override {
    impl_->LookupHostname(std::move(on_resolve), name, default_port);
  };

  void LookupSRV(EventEngine::DNSResolver::LookupSRVCallback on_resolve,
                 absl::string_view /* name */) override {
    engine_->Run([on_resolve = std::move(on_resolve)]() mutable {
      on_resolve(absl::UnimplementedError(
          "The DNS Service resolver does not support looking up SRV records"));
    });
  }

  void LookupTXT(EventEngine::DNSResolver::LookupTXTCallback on_resolve,
                 absl::string_view /* name */) override {
    engine_->Run([on_resolve = std::move(on_resolve)]() mutable {
      on_resolve(absl::UnimplementedError(
          "The DNS Service resolver does not support looking up TXT records"));
    });
  }

 private:
  std::shared_ptr<CFEventEngine> engine_;
  grpc_core::RefCountedPtr<DNSServiceResolverImpl> impl_;
};

}  // namespace grpc_event_engine::experimental

#endif  // AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER
#endif  // GPR_APPLE

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_DNS_SERVICE_RESOLVER_H
