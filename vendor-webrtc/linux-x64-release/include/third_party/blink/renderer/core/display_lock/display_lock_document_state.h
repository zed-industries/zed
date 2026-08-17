// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_DOCUMENT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_DOCUMENT_STATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/display_lock/display_lock_utilities.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class DisplayLockContext;
class Document;
class Element;
class IntersectionObserver;
class IntersectionObserverEntry;

// This class is responsible for keeping document level state for the display
// locking feature.
class CORE_EXPORT DisplayLockDocumentState final
    : public GarbageCollected<DisplayLockDocumentState> {
 public:
  explicit DisplayLockDocumentState(Document* document);

  // GC.
  void Trace(Visitor*) const;

  // Registers a display lock context with the state. This is used to force all
  // activatable locks.
  void AddDisplayLockContext(DisplayLockContext*);
  void RemoveDisplayLockContext(DisplayLockContext*);
  int DisplayLockCount() const;

  // Bookkeeping: the count of all locked display locks.
  void AddLockedDisplayLock();
  void RemoveLockedDisplayLock();
  int LockedDisplayLockCount() const;

  // Bookkeeping: the count of all locked display locks which block all
  // activation (i.e. content-visibility: hidden locks).
  void IncrementDisplayLockBlockingAllActivation();
  void DecrementDisplayLockBlockingAllActivation();
  int DisplayLockBlockingAllActivationCount() const;

  // Register the given element for intersection observation. Used for detecting
  // viewport intersections for content-visibility: auto locks.
  void RegisterDisplayLockActivationObservation(Element*);
  void UnregisterDisplayLockActivationObservation(Element*);

  // Returns true if we have activatable locks.
  // This compares LockedDisplayLockCount() and
  // DisplayLockBlockingAllActivationCount().
  bool HasActivatableLocks() const;

  // Returns true if all activatable locks have been forced.
  bool ActivatableDisplayLocksForced() const;

  // Notifications for elements entering/exiting top layer.
  void ElementAddedToTopLayer(Element*);
  void ElementRemovedFromTopLayer(Element*);

  class CORE_EXPORT ScopedForceActivatableDisplayLocks {
    STACK_ALLOCATED();

   public:
    ScopedForceActivatableDisplayLocks(ScopedForceActivatableDisplayLocks&&);
    ~ScopedForceActivatableDisplayLocks();

    ScopedForceActivatableDisplayLocks& operator=(
        ScopedForceActivatableDisplayLocks&&);

   private:
    friend DisplayLockDocumentState;
    explicit ScopedForceActivatableDisplayLocks(DisplayLockDocumentState*);

    DisplayLockDocumentState* state_;
  };

  ScopedForceActivatableDisplayLocks GetScopedForceActivatableLocks();

  // Notify the display locks that selection was removed.
  void NotifySelectionRemoved();

  // Notify the display locks that view transition pseudo elements have
  // changed.
  void NotifyViewTransitionPseudoTreeChanged();

  // Updates only the ancestor locks of the view transition elements. This is an
  // optimization to be used by the display lock context.
  void UpdateViewTransitionElementAncestorLocks();

  // This is called when the forced scope is created or destroyed in
  // |ScopedForcedUpdate::Impl|. This is used to ensure that we can create new
  // locks that are immediately forced by the existing forced scope.
  //
  // Consider the situation A -> B -> C, where C is the child node which is the
  // target of the forced lock (the parameter passed here), and B is its parent
  // and A is its grandparent. Suppose that A and B have locks, but since style
  // was blocked by A, B's lock has not been created yet. When we force the
  // update from C we call `NotifyNodeForced()`, and A's lock is forced by the
  // given |ScopedForcedUpdate::Impl|. Then we process the style and while
  // processing B's style, we find that there is a new lock there. This lock
  // needs to be forced immediately, since it is in the ancestor chain of C.
  // This is done by calling `ForceLockIfNeeded()` below, which adds B's scope
  // to the chain. At the end of the scope, everything is un-forced and
  // `EndNodeForcedScope()` is called to clean up state.
  //
  // Note that there can only be one scope created at a time, so we don't keep
  // track of more than one of these scopes. This is enforced by private access
  // modifier + friends, as well as DCHECKs.
  void BeginNodeForcedScope(
      const Node* node,
      bool self_was_forced,
      DisplayLockUtilities::ScopedForcedUpdate::Impl* chain);
  void BeginRangeForcedScope(
      const Range* range,
      DisplayLockUtilities::ScopedForcedUpdate::Impl* chain);
  void EndForcedScope(DisplayLockUtilities::ScopedForcedUpdate::Impl* chain);
  bool HasForcedScopes() const {
    return forced_node_infos_.size() > 0 || forced_range_infos_.size() > 0;
  }

  // This is called to make sure that any of the currently forced locks allow at
  // least the specified phase for updates. This is used when a scope is
  // created, for example, to update StyleAndLayoutTree, but then is upgraded to
  // update Layout instead.
  void EnsureMinimumForcedPhase(DisplayLockContext::ForcedPhase phase);

  // Forces the lock on the given element, if it isn't yet forced but appears on
  // the ancestor chain for the forced element (which was set via
  // `BeginNodeForcedScope()`).
  void ForceLockIfNeeded(Element*);

  class ForcedNodeInfo {
    DISALLOW_NEW();
   public:
    ForcedNodeInfo(const Node* node,
                   bool self_forced,
                   DisplayLockUtilities::ScopedForcedUpdate::Impl* chain)
        : chain_(chain), node_(node), self_forced_(self_forced) {}

    void ForceLockIfNeeded(Element* new_locked_element);
    DisplayLockUtilities::ScopedForcedUpdate::Impl* Chain() const {
      return chain_.Get();
    }

    void Trace(Visitor* visitor) const {
      visitor->Trace(chain_);
      visitor->Trace(node_);
    }

   private:
    Member<DisplayLockUtilities::ScopedForcedUpdate::Impl> chain_;
    Member<const Node> node_;
    bool self_forced_;
  };

  class ForcedRangeInfo {
    DISALLOW_NEW();

   public:
    ForcedRangeInfo(const Range* range,
                    DisplayLockUtilities::ScopedForcedUpdate::Impl* chain)
        : chain_(chain), range_(range) {}

    void ForceLockIfNeeded(Element* new_locked_element);
    DisplayLockUtilities::ScopedForcedUpdate::Impl* Chain() const {
      return chain_.Get();
    }

    void Trace(Visitor* visitor) const {
      visitor->Trace(chain_);
      visitor->Trace(range_);
    }

   private:
    Member<DisplayLockUtilities::ScopedForcedUpdate::Impl> chain_;
    Member<const Range> range_;
  };

  void NotifyPrintingOrPreviewChanged();

  base::TimeTicks GetLockUpdateTimestamp();

  static constexpr float kViewportMarginPercentage = 150.f;

  void IssueForcedRenderWarning(Element*);

 private:
  IntersectionObserver& EnsureIntersectionObserver();

  void ProcessDisplayLockActivationObservation(
      const HeapVector<Member<IntersectionObserverEntry>>&);

  void ScheduleAnimation();

  // Mark element's ancestor contexts as having a top layer element. If at least
  // one of the contexts skips its descendants, this return true. Otherwise, it
  // returns false.
  bool MarkAncestorContextsHaveTopLayerElement(Element*);

  Member<Document> document_;

  Member<IntersectionObserver> intersection_observer_ = nullptr;
  HeapHashSet<WeakMember<DisplayLockContext>> display_lock_contexts_;

  // Contains all of the currently forced node infos, each of which represents
  // the node that caused the scope to be created.
  HeapVector<ForcedNodeInfo> forced_node_infos_;
  HeapVector<ForcedRangeInfo> forced_range_infos_;

  int locked_display_lock_count_ = 0;
  int display_lock_blocking_all_activation_count_ = 0;

  // If greater than 0, then the activatable locks are forced.
  int activatable_display_locks_forced_ = 0;

  bool printing_ = false;

  base::TimeTicks last_lock_update_timestamp_ = base::TimeTicks();

  unsigned forced_render_warnings_ = 0;
};

}  // namespace blink

// This ensures |blink::DisplayLockDocumentState::ForcedNodeInfo| does not touch
// other on-heap objects in its destructor and so it can be cleared with memset.
// This is needed to allocate it in HeapVector directly.
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::DisplayLockDocumentState::ForcedNodeInfo)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::DisplayLockDocumentState::ForcedRangeInfo)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_DOCUMENT_STATE_H_
