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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_SUBCHANNEL_INTERFACE_H
#define GRPC_SRC_CORE_LOAD_BALANCING_SUBCHANNEL_INTERFACE_H

#include <grpc/impl/connectivity_state.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// The interface for subchannels that is exposed to LB policy implementations.
class SubchannelInterface : public DualRefCounted<SubchannelInterface> {
 public:
  class ConnectivityStateWatcherInterface {
   public:
    virtual ~ConnectivityStateWatcherInterface() = default;

    // Will be invoked whenever the subchannel's connectivity state changes.
    // If the new state is TRANSIENT_FAILURE, status indicates the reason
    // for the failure.  There will be only one invocation of this method
    // on a given watcher instance at any given time.
    virtual void OnConnectivityStateChange(grpc_connectivity_state new_state,
                                           absl::Status status) = 0;

    // TODO(roth): Remove this as soon as we move to EventManager-based
    // polling.
    virtual grpc_pollset_set* interested_parties() = 0;
  };

  // Opaque interface for watching data of a particular type for this
  // subchannel.
  class DataWatcherInterface {
   public:
    virtual ~DataWatcherInterface() = default;
  };

  explicit SubchannelInterface(const char* trace = nullptr)
      : DualRefCounted<SubchannelInterface>(trace) {}

  ~SubchannelInterface() override = default;

  // Starts watching the subchannel's connectivity state.
  // The first callback to the watcher will be delivered ~immediately.
  // Subsequent callbacks will be delivered as the subchannel's state
  // changes.
  // The watcher will be destroyed either when the subchannel is
  // destroyed or when CancelConnectivityStateWatch() is called.
  // There can be only one watcher of a given subchannel.  It is not
  // valid to call this method a second time without first cancelling
  // the previous watcher using CancelConnectivityStateWatch().
  virtual void WatchConnectivityState(
      std::unique_ptr<ConnectivityStateWatcherInterface> watcher) = 0;

  // Cancels a connectivity state watch.
  // If the watcher has already been destroyed, this is a no-op.
  // TODO(roth): This interface has an ABA issue.  Fix this before we
  // make this API public.
  virtual void CancelConnectivityStateWatch(
      ConnectivityStateWatcherInterface* watcher) = 0;

  // Attempt to connect to the backend.  Has no effect if already connected.
  // If the subchannel is currently in backoff delay due to a previously
  // failed attempt, the new connection attempt will not start until the
  // backoff delay has elapsed.
  virtual void RequestConnection() = 0;

  // Resets the subchannel's connection backoff state.  If RequestConnection()
  // has been called since the subchannel entered TRANSIENT_FAILURE state,
  // starts a new connection attempt immediately; otherwise, a new connection
  // attempt will be started as soon as RequestConnection() is called.
  virtual void ResetBackoff() = 0;

  // Registers a new data watcher.
  virtual void AddDataWatcher(
      std::unique_ptr<DataWatcherInterface> watcher) = 0;

  // Cancels a data watch.
  // TODO(roth): This interface has an ABA issue.  Fix this before we
  // make this API public.
  virtual void CancelDataWatcher(DataWatcherInterface* watcher) = 0;

  // Return the address in URI format.
  virtual std::string address() const = 0;

 protected:
  void Orphaned() override {}
};

// A class that delegates to another subchannel, to be used in cases
// where an LB policy needs to wrap a subchannel.
class DelegatingSubchannel : public SubchannelInterface {
 public:
  explicit DelegatingSubchannel(RefCountedPtr<SubchannelInterface> subchannel)
      : wrapped_subchannel_(std::move(subchannel)) {}

  RefCountedPtr<SubchannelInterface> wrapped_subchannel() const {
    return wrapped_subchannel_;
  }

  void WatchConnectivityState(
      std::unique_ptr<ConnectivityStateWatcherInterface> watcher) override {
    return wrapped_subchannel_->WatchConnectivityState(std::move(watcher));
  }
  void CancelConnectivityStateWatch(
      ConnectivityStateWatcherInterface* watcher) override {
    return wrapped_subchannel_->CancelConnectivityStateWatch(watcher);
  }
  void RequestConnection() override {
    wrapped_subchannel_->RequestConnection();
  }
  void ResetBackoff() override { wrapped_subchannel_->ResetBackoff(); }
  void AddDataWatcher(std::unique_ptr<DataWatcherInterface> watcher) override {
    wrapped_subchannel_->AddDataWatcher(std::move(watcher));
  }
  void CancelDataWatcher(DataWatcherInterface* watcher) override {
    wrapped_subchannel_->CancelDataWatcher(watcher);
  }

  std::string address() const override {
    return wrapped_subchannel_->address();
  }

 private:
  RefCountedPtr<SubchannelInterface> wrapped_subchannel_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_SUBCHANNEL_INTERFACE_H
