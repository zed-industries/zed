// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_OR_WORKER_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_OR_WORKER_SCHEDULER_H_

#include "base/functional/callback.h"
#include "base/memory/raw_ref.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/types/strong_alias.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/feature_and_js_location_blocking_bfcache.h"
#include "third_party/blink/renderer/platform/scheduler/public/scheduling_lifecycle_state.h"
#include "third_party/blink/renderer/platform/scheduler/public/scheduling_policy.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_queue_type.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace {

// Maximum number of back/forward cache blocking details to send to the browser.
// As long as this is a small number, we don't have to worry about the cost of
// linear searches of the vector.
constexpr size_t kMaxNumberOfBackForwardCacheBlockingDetails = 10;

}  // namespace

namespace blink {
class FrameScheduler;
class WebSchedulingTaskQueue;

// This is the base class of FrameScheduler and WorkerScheduler.
class PLATFORM_EXPORT FrameOrWorkerScheduler {
  USING_FAST_MALLOC(FrameOrWorkerScheduler);

 public:
  // Observer type that regulates conditions to invoke callbacks.
  enum class ObserverType { kLoader, kWorkerScheduler };

  // Callback type for receiving scheduling policy change events.
  using OnLifecycleStateChangedCallback =
      base::RepeatingCallback<void(scheduler::SchedulingLifecycleState)>;

  class PLATFORM_EXPORT LifecycleObserverHandle {
    USING_FAST_MALLOC(LifecycleObserverHandle);

   public:
    explicit LifecycleObserverHandle(FrameOrWorkerScheduler* scheduler);
    LifecycleObserverHandle(const LifecycleObserverHandle&) = delete;
    LifecycleObserverHandle& operator=(const LifecycleObserverHandle&) = delete;
    ~LifecycleObserverHandle();

   private:
    base::WeakPtr<FrameOrWorkerScheduler> scheduler_;
  };

  // RAII handle which should be kept alive as long as the feature is active
  // and the policy should be applied.
  // TODO(crbug.com/1366675): Rename SchedulingAffectingFeatureHandle to
  // NonStickyFeatureHandle and move it to
  // back_forward_cache_disabling_feature_tracker.h.
  class PLATFORM_EXPORT SchedulingAffectingFeatureHandle {
    DISALLOW_NEW();

   public:
    SchedulingAffectingFeatureHandle() = default;
    SchedulingAffectingFeatureHandle(
        SchedulingPolicy::Feature feature,
        SchedulingPolicy policy,
        std::unique_ptr<SourceLocation> source_location,
        base::WeakPtr<FrameOrWorkerScheduler>);
    SchedulingAffectingFeatureHandle(SchedulingAffectingFeatureHandle&&);
    SchedulingAffectingFeatureHandle& operator=(
        SchedulingAffectingFeatureHandle&&);

    inline ~SchedulingAffectingFeatureHandle() { reset(); }

    explicit operator bool() const { return scheduler_.get(); }

    inline void reset() {
      if (scheduler_)
        scheduler_->OnStoppedUsingNonStickyFeature(this);
      scheduler_ = nullptr;
    }

    SchedulingPolicy GetPolicy() const;
    SchedulingPolicy::Feature GetFeature() const;

    const FeatureAndJSLocationBlockingBFCache&
    GetFeatureAndJSLocationBlockingBFCache() const;

   private:
    friend class FrameOrWorkerScheduler;

    SchedulingPolicy::Feature feature_ = SchedulingPolicy::Feature::kMaxValue;
    SchedulingPolicy policy_;
    FeatureAndJSLocationBlockingBFCache feature_and_js_location_;
    base::WeakPtr<FrameOrWorkerScheduler> scheduler_;
  };

  // A struct to wrap a vector of `FeatureAndJSLocationBlockingBFCache`.
  struct BFCacheBlockingFeatureAndLocations {
    void MaybeAdd(FeatureAndJSLocationBlockingBFCache details) {
      // Only add `details` when the same one does not exist already in the
      // `details_list` and when the size of the `details_list` is less than
      // `kMaxNumberOfBackForwardCacheBlockingDetails` to avoid sending a big
      // mojo message.
      if (details_list.Find(details) == kNotFound &&
          details_list.size() < kMaxNumberOfBackForwardCacheBlockingDetails) {
        details_list.push_back(details);
      }
    }
    void Erase(FeatureAndJSLocationBlockingBFCache details) {
      wtf_size_t index = details_list.Find(details);
      // Because we avoid duplicates and set a limit, the details might not be
      // found.
      if (index != kNotFound) {
        details_list.EraseAt(index);
      }
    }
    void Clear() { details_list.clear(); }
    bool operator==(BFCacheBlockingFeatureAndLocations& other) {
      return details_list == other.details_list;
    }

    WTF::Vector<FeatureAndJSLocationBlockingBFCache> details_list;
  };

  class PLATFORM_EXPORT Delegate {
   public:
    using BFCacheBlockingFeatureAndLocations =
        FrameOrWorkerScheduler::BFCacheBlockingFeatureAndLocations;

    struct BlockingDetails {
      const raw_ref<const BFCacheBlockingFeatureAndLocations>
          non_sticky_features_and_js_locations;
      const raw_ref<const BFCacheBlockingFeatureAndLocations>
          sticky_features_and_js_locations;
      BlockingDetails(BFCacheBlockingFeatureAndLocations& non_sticky,
                      BFCacheBlockingFeatureAndLocations& sticky)
          : non_sticky_features_and_js_locations(non_sticky),
            sticky_features_and_js_locations(sticky) {}
    };
    virtual ~Delegate() = default;

    // Notifies that the list of active blocking features for this worker has
    // changed when a blocking feature and its JS location are registered or
    // removed.
    virtual void UpdateBackForwardCacheDisablingFeatures(BlockingDetails) = 0;

    base::WeakPtr<Delegate> AsWeakPtr() {
      return weak_ptr_factory_.GetWeakPtr();
    }
    base::WeakPtrFactory<Delegate> weak_ptr_factory_{this};
  };

  virtual ~FrameOrWorkerScheduler();

  // Notifies scheduler that this execution context has started using a feature
  // which impacts scheduling decisions.
  // When the feature stops being used, this handle should be destroyed.
  //
  // Usage:
  // handle = scheduler->RegisterFeature(
  //     kYourFeature, { SchedulingPolicy::DisableSomething() });
  // TODO(crbug.com/1366675): Rename RegisterFeature to
  // RegisterNonStickyFeature.

  [[nodiscard]] SchedulingAffectingFeatureHandle RegisterFeature(
      SchedulingPolicy::Feature feature,
      SchedulingPolicy policy);

  // Register a feature which is used for the rest of the lifetime of
  // the document and can't be unregistered.
  // The policy is reset when the main frame navigates away from the current
  // document.
  void RegisterStickyFeature(SchedulingPolicy::Feature feature,
                             SchedulingPolicy policy);

  // Adds an observer callback to be notified on scheduling policy changed.
  // When a callback is added, the initial state will be notified synchronously
  // through the callback. The callback may be invoked consecutively with the
  // same value. Returns a RAII handle that unregisters the callback when the
  // handle is destroyed.
  //
  // New usage outside of platform/ should be rare. Prefer using
  // ExecutionContextLifecycleStateObserver to observe paused and frozenness
  // changes and PageVisibilityObserver to observe visibility changes. One
  // exception is that this observer enables observing visibility changes of the
  // associated page in workers, whereas PageVisibilityObserver does not
  // (crbug.com/1286570).
  [[nodiscard]] std::unique_ptr<LifecycleObserverHandle> AddLifecycleObserver(
      ObserverType,
      OnLifecycleStateChangedCallback);

  // Creates a new task queue for use with the web-exposed scheduling API with
  // the given priority and type. See https://wicg.github.io/scheduling-apis.
  virtual std::unique_ptr<WebSchedulingTaskQueue> CreateWebSchedulingTaskQueue(
      WebSchedulingQueueType,
      WebSchedulingPriority) = 0;

  virtual FrameScheduler* ToFrameScheduler() { return nullptr; }

  base::WeakPtr<FrameOrWorkerScheduler> GetWeakPtr();

  // Returns a task runner for compositor tasks. This is intended only to be
  // used by specific animation and rendering related tasks (e.g. animated GIFS)
  // and should not generally be used.
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  CompositorTaskRunner() = 0;

  // Returns a WebScopedVirtualTimePauser which can be used to vote for pausing
  // virtual time. Virtual time will be paused if any WebScopedVirtualTimePauser
  // votes to pause it, and only unpaused only if all
  // WebScopedVirtualTimePausers are either destroyed or vote to unpause.  Note
  // the WebScopedVirtualTimePauser returned by this method is initially
  // unpaused.
  // TODO(crbug.com/1416992): consider moving this to ThreadScheduler.
  virtual WebScopedVirtualTimePauser CreateWebScopedVirtualTimePauser(
      const String& name,
      WebScopedVirtualTimePauser::VirtualTaskDuration) = 0;

 protected:
  FrameOrWorkerScheduler();

  void NotifyLifecycleObservers();

  virtual scheduler::SchedulingLifecycleState CalculateLifecycleState(
      ObserverType) const {
    return scheduler::SchedulingLifecycleState::kNotThrottled;
  }

  // |source_location| is nullptr when JS is not running.
  virtual void OnStartedUsingNonStickyFeature(
      SchedulingPolicy::Feature feature,
      const SchedulingPolicy& policy,
      std::unique_ptr<SourceLocation> source_location,
      SchedulingAffectingFeatureHandle* handle) = 0;
  // |source_location| is nullptr when JS is not running.
  virtual void OnStartedUsingStickyFeature(
      SchedulingPolicy::Feature feature,
      const SchedulingPolicy& policy,
      std::unique_ptr<SourceLocation> source_location) = 0;
  virtual void OnStoppedUsingNonStickyFeature(
      SchedulingAffectingFeatureHandle* handle) = 0;

  // Gets a weak pointer for this scheduler that is reset when the influence by
  // registered features to this scheduler is reset.
  virtual base::WeakPtr<FrameOrWorkerScheduler>
  GetFrameOrWorkerSchedulerWeakPtr() = 0;

 private:
  class ObserverState {
   public:
    ObserverState(ObserverType, OnLifecycleStateChangedCallback);
    ObserverState(const ObserverState&) = delete;
    ObserverState& operator=(const ObserverState&) = delete;
    ~ObserverState();

    ObserverType GetObserverType() const { return observer_type_; }
    OnLifecycleStateChangedCallback& GetCallback() { return callback_; }

   private:
    ObserverType observer_type_;
    OnLifecycleStateChangedCallback callback_;
  };

  void RemoveLifecycleObserver(LifecycleObserverHandle* handle);

  HashMap<LifecycleObserverHandle*, std::unique_ptr<ObserverState>>
      lifecycle_observers_;
  base::WeakPtrFactory<FrameOrWorkerScheduler> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FRAME_OR_WORKER_SCHEDULER_H_
