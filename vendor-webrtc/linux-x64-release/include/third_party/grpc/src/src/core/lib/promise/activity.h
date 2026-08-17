// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_ACTIVITY_H
#define GRPC_SRC_CORE_LIB_PROMISE_ACTIVITY_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <algorithm>
#include <atomic>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/detail/status.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/construct_destruct.h"
#include "src/core/util/dump_args.h"
#include "src/core/util/latent_see.h"
#include "src/core/util/no_destruct.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/sync.h"

namespace grpc_core {

class Activity;

// WakeupMask is a bitfield representing which parts of an activity should be
// woken up.
using WakeupMask = uint16_t;

// A Wakeable object is used by queues to wake activities.
class Wakeable {
 public:
  // Wake up the underlying activity.
  // After calling, this Wakeable cannot be used again.
  // WakeupMask comes from the activity that created this Wakeable and specifies
  // the set of promises that should be awoken.
  virtual void Wakeup(WakeupMask wakeup_mask) = 0;
  // Per Wakeup, but guarantee that the activity will be woken up out-of-line.
  // Useful if there may be mutexes or the like held by the current thread.
  virtual void WakeupAsync(WakeupMask wakeup_mask) = 0;
  // Drop this wakeable without waking up the underlying activity.
  virtual void Drop(WakeupMask wakeup_mask) = 0;

  // Return the underlying activity debug tag, or "<unknown>" if not available.
  virtual std::string ActivityDebugTag(WakeupMask wakeup_mask) const = 0;

 protected:
  inline ~Wakeable() {}
};

namespace promise_detail {
struct Unwakeable final : public Wakeable {
  void Wakeup(WakeupMask) override {}
  void WakeupAsync(WakeupMask) override {}
  void Drop(WakeupMask) override {}
  std::string ActivityDebugTag(WakeupMask) const override;
};
static Unwakeable* unwakeable() {
  return NoDestructSingleton<Unwakeable>::Get();
}
}  // namespace promise_detail

// An owning reference to a Wakeable.
// This type is non-copyable but movable.
class Waker {
 public:
  Waker(Wakeable* wakeable, WakeupMask wakeup_mask)
      : wakeable_and_arg_{wakeable, wakeup_mask} {}
  Waker() : Waker(promise_detail::unwakeable(), 0) {}
  ~Waker() { wakeable_and_arg_.Drop(); }
  Waker(const Waker&) = delete;
  Waker& operator=(const Waker&) = delete;
  Waker(Waker&& other) noexcept : wakeable_and_arg_(other.Take()) {}
  Waker& operator=(Waker&& other) noexcept {
    std::swap(wakeable_and_arg_, other.wakeable_and_arg_);
    return *this;
  }

  // Wake the underlying activity.
  void Wakeup() { Take().Wakeup(); }

  void WakeupAsync() { Take().WakeupAsync(); }

  template <typename H>
  friend H AbslHashValue(H h, const Waker& w) {
    return H::combine(H::combine(std::move(h), w.wakeable_and_arg_.wakeable),
                      w.wakeable_and_arg_.wakeup_mask);
  }

  bool operator==(const Waker& other) const noexcept {
    return wakeable_and_arg_ == other.wakeable_and_arg_;
  }

  bool operator!=(const Waker& other) const noexcept {
    return !operator==(other);
  }

  std::string ActivityDebugTag() {
    return wakeable_and_arg_.ActivityDebugTag();
  }

  std::string DebugString() const {
    return absl::StrFormat("Waker{%p, %d}", wakeable_and_arg_.wakeable,
                           wakeable_and_arg_.wakeup_mask);
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const Waker& waker) {
    sink.Append(waker.DebugString());
  }

  // This is for tests to assert that a waker is occupied or not.
  bool is_unwakeable() const {
    return wakeable_and_arg_.wakeable == promise_detail::unwakeable();
  }

 private:
  struct WakeableAndArg {
    Wakeable* wakeable;
    WakeupMask wakeup_mask;

    void Wakeup() { wakeable->Wakeup(wakeup_mask); }
    void WakeupAsync() { wakeable->WakeupAsync(wakeup_mask); }
    void Drop() { wakeable->Drop(wakeup_mask); }
    std::string ActivityDebugTag() const {
      return wakeable == nullptr ? "<unknown>"
                                 : wakeable->ActivityDebugTag(wakeup_mask);
    }
    bool operator==(const WakeableAndArg& other) const noexcept {
      return wakeable == other.wakeable && wakeup_mask == other.wakeup_mask;
    }
  };

  WakeableAndArg Take() {
    return std::exchange(wakeable_and_arg_, {promise_detail::unwakeable(), 0});
  }

  WakeableAndArg wakeable_and_arg_;
};

// Helper type to track wakeups between objects in the same activity.
// Can be fairly fast as no ref counting or locking needs to occur.
class IntraActivityWaiter {
 public:
  // Register for wakeup, return Pending(). If state is not ready to proceed,
  // Promises should bottom out here.
  Pending pending();
  // Wake the activity
  void Wake();

  std::string DebugString() const;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const IntraActivityWaiter& waker) {
    sink.Append(waker.DebugString());
  }

 private:
  WakeupMask wakeups_ = 0;
};

// An Activity tracks execution of a single promise.
// It executes the promise under a mutex.
// When the promise stalls, it registers the containing activity to be woken up
// later.
// The activity takes a callback, which will be called exactly once with the
// result of execution.
// Activity execution may be cancelled by simply deleting the activity. In such
// a case, if execution had not already finished, the done callback would be
// called with absl::CancelledError().
class Activity : public Orphanable {
 public:
  // Force wakeup from the outside.
  // This should be rarely needed, and usages should be accompanied with a note
  // on why it's not possible to wakeup with a Waker object.
  // Nevertheless, it's sometimes useful for integrations with Activity to force
  // an Activity to repoll.
  void ForceWakeup() { MakeOwningWaker().Wakeup(); }

  // Force the current activity to immediately repoll if it doesn't complete.
  virtual void ForceImmediateRepoll(WakeupMask mask) = 0;
  // Legacy version of ForceImmediateRepoll() that uses the current participant.
  // Will go away once Party gets merged with Activity. New usage is banned.
  void ForceImmediateRepoll() { ForceImmediateRepoll(CurrentParticipant()); }

  // Return the current part of the activity as a bitmask
  virtual WakeupMask CurrentParticipant() const { return 1; }

  // Return the current activity.
  // Additionally:
  // - assert that there is a current activity (and catch bugs if there's not)
  // - indicate to thread safety analysis that the current activity is indeed
  //   locked
  // - back up that assertion with a runtime check in debug builds (it's
  //   prohibitively expensive in non-debug builds)
  static Activity* current() { return current_ref(); }

  // Produce an activity-owning Waker. The produced waker will keep the activity
  // alive until it's awoken or dropped.
  virtual Waker MakeOwningWaker() = 0;

  // Produce a non-owning Waker. The waker will own a small heap allocated weak
  // pointer to this activity. This is more suitable for wakeups that may not be
  // delivered until long after the activity should be destroyed.
  virtual Waker MakeNonOwningWaker() = 0;

  // Some descriptive text to add to log messages to identify this activity.
  virtual std::string DebugTag() const;

 protected:
  // Check if this activity is the current activity executing on the current
  // thread.
  bool is_current() const { return this == current(); }
  // Check if there is an activity executing on the current thread.
  static bool have_current() { return current() != nullptr; }
  // Set the current activity at construction, clean it up at destruction.
  class ScopedActivity {
   public:
    explicit ScopedActivity(Activity* activity) : prior_activity_(current()) {
      current_ref() = activity;
    }
    ~ScopedActivity() { current_ref() = prior_activity_; }
    ScopedActivity(const ScopedActivity&) = delete;
    ScopedActivity& operator=(const ScopedActivity&) = delete;

   private:
    Activity* const prior_activity_;
  };

 private:
  static Activity*& current_ref() {
#if !defined(_WIN32) || !defined(_DLL)
    return g_current_activity_;
#else
    // Set during RunLoop to the Activity that's executing.
    // Being set implies that mu_ is held.
    static thread_local Activity* current_activity;
    return current_activity;
#endif
  }
#if !defined(_WIN32) || !defined(_DLL)
  // Set during RunLoop to the Activity that's executing.
  // Being set implies that mu_ is held.
  static thread_local Activity* g_current_activity_;
#endif
};

// Owned pointer to one Activity.
using ActivityPtr = OrphanablePtr<Activity>;

namespace promise_detail {

template <typename Context>
class ContextHolder {
 public:
  using ContextType = Context;

  explicit ContextHolder(Context value) : value_(std::move(value)) {}
  Context* GetContext() { return &value_; }

 private:
  Context value_;
};

template <typename Context>
class ContextHolder<Context*> {
 public:
  using ContextType = Context;

  explicit ContextHolder(Context* value) : value_(value) {}
  Context* GetContext() { return value_; }

 private:
  Context* value_;
};

template <typename Context, typename Deleter>
class ContextHolder<std::unique_ptr<Context, Deleter>> {
 public:
  using ContextType = Context;

  explicit ContextHolder(std::unique_ptr<Context, Deleter> value)
      : value_(std::move(value)) {}
  Context* GetContext() { return value_.get(); }

 private:
  std::unique_ptr<Context, Deleter> value_;
};

template <typename Context>
class ContextHolder<RefCountedPtr<Context>> {
 public:
  using ContextType = Context;

  explicit ContextHolder(RefCountedPtr<Context> value)
      : value_(std::move(value)) {}
  Context* GetContext() { return value_.get(); }

 private:
  RefCountedPtr<Context> value_;
};

template <>
class Context<Activity> {
 public:
  static Activity* get() { return Activity::current(); }
};

template <typename HeldContext>
using ContextTypeFromHeld = typename ContextHolder<
    typename std::remove_reference<HeldContext>::type>::ContextType;

template <typename... Contexts>
class ActivityContexts
    : public ContextHolder<typename std::remove_reference<Contexts>::type>... {
 public:
  explicit ActivityContexts(Contexts&&... contexts)
      : ContextHolder<typename std::remove_reference<Contexts>::type>(
            std::forward<Contexts>(contexts))... {}

  class ScopedContext : public Context<ContextTypeFromHeld<Contexts>>... {
   public:
    explicit ScopedContext(ActivityContexts* contexts)
        : Context<ContextTypeFromHeld<Contexts>>(
              static_cast<ContextHolder<
                  typename std::remove_reference<Contexts>::type>*>(contexts)
                  ->GetContext())... {
      // Silence `unused-but-set-parameter` in case of Contexts = {}
      (void)contexts;
    }
  };
};

// A free standing activity: an activity that owns its own synchronization and
// memory.
// The alternative is an activity that's somehow tied into another system, for
// instance the type seen in promise_based_filter.h as we're transitioning from
// the old filter stack to the new system.
// FreestandingActivity is-a Wakeable, but needs to increment a refcount before
// returning that Wakeable interface. Additionally, we want to keep
// FreestandingActivity as small as is possible, since it will be used
// everywhere. So we use inheritance to provide the Wakeable interface: this
// makes it zero sized, and we make the inheritance private to prevent
// accidental casting.
class FreestandingActivity : public Activity, private Wakeable {
 public:
  Waker MakeOwningWaker() final {
    Ref();
    return Waker(this, 0);
  }
  Waker MakeNonOwningWaker() final;

  void Orphan() final {
    Cancel();
    Unref();
  }

  void ForceImmediateRepoll(WakeupMask) final {
    mu_.AssertHeld();
    SetActionDuringRun(ActionDuringRun::kWakeup);
  }

 protected:
  // Action received during a run, in priority order.
  // If more than one action is received during a run, we use max() to resolve
  // which one to report (so Cancel overrides Wakeup).
  enum class ActionDuringRun : uint8_t {
    kNone,    // No action occurred during run.
    kWakeup,  // A wakeup occurred during run.
    kCancel,  // Cancel was called during run.
  };

  inline ~FreestandingActivity() override {
    if (handle_) {
      DropHandle();
    }
  }

  // Check if we got an internal wakeup since the last time this function was
  // called.
  ActionDuringRun GotActionDuringRun() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    return std::exchange(action_during_run_, ActionDuringRun::kNone);
  }

  // Implementors of Wakeable::Wakeup should call this after the wakeup has
  // completed.
  void WakeupComplete() { Unref(); }

  // Set the action that occurred during this run.
  // We use max to combine actions so that cancellation overrides wakeups.
  void SetActionDuringRun(ActionDuringRun action)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    action_during_run_ = std::max(action_during_run_, action);
  }

  Mutex* mu() ABSL_LOCK_RETURNED(mu_) { return &mu_; }

  std::string ActivityDebugTag(WakeupMask) const override { return DebugTag(); }

 private:
  class Handle;

  // Cancel execution of the underlying promise.
  virtual void Cancel() = 0;

  void Ref() { refs_.fetch_add(1, std::memory_order_relaxed); }
  void Unref() {
    if (1 == refs_.fetch_sub(1, std::memory_order_acq_rel)) {
      delete this;
    }
  }

  // Return a Handle instance with a ref so that it can be stored waiting for
  // some wakeup.
  Handle* RefHandle() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
  // If our refcount is non-zero, ref and return true.
  // Otherwise, return false.
  bool RefIfNonzero();
  // Drop the (proved existing) wait handle.
  void DropHandle() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  // All promise execution occurs under this mutex.
  Mutex mu_;

  // Current refcount.
  std::atomic<uint32_t> refs_{1};
  // If wakeup is called during Promise polling, we set this to Wakeup and
  // repoll. If cancel is called during Promise polling, we set this to Cancel
  // and cancel at the end of polling.
  ActionDuringRun action_during_run_ ABSL_GUARDED_BY(mu_) =
      ActionDuringRun::kNone;
  // Handle for long waits. Allows a very small weak pointer type object to
  // queue for wakeups while Activity may be deleted earlier.
  Handle* handle_ ABSL_GUARDED_BY(mu_) = nullptr;
};

// Implementation details for an Activity of an arbitrary type of promise.
// There should exist an inner template class `BoundScheduler` that provides
// the following interface:
// struct WakeupScheduler {
//   template <typename ActivityType>
//   class BoundScheduler {
//    public:
//     BoundScheduler(WakeupScheduler);
//     void ScheduleWakeup();
//   };
// };
// The ScheduleWakeup function should arrange that
// static_cast<ActivityType*>(this)->RunScheduledWakeup() be invoked at the
// earliest opportunity.
// It can assume that activity will remain live until RunScheduledWakeup() is
// invoked, and that a given activity will not be concurrently scheduled again
// until its RunScheduledWakeup() has been invoked.
// We use private inheritance here as a way of getting private members for each
// of the contexts.
// TODO(ctiller): We can probably reconsider the private inheritance here
// when we move away from C++11 and have more powerful template features.
template <class F, class WakeupScheduler, class OnDone, typename... Contexts>
class PromiseActivity final
    : public FreestandingActivity,
      public WakeupScheduler::template BoundScheduler<
          PromiseActivity<F, WakeupScheduler, OnDone, Contexts...>>,
      private ActivityContexts<Contexts...> {
 public:
  using Factory = OncePromiseFactory<void, F>;
  using ResultType = typename Factory::Promise::Result;

  PromiseActivity(F promise_factory, WakeupScheduler wakeup_scheduler,
                  OnDone on_done, Contexts&&... contexts)
      : FreestandingActivity(),
        WakeupScheduler::template BoundScheduler<PromiseActivity>(
            std::move(wakeup_scheduler)),
        ActivityContexts<Contexts...>(std::forward<Contexts>(contexts)...),
        on_done_(std::move(on_done)) {
    // Lock, construct an initial promise from the factory, and step it.
    // This may hit a waiter, which could expose our this pointer to other
    // threads, meaning we do need to hold this mutex even though we're still
    // constructing.
    mu()->Lock();
    auto status = Start(Factory(std::move(promise_factory)));
    mu()->Unlock();
    // We may complete immediately.
    if (status.has_value()) {
      on_done_(std::move(*status));
    }
  }

  ~PromiseActivity() override {
    // We shouldn't destruct without calling Cancel() first, and that must get
    // us to be done_, so we assume that and have no logic to destruct the
    // promise here.
    CHECK(done_);
  }

  void RunScheduledWakeup() {
    CHECK(wakeup_scheduled_.exchange(false, std::memory_order_acq_rel));
    Step();
    WakeupComplete();
  }

 private:
  using typename ActivityContexts<Contexts...>::ScopedContext;

  void Cancel() final {
    if (Activity::is_current()) {
      mu()->AssertHeld();
      SetActionDuringRun(ActionDuringRun::kCancel);
      return;
    }
    bool was_done;
    {
      MutexLock lock(mu());
      // Check if we were done, and flag done.
      was_done = done_;
      if (!done_) {
        ScopedActivity scoped_activity(this);
        ScopedContext contexts(this);
        MarkDone();
      }
    }
    // If we were not done, then call the on_done callback.
    if (!was_done) {
      on_done_(absl::CancelledError());
    }
  }

  // Wakeup this activity. Arrange to poll the activity again at a convenient
  // time: this could be inline if it's deemed safe, or it could be by passing
  // the activity to an external threadpool to run. If the activity is already
  // running on this thread, a note is taken of such and the activity is
  // repolled if it doesn't complete.
  void Wakeup(WakeupMask m) final {
    // If there is an active activity, but hey it's us, flag that and we'll loop
    // in RunLoop (that's calling from above here!).
    if (Activity::is_current()) {
      mu()->AssertHeld();
      SetActionDuringRun(ActionDuringRun::kWakeup);
      WakeupComplete();
      return;
    }
    WakeupAsync(m);
  }

  void WakeupAsync(WakeupMask) final {
    GRPC_LATENT_SEE_INNER_SCOPE("PromiseActivity::WakeupAsync");
    wakeup_flow_.Begin(GRPC_LATENT_SEE_METADATA("Activity::Wakeup"));
    if (!wakeup_scheduled_.exchange(true, std::memory_order_acq_rel)) {
      // Can't safely run, so ask to run later.
      this->ScheduleWakeup();
    } else {
      // Already a wakeup scheduled for later, drop ref.
      WakeupComplete();
    }
  }

  // Drop a wakeup
  void Drop(WakeupMask) final { this->WakeupComplete(); }

  // Notification that we're no longer executing - it's ok to destruct the
  // promise.
  void MarkDone() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu()) {
    CHECK(!std::exchange(done_, true));
    ScopedContext contexts(this);
    Destruct(&promise_holder_.promise);
  }

  // In response to Wakeup, run the Promise state machine again until it
  // settles. Then check for completion, and if we have completed, call on_done.
  void Step() ABSL_LOCKS_EXCLUDED(mu()) {
    GRPC_LATENT_SEE_PARENT_SCOPE("PromiseActivity::Step");
    wakeup_flow_.End();
    // Poll the promise until things settle out under a lock.
    mu()->Lock();
    if (done_) {
      // We might get some spurious wakeups after finishing.
      mu()->Unlock();
      return;
    }
    auto status = RunStep();
    mu()->Unlock();
    if (status.has_value()) {
      on_done_(std::move(*status));
    }
  }

  // The main body of a step: set the current activity, and any contexts, and
  // then run the main polling loop. Contained in a function by itself in
  // order to keep the scoping rules a little easier in Step().
  std::optional<ResultType> RunStep() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu()) {
    ScopedActivity scoped_activity(this);
    ScopedContext contexts(this);
    return StepLoop();
  }

  // Similarly to RunStep, but additionally construct the promise from a
  // promise factory before entering the main loop. Called once from the
  // constructor.
  std::optional<ResultType> Start(Factory promise_factory)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu()) {
    ScopedActivity scoped_activity(this);
    ScopedContext contexts(this);
    Construct(&promise_holder_.promise, promise_factory.Make());
    return StepLoop();
  }

  // Until there are no wakeups from within and the promise is incomplete:
  // poll the promise.
  std::optional<ResultType> StepLoop() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu()) {
    CHECK(is_current());
    while (true) {
      // Run the promise.
      CHECK(!done_);
      auto r = promise_holder_.promise();
      if (auto* status = r.value_if_ready()) {
        // If complete, destroy the promise, flag done, and exit this loop.
        MarkDone();
        return IntoStatus(status);
      }
      // Continue looping til no wakeups occur.
      switch (GotActionDuringRun()) {
        case ActionDuringRun::kNone:
          return {};
        case ActionDuringRun::kWakeup:
          break;
        case ActionDuringRun::kCancel:
          MarkDone();
          return absl::CancelledError();
      }
    }
  }

  using Promise = typename Factory::Promise;
  // Callback on completion of the promise.
  GPR_NO_UNIQUE_ADDRESS OnDone on_done_;
  // Has execution completed?
  GPR_NO_UNIQUE_ADDRESS bool done_ ABSL_GUARDED_BY(mu()) = false;
  // Is there a wakeup scheduled?
  GPR_NO_UNIQUE_ADDRESS std::atomic<bool> wakeup_scheduled_{false};
  // We wrap the promise in a union to allow control over the construction
  // simultaneously with annotating mutex requirements and noting that the
  // promise contained may not use any memory.
  union PromiseHolder {
    PromiseHolder() {}
    ~PromiseHolder() {}
    GPR_NO_UNIQUE_ADDRESS Promise promise;
  };
  GPR_NO_UNIQUE_ADDRESS PromiseHolder promise_holder_ ABSL_GUARDED_BY(mu());
  GPR_NO_UNIQUE_ADDRESS latent_see::Flow wakeup_flow_;
};

}  // namespace promise_detail

// Given a functor that returns a promise (a promise factory), a callback for
// completion, and a callback scheduler, construct an activity.
template <typename Factory, typename WakeupScheduler, typename OnDone,
          typename... Contexts>
ActivityPtr MakeActivity(Factory promise_factory,
                         WakeupScheduler wakeup_scheduler, OnDone on_done,
                         Contexts&&... contexts) {
  return ActivityPtr(
      new promise_detail::PromiseActivity<Factory, WakeupScheduler, OnDone,
                                          Contexts...>(
          std::move(promise_factory), std::move(wakeup_scheduler),
          std::move(on_done), std::forward<Contexts>(contexts)...));
}

inline Pending IntraActivityWaiter::pending() {
  const auto new_wakeups = GetContext<Activity>()->CurrentParticipant();
  GRPC_TRACE_LOG(promise_primitives, INFO)
      << "IntraActivityWaiter::pending: "
      << GRPC_DUMP_ARGS(this, new_wakeups, wakeups_);
  wakeups_ |= new_wakeups;
  return Pending();
}

inline void IntraActivityWaiter::Wake() {
  if (wakeups_ == 0) return;
  GetContext<Activity>()->ForceImmediateRepoll(std::exchange(wakeups_, 0));
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_ACTIVITY_H
