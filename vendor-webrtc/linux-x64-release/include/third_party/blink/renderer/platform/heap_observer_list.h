// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_OBSERVER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_OBSERVER_LIST_H_

#include "base/auto_reset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

// A list of observers. Ensures list is not mutated while iterating. Observers
// are not retained by the list. The implementation favors performance of
// iteration instead of modifications.
template <class ObserverType>
class PLATFORM_EXPORT HeapObserverList final {
  DISALLOW_NEW();

 public:
  // Add an observer to this list. An observer must not be added to the same
  // list more than once. Adding an observer is O(n).
  void AddObserver(ObserverType* observer) {
    CHECK(mutation_state_ & kAllowAddition);
    DCHECK(!HasObserver(observer));
    observers_.push_back(observer);
    if (check_capacity_) [[unlikely]] {
      // On first addition after GC, check whether we need to shrink to a
      // reasoanble capacity.
      observers_.ShrinkToReasonableCapacity();
      check_capacity_ = false;
    }
  }

  // Removes the given observer from this list. Does nothing if this observer is
  // not in this list. Removing an observer is O(n).
  void RemoveObserver(ObserverType* observer) {
    CHECK(mutation_state_ & kAllowRemoval);
    const auto it = std::find(observers_.begin(), observers_.end(), observer);
    if (it != observers_.end()) {
      observers_.erase(it);
    }
  }

  // Determine whether a particular observer is in the list. Checking
  // containment is O(n).
  bool HasObserver(ObserverType* observer) const {
    DCHECK(!IsIteratingOverObservers());
    return observers_.Contains(observer);
  }

  // Returns true if the list is being iterated over.
  bool IsIteratingOverObservers() const { return mutation_state_ != kAllowAll; }

  // Removes all the observers from this list.
  void Clear() {
    CHECK(mutation_state_ & kAllowRemoval);
    observers_.clear();
  }

  // Iterate over the registered lifecycle observers in an unpredictable order.
  //
  // Adding or removing observers is not allowed during iteration. The callable
  // will be called synchronously inside `ForEachObserver()`.
  //
  // Sample usage:
  //     ForEachObserver([](ObserverType* observer) {
  //       observer->SomeMethod();
  //     });
  template <typename ForEachCallable>
  void ForEachObserver(const ForEachCallable& callable) const {
    base::AutoReset<MutationState> scope(&mutation_state_, kNoMutationAllowed);
    for (ObserverType* observer : observers_) {
      // See CleanupDeadObservers() for why we can receive nullptr here.
      if (observer) {
        callable(observer);
      }
    }
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(observers_);
    visitor->RegisterWeakCallbackMethod<
        HeapObserverList, &HeapObserverList::CleanupDeadObservers>(this);
  }

 private:
  // HeapVector doesn't allow WeakMember at this point. Instead, use a
  // UntracedMember with a custom weak callback.
  using ObserverList = HeapVector<UntracedMember<ObserverType>>;

  void CleanupDeadObservers(const LivenessBroker& broker) {
    // This method must not allocate.

    // The GC currently does not strongify UntracedMember, even during
    // iteration. This implies that we must not move around items as using the
    // erase/remove_if pattern may move a valid object in a slot where we just
    // retrieved a nullptr.
    if (mutation_state_ == kNoMutationAllowed) {
      for (auto& observer : observers_) {
        if (!broker.IsHeapObjectAlive(observer)) {
          observer.Clear();
        }
      }
    } else {
      // We are not iterating, so we we can prepare the backing store by moving
      // dead slots to the end. Backing store reallocations will happen during
      // addition and removal.
      observers_.erase(
          std::remove_if(observers_.begin(), observers_.end(),
                         [broker](auto& observer) {
                           return !broker.IsHeapObjectAlive(observer);
                         }),
          observers_.end());
      check_capacity_ = true;
    }
  }

  // TODO(keishi): Clean up iteration state once transition from
  // LifecycleObserver is complete.
  enum MutationState {
    kNoMutationAllowed = 0,
    kAllowAddition = 1,
    kAllowRemoval = 1 << 1,
    kAllowAll = kAllowAddition | kAllowRemoval,
  };

  // MutationState records whether mutations are allowed to the set of
  // observers. Iteration e.g. prohibits any mutations.
  mutable MutationState mutation_state_ = kAllowAll;
  bool check_capacity_ = false;
  ObserverList observers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_OBSERVER_LIST_H_
