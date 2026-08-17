// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_SCHEDULER_H_

#include <map>
#include <set>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace base {
class Clock;
}

namespace blink {

class DetachableConsoleLogger;
class DetachableResourceFetcherProperties;
class LoadingBehaviorObserver;

// Client interface to use the throttling/scheduling functionality that
// ResourceLoadScheduler provides.
class PLATFORM_EXPORT ResourceLoadSchedulerClient
    : public GarbageCollectedMixin {
 public:
  // Called when the request is granted to run.
  virtual void Run() = 0;

  void Trace(Visitor* visitor) const override {}
};

// ResourceLoadScheduler provides a unified per-frame infrastructure to schedule
// loading requests. When Request() is called with a
// ResourceLoadSchedulerClient |client|, it calls |client|'s Run() method
// synchronously or asynchronously to notify that |client| can start loading.
//
// A ResourceLoadScheduler may initiate a new resource loading in the following
// cases:
// - When Request() is called
// - When LoosenThrottlingPolicy() is called
// - When SetPriority() is called
// - When Release() is called with kReleaseAndSchedule
// - When OnThrottlingStateChanged() is called
//
// A ResourceLoadScheduler determines if a request can be throttleable or not,
// and keeps track of pending throttleable requests with priority information
// (i.e., ResourceLoadPriority accompanied with an integer called
// "intra-priority"). Here are the general principles:
//  - A ResourceLoadScheduler does not throttle requests that cannot be
//    throttleable. It will call client's Run() method as soon as possible.
//  - A ResourceLoadScheduler determines whether a request can be throttleable
//    by seeing Request()'s ThrottleOption argument and requests' priority
//    information. Requests' priority information can be modified via
//    SetPriority().
//  - A ResourceLoadScheulder won't initiate a new resource loading which can
//    be throttleable when there are more active throttleable requests loading
//    activities more than its internal threshold (i.e., what
//    GetOutstandingLimit() returns)".
//  - Resource loading requests are not throttled when the frame is in the
//    foreground tab.
//  - Resource loading requests are throttled when the frame is in a
//    background tab. It has different thresholds for the main frame
//    and sub frames. When the frame has been background for more than five
//    minutes, all throttleable resource loading requests are throttled
//    indefinitely (i.e., threshold is zero in such a circumstance).
//
//  ResourceLoadScheduler has two modes each of which has its own threshold.
//   - Tight mode (used until the frame sees a <body> element):
//     ResourceLoadScheduler considers a request throttleable if its priority
//     is less than |kHigh|.
//   - Normal mode:
//     ResourceLoadScheduler considers a request throttleable if its priority
//     is less than |kMedium|.
//
// Here is an running experiment:
//   - (As of M86): Low-priority requests are delayed behind "important"
//     requests before some general loading milestone has been reached.
//     "Important", for the experiment means either kHigh or kMedium priority,
//     and the milestones being experimented with are first paint and first
//     contentful paint so far.
//
// Here is a planned experiment (not started yet):
//   - network::VisibilityAwareResourceScheduler
//     An experimental feature with the goal of integrating
//     blink::ResourceLoadScheduler into network::ResourceScheduler for better
//     resource scheduling across multiple frames.
//     TODO(https://crbug.com/1457817): Disable or relax throttling/stopping
//     requests in this class once network::ResoureceScheduler implements
//     similar capabilities.
class PLATFORM_EXPORT ResourceLoadScheduler final
    : public GarbageCollected<ResourceLoadScheduler> {
 public:
  // An option to use in calling Request(). If kCanNotBeStoppedOrThrottled is
  // specified, the request should be granted and Run() should be called
  // synchronously. If kStoppable is specified, Run() will be called immediately
  // unless resource loading is stopped. Otherwise, OnRequestGranted() could be
  // called later when other outstanding requests are finished.
  enum class ThrottleOption {
    kThrottleable = 0,
    kStoppable = 1,
    kCanNotBeStoppedOrThrottled = 2,
  };

  // In some cases we may want to override the default ThrottleOption.  For
  // example, service workers can only perform requests that are normally
  // stoppable, but we want to be able to throttle these requests in some
  // cases.  This enum is used to indicate what kind of override should be
  // applied.
  enum class ThrottleOptionOverride {
    // Use the default ThrottleOption for the request type.
    kNone,
    // Treat stoppable requests as throttleable.
    kStoppableAsThrottleable,
  };

  // An option to use in calling Release(). If kReleaseOnly is specified,
  // the specified request should be released, but no other requests should
  // be scheduled within the call.
  enum class ReleaseOption { kReleaseOnly, kReleaseAndSchedule };

  // A class to pass traffic report hints on calling Release().
  class TrafficReportHints {
   public:
    // |encoded_data_length| is payload size in bytes sent over the network.
    // |decoded_body_length| is received resource data size in bytes.
    TrafficReportHints(int64_t encoded_data_length, int64_t decoded_body_length)
        : valid_(true),
          encoded_data_length_(encoded_data_length),
          decoded_body_length_(decoded_body_length) {}

    // Returns the instance that represents an invalid report, which can be
    // used when a caller don't want to report traffic, i.e. on a failure.
    static PLATFORM_EXPORT TrafficReportHints InvalidInstance() {
      return TrafficReportHints();
    }

    bool IsValid() const { return valid_; }

    int64_t encoded_data_length() const {
      DCHECK(valid_);
      return encoded_data_length_;
    }
    int64_t decoded_body_length() const {
      DCHECK(valid_);
      return decoded_body_length_;
    }

   private:
    // Default constructor makes an invalid instance that won't be recorded.
    TrafficReportHints() = default;

    bool valid_ = false;
    int64_t encoded_data_length_ = 0;
    int64_t decoded_body_length_ = 0;
  };

  // ResourceLoadScheduler has two policies: |kTight| and |kNormal|. Currently
  // this is used to support aggressive throttling while the corresponding frame
  // is in layout-blocking phase. There is only one state transition,
  // |kTight| => |kNormal|, which is done by |LoosenThrottlingPolicy|.
  enum class ThrottlingPolicy { kTight, kNormal };

  // Returned on Request(). Caller should need to return it via Release().
  using ClientId = uint64_t;

  static constexpr ClientId kInvalidClientId = 0u;

  static constexpr size_t kOutstandingUnlimited =
      std::numeric_limits<size_t>::max();

  ResourceLoadScheduler(ThrottlingPolicy initial_throttling_poilcy,
                        ThrottleOptionOverride throttle_option_override,
                        const DetachableResourceFetcherProperties&,
                        FrameOrWorkerScheduler*,
                        DetachableConsoleLogger& console_logger,
                        LoadingBehaviorObserver* loading_behavior_observer);
  ResourceLoadScheduler(const ResourceLoadScheduler&) = delete;
  ResourceLoadScheduler& operator=(const ResourceLoadScheduler&) = delete;
  ~ResourceLoadScheduler();

  void Trace(Visitor*) const;

  // Changes the policy from |kTight| to |kNormal|. This function can be called
  // multiple times, and does nothing when the scheduler is already working with
  // the normal policy. This function may initiate a new resource loading.
  void LoosenThrottlingPolicy();

  // Stops all operations including observing throttling signals.
  // ResourceLoadSchedulerClient::Run() will not be called once this method is
  // called. This method can be called multiple times safely.
  void Shutdown();

  // Makes a request. This may synchronously call
  // ResourceLoadSchedulerClient::Run(), but it is guaranteed that ClientId is
  // populated before ResourceLoadSchedulerClient::Run() is called, so that the
  // caller can call Release() with the assigned ClientId correctly.
  void Request(ResourceLoadSchedulerClient*,
               ThrottleOption,
               ResourceLoadPriority,
               int intra_priority,
               ClientId*);

  // Updates the priority information of the given client. This function may
  // initiate a new resource loading.
  void SetPriority(ClientId, ResourceLoadPriority, int intra_priority);

  // ResourceLoadSchedulerClient should call this method when the loading is
  // finished, or canceled. This method can be called in a pre-finalization
  // step, bug the ReleaseOption must be kReleaseOnly in such a case.
  // TrafficReportHints is for reporting histograms.
  // TrafficReportHints::InvalidInstance() can be used to omit reporting.
  bool Release(ClientId, ReleaseOption, const TrafficReportHints&);

  // Checks if the specified client was already scheduled to call Run(), but
  // haven't call Release() yet.
  bool IsRunning(ClientId id) { return running_requests_.Contains(id); }

  // Sets outstanding limit for testing.
  void SetOutstandingLimitForTesting(size_t limit) {
    SetOutstandingLimitForTesting(limit, limit);
  }
  void SetOutstandingLimitForTesting(size_t tight_limit,
                                     size_t normal_limit,
                                     size_t tight_medium_limit = 0);

  // FrameOrWorkerScheduler lifecycle observer callback.
  void OnLifecycleStateChanged(scheduler::SchedulingLifecycleState);

  // The caller is the owner of the |clock|. The |clock| must outlive the
  // ResourceLoadScheduler.
  void SetClockForTesting(const base::Clock* clock);

  void SetThrottleOptionOverride(
      ThrottleOptionOverride throttle_option_override) {
    throttle_option_override_ = throttle_option_override;
  }

 private:
  class ClientIdWithPriority {
   public:
    struct Compare {
      bool operator()(const ClientIdWithPriority& x,
                      const ClientIdWithPriority& y) const {
        if (x.priority != y.priority)
          return x.priority > y.priority;
        if (x.intra_priority != y.intra_priority)
          return x.intra_priority > y.intra_priority;
        return x.client_id < y.client_id;
      }
    };

    ClientIdWithPriority(ClientId client_id,
                         WebURLRequest::Priority priority,
                         int intra_priority)
        : client_id(client_id),
          priority(priority),
          intra_priority(intra_priority) {}

    const ClientId client_id;
    const ResourceLoadPriority priority;
    const int intra_priority;
  };

  struct ClientInfo : public GarbageCollected<ClientInfo> {
    ClientInfo(ResourceLoadSchedulerClient* client,
               ThrottleOption option,
               ResourceLoadPriority priority,
               int intra_priority)
        : client(client),
          option(option),
          priority(priority),
          intra_priority(intra_priority) {}

    void Trace(Visitor* visitor) const { visitor->Trace(client); }

    Member<ResourceLoadSchedulerClient> client;
    ThrottleOption option;
    ResourceLoadPriority priority;
    int intra_priority;
  };

  using PendingRequestMap = HeapHashMap<ClientId, Member<ClientInfo>>;

  // Checks if |pending_requests_| for the specified option is effectively
  // empty, that means it does not contain any request that is still alive in
  // |pending_request_map_|.
  bool IsPendingRequestEffectivelyEmpty(ThrottleOption option);

  // Gets the highest priority pending request that is allowed to be run.
  bool GetNextPendingRequest(ClientId* id);

  // Returns whether we can throttle a request with the given option based
  // on life cycle state.
  bool IsClientDelayable(ThrottleOption option) const;

  // Generates the next ClientId.
  ClientId GenerateClientId();

  // Picks up one client if there is a budget and route it to run.
  void MaybeRun();

  // Grants a client to run,
  void Run(ClientId,
           ResourceLoadSchedulerClient*,
           bool throttleable,
           ResourceLoadPriority priority);

  size_t GetOutstandingLimit(ResourceLoadPriority priority) const;

  void ShowConsoleMessageIfNeeded();

  bool IsRunningThrottleableRequestsLessThanOutStandingLimit(
      size_t out_standing_limit,
      ResourceLoadPriority priority);

  const Member<const DetachableResourceFetcherProperties>
      resource_fetcher_properties_;

  // A flag to indicate an internal running state.
  bool is_shutdown_ = false;

  ThrottlingPolicy policy_ = ThrottlingPolicy::kNormal;

  // ResourceLoadScheduler threshold values for various circumstances. Some
  // conditions can overlap, and ResourceLoadScheduler chooses the smallest
  // value in such cases.

  // Used when |policy_| is |kTight|.
  size_t tight_outstanding_limit_ = kOutstandingUnlimited;
  size_t tight_medium_limit_ = 0u;

  // Used when |policy_| is |kNormal|.
  size_t normal_outstanding_limit_ = kOutstandingUnlimited;

  // Used when |frame_scheduler_throttling_state_| is |kThrottled|.
  const size_t outstanding_limit_for_throttled_frame_scheduler_;

  // The last used ClientId to calculate the next.
  ClientId current_id_ = kInvalidClientId;

  // Holds clients that were granted.
  HashSet<ClientId> running_requests_;

  HashSet<ClientId> running_throttleable_requests_;
  HashSet<ClientId> running_medium_requests_;

  // Holds a flag to omit repeating console messages.
  bool is_console_info_shown_ = false;

  scheduler::SchedulingLifecycleState frame_scheduler_lifecycle_state_ =
      scheduler::SchedulingLifecycleState::kNotThrottled;

  // Holds clients that haven't been granted, and are waiting for a grant.
  PendingRequestMap pending_request_map_;

  // We use std::set here because WTF doesn't have its counterpart.
  // This tracks two sets of requests, throttleable and stoppable.
  std::map<ThrottleOption,
           std::set<ClientIdWithPriority, ClientIdWithPriority::Compare>>
      pending_requests_ ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)");

  // Remembers elapsed times in seconds when the top request in each queue is
  // processed.
  std::map<ThrottleOption, base::Time> pending_queue_update_times_
      ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)");

  // Handle to throttling observer.
  std::unique_ptr<FrameOrWorkerScheduler::LifecycleObserverHandle>
      scheduler_observer_handle_;

  const Member<DetachableConsoleLogger> console_logger_;

  raw_ptr<const base::Clock> clock_;

  ThrottleOptionOverride throttle_option_override_;

  Member<LoadingBehaviorObserver> loading_behavior_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_SCHEDULER_H_
