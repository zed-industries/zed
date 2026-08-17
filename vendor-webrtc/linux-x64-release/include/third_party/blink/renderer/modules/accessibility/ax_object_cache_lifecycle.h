// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_LIFECYCLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_LIFECYCLE_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// AXObjectCacheLifecycle describes which step of the a11y tree update algorithm
// is currently running. It borrows concepts from DocumentLifecycle. Benefits:
// - Ensure that code paths do not attempt to access data, such as layout or
//   style, when it is unsafe to do so or when the data would not be correct.
// - Ensure completeness of generated data, such as the tree structure and
//   cached properties, as appropriate for the given stage.

class AXObjectCacheLifecycle {
  DISALLOW_NEW();

 public:
  enum LifecycleState {
    kUninitialized,

    // When the AXObjectCache is active, it traverses these states.

    // Listen for changes to DOM and layout, and defer/queue future work to the
    // two tree update callback queues (main & popup document). During this
    // time:
    // - DocumentLifecycle-dependent data (in particular the flat tree, style
    // and layout) cannot be read as it is not clean.
    // - Other DOM methods can be utilized
    // - AX tree updates can be deferred/queued (they cannot after this state)
    kDeferTreeUpdates,

    // Process tree updates according to their TreeUpdateReason and target,
    // updating the tree structure and cached values, while queuing up dirty
    // objects and events for the serializer.
    // The DocumentLifecycle is clean at this time and for later states in the
    // AXObjectCacheLifecycle.
    kProcessDeferredUpdates,

    // This ensures that the the tree model is final: the structure is up to
    // date, all objects have updated cached values, and no nodes are orphaned.
    // AX tree updates queued in the kDeferTreeUpdates state must all have been
    // run earlier.
    kFinalizingTree,

    // Prepare a set of AXTreeUpdates containing AXNodeData for all nodes
    // updated since the last serialization, as well as events. As the tree is
    // now up-to-date and frozen, no AXObject creation or cached value updates
    // are allowed during this time.
    kSerialize,

    // Begin AXObjectCacheImpl::Dispose().
    kDisposing,

    // Dispose() is complete.
    kDisposed,
  };

  AXObjectCacheLifecycle() = default;
  AXObjectCacheLifecycle(const AXObjectCacheLifecycle&) = delete;
  AXObjectCacheLifecycle& operator=(const AXObjectCacheLifecycle&) = delete;

  bool IsActive() const {
    return state_ > kUninitialized && state_ < kDisposing;
  }
  LifecycleState GetState() const { return state_; }

  bool StateAllowsDeferTreeUpdates() const;
  bool StateAllowsImmediateTreeUpdates() const;
  bool StateAllowsRemovingAXObjects() const;
  bool StateAllowsReparentingAXObjects() const;
  bool StateAllowsSerialization() const;
  bool StateAllowsAXObjectsToBeDirtied() const;
  bool StateAllowsAXObjectsToGainFinalizationNeededBit() const;
  bool StateAllowsQueueingEventsForSerialization() const;
  bool StateAllowsQueueingAXObjectsForSerialization() const;

  void AdvanceTo(LifecycleState);
  void EnsureStateAtMost(LifecycleState);

  WTF::String ToString() const;

#if DCHECK_IS_ON()
  bool CanAdvanceTo(LifecycleState) const;
  bool CanRewindTo(LifecycleState) const;
#endif

  LifecycleState state_ = kUninitialized;
};

inline bool AXObjectCacheLifecycle::StateAllowsDeferTreeUpdates() const {
  return state_ == kDeferTreeUpdates;
}

inline bool AXObjectCacheLifecycle::StateAllowsImmediateTreeUpdates() const {
  return state_ == kProcessDeferredUpdates || state_ == kFinalizingTree;
}

inline bool AXObjectCacheLifecycle::StateAllowsRemovingAXObjects() const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates ||
         state_ == kFinalizingTree || state_ == kDisposing;
}

inline bool AXObjectCacheLifecycle::StateAllowsReparentingAXObjects() const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates;
}

inline bool AXObjectCacheLifecycle::StateAllowsSerialization() const {
  return state_ == kSerialize;
}

inline bool AXObjectCacheLifecycle::StateAllowsAXObjectsToBeDirtied() const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates ||
         state_ == kFinalizingTree;
}

inline bool
AXObjectCacheLifecycle::StateAllowsAXObjectsToGainFinalizationNeededBit()
    const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates;
}

inline bool AXObjectCacheLifecycle::StateAllowsQueueingEventsForSerialization()
    const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates;
}

inline bool
AXObjectCacheLifecycle::StateAllowsQueueingAXObjectsForSerialization() const {
  return state_ == kDeferTreeUpdates || state_ == kProcessDeferredUpdates ||
         state_ == kFinalizingTree;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_LIFECYCLE_H_
