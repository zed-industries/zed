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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_INTERNAL_H
#define GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_INTERNAL_H

#include <grpc/impl/connectivity_state.h>
#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/client_channel/subchannel.h"
#include "src/core/client_channel/subchannel_interface_internal.h"
#include "src/core/client_channel/subchannel_stream_client.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/load_balancing/subchannel_interface.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/work_serializer.h"

namespace grpc_core {

class HealthWatcher;

// This producer is registered with a subchannel.  It creates a streaming
// health watch call for each health check service name that is being
// watched and reports the resulting connectivity state to all
// registered watchers.
class HealthProducer final : public Subchannel::DataProducerInterface {
 public:
  HealthProducer() : interested_parties_(grpc_pollset_set_create()) {}
  ~HealthProducer() override { grpc_pollset_set_destroy(interested_parties_); }

  void Start(RefCountedPtr<Subchannel> subchannel);

  static UniqueTypeName Type() {
    static UniqueTypeName::Factory kFactory("health_check");
    return kFactory.Create();
  }

  UniqueTypeName type() const override { return Type(); }

  void AddWatcher(HealthWatcher* watcher,
                  const std::optional<std::string>& health_check_service_name);
  void RemoveWatcher(
      HealthWatcher* watcher,
      const std::optional<std::string>& health_check_service_name);

 private:
  class ConnectivityWatcher;

  // Health checker for a given health check service name.  Contains the
  // health check client and the list of watchers.
  class HealthChecker final : public InternallyRefCounted<HealthChecker> {
   public:
    HealthChecker(WeakRefCountedPtr<HealthProducer> producer,
                  absl::string_view health_check_service_name);

    // Disable thread-safety analysis because this method is called via
    // OrphanablePtr<>, but there's no way to pass the lock annotation
    // through there.
    void Orphan() override ABSL_NO_THREAD_SAFETY_ANALYSIS;

    void AddWatcherLocked(HealthWatcher* watcher)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&HealthProducer::mu_);

    // Returns true if this was the last watcher.
    bool RemoveWatcherLocked(HealthWatcher* watcher)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&HealthProducer::mu_);

    // Called when the subchannel's connectivity state changes.
    void OnConnectivityStateChangeLocked(grpc_connectivity_state state,
                                         const absl::Status& status)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&HealthProducer::mu_);

   private:
    class HealthStreamEventHandler;

    // Starts a new stream if we have a connected subchannel.
    // Called whenever the subchannel transitions to state READY or when a
    // watcher is added.
    void StartHealthStreamLocked()
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&HealthProducer::mu_);

    // Notifies watchers of a new state.
    // Called while holding the SubchannelStreamClient lock and possibly
    // the producer lock, so must notify asynchronously, but in guaranteed
    // order (hence the use of WorkSerializer).
    void NotifyWatchersLocked(grpc_connectivity_state state,
                              absl::Status status)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&HealthProducer::mu_);

    // Called by the health check client when receiving an update.
    void OnHealthWatchStatusChange(grpc_connectivity_state state,
                                   const absl::Status& status);

    WeakRefCountedPtr<HealthProducer> producer_;
    absl::string_view health_check_service_name_;
    std::shared_ptr<WorkSerializer> work_serializer_ =
        std::make_shared<WorkSerializer>(
            producer_->subchannel_->event_engine());

    std::optional<grpc_connectivity_state> state_
        ABSL_GUARDED_BY(&HealthProducer::mu_);
    absl::Status status_ ABSL_GUARDED_BY(&HealthProducer::mu_);
    OrphanablePtr<SubchannelStreamClient> stream_client_
        ABSL_GUARDED_BY(&HealthProducer::mu_);
    std::set<HealthWatcher*> watchers_ ABSL_GUARDED_BY(&HealthProducer::mu_);
  };

  // Handles a connectivity state change on the subchannel.
  void OnConnectivityStateChange(grpc_connectivity_state state,
                                 const absl::Status& status);
  void Orphaned() override;

  RefCountedPtr<Subchannel> subchannel_;
  ConnectivityWatcher* connectivity_watcher_;
  grpc_pollset_set* interested_parties_;

  Mutex mu_;
  std::optional<grpc_connectivity_state> state_ ABSL_GUARDED_BY(&mu_);
  absl::Status status_ ABSL_GUARDED_BY(&mu_);
  RefCountedPtr<ConnectedSubchannel> connected_subchannel_
      ABSL_GUARDED_BY(&mu_);
  std::map<std::string /*health_check_service_name*/,
           OrphanablePtr<HealthChecker>>
      health_checkers_ ABSL_GUARDED_BY(&mu_);
  std::set<HealthWatcher*> non_health_watchers_ ABSL_GUARDED_BY(&mu_);
};

// A data watcher that handles health checking.
class HealthWatcher final : public InternalSubchannelDataWatcherInterface {
 public:
  HealthWatcher(
      std::shared_ptr<WorkSerializer> work_serializer,
      std::optional<std::string> health_check_service_name,
      std::unique_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
          watcher)
      : work_serializer_(std::move(work_serializer)),
        health_check_service_name_(std::move(health_check_service_name)),
        watcher_(std::move(watcher)) {}
  ~HealthWatcher() override;

  UniqueTypeName type() const override { return HealthProducer::Type(); }

  // When the client channel sees this wrapper, it will pass it the real
  // subchannel to use.
  void SetSubchannel(Subchannel* subchannel) override;

  // For intercepting the watcher before it gets up to the real subchannel.
  std::shared_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
  TakeWatcher() {
    return std::move(watcher_);
  }
  void SetWatcher(
      std::shared_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
          watcher) {
    watcher_ = std::move(watcher);
  }

  void Notify(grpc_connectivity_state state, absl::Status status);

  grpc_pollset_set* interested_parties() const {
    return watcher_->interested_parties();
  }

 private:
  std::shared_ptr<WorkSerializer> work_serializer_;
  std::optional<std::string> health_check_service_name_;
  std::shared_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
      watcher_;
  RefCountedPtr<HealthProducer> producer_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_HEALTH_CHECK_CLIENT_INTERNAL_H
