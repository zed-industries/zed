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

#ifndef GRPC_TEST_CORE_LOAD_BALANCING_LB_POLICY_TEST_LIB_H
#define GRPC_TEST_CORE_LOAD_BALANCING_LB_POLICY_TEST_LIB_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/support/alloc.h>
#include <grpc/support/port_platform.h>
#include <inttypes.h>
#include <stddef.h>

#include <algorithm>
#include <chrono>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/notification.h"
#include "absl/types/span.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "src/core/client_channel/client_channel_internal.h"
#include "src/core/client_channel/subchannel_interface_internal.h"
#include "src/core/client_channel/subchannel_pool_interface.h"
#include "src/core/config/core_configuration.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/address_utils/parse_address.h"
#include "src/core/lib/address_utils/sockaddr_utils.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/core/lib/experiments/experiments.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/lib/iomgr/timer_manager.h"
#include "src/core/lib/transport/connectivity_state.h"
#include "src/core/load_balancing/backend_metric_data.h"
#include "src/core/load_balancing/health_check_client_internal.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/load_balancing/lb_policy_registry.h"
#include "src/core/load_balancing/oob_backend_metric.h"
#include "src/core/load_balancing/oob_backend_metric_internal.h"
#include "src/core/load_balancing/subchannel_interface.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/service_config/service_config_call_data.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/json/json.h"
#include "src/core/util/match.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/uri.h"
#include "src/core/util/wait_for_single_owner.h"
#include "src/core/util/work_serializer.h"
#include "test/core/event_engine/event_engine_test_utils.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"

namespace grpc_core {
namespace testing {

class LoadBalancingPolicyTest : public ::testing::Test {
 protected:
  using EventEngine = grpc_event_engine::experimental::EventEngine;
  using FuzzingEventEngine =
      grpc_event_engine::experimental::FuzzingEventEngine;

  using CallAttributes =
      std::vector<ServiceConfigCallData::CallAttributeInterface*>;

  // Channel-level subchannel state for a specific address and channel args.
  // This is analogous to the real subchannel in the ClientChannel code.
  class SubchannelState {
   public:
    // A fake SubchannelInterface object, to be returned to the LB
    // policy when it calls the helper's CreateSubchannel() method.
    // There may be multiple FakeSubchannel objects associated with a
    // given SubchannelState object.
    class FakeSubchannel : public SubchannelInterface {
     public:
      explicit FakeSubchannel(SubchannelState* state) : state_(state) {}

      ~FakeSubchannel() override {
        if (orca_watcher_ != nullptr) {
          MutexLock lock(&state_->backend_metric_watcher_mu_);
          state_->orca_watchers_.erase(orca_watcher_.get());
        }
        for (const auto& p : watcher_map_) {
          state_->state_tracker_.RemoveWatcher(p.second);
        }
      }

      SubchannelState* state() const { return state_; }

      std::string address() const override { return state_->address_; }

     private:
      // Converts between
      // SubchannelInterface::ConnectivityStateWatcherInterface and
      // ConnectivityStateWatcherInterface.
      //
      // We support both unique_ptr<> and shared_ptr<>, since raw
      // connectivity watches use the latter but health watches use the
      // former.
      // TODO(roth): Clean this up.
      class WatcherWrapper : public AsyncConnectivityStateWatcherInterface {
       public:
        WatcherWrapper(
            std::shared_ptr<WorkSerializer> work_serializer,
            std::unique_ptr<
                SubchannelInterface::ConnectivityStateWatcherInterface>
                watcher)
            : AsyncConnectivityStateWatcherInterface(
                  std::move(work_serializer)),
              watcher_(std::move(watcher)) {}

        WatcherWrapper(
            std::shared_ptr<WorkSerializer> work_serializer,
            std::shared_ptr<
                SubchannelInterface::ConnectivityStateWatcherInterface>
                watcher)
            : AsyncConnectivityStateWatcherInterface(
                  std::move(work_serializer)),
              watcher_(std::move(watcher)) {}

        void OnConnectivityStateChange(grpc_connectivity_state new_state,
                                       const absl::Status& status) override {
          LOG(INFO) << "notifying watcher: state="
                    << ConnectivityStateName(new_state) << " status=" << status;
          watcher_->OnConnectivityStateChange(new_state, status);
        }

       private:
        std::shared_ptr<SubchannelInterface::ConnectivityStateWatcherInterface>
            watcher_;
      };

      void WatchConnectivityState(
          std::unique_ptr<
              SubchannelInterface::ConnectivityStateWatcherInterface>
              watcher) override
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(*state_->test_->work_serializer_) {
        auto* watcher_ptr = watcher.get();
        auto watcher_wrapper = MakeOrphanable<WatcherWrapper>(
            state_->work_serializer(), std::move(watcher));
        watcher_map_[watcher_ptr] = watcher_wrapper.get();
        state_->state_tracker_.AddWatcher(GRPC_CHANNEL_SHUTDOWN,
                                          std::move(watcher_wrapper));
      }

      void CancelConnectivityStateWatch(
          ConnectivityStateWatcherInterface* watcher) override
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(*state_->test_->work_serializer_) {
        auto it = watcher_map_.find(watcher);
        if (it == watcher_map_.end()) return;
        state_->state_tracker_.RemoveWatcher(it->second);
        watcher_map_.erase(it);
      }

      void RequestConnection() override {
        MutexLock lock(&state_->requested_connection_mu_);
        state_->requested_connection_ = true;
      }

      void AddDataWatcher(
          std::unique_ptr<DataWatcherInterface> watcher) override
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(*state_->test_->work_serializer_) {
        MutexLock lock(&state_->backend_metric_watcher_mu_);
        auto* w =
            static_cast<InternalSubchannelDataWatcherInterface*>(watcher.get());
        if (w->type() == OrcaProducer::Type()) {
          CHECK(orca_watcher_ == nullptr);
          orca_watcher_.reset(static_cast<OrcaWatcher*>(watcher.release()));
          state_->orca_watchers_.insert(orca_watcher_.get());
        } else if (w->type() == HealthProducer::Type()) {
          // TODO(roth): Support health checking in test framework.
          // For now, we just hard-code this to the raw connectivity state.
          CHECK(health_watcher_ == nullptr);
          CHECK_EQ(health_watcher_wrapper_, nullptr);
          health_watcher_.reset(static_cast<HealthWatcher*>(watcher.release()));
          auto connectivity_watcher = health_watcher_->TakeWatcher();
          auto* connectivity_watcher_ptr = connectivity_watcher.get();
          auto watcher_wrapper = MakeOrphanable<WatcherWrapper>(
              state_->work_serializer(), std::move(connectivity_watcher));
          health_watcher_wrapper_ = watcher_wrapper.get();
          state_->state_tracker_.AddWatcher(GRPC_CHANNEL_SHUTDOWN,
                                            std::move(watcher_wrapper));
          LOG(INFO) << "AddDataWatcher(): added HealthWatch="
                    << health_watcher_.get()
                    << " connectivity_watcher=" << connectivity_watcher_ptr
                    << " watcher_wrapper=" << health_watcher_wrapper_;
        }
      }

      void CancelDataWatcher(DataWatcherInterface* watcher) override
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(*state_->test_->work_serializer_) {
        MutexLock lock(&state_->backend_metric_watcher_mu_);
        auto* w = static_cast<InternalSubchannelDataWatcherInterface*>(watcher);
        if (w->type() == OrcaProducer::Type()) {
          if (orca_watcher_.get() != static_cast<OrcaWatcher*>(watcher)) return;
          state_->orca_watchers_.erase(orca_watcher_.get());
          orca_watcher_.reset();
        } else if (w->type() == HealthProducer::Type()) {
          if (health_watcher_.get() != static_cast<HealthWatcher*>(watcher)) {
            return;
          }
          LOG(INFO) << "CancelDataWatcher(): cancelling HealthWatch="
                    << health_watcher_.get()
                    << " watcher_wrapper=" << health_watcher_wrapper_;
          state_->state_tracker_.RemoveWatcher(health_watcher_wrapper_);
          health_watcher_wrapper_ = nullptr;
          health_watcher_.reset();
        }
      }

      // Don't need this method, so it's a no-op.
      void ResetBackoff() override {}

      SubchannelState* state_;
      std::map<SubchannelInterface::ConnectivityStateWatcherInterface*,
               WatcherWrapper*>
          watcher_map_;
      std::unique_ptr<HealthWatcher> health_watcher_;
      WatcherWrapper* health_watcher_wrapper_ = nullptr;
      std::unique_ptr<OrcaWatcher> orca_watcher_;
    };

    SubchannelState(absl::string_view address, LoadBalancingPolicyTest* test)
        : address_(address),
          test_(test),
          state_tracker_("LoadBalancingPolicyTest") {}

    const std::string& address() const { return address_; }

    void AssertValidConnectivityStateTransition(
        grpc_connectivity_state from_state, grpc_connectivity_state to_state,
        SourceLocation location = SourceLocation()) {
      switch (from_state) {
        case GRPC_CHANNEL_IDLE:
          ASSERT_EQ(to_state, GRPC_CHANNEL_CONNECTING)
              << ConnectivityStateName(from_state) << "=>"
              << ConnectivityStateName(to_state) << "\n"
              << location.file() << ":" << location.line();
          break;
        case GRPC_CHANNEL_CONNECTING:
          ASSERT_THAT(to_state,
                      ::testing::AnyOf(GRPC_CHANNEL_READY,
                                       GRPC_CHANNEL_TRANSIENT_FAILURE))
              << ConnectivityStateName(from_state) << "=>"
              << ConnectivityStateName(to_state) << "\n"
              << location.file() << ":" << location.line();
          break;
        case GRPC_CHANNEL_READY:
          ASSERT_EQ(to_state, GRPC_CHANNEL_IDLE)
              << ConnectivityStateName(from_state) << "=>"
              << ConnectivityStateName(to_state) << "\n"
              << location.file() << ":" << location.line();
          break;
        case GRPC_CHANNEL_TRANSIENT_FAILURE:
          ASSERT_EQ(to_state, GRPC_CHANNEL_IDLE)
              << ConnectivityStateName(from_state) << "=>"
              << ConnectivityStateName(to_state) << "\n"
              << location.file() << ":" << location.line();
          break;
        default:
          FAIL() << ConnectivityStateName(from_state) << "=>"
                 << ConnectivityStateName(to_state) << "\n"
                 << location.file() << ":" << location.line();
          break;
      }
    }

    // Sets the connectivity state for this subchannel.  The updated state
    // will be reported to all associated SubchannelInterface objects.
    void SetConnectivityState(
        grpc_connectivity_state state,
        const absl::Status& status = absl::OkStatus(),
        bool validate_state_transition = true,
        absl::AnyInvocable<void()> run_before_flush = nullptr,
        SourceLocation location = SourceLocation()) {
      ExecCtx exec_ctx;
      if (state == GRPC_CHANNEL_TRANSIENT_FAILURE) {
        EXPECT_FALSE(status.ok())
            << "bug in test: TRANSIENT_FAILURE must have non-OK status";
      } else {
        EXPECT_TRUE(status.ok())
            << "bug in test: " << ConnectivityStateName(state)
            << " must have OK status: " << status;
      }
      // Updating the state in the state tracker will enqueue
      // notifications to watchers on the WorkSerializer.  If any
      // subchannel reports READY, the pick_first leaf policy will then
      // start a health watch, whose initial notification will also be
      // scheduled on the WorkSerializer.  We don't want to return until
      // all of those notifications have been delivered.
      absl::Notification notification;
      test_->work_serializer_->Run(
          [&]() ABSL_EXCLUSIVE_LOCKS_REQUIRED(*test_->work_serializer_) {
            if (validate_state_transition) {
              AssertValidConnectivityStateTransition(state_tracker_.state(),
                                                     state, location);
            }
            LOG(INFO) << "Setting state on tracker";
            state_tracker_.SetState(state, status, "set from test");
            // SetState() enqueued the connectivity state notifications for
            // the subchannel, so we add another callback to the queue to be
            // executed after that state notifications has been delivered.
            if (run_before_flush != nullptr) run_before_flush();
            LOG(INFO) << "Waiting for state notifications to be delivered";
            test_->work_serializer_->Run([&]() {
              LOG(INFO) << "State notifications delivered, waiting for "
                           "health notifications";
              // Now the connectivity state notifications has been
              // delivered. If the state reported was READY, then the
              // pick_first leaf policy will have started a health watch, so
              // we add another callback to the queue to be executed after
              // the initial health watch notification has been delivered.
              test_->work_serializer_->Run([&]() { notification.Notify(); });
            });
          });
      while (!notification.HasBeenNotified()) {
        test_->fuzzing_ee_->Tick();
      }
      LOG(INFO) << "Health notifications delivered";
    }

    // Indicates if any of the associated SubchannelInterface objects
    // have requested a connection attempt since the last time this
    // method was called.
    bool ConnectionRequested() {
      MutexLock lock(&requested_connection_mu_);
      return std::exchange(requested_connection_, false);
    }

    // To be invoked by FakeHelper.
    RefCountedPtr<SubchannelInterface> CreateSubchannel() {
      return MakeRefCounted<FakeSubchannel>(this);
    }

    // Sends an OOB backend metric report to all watchers.
    void SendOobBackendMetricReport(const BackendMetricData& backend_metrics) {
      MutexLock lock(&backend_metric_watcher_mu_);
      for (const auto* watcher : orca_watchers_) {
        watcher->watcher()->OnBackendMetricReport(backend_metrics);
      }
    }

    // Checks that all OOB watchers have the expected reporting period.
    void CheckOobReportingPeriod(Duration expected,
                                 SourceLocation location = SourceLocation()) {
      MutexLock lock(&backend_metric_watcher_mu_);
      for (const auto* watcher : orca_watchers_) {
        EXPECT_EQ(watcher->report_interval(), expected)
            << location.file() << ":" << location.line();
      }
    }

    size_t NumWatchers() const {
      size_t num_watchers;
      absl::Notification notification;
      work_serializer()->Run(
          [&]() ABSL_EXCLUSIVE_LOCKS_REQUIRED(*test_->work_serializer_) {
            num_watchers = state_tracker_.NumWatchers();
            notification.Notify();
          });
      while (!notification.HasBeenNotified()) {
        test_->fuzzing_ee_->Tick();
      }
      return num_watchers;
    }

    std::shared_ptr<WorkSerializer> work_serializer() const {
      return test_->work_serializer_;
    }

    ConnectivityStateTracker& state_tracker() { return state_tracker_; }

   private:
    const std::string address_;
    LoadBalancingPolicyTest* const test_;
    ConnectivityStateTracker state_tracker_
        ABSL_GUARDED_BY(*test_->work_serializer_);

    Mutex requested_connection_mu_;
    bool requested_connection_ ABSL_GUARDED_BY(&requested_connection_mu_) =
        false;

    Mutex backend_metric_watcher_mu_;
    std::set<OrcaWatcher*> orca_watchers_
        ABSL_GUARDED_BY(&backend_metric_watcher_mu_);
  };

  // A fake helper to be passed to the LB policy.
  class FakeHelper : public LoadBalancingPolicy::ChannelControlHelper {
   public:
    // Represents a state update reported by the LB policy.
    struct StateUpdate {
      grpc_connectivity_state state;
      absl::Status status;
      RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> picker;

      std::string ToString() const {
        return absl::StrFormat("UPDATE{state=%s, status=%s, picker=%p}",
                               ConnectivityStateName(state), status.ToString(),
                               picker.get());
      }
    };

    // Represents a re-resolution request from the LB policy.
    struct ReresolutionRequested {
      std::string ToString() const { return "RERESOLUTION"; }
    };

    explicit FakeHelper(LoadBalancingPolicyTest* test) : test_(test) {}

    bool QueueEmpty() {
      MutexLock lock(&mu_);
      return queue_.empty();
    }

    // Called at test tear-down time to ensure that we have not left any
    // unexpected events in the queue.
    void ExpectQueueEmpty(SourceLocation location = SourceLocation()) {
      MutexLock lock(&mu_);
      EXPECT_TRUE(queue_.empty())
          << location.file() << ":" << location.line() << "\n"
          << QueueString();
    }

    // Returns the next event in the queue if it is a state update.
    // If the queue is empty or the next event is not a state update,
    // fails the test and returns nullopt without removing anything from
    // the queue.
    std::optional<StateUpdate> GetNextStateUpdate(
        SourceLocation location = SourceLocation()) {
      MutexLock lock(&mu_);
      EXPECT_FALSE(queue_.empty()) << location.file() << ":" << location.line();
      if (queue_.empty()) return std::nullopt;
      Event& event = queue_.front();
      auto* update = std::get_if<StateUpdate>(&event);
      EXPECT_NE(update, nullptr)
          << "unexpected event " << EventString(event) << " at "
          << location.file() << ":" << location.line();
      if (update == nullptr) return std::nullopt;
      StateUpdate result = std::move(*update);
      LOG(INFO) << "dequeued next state update: " << result.ToString();
      queue_.pop_front();
      return std::move(result);
    }

    // Returns the next event in the queue if it is a re-resolution.
    // If the queue is empty or the next event is not a re-resolution,
    // fails the test and returns nullopt without removing anything
    // from the queue.
    std::optional<ReresolutionRequested> GetNextReresolution(
        SourceLocation location = SourceLocation()) {
      MutexLock lock(&mu_);
      EXPECT_FALSE(queue_.empty()) << location.file() << ":" << location.line();
      if (queue_.empty()) return std::nullopt;
      Event& event = queue_.front();
      auto* reresolution = std::get_if<ReresolutionRequested>(&event);
      EXPECT_NE(reresolution, nullptr)
          << "unexpected event " << EventString(event) << " at "
          << location.file() << ":" << location.line();
      if (reresolution == nullptr) return std::nullopt;
      ReresolutionRequested result = *reresolution;
      queue_.pop_front();
      return result;
    }

   private:
    // Represents an event reported by the LB policy.
    using Event = std::variant<StateUpdate, ReresolutionRequested>;

    // Returns a human-readable representation of an event.
    static std::string EventString(const Event& event) {
      return Match(
          event, [](const StateUpdate& update) { return update.ToString(); },
          [](const ReresolutionRequested& reresolution) {
            return reresolution.ToString();
          });
    }

    std::string QueueString() const ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_) {
      std::vector<std::string> parts = {"Queue:"};
      for (const Event& event : queue_) {
        parts.push_back(EventString(event));
      }
      return absl::StrJoin(parts, "\n  ");
    }

    RefCountedPtr<SubchannelInterface> CreateSubchannel(
        const grpc_resolved_address& address,
        const ChannelArgs& /*per_address_args*/,
        const ChannelArgs& args) override {
      // TODO(roth): Need to use per_address_args here.
      SubchannelKey key(
          address, args.RemoveAllKeysWithPrefix(GRPC_ARG_NO_SUBCHANNEL_PREFIX));
      auto it = test_->subchannel_pool_.find(key);
      if (it == test_->subchannel_pool_.end()) {
        auto address_uri = grpc_sockaddr_to_uri(&address);
        CHECK(address_uri.ok());
        it = test_->subchannel_pool_
                 .emplace(std::piecewise_construct, std::forward_as_tuple(key),
                          std::forward_as_tuple(std::move(*address_uri), test_))
                 .first;
      }
      return it->second.CreateSubchannel();
    }

    void UpdateState(
        grpc_connectivity_state state, const absl::Status& status,
        RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> picker) override {
      MutexLock lock(&mu_);
      StateUpdate update{state, status, std::move(picker)};
      LOG(INFO) << "enqueuing state update from LB policy: "
                << update.ToString();
      queue_.push_back(std::move(update));
    }

    void RequestReresolution() override {
      MutexLock lock(&mu_);
      queue_.push_back(ReresolutionRequested());
    }

    absl::string_view GetTarget() override { return test_->target_; }

    absl::string_view GetAuthority() override { return test_->authority_; }

    RefCountedPtr<grpc_channel_credentials> GetChannelCredentials() override {
      return nullptr;
    }

    RefCountedPtr<grpc_channel_credentials> GetUnsafeChannelCredentials()
        override {
      return nullptr;
    }

    EventEngine* GetEventEngine() override { return test_->fuzzing_ee_.get(); }

    GlobalStatsPluginRegistry::StatsPluginGroup& GetStatsPluginGroup()
        override {
      return test_->stats_plugin_group_;
    }

    void AddTraceEvent(TraceSeverity, absl::string_view) override {}

    LoadBalancingPolicyTest* test_;

    Mutex mu_;
    std::deque<Event> queue_ ABSL_GUARDED_BY(&mu_);
  };

  // A fake MetadataInterface implementation, for use in PickArgs.
  class FakeMetadata : public LoadBalancingPolicy::MetadataInterface {
   public:
    explicit FakeMetadata(std::map<std::string, std::string> metadata)
        : metadata_(std::move(metadata)) {}

   private:
    std::optional<absl::string_view> Lookup(
        absl::string_view key, std::string* /*buffer*/) const override {
      auto it = metadata_.find(std::string(key));
      if (it == metadata_.end()) return std::nullopt;
      return it->second;
    }

    std::map<std::string, std::string> metadata_;
  };

  // A fake CallState implementation, for use in PickArgs.
  class FakeCallState : public ClientChannelLbCallState {
   public:
    explicit FakeCallState(const CallAttributes& attributes) {
      for (const auto& attribute : attributes) {
        attributes_.emplace(attribute->type(), attribute);
      }
    }

    ~FakeCallState() override {
      for (void* allocation : allocations_) {
        gpr_free(allocation);
      }
    }

   private:
    void* Alloc(size_t size) override {
      void* allocation = gpr_malloc(size);
      allocations_.push_back(allocation);
      return allocation;
    }

    ServiceConfigCallData::CallAttributeInterface* GetCallAttribute(
        UniqueTypeName type) const override {
      auto it = attributes_.find(type);
      if (it != attributes_.end()) {
        return it->second;
      }
      return nullptr;
    }

    ClientCallTracer::CallAttemptTracer* GetCallAttemptTracer() const override {
      return nullptr;
    }

    std::vector<void*> allocations_;
    std::map<UniqueTypeName, ServiceConfigCallData::CallAttributeInterface*>
        attributes_;
  };

  // A fake BackendMetricAccessor implementation, for passing to
  // SubchannelCallTrackerInterface::Finish().
  class FakeBackendMetricAccessor
      : public LoadBalancingPolicy::BackendMetricAccessor {
   public:
    explicit FakeBackendMetricAccessor(
        std::optional<BackendMetricData> backend_metric_data)
        : backend_metric_data_(std::move(backend_metric_data)) {}

    const BackendMetricData* GetBackendMetricData() override {
      if (backend_metric_data_.has_value()) return &*backend_metric_data_;
      return nullptr;
    }

   private:
    const std::optional<BackendMetricData> backend_metric_data_;
  };

  explicit LoadBalancingPolicyTest(absl::string_view lb_policy_name,
                                   ChannelArgs channel_args = ChannelArgs())
      : lb_policy_name_(lb_policy_name),
        channel_args_(std::move(channel_args)) {}

  void SetUp() override {
    // Order is important here: Fuzzing EE needs to be created before
    // grpc_init().
    fuzzing_ee_ = std::make_shared<FuzzingEventEngine>(
        FuzzingEventEngine::Options(), fuzzing_event_engine::Actions());
    grpc_timer_manager_set_start_threaded(false);
    grpc_init();
    work_serializer_ = std::make_shared<WorkSerializer>(fuzzing_ee_);
    auto helper = std::make_unique<FakeHelper>(this);
    helper_ = helper.get();
    LoadBalancingPolicy::Args args = {work_serializer_, std::move(helper),
                                      channel_args_};
    lb_policy_ =
        CoreConfiguration::Get().lb_policy_registry().CreateLoadBalancingPolicy(
            lb_policy_name_, std::move(args));
    CHECK(lb_policy_ != nullptr);
  }

  void TearDown() override {
    ExecCtx exec_ctx;
    fuzzing_ee_->FuzzingDone();
    // Make sure pickers (and transitively, subchannels) are unreffed before
    // destroying the fixture.
    WaitForWorkSerializerToFlush();
    work_serializer_.reset();
    exec_ctx.Flush();
    if (lb_policy_ != nullptr) {
      // Note: Can't safely trigger this from inside the FakeHelper dtor,
      // because if there is a picker in the queue that is holding a ref
      // to the LB policy, that will prevent the LB policy from being
      // destroyed, and therefore the FakeHelper will not be destroyed.
      // (This will cause an ASAN failure, but it will not display the
      // queued events, so the failure will be harder to diagnose.)
      helper_->ExpectQueueEmpty();
      lb_policy_.reset();
    }
    fuzzing_ee_->TickUntilIdle();
    WaitForSingleOwner(std::move(fuzzing_ee_));
    grpc_shutdown_blocking();
  }

  LoadBalancingPolicy* lb_policy() const {
    CHECK(lb_policy_ != nullptr);
    return lb_policy_.get();
  }

  // Creates an LB policy config from json.
  static RefCountedPtr<LoadBalancingPolicy::Config> MakeConfig(
      const Json& json, SourceLocation location = SourceLocation()) {
    auto status_or_config =
        CoreConfiguration::Get().lb_policy_registry().ParseLoadBalancingConfig(
            json);
    EXPECT_TRUE(status_or_config.ok())
        << status_or_config.status() << "\n"
        << location.file() << ":" << location.line();
    return status_or_config.value();
  }

  // Converts an address URI into a grpc_resolved_address.
  static grpc_resolved_address MakeAddress(absl::string_view address_uri) {
    auto uri = URI::Parse(address_uri);
    CHECK(uri.ok());
    grpc_resolved_address address;
    CHECK(grpc_parse_uri(*uri, &address));
    return address;
  }

  std::vector<grpc_resolved_address> MakeAddressList(
      absl::Span<const absl::string_view> addresses) {
    std::vector<grpc_resolved_address> addrs;
    for (const absl::string_view& address : addresses) {
      addrs.emplace_back(MakeAddress(address));
    }
    return addrs;
  }

  EndpointAddresses MakeEndpointAddresses(
      absl::Span<const absl::string_view> addresses,
      const ChannelArgs& args = ChannelArgs()) {
    return EndpointAddresses(MakeAddressList(addresses), args);
  }

  // Constructs an update containing a list of endpoints.
  LoadBalancingPolicy::UpdateArgs BuildUpdate(
      absl::Span<const EndpointAddresses> endpoints,
      RefCountedPtr<LoadBalancingPolicy::Config> config,
      ChannelArgs args = ChannelArgs()) {
    LoadBalancingPolicy::UpdateArgs update;
    update.addresses = std::make_shared<EndpointAddressesListIterator>(
        EndpointAddressesList(endpoints.begin(), endpoints.end()));
    update.config = std::move(config);
    update.args = std::move(args);
    return update;
  }

  std::vector<EndpointAddresses> MakeEndpointAddressesListFromAddressList(
      absl::Span<const absl::string_view> addresses) {
    std::vector<EndpointAddresses> endpoints;
    for (const absl::string_view address : addresses) {
      endpoints.emplace_back(MakeAddress(address), ChannelArgs());
    }
    return endpoints;
  }

  // Convenient overload that takes a flat address list.
  LoadBalancingPolicy::UpdateArgs BuildUpdate(
      absl::Span<const absl::string_view> addresses,
      RefCountedPtr<LoadBalancingPolicy::Config> config,
      ChannelArgs args = ChannelArgs()) {
    return BuildUpdate(MakeEndpointAddressesListFromAddressList(addresses),
                       std::move(config), std::move(args));
  }

  // Applies the update on the LB policy.
  absl::Status ApplyUpdate(LoadBalancingPolicy::UpdateArgs update_args,
                           LoadBalancingPolicy* lb_policy) {
    ExecCtx exec_ctx;
    absl::Status status;
    // When the LB policy gets the update, it will create new
    // subchannels, and it will register connectivity state watchers and
    // optionally health watchers for each one.  We don't want to return
    // until all the initial notifications for all of those watchers
    // have been delivered to the LB policy.
    absl::Notification notification;
    work_serializer_->Run([&]() {
      status = lb_policy->UpdateLocked(std::move(update_args));
      // UpdateLocked() enqueued the initial connectivity state
      // notifications for the subchannels, so we add another
      // callback to the queue to be executed after those initial
      // state notifications have been delivered.
      LOG(INFO) << "Applied update, waiting for initial connectivity state "
                   "notifications";
      work_serializer_->Run([&]() {
        LOG(INFO) << "Initial connectivity state notifications "
                     "delivered; waiting for health notifications";
        // Now that the initial state notifications have been
        // delivered, the queue will contain the health watch
        // notifications for any subchannels in state READY,
        // so we add another callback to the queue to be
        // executed after those health watch notifications have
        // been delivered.
        work_serializer_->Run([&]() { notification.Notify(); });
      });
    });
    while (!notification.HasBeenNotified()) {
      fuzzing_ee_->Tick();
    }
    LOG(INFO) << "health notifications delivered";
    return status;
  }

  // Invoke ExitIdle on the LB policy
  void ExitIdle() {
    ExecCtx exec_ctx;
    absl::Notification notification;
    // Note: ExitIdle() will enqueue a bunch of connectivity state
    // notifications on the WorkSerializer, and we want to wait until
    // those are delivered to the LB policy.
    work_serializer_->Run([&]() {
      lb_policy_->ExitIdleLocked();
      work_serializer_->Run([&]() { notification.Notify(); });
    });
    while (!notification.HasBeenNotified()) {
      fuzzing_ee_->Tick();
    }
  }

  void ExpectQueueEmpty(SourceLocation location = SourceLocation()) {
    helper_->ExpectQueueEmpty(location);
  }

  // Keeps reading state updates until continue_predicate() returns false.
  // Returns false if the helper reports no events or if the event is
  // not a state update; otherwise (if continue_predicate() tells us to
  // stop) returns true.
  bool WaitForStateUpdate(
      std::function<bool(FakeHelper::StateUpdate update)> continue_predicate,
      SourceLocation location = SourceLocation()) {
    LOG(INFO) << "==> WaitForStateUpdate()";
    while (true) {
      auto update = helper_->GetNextStateUpdate(location);
      if (!update.has_value()) {
        LOG(INFO) << "WaitForStateUpdate() returning false";
        return false;
      }
      if (!continue_predicate(std::move(*update))) {
        LOG(INFO) << "WaitForStateUpdate() returning true";
        return true;
      }
    }
  }

  void ExpectReresolutionRequest(SourceLocation location = SourceLocation()) {
    ASSERT_TRUE(helper_->GetNextReresolution(location))
        << location.file() << ":" << location.line();
  }

  // Expects that the LB policy has reported the specified connectivity
  // state to helper_.  Returns the picker from the state update.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> ExpectState(
      grpc_connectivity_state expected_state,
      absl::Status expected_status = absl::OkStatus(),
      SourceLocation location = SourceLocation()) {
    auto update = helper_->GetNextStateUpdate(location);
    if (!update.has_value()) return nullptr;
    EXPECT_EQ(update->state, expected_state)
        << "got " << ConnectivityStateName(update->state) << ", expected "
        << ConnectivityStateName(expected_state) << "\n"
        << "at " << location.file() << ":" << location.line();
    EXPECT_EQ(update->status, expected_status)
        << update->status << "\n"
        << location.file() << ":" << location.line();
    EXPECT_NE(update->picker, nullptr)
        << location.file() << ":" << location.line();
    return std::move(update->picker);
  }

  // Waits for the LB policy to get connected, then returns the final
  // picker.  There can be any number of CONNECTING updates, each of
  // which must return a picker that queues picks, followed by one
  // update for state READY, whose picker is returned.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> WaitForConnected(
      SourceLocation location = SourceLocation()) {
    LOG(INFO) << "==> WaitForConnected()";
    RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> final_picker;
    WaitForStateUpdate(
        [&](FakeHelper::StateUpdate update) {
          if (update.state == GRPC_CHANNEL_CONNECTING) {
            EXPECT_TRUE(update.status.ok())
                << update.status << " at " << location.file() << ":"
                << location.line();
            ExpectPickQueued(update.picker.get(), {}, {}, location);
            return true;  // Keep going.
          }
          EXPECT_EQ(update.state, GRPC_CHANNEL_READY)
              << ConnectivityStateName(update.state) << " at "
              << location.file() << ":" << location.line();
          final_picker = std::move(update.picker);
          return false;  // Stop.
        },
        location);
    return final_picker;
  }

  void ExpectTransientFailureUpdate(
      absl::Status expected_status,
      SourceLocation location = SourceLocation()) {
    auto picker =
        ExpectState(GRPC_CHANNEL_TRANSIENT_FAILURE, expected_status, location);
    ASSERT_NE(picker, nullptr);
    ExpectPickFail(
        picker.get(),
        [&](const absl::Status& status) {
          EXPECT_EQ(status, expected_status)
              << location.file() << ":" << location.line();
        },
        location);
  }

  // Waits for the LB policy to fail a connection attempt.  There can be
  // any number of CONNECTING updates, each of which must return a picker
  // that queues picks, followed by one update for state TRANSIENT_FAILURE,
  // whose status is passed to check_status() and whose picker must fail
  // picks with a status that is passed to check_status().
  // Returns true if the reported states match expectations.
  bool WaitForConnectionFailed(
      std::function<void(const absl::Status&)> check_status,
      SourceLocation location = SourceLocation()) {
    bool retval = false;
    WaitForStateUpdate(
        [&](FakeHelper::StateUpdate update) {
          if (update.state == GRPC_CHANNEL_CONNECTING) {
            EXPECT_TRUE(update.status.ok())
                << update.status << " at " << location.file() << ":"
                << location.line();
            ExpectPickQueued(update.picker.get(), {}, {}, location);
            return true;  // Keep going.
          }
          EXPECT_EQ(update.state, GRPC_CHANNEL_TRANSIENT_FAILURE)
              << ConnectivityStateName(update.state) << " at "
              << location.file() << ":" << location.line();
          check_status(update.status);
          ExpectPickFail(update.picker.get(), check_status, location);
          retval = update.state == GRPC_CHANNEL_TRANSIENT_FAILURE;
          return false;  // Stop.
        },
        location);
    return retval;
  }

  // Waits for the round_robin policy to start using an updated address list.
  // There can be any number of READY updates where the picker is still using
  // the old list followed by one READY update where the picker is using the
  // new list.  Returns a picker if the reported states match expectations.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker>
  WaitForRoundRobinListChange(absl::Span<const absl::string_view> old_addresses,
                              absl::Span<const absl::string_view> new_addresses,
                              const CallAttributes& call_attributes = {},
                              size_t num_iterations = 3,
                              SourceLocation location = SourceLocation()) {
    LOG(INFO) << "Waiting for expected RR addresses...";
    RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> retval;
    size_t num_picks =
        std::max(new_addresses.size(), old_addresses.size()) * num_iterations;
    WaitForStateUpdate(
        [&](FakeHelper::StateUpdate update) {
          EXPECT_EQ(update.state, GRPC_CHANNEL_READY)
              << location.file() << ":" << location.line();
          if (update.state != GRPC_CHANNEL_READY) return false;
          // Get enough picks to round-robin num_iterations times across all
          // expected addresses.
          auto picks = GetCompletePicks(update.picker.get(), num_picks,
                                        call_attributes, nullptr, location);
          EXPECT_TRUE(picks.has_value())
              << location.file() << ":" << location.line();
          if (!picks.has_value()) return false;
          LOG(INFO) << "PICKS: " << absl::StrJoin(*picks, " ");
          // If the picks still match the old list, then keep going.
          if (PicksAreRoundRobin(old_addresses, *picks)) return true;
          // Otherwise, the picks should match the new list.
          bool matches = PicksAreRoundRobin(new_addresses, *picks);
          EXPECT_TRUE(matches)
              << "Expected: " << absl::StrJoin(new_addresses, ", ")
              << "\nActual: " << absl::StrJoin(*picks, ", ") << "\nat "
              << location.file() << ":" << location.line();
          if (matches) {
            retval = std::move(update.picker);
          }
          return false;  // Stop.
        },
        location);
    LOG(INFO) << "done waiting for expected RR addresses";
    return retval;
  }

  // Expects a state update for the specified state and status, and then
  // expects the resulting picker to queue picks.
  bool ExpectStateAndQueuingPicker(
      grpc_connectivity_state expected_state,
      absl::Status expected_status = absl::OkStatus(),
      SourceLocation location = SourceLocation()) {
    auto picker = ExpectState(expected_state, expected_status, location);
    return ExpectPickQueued(picker.get(), {}, {}, location);
  }

  // Convenient frontend to ExpectStateAndQueuingPicker() for CONNECTING.
  bool ExpectConnectingUpdate(SourceLocation location = SourceLocation()) {
    return ExpectStateAndQueuingPicker(GRPC_CHANNEL_CONNECTING,
                                       absl::OkStatus(), location);
  }

  static std::unique_ptr<LoadBalancingPolicy::MetadataInterface> MakeMetadata(
      std::map<std::string, std::string> init = {}) {
    return std::make_unique<FakeMetadata>(init);
  }

  // Does a pick and returns the result.
  LoadBalancingPolicy::PickResult DoPick(
      LoadBalancingPolicy::SubchannelPicker* picker,
      const CallAttributes& call_attributes = {},
      const std::map<std::string, std::string>& metadata = {}) {
    ExecCtx exec_ctx;
    FakeMetadata md(metadata);
    FakeCallState call_state(call_attributes);
    return picker->Pick({"/service/method", &md, &call_state});
  }

  // Requests a pick on picker and expects a Queue result.
  bool ExpectPickQueued(LoadBalancingPolicy::SubchannelPicker* picker,
                        const CallAttributes call_attributes = {},
                        const std::map<std::string, std::string>& metadata = {},
                        SourceLocation location = SourceLocation()) {
    EXPECT_NE(picker, nullptr) << location.file() << ":" << location.line();
    if (picker == nullptr) return false;
    auto pick_result = DoPick(picker, call_attributes, metadata);
    EXPECT_TRUE(std::holds_alternative<LoadBalancingPolicy::PickResult::Queue>(
        pick_result.result))
        << PickResultString(pick_result) << "\nat " << location.file() << ":"
        << location.line();
    return std::holds_alternative<LoadBalancingPolicy::PickResult::Queue>(
        pick_result.result);
  }

  // Requests a pick on picker and expects a Complete result.
  // The address of the resulting subchannel is returned, or nullopt if
  // the result was something other than Complete.
  // If the complete pick includes a SubchannelCallTrackerInterface, then if
  // subchannel_call_tracker is non-null, it will be set to point to the
  // call tracker; otherwise, the call tracker will be invoked
  // automatically to represent a complete call with no backend metric data.
  std::optional<std::string> ExpectPickComplete(
      LoadBalancingPolicy::SubchannelPicker* picker,
      const CallAttributes& call_attributes = {},
      const std::map<std::string, std::string>& metadata = {},
      std::unique_ptr<LoadBalancingPolicy::SubchannelCallTrackerInterface>*
          subchannel_call_tracker = nullptr,
      SubchannelState::FakeSubchannel** picked_subchannel = nullptr,
      SourceLocation location = SourceLocation()) {
    EXPECT_NE(picker, nullptr);
    if (picker == nullptr) {
      return std::nullopt;
    }
    auto pick_result = DoPick(picker, call_attributes, metadata);
    auto* complete = std::get_if<LoadBalancingPolicy::PickResult::Complete>(
        &pick_result.result);
    EXPECT_NE(complete, nullptr) << PickResultString(pick_result) << " at "
                                 << location.file() << ":" << location.line();
    if (complete == nullptr) return std::nullopt;
    auto* subchannel = static_cast<SubchannelState::FakeSubchannel*>(
        complete->subchannel.get());
    if (picked_subchannel != nullptr) *picked_subchannel = subchannel;
    std::string address = subchannel->state()->address();
    if (complete->subchannel_call_tracker != nullptr) {
      if (subchannel_call_tracker != nullptr) {
        *subchannel_call_tracker = std::move(complete->subchannel_call_tracker);
      } else {
        ReportCompletionToCallTracker(
            std::move(complete->subchannel_call_tracker), address);
      }
    }
    return address;
  }

  void ReportCompletionToCallTracker(
      std::unique_ptr<LoadBalancingPolicy::SubchannelCallTrackerInterface>
          subchannel_call_tracker,
      absl::string_view address, absl::Status status = absl::OkStatus()) {
    subchannel_call_tracker->Start();
    FakeMetadata metadata({});
    FakeBackendMetricAccessor backend_metric_accessor({});
    LoadBalancingPolicy::SubchannelCallTrackerInterface::FinishArgs args = {
        address, status, &metadata, &backend_metric_accessor};
    subchannel_call_tracker->Finish(args);
  }

  // Gets num_picks complete picks from picker and returns the resulting
  // list of addresses, or nullopt if a non-complete pick was returned.
  std::optional<std::vector<std::string>> GetCompletePicks(
      LoadBalancingPolicy::SubchannelPicker* picker, size_t num_picks,
      const CallAttributes& call_attributes = {},
      std::vector<
          std::unique_ptr<LoadBalancingPolicy::SubchannelCallTrackerInterface>>*
          subchannel_call_trackers = nullptr,
      SourceLocation location = SourceLocation()) {
    EXPECT_NE(picker, nullptr);
    if (picker == nullptr) {
      return std::nullopt;
    }
    std::vector<std::string> results;
    for (size_t i = 0; i < num_picks; ++i) {
      std::unique_ptr<LoadBalancingPolicy::SubchannelCallTrackerInterface>
          subchannel_call_tracker;
      auto address = ExpectPickComplete(picker, call_attributes,
                                        /*metadata=*/{},
                                        subchannel_call_trackers == nullptr
                                            ? nullptr
                                            : &subchannel_call_tracker,
                                        nullptr, location);
      if (!address.has_value()) return std::nullopt;
      results.emplace_back(std::move(*address));
      if (subchannel_call_trackers != nullptr) {
        subchannel_call_trackers->emplace_back(
            std::move(subchannel_call_tracker));
      }
    }
    return results;
  }

  // Returns true if the list of actual pick result addresses matches the
  // list of expected addresses for round_robin.  Note that the actual
  // addresses may start anywhere in the list of expected addresses but
  // must then continue in round-robin fashion, with wrap-around.
  bool PicksAreRoundRobin(absl::Span<const absl::string_view> expected,
                          absl::Span<const std::string> actual) {
    std::optional<size_t> expected_index;
    for (const auto& address : actual) {
      auto it = std::find(expected.begin(), expected.end(), address);
      if (it == expected.end()) return false;
      size_t index = it - expected.begin();
      if (expected_index.has_value() && index != *expected_index) return false;
      expected_index = (index + 1) % expected.size();
    }
    return true;
  }

  // Checks that the picker has round-robin behavior over the specified
  // set of addresses.
  void ExpectRoundRobinPicks(LoadBalancingPolicy::SubchannelPicker* picker,
                             absl::Span<const absl::string_view> addresses,
                             const CallAttributes& call_attributes = {},
                             size_t num_iterations = 3,
                             SourceLocation location = SourceLocation()) {
    auto picks = GetCompletePicks(picker, num_iterations * addresses.size(),
                                  call_attributes, nullptr, location);
    ASSERT_TRUE(picks.has_value()) << location.file() << ":" << location.line();
    EXPECT_TRUE(PicksAreRoundRobin(addresses, *picks))
        << "  Actual: " << absl::StrJoin(*picks, ", ")
        << "\n  Expected: " << absl::StrJoin(addresses, ", ") << "\n"
        << location.file() << ":" << location.line();
  }

  // Expect startup with RR with a set of addresses.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> ExpectRoundRobinStartup(
      absl::Span<const EndpointAddresses> endpoints,
      SourceLocation location = SourceLocation()) {
    CHECK(!endpoints.empty());
    // There should be a subchannel for every address.
    // We will wind up connecting to the first address for every endpoint.
    std::vector<std::vector<SubchannelState*>> endpoint_subchannels;
    endpoint_subchannels.reserve(endpoints.size());
    std::vector<std::string> chosen_addresses_storage;
    chosen_addresses_storage.reserve(endpoints.size());
    std::vector<absl::string_view> chosen_addresses;
    chosen_addresses.reserve(endpoints.size());
    for (const EndpointAddresses& endpoint : endpoints) {
      endpoint_subchannels.emplace_back();
      endpoint_subchannels.back().reserve(endpoint.addresses().size());
      for (size_t i = 0; i < endpoint.addresses().size(); ++i) {
        const grpc_resolved_address& address = endpoint.addresses()[i];
        std::string address_str = grpc_sockaddr_to_uri(&address).value();
        auto* subchannel = FindSubchannel(address_str);
        EXPECT_NE(subchannel, nullptr)
            << address_str << "\n"
            << location.file() << ":" << location.line();
        if (subchannel == nullptr) return nullptr;
        endpoint_subchannels.back().push_back(subchannel);
        if (i == 0) {
          chosen_addresses_storage.emplace_back(std::move(address_str));
          chosen_addresses.emplace_back(chosen_addresses_storage.back());
        }
      }
    }
    // We should request a connection to the first address of each endpoint,
    // and not to any of the subsequent addresses.
    for (const auto& subchannels : endpoint_subchannels) {
      EXPECT_TRUE(subchannels[0]->ConnectionRequested())
          << location.file() << ":" << location.line();
      for (size_t i = 1; i < subchannels.size(); ++i) {
        EXPECT_FALSE(subchannels[i]->ConnectionRequested())
            << "i=" << i << "\n"
            << location.file() << ":" << location.line();
      }
    }
    // The subchannels that we've asked to connect should report
    // CONNECTING state.
    for (size_t i = 0; i < endpoint_subchannels.size(); ++i) {
      endpoint_subchannels[i][0]->SetConnectivityState(GRPC_CHANNEL_CONNECTING);
      if (i == 0) ExpectConnectingUpdate(location);
    }
    // The connection attempts should succeed.
    RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> picker;
    for (size_t i = 0; i < endpoint_subchannels.size(); ++i) {
      endpoint_subchannels[i][0]->SetConnectivityState(GRPC_CHANNEL_READY);
      if (i == 0) {
        // When the first subchannel becomes READY, accept any number of
        // CONNECTING updates with a picker that queues followed by a READY
        // update with a picker that repeatedly returns only the first address.
        picker = WaitForConnected(location);
        ExpectRoundRobinPicks(picker.get(), {chosen_addresses[0]}, {}, 3,
                              location);
      } else {
        // When each subsequent subchannel becomes READY, we accept any number
        // of READY updates where the picker returns only the previously
        // connected subchannel(s) followed by a READY update where the picker
        // returns the previously connected subchannel(s) *and* the newly
        // connected subchannel.
        picker = WaitForRoundRobinListChange(
            absl::MakeSpan(chosen_addresses).subspan(0, i),
            absl::MakeSpan(chosen_addresses).subspan(0, i + 1), {}, 3,
            location);
      }
    }
    return picker;
  }

  // A convenient override that takes a flat list of addresses, one per
  // endpoint.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> ExpectRoundRobinStartup(
      absl::Span<const absl::string_view> addresses,
      SourceLocation location = SourceLocation()) {
    return ExpectRoundRobinStartup(
        MakeEndpointAddressesListFromAddressList(addresses), location);
  }

  // Expects zero or more picker updates, each of which returns
  // round-robin picks for the specified set of addresses.
  RefCountedPtr<LoadBalancingPolicy::SubchannelPicker>
  DrainRoundRobinPickerUpdates(absl::Span<const absl::string_view> addresses,
                               SourceLocation location = SourceLocation()) {
    LOG(INFO) << "Draining RR picker updates...";
    RefCountedPtr<LoadBalancingPolicy::SubchannelPicker> picker;
    while (!helper_->QueueEmpty()) {
      auto update = helper_->GetNextStateUpdate(location);
      EXPECT_TRUE(update.has_value())
          << location.file() << ":" << location.line();
      if (!update.has_value()) return nullptr;
      EXPECT_EQ(update->state, GRPC_CHANNEL_READY)
          << location.file() << ":" << location.line();
      if (update->state != GRPC_CHANNEL_READY) return nullptr;
      ExpectRoundRobinPicks(update->picker.get(), addresses,
                            /*call_attributes=*/{}, /*num_iterations=*/3,
                            location);
      picker = std::move(update->picker);
    }
    LOG(INFO) << "Done draining RR picker updates";
    return picker;
  }

  // Expects zero or more CONNECTING updates.
  void DrainConnectingUpdates(SourceLocation location = SourceLocation()) {
    LOG(INFO) << "Draining CONNECTING updates...";
    while (!helper_->QueueEmpty()) {
      ASSERT_TRUE(ExpectConnectingUpdate(location));
    }
    LOG(INFO) << "Done draining CONNECTING updates";
  }

  // Triggers a connection failure for the current address for an
  // endpoint and expects a reconnection to the specified new address.
  void ExpectEndpointAddressChange(
      absl::Span<const absl::string_view> addresses, size_t current_index,
      size_t new_index, absl::AnyInvocable<void()> expect_after_disconnect,
      SourceLocation location = SourceLocation()) {
    LOG(INFO) << "Expecting endpoint address change: addresses={"
              << absl::StrJoin(addresses, ", ")
              << "}, current_index=" << current_index
              << ", new_index=" << new_index;
    ASSERT_LT(current_index, addresses.size());
    ASSERT_LT(new_index, addresses.size());
    // Find all subchannels.
    std::vector<SubchannelState*> subchannels;
    subchannels.reserve(addresses.size());
    for (absl::string_view address : addresses) {
      SubchannelState* subchannel = FindSubchannel(address);
      ASSERT_NE(subchannel, nullptr)
          << "can't find subchannel for " << address << "\n"
          << location.file() << ":" << location.line();
      subchannels.push_back(subchannel);
    }
    // Cause current_address to become disconnected.
    subchannels[current_index]->SetConnectivityState(GRPC_CHANNEL_IDLE);
    ExpectReresolutionRequest(location);
    if (expect_after_disconnect != nullptr) expect_after_disconnect();
    // Attempt each address in the list until we hit the desired new address.
    for (size_t i = 0; i < subchannels.size(); ++i) {
      // A connection should be requested on the subchannel for this
      // index, and none of the others.
      for (size_t j = 0; j < addresses.size(); ++j) {
        EXPECT_EQ(subchannels[j]->ConnectionRequested(), j == i)
            << location.file() << ":" << location.line();
      }
      // Subchannel will report CONNECTING.
      SubchannelState* subchannel = subchannels[i];
      subchannel->SetConnectivityState(GRPC_CHANNEL_CONNECTING);
      // If this is the one we want to stick with, it will report READY.
      if (i == new_index) {
        subchannel->SetConnectivityState(GRPC_CHANNEL_READY);
        break;
      }
      // Otherwise, report TF.
      subchannel->SetConnectivityState(
          GRPC_CHANNEL_TRANSIENT_FAILURE,
          absl::UnavailableError("connection failed"));
      // Report IDLE to leave it in the expected state in case the test
      // interacts with it again.
      subchannel->SetConnectivityState(GRPC_CHANNEL_IDLE);
    }
    LOG(INFO) << "Done with endpoint address change";
  }

  // Requests a picker on picker and expects a Fail result.
  // The failing status is passed to check_status.
  void ExpectPickFail(LoadBalancingPolicy::SubchannelPicker* picker,
                      std::function<void(const absl::Status&)> check_status,
                      SourceLocation location = SourceLocation()) {
    auto pick_result = DoPick(picker);
    auto* fail =
        std::get_if<LoadBalancingPolicy::PickResult::Fail>(&pick_result.result);
    ASSERT_NE(fail, nullptr) << PickResultString(pick_result) << " at "
                             << location.file() << ":" << location.line();
    check_status(fail->status);
  }

  // Returns a human-readable string for a pick result.
  static std::string PickResultString(
      const LoadBalancingPolicy::PickResult& result) {
    return Match(
        result.result,
        [](const LoadBalancingPolicy::PickResult::Complete& complete) {
          auto* subchannel = static_cast<SubchannelState::FakeSubchannel*>(
              complete.subchannel.get());
          return absl::StrFormat(
              "COMPLETE{subchannel=%s, subchannel_call_tracker=%p}",
              subchannel->state()->address(),
              complete.subchannel_call_tracker.get());
        },
        [](const LoadBalancingPolicy::PickResult::Queue&) -> std::string {
          return "QUEUE{}";
        },
        [](const LoadBalancingPolicy::PickResult::Fail& fail) -> std::string {
          return absl::StrFormat("FAIL{%s}", fail.status.ToString());
        },
        [](const LoadBalancingPolicy::PickResult::Drop& drop) -> std::string {
          return absl::StrFormat("FAIL{%s}", drop.status.ToString());
        });
  }

  // Returns the entry in the subchannel pool, or null if not present.
  SubchannelState* FindSubchannel(absl::string_view address,
                                  const ChannelArgs& args = ChannelArgs()) {
    SubchannelKey key(MakeAddress(address), args);
    auto it = subchannel_pool_.find(key);
    if (it == subchannel_pool_.end()) return nullptr;
    return &it->second;
  }

  // Creates and returns an entry in the subchannel pool.
  // This can be used in cases where we want to test that a subchannel
  // already exists when the LB policy creates it (e.g., due to it being
  // created by another channel and shared via the global subchannel
  // pool, or by being created by another LB policy in this channel).
  SubchannelState* CreateSubchannel(absl::string_view address,
                                    const ChannelArgs& args = ChannelArgs()) {
    SubchannelKey key(MakeAddress(address), args);
    auto it = subchannel_pool_
                  .emplace(std::piecewise_construct, std::forward_as_tuple(key),
                           std::forward_as_tuple(address, this))
                  .first;
    return &it->second;
  }

  void WaitForWorkSerializerToFlush() {
    ExecCtx exec_ctx;
    LOG(INFO) << "waiting for WorkSerializer to flush...";
    absl::Notification notification;
    work_serializer_->Run([&]() { notification.Notify(); });
    while (!notification.HasBeenNotified()) {
      fuzzing_ee_->Tick();
    }
    LOG(INFO) << "WorkSerializer flush complete";
  }

  void IncrementTimeBy(Duration duration, bool flush_work_serializer = true) {
    ExecCtx exec_ctx;
    LOG(INFO) << "Incrementing time by " << duration;
    fuzzing_ee_->TickForDuration(duration);
    LOG(INFO) << "Done incrementing time";
    // Flush WorkSerializer, in case the timer callback enqueued anything.
    if (flush_work_serializer) WaitForWorkSerializerToFlush();
  }

  void SetExpectedTimerDuration(std::optional<EventEngine::Duration> duration,
                                SourceLocation location = SourceLocation()) {
    if (duration.has_value()) {
      fuzzing_ee_->SetRunAfterDurationCallback(
          [expected = *duration,
           location = location](EventEngine::Duration duration) {
            EXPECT_EQ(duration, expected)
                << "Expected: " << expected.count()
                << "ns\n  Actual: " << duration.count() << "ns\n"
                << location.file() << ":" << location.line();
          });
    } else {
      fuzzing_ee_->SetRunAfterDurationCallback(nullptr);
    }
  }

  std::shared_ptr<FuzzingEventEngine> fuzzing_ee_;
  std::shared_ptr<WorkSerializer> work_serializer_;
  FakeHelper* helper_ = nullptr;
  std::map<SubchannelKey, SubchannelState> subchannel_pool_;
  OrphanablePtr<LoadBalancingPolicy> lb_policy_;
  const absl::string_view lb_policy_name_;
  const ChannelArgs channel_args_;
  GlobalStatsPluginRegistry::StatsPluginGroup stats_plugin_group_;
  std::string target_ = "dns:server.example.com";
  std::string authority_ = "server.example.com";
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_LOAD_BALANCING_LB_POLICY_TEST_LIB_H
