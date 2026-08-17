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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_OOB_BACKEND_METRIC_INTERNAL_H
#define GRPC_SRC_CORE_LOAD_BALANCING_OOB_BACKEND_METRIC_INTERNAL_H

#include <grpc/impl/connectivity_state.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <set>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "src/core/client_channel/subchannel.h"
#include "src/core/client_channel/subchannel_interface_internal.h"
#include "src/core/client_channel/subchannel_stream_client.h"
#include "src/core/load_balancing/backend_metric_data.h"
#include "src/core/load_balancing/oob_backend_metric.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/unique_type_name.h"

namespace grpc_core {

class OrcaWatcher;

// This producer is registered with a subchannel.  It creates a
// streaming ORCA call and reports the resulting backend metrics to all
// registered watchers.
class OrcaProducer final : public Subchannel::DataProducerInterface {
 public:
  void Start(RefCountedPtr<Subchannel> subchannel);

  static UniqueTypeName Type() {
    static UniqueTypeName::Factory kFactory("orca");
    return kFactory.Create();
  }

  UniqueTypeName type() const override { return Type(); }

  // Adds and removes watchers.
  void AddWatcher(OrcaWatcher* watcher);
  void RemoveWatcher(OrcaWatcher* watcher);

 private:
  class ConnectivityWatcher;
  class OrcaStreamEventHandler;

  void Orphaned() override;

  // Returns the minimum requested reporting interval across all watchers.
  Duration GetMinIntervalLocked() const ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Starts a new stream if we have a connected subchannel.
  // Called whenever the reporting interval changes or the subchannel
  // transitions to state READY.
  void MaybeStartStreamLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Handles a connectivity state change on the subchannel.
  void OnConnectivityStateChange(grpc_connectivity_state state);

  // Called to notify watchers of a new backend metric report.
  void NotifyWatchers(const BackendMetricData& backend_metric_data);

  RefCountedPtr<Subchannel> subchannel_;
  RefCountedPtr<ConnectedSubchannel> connected_subchannel_;
  ConnectivityWatcher* connectivity_watcher_;
  Mutex mu_;
  std::set<OrcaWatcher*> watchers_ ABSL_GUARDED_BY(mu_);
  Duration report_interval_ ABSL_GUARDED_BY(mu_) = Duration::Infinity();
  OrphanablePtr<SubchannelStreamClient> stream_client_ ABSL_GUARDED_BY(mu_);
};

// This watcher is returned to the LB policy and added to the
// client channel SubchannelWrapper.
class OrcaWatcher final : public InternalSubchannelDataWatcherInterface {
 public:
  OrcaWatcher(Duration report_interval,
              std::unique_ptr<OobBackendMetricWatcher> watcher)
      : report_interval_(report_interval), watcher_(std::move(watcher)) {}
  ~OrcaWatcher() override;

  Duration report_interval() const { return report_interval_; }
  OobBackendMetricWatcher* watcher() const { return watcher_.get(); }

  UniqueTypeName type() const override { return OrcaProducer::Type(); }

  // When the client channel sees this wrapper, it will pass it the real
  // subchannel to use.
  void SetSubchannel(Subchannel* subchannel) override;

 private:
  const Duration report_interval_;
  std::unique_ptr<OobBackendMetricWatcher> watcher_;
  RefCountedPtr<OrcaProducer> producer_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_OOB_BACKEND_METRIC_INTERNAL_H
