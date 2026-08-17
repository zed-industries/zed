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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_CLIENT_GRPC_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_CLIENT_GRPC_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/useful.h"
#include "src/core/xds/grpc/certificate_provider_store.h"
#include "src/core/xds/grpc/xds_bootstrap_grpc.h"
#include "src/core/xds/xds_client/lrs_client.h"
#include "src/core/xds/xds_client/xds_client.h"
#include "src/core/xds/xds_client/xds_transport.h"

namespace grpc_core {

class GrpcXdsClient final : public XdsClient {
 public:
  // The key to pass to GetOrCreate() for gRPC servers.
  static constexpr absl::string_view kServerKey = "#server";

  // Factory function to get or create the global XdsClient instance.
  static absl::StatusOr<RefCountedPtr<GrpcXdsClient>> GetOrCreate(
      absl::string_view key, const ChannelArgs& args, const char* reason);

  // Do not instantiate directly -- use GetOrCreate() instead.
  // TODO(roth): The transport factory is injectable here to support
  // tests that want to use a fake transport factory with code that
  // expects a GrpcXdsClient instead of an XdsClient, typically because
  // it needs to call the interested_parties() method.  Once we
  // finish the EventEngine migration and remove the interested_parties()
  // method, consider instead changing callers to an approach where the
  // production code uses XdsClient instead of GrpcXdsClient, and then
  // passing in a fake XdsClient impl in the tests.  Note that this will
  // work for callers that use interested_parties() but not for callers
  // that also use certificate_provider_store(), but we should consider
  // alternatives for that case as well.
  GrpcXdsClient(absl::string_view key,
                std::shared_ptr<GrpcXdsBootstrap> bootstrap,
                const ChannelArgs& args,
                RefCountedPtr<XdsTransportFactory> transport_factory,
                std::shared_ptr<GlobalStatsPluginRegistry::StatsPluginGroup>
                    stats_plugin_group);

  // Helpers for encoding the XdsClient object in channel args.
  static absl::string_view ChannelArgName() {
    return GRPC_ARG_NO_SUBCHANNEL_PREFIX "xds_client";
  }
  static int ChannelArgsCompare(const XdsClient* a, const XdsClient* b) {
    return QsortCompare(a, b);
  }

  void ResetBackoff() override;

  grpc_pollset_set* interested_parties() const;

  CertificateProviderStore& certificate_provider_store() const {
    return *certificate_provider_store_;
  }

  absl::string_view key() const { return key_; }

  LrsClient& lrs_client() { return *lrs_client_; }

  // Builds ClientStatusResponse containing all resources from all XdsClients
  static grpc_slice DumpAllClientConfigs();

 private:
  class MetricsReporter;

  void ReportCallbackMetrics(CallbackMetricReporter& reporter);
  void Orphaned() override;

  std::string key_;
  OrphanablePtr<CertificateProviderStore> certificate_provider_store_;
  std::shared_ptr<GlobalStatsPluginRegistry::StatsPluginGroup>
      stats_plugin_group_;
  std::unique_ptr<RegisteredMetricCallback> registered_metric_callback_;
  RefCountedPtr<LrsClient> lrs_client_;
};

namespace internal {
void SetXdsChannelArgsForTest(grpc_channel_args* args);
void UnsetGlobalXdsClientsForTest();
// Sets bootstrap config to be used when no env var is set.
// Does not take ownership of config.
void SetXdsFallbackBootstrapConfig(const char* config);
}  // namespace internal

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_CLIENT_GRPC_H
