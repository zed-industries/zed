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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_H
#define GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/slice.h>
#include <grpc/grpc.h>
#include <grpc/impl/connectivity_state.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>

#include "absl/base/thread_annotations.h"
#include "absl/container/inlined_vector.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/load_balancing/backend_metric_data.h"
#include "src/core/load_balancing/subchannel_interface.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/work_serializer.h"

namespace grpc_core {

/// Interface for load balancing policies.
///
/// The following concepts are used here:
///
/// Channel: An abstraction that manages connections to backend servers
///   on behalf of a client application.  The application creates a channel
///   for a given server name and then sends calls (RPCs) on it, and the
///   channel figures out which backend server to send each call to.  A channel
///   contains a resolver, a load balancing policy (or a tree of LB policies),
///   and a set of one or more subchannels.
///
/// Subchannel: A subchannel represents a connection to one backend server.
///   The LB policy decides which subchannels to create, manages the
///   connectivity state of those subchannels, and decides which subchannel
///   to send any given call to.
///
/// Resolver: A plugin that takes a gRPC server URI and resolves it to a
///   list of one or more addresses and a service config, as described
///   in https://github.com/grpc/grpc/blob/master/doc/naming.md.  See
///   resolver.h for the resolver API.
///
/// Load Balancing (LB) Policy: A plugin that takes a list of addresses
///   from the resolver, maintains and manages a subchannel for each
///   backend address, and decides which subchannel to send each call on.
///   An LB policy has two parts:
///   - A LoadBalancingPolicy, which deals with the control plane work of
///     managing subchannels.
///   - A SubchannelPicker, which handles the data plane work of
///     determining which subchannel a given call should be sent on.

/// LoadBalancingPolicy API.
///
/// Note: All methods with a "Locked" suffix must be called from the
/// work_serializer passed to the constructor.
///
/// Any I/O done by the LB policy should be done under the pollset_set
/// returned by \a interested_parties().
// TODO(roth): Once we move to EventManager-based polling, remove the
// interested_parties() hooks from the API.
class LoadBalancingPolicy : public InternallyRefCounted<LoadBalancingPolicy> {
 public:
  /// Interface for accessing per-call state.
  /// Implemented by the client channel and used by the SubchannelPicker.
  class CallState {
   public:
    CallState() = default;
    virtual ~CallState() = default;

    /// Allocates memory associated with the call, which will be
    /// automatically freed when the call is complete.
    /// It is more efficient to use this than to allocate memory directly
    /// for allocations that need to be made on a per-call basis.
    virtual void* Alloc(size_t size) = 0;
  };

  /// Interface for accessing metadata.
  /// Implemented by the client channel and used by the SubchannelPicker.
  class MetadataInterface {
   public:
    virtual ~MetadataInterface() = default;

    virtual std::optional<absl::string_view> Lookup(
        absl::string_view key, std::string* buffer) const = 0;
  };

  /// A list of metadata mutations to be returned along with a PickResult.
  class MetadataMutations {
   public:
    /// Sets a key/value pair.  If the key is already present, it will
    /// be replaced with the new value.
    void Set(absl::string_view key, absl::string_view value) {
      Set(key, grpc_event_engine::experimental::Slice::FromCopiedString(value));
    }
    void Set(absl::string_view key,
             grpc_event_engine::experimental::Slice value) {
      metadata_.push_back({key, std::move(value)});
    }

   private:
    friend class MetadataMutationHandler;

    // Avoid allocation if up to 3 additions per LB pick.  Most expected
    // use cases should be no more than 2, so this gives us a bit of slack.
    // But it should be cheap to increase this value if we start seeing use
    // cases with more than 3 additions.
    absl::InlinedVector<
        std::pair<absl::string_view, grpc_event_engine::experimental::Slice>, 3>
        metadata_;
  };

  /// Arguments used when picking a subchannel for a call.
  struct PickArgs {
    /// The path of the call.  Indicates the RPC service and method name.
    absl::string_view path;
    /// Initial metadata associated with the picking call.
    /// The LB policy may use the existing metadata to influence its routing
    /// decision, and it may add new metadata elements to be sent with the
    /// call to the chosen backend.
    MetadataInterface* initial_metadata;
    /// An interface for accessing call state.  Can be used to allocate
    /// memory associated with the call in an efficient way.
    CallState* call_state;
  };

  /// Interface for accessing backend metric data.
  /// Implemented by the client channel and used by
  /// SubchannelCallTrackerInterface.
  class BackendMetricAccessor {
   public:
    virtual ~BackendMetricAccessor() = default;

    /// Returns the backend metric data returned by the server for the call,
    /// or null if no backend metric data was returned.
    virtual const BackendMetricData* GetBackendMetricData() = 0;
  };

  /// Interface for tracking subchannel calls.
  /// Implemented by LB policy and used by the channel.
  // TODO(roth): Before making this API public, consider whether we
  // should just replace this with a CallTracer, similar to what Java does.
  class SubchannelCallTrackerInterface {
   public:
    virtual ~SubchannelCallTrackerInterface() = default;

    /// Called when a subchannel call is started after an LB pick.
    virtual void Start() = 0;

    /// Called when a subchannel call is completed.
    /// The metadata may be modified by the implementation.  However, the
    /// implementation does not take ownership, so any data that needs to be
    /// used after returning must be copied.
    struct FinishArgs {
      absl::string_view peer_address;
      absl::Status status;
      MetadataInterface* trailing_metadata;
      BackendMetricAccessor* backend_metric_accessor;
    };
    virtual void Finish(FinishArgs args) = 0;
  };

  /// The result of picking a subchannel for a call.
  struct PickResult {
    /// A successful pick.
    struct Complete {
      /// The subchannel to be used for the call.  Must be non-null.
      RefCountedPtr<SubchannelInterface> subchannel;

      /// Optionally set by the LB policy when it wishes to be notified
      /// about the resulting subchannel call.
      /// Note that if the pick is abandoned by the channel, this may never
      /// be used.
      std::unique_ptr<SubchannelCallTrackerInterface> subchannel_call_tracker;

      /// Metadata mutations to be applied to the call.
      MetadataMutations metadata_mutations;

      /// Authority override for the RPC.
      /// Will be used only if the application has not explicitly set
      /// the authority for the RPC.
      grpc_event_engine::experimental::Slice authority_override;

      explicit Complete(
          RefCountedPtr<SubchannelInterface> sc,
          std::unique_ptr<SubchannelCallTrackerInterface> tracker = nullptr,
          MetadataMutations md = MetadataMutations(),
          grpc_event_engine::experimental::Slice authority =
              grpc_event_engine::experimental::Slice())
          : subchannel(std::move(sc)),
            subchannel_call_tracker(std::move(tracker)),
            metadata_mutations(std::move(md)),
            authority_override(std::move(authority)) {}
    };

    /// Pick cannot be completed until something changes on the control
    /// plane.  The client channel will queue the pick and try again the
    /// next time the picker is updated.
    struct Queue {};

    /// Pick failed.  If the call is wait_for_ready, the client channel
    /// will wait for the next picker and try again; otherwise, it
    /// will immediately fail the call with the status indicated (although
    /// the call may be retried if the client channel is configured to do so).
    struct Fail {
      absl::Status status;

      explicit Fail(absl::Status s) : status(s) {}
    };

    /// Pick will be dropped with the status specified.
    /// Unlike FailPick, the call will be dropped even if it is
    /// wait_for_ready, and retries (if configured) will be inhibited.
    struct Drop {
      absl::Status status;

      explicit Drop(absl::Status s) : status(s) {}
    };

    // A pick result must be one of these types.
    // Default to Queue, just to allow default construction.
    std::variant<Complete, Queue, Fail, Drop> result = Queue();

    PickResult() = default;
    // NOLINTNEXTLINE(google-explicit-constructor)
    PickResult(Complete complete) : result(std::move(complete)) {}
    // NOLINTNEXTLINE(google-explicit-constructor)
    PickResult(Queue queue) : result(queue) {}
    // NOLINTNEXTLINE(google-explicit-constructor)
    PickResult(Fail fail) : result(std::move(fail)) {}
    // NOLINTNEXTLINE(google-explicit-constructor)
    PickResult(Drop drop) : result(std::move(drop)) {}
  };

  /// A subchannel picker is the object used to pick the subchannel to
  /// use for a given call.  This is implemented by the LB policy and
  /// used by the client channel to perform picks.
  ///
  /// Pickers are intended to encapsulate all of the state and logic
  /// needed on the data plane (i.e., to actually process picks for
  /// individual calls sent on the channel) while excluding all of the
  /// state and logic needed on the control plane (i.e., resolver
  /// updates, connectivity state notifications, etc); the latter should
  /// live in the LB policy object itself.
  class SubchannelPicker : public DualRefCounted<SubchannelPicker> {
   public:
    SubchannelPicker();

    virtual PickResult Pick(PickArgs args) = 0;

   protected:
    void Orphaned() override {}
  };

  /// A proxy object implemented by the client channel and used by the
  /// LB policy to communicate with the channel.
  class ChannelControlHelper {
   public:
    ChannelControlHelper() = default;
    virtual ~ChannelControlHelper() = default;

    /// Creates a new subchannel with the specified channel args.
    /// The args and per_address_args will be merged by the channel.
    virtual RefCountedPtr<SubchannelInterface> CreateSubchannel(
        const grpc_resolved_address& address,
        const ChannelArgs& per_address_args, const ChannelArgs& args) = 0;

    /// Sets the connectivity state and returns a new picker to be used
    /// by the client channel.
    virtual void UpdateState(grpc_connectivity_state state,
                             const absl::Status& status,
                             RefCountedPtr<SubchannelPicker> picker) = 0;

    /// Requests that the resolver re-resolve.
    virtual void RequestReresolution() = 0;

    /// Returns the channel target.
    virtual absl::string_view GetTarget() = 0;

    /// Returns the channel authority.
    virtual absl::string_view GetAuthority() = 0;

    /// Returns the channel credentials from the parent channel.  This can
    /// be used to create a control-plane channel inside an LB policy.
    virtual RefCountedPtr<grpc_channel_credentials> GetChannelCredentials() = 0;

    /// Returns the UNSAFE ChannelCredentials used to construct the channel,
    /// including bearer tokens.  LB policies should generally have no use for
    /// these credentials, and use of them is heavily discouraged.  These must
    /// be used VERY carefully to avoid sending bearer tokens to untrusted
    /// servers, as the server could then impersonate the client.  Generally,
    /// it is safe to use these credentials only when communicating with the
    /// backends.
    virtual RefCountedPtr<grpc_channel_credentials>
    GetUnsafeChannelCredentials() = 0;

    /// Returns the EventEngine to use for timers and async work.
    virtual grpc_event_engine::experimental::EventEngine* GetEventEngine() = 0;

    /// Returns the stats plugin group for reporting metrics.
    virtual GlobalStatsPluginRegistry::StatsPluginGroup&
    GetStatsPluginGroup() = 0;

    /// Adds a trace message associated with the channel.
    enum TraceSeverity { TRACE_INFO, TRACE_WARNING, TRACE_ERROR };
    virtual void AddTraceEvent(TraceSeverity severity,
                               absl::string_view message) = 0;
  };

  class DelegatingChannelControlHelper;

  template <typename ParentPolicy>
  class ParentOwningDelegatingChannelControlHelper;

  /// Interface for configuration data used by an LB policy implementation.
  /// Individual implementations will create a subclass that adds methods to
  /// return the parameters they need.
  class Config : public RefCounted<Config> {
   public:
    ~Config() override = default;

    // Returns the load balancing policy name
    virtual absl::string_view name() const = 0;
  };

  /// Data passed to the UpdateLocked() method when new addresses and
  /// config are available.
  struct UpdateArgs {
    /// A list of endpoints, each with one or more address, or an error
    /// indicating a failure to obtain the list of addresses.
    absl::StatusOr<std::shared_ptr<EndpointAddressesIterator>> addresses;
    /// The LB policy config.
    RefCountedPtr<Config> config;
    /// A human-readable note providing context about the name resolution that
    /// provided this update.  LB policies may wish to include this message
    /// in RPC failure status messages.  For example, if the update has an
    /// empty list of addresses, this message might say "no DNS entries
    /// found for <name>".
    std::string resolution_note;

    // TODO(roth): Before making this a public API, find a better
    // abstraction for representing channel args.
    ChannelArgs args;
  };

  /// Args used to instantiate an LB policy.
  struct Args {
    /// The work_serializer under which all LB policy calls will be run.
    std::shared_ptr<WorkSerializer> work_serializer;
    /// Channel control helper.
    /// Note: LB policies MUST NOT call any method on the helper from
    /// their constructor.
    std::unique_ptr<ChannelControlHelper> channel_control_helper;
    /// Channel args.
    // TODO(roth): Find a better channel args representation for this API.
    ChannelArgs args;
  };

  explicit LoadBalancingPolicy(Args args, intptr_t initial_refcount = 1);
  ~LoadBalancingPolicy() override;

  // Not copyable nor movable.
  LoadBalancingPolicy(const LoadBalancingPolicy&) = delete;
  LoadBalancingPolicy& operator=(const LoadBalancingPolicy&) = delete;

  /// Returns the name of the LB policy.
  virtual absl::string_view name() const = 0;

  /// Updates the policy with new data from the resolver.  Will be invoked
  /// immediately after LB policy is constructed, and then again whenever
  /// the resolver returns a new result.  The returned status indicates
  /// whether the LB policy accepted the update; if non-OK, informs
  /// polling-based resolvers that they should go into backoff delay and
  /// eventually reattempt the resolution.
  ///
  /// The first time that UpdateLocked() is called, the LB policy will
  /// generally not be able to determine the appropriate connectivity
  /// state by the time UpdateLocked() returns (e.g., it will need to
  /// wait for connectivity state notifications from each subchannel,
  /// which will be delivered asynchronously).  In this case, the LB
  /// policy should not call the helper's UpdateState() method until it
  /// does have a clear picture of the connectivity state (e.g., it
  /// should wait for all subchannels to report connectivity state
  /// before calling the helper's UpdateState() method), although it is
  /// expected to do so within some short period of time.  The parent of
  /// the LB policy will assume that the policy's initial state is
  /// CONNECTING and that picks should be queued.
  virtual absl::Status UpdateLocked(UpdateArgs) = 0;  // NOLINT

  /// Tries to enter a READY connectivity state.
  /// This is a no-op by default, since most LB policies never go into
  /// IDLE state.
  virtual void ExitIdleLocked() {}

  /// Resets connection backoff.
  virtual void ResetBackoffLocked() = 0;

  grpc_pollset_set* interested_parties() const { return interested_parties_; }

  // Note: This must be invoked while holding the work_serializer.
  void Orphan() override;

  // A picker that returns PickResult::Queue for all picks.
  // Also calls the parent LB policy's ExitIdleLocked() method when the
  // first pick is seen.
  class QueuePicker final : public SubchannelPicker {
   public:
    explicit QueuePicker(RefCountedPtr<LoadBalancingPolicy> parent)
        : parent_(std::move(parent)) {}

    ~QueuePicker() override { parent_.reset(DEBUG_LOCATION, "QueuePicker"); }

    PickResult Pick(PickArgs args) override;

   private:
    Mutex mu_;
    RefCountedPtr<LoadBalancingPolicy> parent_ ABSL_GUARDED_BY(&mu_);
  };

  // A picker that returns PickResult::Fail for all picks.
  class TransientFailurePicker final : public SubchannelPicker {
   public:
    explicit TransientFailurePicker(absl::Status status) : status_(status) {}

    PickResult Pick(PickArgs /*args*/) override {
      return PickResult::Fail(status_);
    }

   private:
    absl::Status status_;
  };

  // A picker that returns PickResult::Drop for all picks.
  class DropPicker final : public SubchannelPicker {
   public:
    explicit DropPicker(absl::Status status) : status_(status) {}

    PickResult Pick(PickArgs /*args*/) override {
      return PickResult::Drop(status_);
    }

   private:
    absl::Status status_;
  };

 protected:
  std::shared_ptr<WorkSerializer> work_serializer() const {
    return work_serializer_;
  }

  const ChannelArgs& channel_args() const { return channel_args_; }

  // Note: LB policies MUST NOT call any method on the helper from their
  // constructor.
  ChannelControlHelper* channel_control_helper() const {
    return channel_control_helper_.get();
  }

  /// Shuts down the policy.
  virtual void ShutdownLocked() = 0;

 private:
  /// Work Serializer under which LB policy actions take place.
  std::shared_ptr<WorkSerializer> work_serializer_;
  /// Owned pointer to interested parties in load balancing decisions.
  grpc_pollset_set* interested_parties_;
  /// Channel control helper.
  std::unique_ptr<ChannelControlHelper> channel_control_helper_;
  /// Channel args passed in.
  // TODO(roth): Rework Args so that we don't need to capture channel args here.
  ChannelArgs channel_args_;
};

template <>
struct ArenaContextType<LoadBalancingPolicy::SubchannelCallTrackerInterface> {
  static void Destroy(LoadBalancingPolicy::SubchannelCallTrackerInterface*) {}
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_H
