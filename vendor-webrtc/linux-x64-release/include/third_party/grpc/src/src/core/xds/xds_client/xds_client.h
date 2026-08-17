//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CLIENT_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CLIENT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_set.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "envoy/admin/v3/config_dump_shared.upb.h"
#include "envoy/service/status/v3/csds.upb.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/uri.h"
#include "src/core/util/work_serializer.h"
#include "src/core/xds/xds_client/xds_api.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"
#include "src/core/xds/xds_client/xds_locality.h"
#include "src/core/xds/xds_client/xds_metrics.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "src/core/xds/xds_client/xds_transport.h"
#include "upb/reflection/def.hpp"

namespace grpc_core {

namespace testing {
class XdsClientTestPeer;
}

class XdsClient : public DualRefCounted<XdsClient> {
 public:
  // The authority reported for old-style (non-xdstp) resource names.
  static constexpr absl::string_view kOldStyleAuthority = "#old";

  class ReadDelayHandle : public RefCounted<ReadDelayHandle> {
   public:
    static RefCountedPtr<ReadDelayHandle> NoWait() { return nullptr; }
  };

  // Resource watcher interface.  Implemented by callers.
  // Note: Most callers will not use this API directly but rather via a
  // resource-type-specific wrapper API provided by the relevant
  // XdsResourceType implementation.
  class ResourceWatcherInterface : public RefCounted<ResourceWatcherInterface> {
   public:
    virtual void OnGenericResourceChanged(
        absl::StatusOr<std::shared_ptr<const XdsResourceType::ResourceData>>
            resource,
        RefCountedPtr<ReadDelayHandle> read_delay_handle)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&work_serializer_) = 0;
    virtual void OnAmbientError(
        absl::Status status, RefCountedPtr<ReadDelayHandle> read_delay_handle)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&work_serializer_) = 0;
  };

  XdsClient(
      std::shared_ptr<XdsBootstrap> bootstrap,
      RefCountedPtr<XdsTransportFactory> transport_factory,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine,
      std::unique_ptr<XdsMetricsReporter> metrics_reporter,
      std::string user_agent_name, std::string user_agent_version,
      // This parameter overrides the timer duration for testing
      // purposes only -- do not use in production.
      Duration resource_request_timeout = Duration::Zero());
  ~XdsClient() override;

  // Start and cancel watch for a resource.
  //
  // The XdsClient takes ownership of the watcher, but the caller may
  // keep a raw pointer to the watcher, which may be used only for
  // cancellation.  (Because the caller does not own the watcher, the
  // pointer must not be used for any other purpose.)
  // If the caller is going to start a new watch after cancelling the
  // old one, it should set delay_unsubscription to true.
  //
  // The resource type object must be a global singleton, since the first
  // time the XdsClient sees a particular resource type object, it will
  // store the pointer to that object as the authoritative implementation for
  // its type URLs.  The resource type object must outlive the XdsClient object,
  // and it is illegal to start a subsequent watch for the same type URLs using
  // a different resource type object.
  //
  // Note: Most callers will not use this API directly but rather via a
  // resource-type-specific wrapper API provided by the relevant
  // XdsResourceType implementation.
  void WatchResource(const XdsResourceType* type, absl::string_view name,
                     RefCountedPtr<ResourceWatcherInterface> watcher);
  void CancelResourceWatch(const XdsResourceType* type,
                           absl::string_view listener_name,
                           ResourceWatcherInterface* watcher,
                           bool delay_unsubscription = false);

  // Resets connection backoff state.
  virtual void ResetBackoff();

  const XdsBootstrap& bootstrap() const {
    return *bootstrap_;  // ctor asserts that it is non-null
  }

  XdsTransportFactory* transport_factory() const {
    return transport_factory_.get();
  }

  grpc_event_engine::experimental::EventEngine* engine() {
    return engine_.get();
  }

 protected:
  void Orphaned() override;

  Mutex* mu() ABSL_LOCK_RETURNED(&mu_) { return &mu_; }

  // Dumps the active xDS config to the provided
  // envoy.service.status.v3.ClientConfig message.
  void DumpClientConfig(std::set<std::string>* string_pool, upb_Arena* arena,
                        envoy_service_status_v3_ClientConfig* client_config)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Invokes func once for each combination of labels to report the
  // resource count for those labels.
  struct ResourceCountLabels {
    absl::string_view xds_authority;
    absl::string_view resource_type;
    absl::string_view cache_state;
  };
  void ReportResourceCounts(
      absl::FunctionRef<void(const ResourceCountLabels&, uint64_t)> func)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Invokes func once for each xDS server to report whether the
  // connection to that server is working.
  void ReportServerConnections(
      absl::FunctionRef<void(absl::string_view /*xds_server*/, bool)> func)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

 private:
  friend testing::XdsClientTestPeer;

  struct XdsResourceKey {
    std::string id;
    std::vector<URI::QueryParam> query_params;

    bool operator<(const XdsResourceKey& other) const {
      int c = id.compare(other.id);
      if (c != 0) return c < 0;
      return query_params < other.query_params;
    }
  };

  struct AuthorityState;

  struct XdsResourceName {
    std::string authority;
    XdsResourceKey key;
  };

  // Contains a channel to the xds server and all the data related to the
  // channel.  Holds a ref to the xds client object.
  class XdsChannel final : public DualRefCounted<XdsChannel> {
   public:
    template <typename T>
    class RetryableCall;

    class AdsCall;

    XdsChannel(WeakRefCountedPtr<XdsClient> xds_client,
               const XdsBootstrap::XdsServer& server);
    ~XdsChannel() override;

    XdsClient* xds_client() const { return xds_client_.get(); }

    void ResetBackoff();

    // Returns non-OK if there has been an error since the last time the
    // ADS stream saw a response.
    const absl::Status& status() const { return status_; }

    void SubscribeLocked(const XdsResourceType* type,
                         const XdsResourceName& name)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&XdsClient::mu_);
    void UnsubscribeLocked(const XdsResourceType* type,
                           const XdsResourceName& name,
                           bool delay_unsubscription)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&XdsClient::mu_);

    absl::string_view server_uri() const {
      return server_.target()->server_uri();
    }

   private:
    class ConnectivityFailureWatcher;

    // Attempts to find a suitable Xds fallback server. Returns true if
    // a connection to a suitable server had been established.
    bool MaybeFallbackLocked(const std::string& authority,
                             XdsClient::AuthorityState& authority_state)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&XdsClient::mu_);
    void SetHealthyLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&XdsClient::mu_);
    void Orphaned() override;

    void OnConnectivityFailure(absl::Status status);

    // Enqueues error notifications to watchers.  Caller must drain
    // XdsClient::work_serializer_ after releasing the lock.
    void SetChannelStatusLocked(absl::Status status)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&XdsClient::mu_);

    // The owning xds client.
    WeakRefCountedPtr<XdsClient> xds_client_;

    const XdsBootstrap::XdsServer& server_;  // Owned by bootstrap.

    RefCountedPtr<XdsTransportFactory::XdsTransport> transport_;
    RefCountedPtr<XdsTransportFactory::XdsTransport::ConnectivityFailureWatcher>
        failure_watcher_;

    bool shutting_down_ = false;

    // The retryable ADS and LRS calls.
    OrphanablePtr<RetryableCall<AdsCall>> ads_call_;

    // Stores the most recent accepted resource version for each resource type.
    std::map<const XdsResourceType*, std::string /*version*/>
        resource_type_version_map_;

    absl::Status status_;
  };

  using WatcherSet =
      absl::flat_hash_set<RefCountedPtr<ResourceWatcherInterface>,
                          RefCountedPtrHash<ResourceWatcherInterface>,
                          RefCountedPtrEq<ResourceWatcherInterface>>;

  class ResourceState {
   public:
    // Resource status from the view of a xDS client, which tells the
    // synchronization status between the xDS client and the xDS server.
    enum ClientResourceStatus {
      // Client requested this resource but hasn't received any update from
      // management server. The client will not fail requests, but will queue
      // them until update arrives or the client times out waiting for the
      // resource.
      REQUESTED = 1,
      // This resource has been requested by the client but has either not been
      // delivered by the server or was previously delivered by the server and
      // then subsequently removed from resources provided by the server.
      DOES_NOT_EXIST,
      // Client received this resource and replied with ACK.
      ACKED,
      // Client received this resource and replied with NACK.
      NACKED,
      // Server sent an error for the resource.
      RECEIVED_ERROR,
      // Client encountered timeout getting resource from server.
      TIMEOUT,
    };
    static_assert(static_cast<ClientResourceStatus>(envoy_admin_v3_REQUESTED) ==
                  ClientResourceStatus::REQUESTED);
    static_assert(
        static_cast<ClientResourceStatus>(envoy_admin_v3_DOES_NOT_EXIST) ==
        ClientResourceStatus::DOES_NOT_EXIST);
    static_assert(static_cast<ClientResourceStatus>(envoy_admin_v3_ACKED) ==
                  ClientResourceStatus::ACKED);
    static_assert(static_cast<ClientResourceStatus>(envoy_admin_v3_NACKED) ==
                  ClientResourceStatus::NACKED);
    static_assert(
        static_cast<ClientResourceStatus>(envoy_admin_v3_RECEIVED_ERROR) ==
        ClientResourceStatus::RECEIVED_ERROR);
    static_assert(static_cast<ClientResourceStatus>(envoy_admin_v3_TIMEOUT) ==
                  ClientResourceStatus::TIMEOUT);

    void AddWatcher(RefCountedPtr<ResourceWatcherInterface> watcher) {
      watchers_.insert(std::move(watcher));
    }
    void RemoveWatcher(ResourceWatcherInterface* watcher) {
      watchers_.erase(watcher);
    }
    bool HasWatchers() const { return !watchers_.empty(); }
    const WatcherSet& watchers() const { return watchers_; }

    void SetAcked(std::shared_ptr<const XdsResourceType::ResourceData> resource,
                  std::string serialized_proto, std::string version,
                  Timestamp update_time);
    void SetNacked(const std::string& version, absl::string_view details,
                   Timestamp update_time, bool drop_cached_resource);
    void SetReceivedError(const std::string& version, absl::Status status,
                          Timestamp update_time, bool drop_cached_resource);
    void SetDoesNotExistOnLdsOrCdsDeletion(const std::string& version,
                                           Timestamp update_time,
                                           bool drop_cached_resource);
    void SetDoesNotExistOnTimeout();
    void SetTimeout(const std::string& details);

    ClientResourceStatus client_status() const { return client_status_; }
    absl::string_view CacheStateString() const;

    bool HasResource() const { return resource_ != nullptr; }
    std::shared_ptr<const XdsResourceType::ResourceData> resource() const {
      return resource_;
    }

    const absl::Status& failed_status() const { return failed_status_; }

    void FillGenericXdsConfig(
        upb_StringView type_url, upb_StringView resource_name, upb_Arena* arena,
        envoy_service_status_v3_ClientConfig_GenericXdsConfig* entry) const;

   private:
    WatcherSet watchers_;
    // The latest data seen for the resource.
    std::shared_ptr<const XdsResourceType::ResourceData> resource_;
    // Cache state.
    ClientResourceStatus client_status_ = REQUESTED;
    // The serialized bytes of the last successfully updated raw xDS resource.
    std::string serialized_proto_;
    // The timestamp when the resource was last successfully updated.
    Timestamp update_time_;
    // The last successfully updated version of the resource.
    std::string version_;
    // Details about the last failed update attempt or transient error.
    absl::Status failed_status_;
    // The rejected version string of the last failed update attempt.
    std::string failed_version_;
    // Timestamp of the last failed update attempt.
    // Used only if failed_version_ is non-empty.
    Timestamp failed_update_time_;
  };

  struct AuthorityState {
    std::vector<RefCountedPtr<XdsChannel>> xds_channels;
    std::map<const XdsResourceType*, std::map<XdsResourceKey, ResourceState>>
        type_map;
  };

  absl::Status AppendNodeToStatus(const absl::Status& status) const;

  // Sends an OnResourceChanged() notification to a specific set of watchers.
  void NotifyWatchersOnResourceChanged(
      absl::StatusOr<std::shared_ptr<const XdsResourceType::ResourceData>>
          resource,
      WatcherSet watchers, RefCountedPtr<ReadDelayHandle> read_delay_handle);
  // Sends an OnAmbientError() notification to a specific set of watchers.
  void NotifyWatchersOnAmbientError(
      absl::Status status, WatcherSet watchers,
      RefCountedPtr<ReadDelayHandle> read_delay_handle);
  // Notifies watchers for resource_state of an error, using
  // OnResourceChanged() if there is no cached resource or
  // OnAmbientError() if there is a cached resource.
  void NotifyWatchersOnError(const ResourceState& resource_state,
                             RefCountedPtr<ReadDelayHandle> read_delay_handle,
                             // If empty, will use resource_state.watchers().
                             WatcherSet watchers = {},
                             // If OK, will use resource_state.failed_status().
                             absl::Status status = absl::OkStatus());

  void MaybeRegisterResourceTypeLocked(const XdsResourceType* resource_type)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  // Gets the type for resource_type, or null if the type is unknown.
  const XdsResourceType* GetResourceTypeLocked(absl::string_view resource_type)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  bool HasUncachedResources(const AuthorityState& authority_state);

  void MaybeRemoveUnsubscribedCacheEntriesForTypeLocked(
      XdsChannel* xds_channel, const XdsResourceType* type)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  absl::StatusOr<XdsResourceName> ParseXdsResourceName(
      absl::string_view name, const XdsResourceType* type);
  static std::string ConstructFullXdsResourceName(
      absl::string_view authority, absl::string_view resource_type,
      const XdsResourceKey& key);

  RefCountedPtr<XdsChannel> GetOrCreateXdsChannelLocked(
      const XdsBootstrap::XdsServer& server, const char* reason)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  std::shared_ptr<XdsBootstrap> bootstrap_;
  const std::string user_agent_name_;
  const std::string user_agent_version_;
  RefCountedPtr<XdsTransportFactory> transport_factory_;
  const Duration request_timeout_;
  const bool xds_federation_enabled_;
  WorkSerializer work_serializer_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine_;
  std::unique_ptr<XdsMetricsReporter> metrics_reporter_;

  Mutex mu_;

  // Stores resource type objects seen by type URL.
  std::map<absl::string_view /*resource_type*/, const XdsResourceType*>
      resource_types_ ABSL_GUARDED_BY(mu_);
  upb::DefPool def_pool_ ABSL_GUARDED_BY(mu_);

  // Map of existing xDS server channels.
  std::map<std::string /*XdsServer key*/, XdsChannel*> xds_channel_map_
      ABSL_GUARDED_BY(mu_);

  std::map<std::string /*authority*/, AuthorityState> authority_state_map_
      ABSL_GUARDED_BY(mu_);

  // Stores started watchers whose resource name was not parsed successfully,
  // waiting to be cancelled or reset in Orphan().
  WatcherSet invalid_watchers_ ABSL_GUARDED_BY(mu_);

  bool shutting_down_ ABSL_GUARDED_BY(mu_) = false;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CLIENT_H
