// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_CONTEXT_H_

#include <array>
#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_engine.h"
#include "third_party/blink/renderer/core/css/style_recalc_change.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;
class Element;

enum class DisplayLockActivationReason {
  // Accessibility driven activation
  kAccessibility = 1 << 0,
  // Activation as a result of find-in-page
  kFindInPage = 1 << 1,
  // Fragment link navigation
  kFragmentNavigation = 1 << 2,
  // Script invoked focus().
  kScriptFocus = 1 << 3,
  // scrollIntoView()
  kScrollIntoView = 1 << 4,
  // User / script selection
  kSelection = 1 << 5,
  // Simulated click (Node::DispatchSimulatedClick)
  kSimulatedClick = 1 << 6,
  // User focus (e.g. tab navigation)
  kUserFocus = 1 << 7,
  // Intersection observer activation
  kViewportIntersection = 1 << 8,

  // Shorthands
  kViewport = static_cast<uint16_t>(kSelection) |
              static_cast<uint16_t>(kUserFocus) |
              static_cast<uint16_t>(kViewportIntersection) |
              static_cast<uint16_t>(kAccessibility),
  kAny = static_cast<uint16_t>(kAccessibility) |
         static_cast<uint16_t>(kFindInPage) |
         static_cast<uint16_t>(kFragmentNavigation) |
         static_cast<uint16_t>(kScriptFocus) |
         static_cast<uint16_t>(kScrollIntoView) |
         static_cast<uint16_t>(kSelection) |
         static_cast<uint16_t>(kSimulatedClick) |
         static_cast<uint16_t>(kUserFocus) |
         static_cast<uint16_t>(kViewportIntersection),
};

// Instead of specifying an underlying type, which would propagate throughout
// forward declarations, we static assert that the activation reasons enum is
// small-ish.
static_assert(static_cast<uint32_t>(DisplayLockActivationReason::kAny) <
                  std::numeric_limits<uint16_t>::max(),
              "DisplayLockActivationReason is too large");

class CORE_EXPORT DisplayLockContext final
    : public GarbageCollected<DisplayLockContext>,
      public LocalFrameView::LifecycleNotificationObserver,
      public ElementRareDataField {
 public:
  // Note the order of the phases matters. Each phase implies all previous ones
  // as well.
  enum class ForcedPhase { kNone, kStyleAndLayoutTree, kLayout, kPrePaint };

  explicit DisplayLockContext(Element*);
  ~DisplayLockContext() = default;

  // Called by style to update the current state of content-visibility.
  void SetRequestedState(EContentVisibility state);
  // Called by style to adjust the element's style based on the current state.
  const ComputedStyle* AdjustElementStyle(const ComputedStyle*) const;

  // Is called by the intersection observer callback to inform us of the
  // intersection state.
  void NotifyIsIntersectingViewport();
  void NotifyIsNotIntersectingViewport();

  // Lifecycle state functions.
  ALWAYS_INLINE bool ShouldStyleChildren() const {
    // Any of the following allows style:
    // - This isn't locked.
    // - Style and layout tree are forced for this lock.
    // - This is an activatable lock and all activatable locks are forced.
    // - This is content-visibility: auto within an element with a non-none
    //   scroll-marker-group property.
    // - This is an activatable for a11y lock and a11y is enabled.
    return !is_locked_ ||
           forced_info_.is_forced(ForcedPhase::kStyleAndLayoutTree) ||
           (IsActivatable(DisplayLockActivationReason::kAny) &&
            ActivatableDisplayLocksForced()) ||
           (IsAuto() && HasScrollerWithScrollMarkerGroup()) ||
           (IsActivatable(DisplayLockActivationReason::kAccessibility) &&
            document_->ExistingAXObjectCache());
  }

  void DidStyleSelf();
  void DidStyleChildren();
  ALWAYS_INLINE bool ShouldLayoutChildren() const {
    // Any of the following allows layout:
    // - This isn't locked.
    // - Layout is forced for this lock.
    // - This is an activatable lock and all activatable locks are forced.
    // - We're doing a container query, and one of the following is true:
    //   - This is content-visibility: auto within an element with a non-none
    //     scroll-marker-group property.
    //   - This is an activatable for a11y lock and a11y is enabled.
    // TODO(400977357): Optimize layout for the scroll-marker-group cases.
    return !is_locked_ || forced_info_.is_forced(ForcedPhase::kLayout) ||
           (IsActivatable(DisplayLockActivationReason::kAny) &&
            ActivatableDisplayLocksForced()) ||
           (IsAuto() && HasScrollerWithScrollMarkerGroup()) ||
           (document_->GetStyleEngine().SkippedContainerRecalc() &&
            IsActivatable(DisplayLockActivationReason::kAccessibility) &&
            document_->ExistingAXObjectCache());
  }
  void DidLayoutChildren();
  ALWAYS_INLINE bool ShouldPrePaintChildren() const {
    return !is_locked_ || forced_info_.is_forced(ForcedPhase::kPrePaint) ||
           (IsActivatable(DisplayLockActivationReason::kAny) &&
            ActivatableDisplayLocksForced());
  }
  ALWAYS_INLINE bool ShouldPaintChildren() const { return !is_locked_; }

  // Returns true if the last style recalc traversal was blocked at this
  // element.
  bool StyleTraversalWasBlocked() const {
    return !blocked_child_recalc_change_.IsEmpty();
  }

  // Returns true if the contents of the associated element should be visible
  // from and activatable by a specified reason. Note that passing
  // kAny will return true if the lock is activatable for any
  // reason.
  ALWAYS_INLINE bool IsActivatable(DisplayLockActivationReason reason) const {
    return activatable_mask_ & static_cast<uint16_t>(reason);
  }

  // Trigger commit because of activation from tab order, url fragment,
  // find-in-page, scrolling, etc.
  // The reason is specified for metrics.
  void CommitForActivation(DisplayLockActivationReason reason);

  bool ShouldCommitForActivation(DisplayLockActivationReason reason) const;

  // Returns true if this context is locked.
  bool IsLocked() const { return is_locked_; }

  EContentVisibility GetState() { return state_; }

  // This is called when the element with which this context is associated is
  // moved to a new document. Used to listen to the lifecycle update from the
  // right document's view.
  void DidMoveToNewDocument(Document& old_document);

  // LifecycleNotificationObserver overrides.
  void WillStartLifecycleUpdate(const LocalFrameView&) override;
  void DidFinishLayout() override;

  // Inform the display lock that it prevented a style change. This is used to
  // invalidate style when we need to update it in the future.
  void NotifyChildStyleRecalcWasBlocked(const StyleRecalcChange& change) {
    blocked_child_recalc_change_ = blocked_child_recalc_change_.Combine(change);
  }

  StyleRecalcChange TakeBlockedStyleRecalcChange() {
    return std::exchange(blocked_child_recalc_change_, StyleRecalcChange());
  }

  void NotifyReattachLayoutTreeWasBlocked() {
    blocked_child_recalc_change_ =
        blocked_child_recalc_change_.ForceReattachLayoutTree();
  }

  void NotifyChildLayoutWasBlocked() { child_layout_was_blocked_ = true; }

  void NotifyCompositingDescendantDependentFlagUpdateWasBlocked() {
    needs_compositing_dependent_flag_update_ = true;
  }

  // Notify this element will be disconnected.
  void NotifyWillDisconnect();

  // Called when the element disconnects or connects.
  void ElementDisconnected();
  void ElementConnected();

  void DetachLayoutTree();

  void NotifySubtreeLostFocus();
  void NotifySubtreeGainedFocus();

  void NotifySubtreeLostSelection();
  void NotifySubtreeGainedSelection();

  void SetNeedsPrePaintSubtreeWalk(
      bool needs_effective_allowed_touch_action_update,
      bool needs_blocking_wheel_event_handler_update) {
    needs_effective_allowed_touch_action_update_ =
        needs_effective_allowed_touch_action_update;
    needs_blocking_wheel_event_handler_update_ =
        needs_blocking_wheel_event_handler_update;
    needs_prepaint_subtree_walk_ = true;
  }

  void DidForceActivatableDisplayLocks() {
    if (IsLocked() && IsActivatable(DisplayLockActivationReason::kAny)) {
      MarkForStyleRecalcIfNeeded();
      MarkForLayoutIfNeeded();
      MarkAncestorsForPrePaintIfNeeded();
    }
  }

  bool HadAnyViewportIntersectionNotifications() const {
    return had_any_viewport_intersection_notifications_;
  }

  // GC functions.
  void Trace(Visitor*) const override;

  // Debugging functions.
  String RenderAffectingStateToString() const;

  bool IsAuto() const { return state_ == EContentVisibility::kAuto; }
  bool HadLifecycleUpdateSinceLastUnlock() const {
    return had_lifecycle_update_since_last_unlock_;
  }

  // We unlock auto locks for printing, which is set here.
  void SetShouldUnlockAutoForPrint(bool);

  void SetIsHiddenUntilFoundElement(bool is_hidden_until_found) {
    is_hidden_until_found_ = is_hidden_until_found;
  }

  void SetIsDetailsSlotElement(bool is_details_slot) {
    is_details_slot_ = is_details_slot;
  }

  bool HasElement() const { return element_ != nullptr; }

  // Top layer implementation.
  void NotifyHasTopLayerElement();
  void ClearHasTopLayerElement();

  void ScheduleTopLayerCheck();

  // State control for view transition element render affecting state.
  void ResetDescendantIsViewTransitionElement();
  void SetDescendantIsViewTransitionElement();

  void SetAffectedByAnchorPositioning(bool);

  // Mark this display lock as needing to recompute whether it has anchors
  // below it that prevent it from becoming skipped.
  void SetAnchorPositioningRenderStateMayHaveChanged();

  // Computes whether there is a descendant that is the anchor target of
  // an OOF positioned element from outside the display lock's subtree.
  bool DescendantIsAnchorTargetFromOutsideDisplayLock();

  // Sets whether this lock is in a subtree of an element with a
  // scroll-marker-group non-none property. This is set after setting the
  // requested state of the lock. Note that this affects whether auto locks
  // process style and layout, but does not affect whether the context is
  // unlocked.
  void SetHasScrollerWithScrollMarkerGroup(bool flag) {
    render_affecting_state_[static_cast<int>(
        RenderAffectingState::kHasScrollerWithScrollMarkerGroup)] = flag;
  }

 private:
  // Give access to |NotifyForcedUpdateScopeStarted()| and
  // |NotifyForcedUpdateScopeEnded()|.
  friend class DisplayLockUtilities;

  // Test friends.
  friend class DisplayLockContextRenderingTest;
  friend class DisplayLockContextTest;

  // Request that this context be locked. Called when style determines that the
  // subtree rooted at this element should be skipped, unless things like
  // viewport intersection prevent it from doing so.
  void RequestLock(uint16_t activation_mask);
  // Request that this context be unlocked. Called when style determines that
  // the subtree rooted at this element should be rendered.
  void RequestUnlock();

  // Called in |DisplayLockUtilities| to notify the state of scope.
  void NotifyForcedUpdateScopeStarted(ForcedPhase phase, bool emit_warnings) {
    UpgradeForcedScope(ForcedPhase::kNone, phase, emit_warnings);
  }
  void NotifyForcedUpdateScopeEnded(ForcedPhase phase);
  void UpgradeForcedScope(ForcedPhase old_phase,
                          ForcedPhase new_phase,
                          bool emit_warnings);

  // Records the locked context counts on the document as well as context that
  // block all activation.
  void UpdateDocumentBookkeeping(bool was_locked,
                                 bool all_activation_was_blocked,
                                 bool is_locked,
                                 bool all_activation_is_blocked);

  // Set which reasons activate, as a mask of DisplayLockActivationReason enums.
  void UpdateActivationMask(uint16_t activatable_mask);

  // Clear the activated flag.
  void ResetActivation();

  // Returns true if activatable display locks are being currently forced.
  bool ActivatableDisplayLocksForced() const;

  // The following functions propagate dirty bits from the locked element up to
  // the ancestors in order to be reached, and update dirty bits for the element
  // as well if needed. They return true if the element or its subtree were
  // dirty, and false otherwise.
  bool MarkForStyleRecalcIfNeeded();
  bool MarkForLayoutIfNeeded();
  bool MarkAncestorsForPrePaintIfNeeded();
  bool MarkNeedsRepaintAndPaintArtifactCompositorUpdate();
  bool MarkNeedsCullRectUpdate();
  bool MarkForCompositingUpdatesIfNeeded();

  bool IsElementDirtyForStyleRecalc() const;
  bool IsElementDirtyForLayout() const;
  bool IsElementDirtyForPrePaint() const;

  // Helper to schedule an animation to delay lifecycle updates for the next
  // frame.
  void ScheduleAnimation();

  // Checks whether we should force unlock the lock (due to not meeting
  // containment/display requirements), returns a string from rejection_names
  // if we should, nullptr if not. Note that this can only be called if the
  // style is clean. It checks the layout object if it exists. Otherwise,
  // falls back to checking computed style.
  const char* ShouldForceUnlock() const;

  // Unlocks the lock if the element doesn't meet requirements
  // (containment/display type). Returns true if we did unlock.
  bool ForceUnlockIfNeeded();

  // Returns true if the element is connected to a document that has a view.
  // If we're not connected,  or if we're connected but the document doesn't
  // have a view (e.g. templates) we shouldn't do style calculations etc and
  // when acquiring this lock should immediately resolve the acquire promise.
  bool ConnectedToView() const;

  // Registers or unregisters the element for intersection observations in the
  // document. This is used to activate on visibily changes. This can be safely
  // called even if changes are not required, since it will only act if a
  // register/unregister is required.
  void UpdateActivationObservationIfNeeded();

  // Determines whether or not we need lifecycle notifications.
  bool NeedsLifecycleNotifications() const;
  // Updates the lifecycle notification registration based on whether we need
  // the notifications.
  void UpdateLifecycleNotificationRegistration();

  // Locks the context.
  void Lock();
  // Unlocks the context.
  void Unlock();

  // Determines if the subtree has focus. This is a linear walk from the focused
  // element to its root element.
  void DetermineIfSubtreeHasFocus();

  // Determines if the subtree has selection. This will walk from each of the
  // selected notes up to its root looking for `element_`.
  void DetermineIfSubtreeHasSelection();

  // Determines if the subtree has a top layer element. This is a walk from each
  // top layer node up the ancestor chain looking for `element_`.
  void DetermineIfSubtreeHasTopLayerElement();

  // Determines if there are view transition elements in the subtree of this
  // element.
  void DetermineIfDescendantIsViewTransitionElement();

  // Detaching the layout tree from the top layers nested under this lock.
  void DetachDescendantTopLayerElements();

  // Keep this context unlocked until the beginning of lifecycle. Effectively
  // keeps this context unlocked for the next `count` frames. It also schedules
  // a frame to ensure the lifecycle happens. Only affects locks with 'auto'
  // setting.
  void SetKeepUnlockedUntilLifecycleCount(int count);

  // Returns true if the context can dirty element's style in the current
  // processing. Note that this returns false if the document is doing a style
  // recalc, or if we're currently setting a new requested state which happens
  // in style adjustment.
  bool CanDirtyStyle() const;

  // When a scroller becomes locked, we store off its current scroll offset, to
  // avoid losing the offset when the scroller becomes unlocked in the future.
  // The following functions enable this functionality.
  void StashScrollOffsetIfAvailable();
  void RestoreScrollOffsetIfStashed();
  bool HasStashedScrollOffset() const;

  bool SubtreeHasTopLayerElement() const;

  void ScheduleStateChangeEventIfNeeded();
  void DispatchStateChangeEventIfNeeded();

  // Returns true if this lock is within an element with a non-none
  // scroll-marker-group property.
  ALWAYS_INLINE bool HasScrollerWithScrollMarkerGroup() const {
    return render_affecting_state_[static_cast<int>(
        RenderAffectingState::kHasScrollerWithScrollMarkerGroup)];
  }

  WeakMember<Element> element_;
  WeakMember<Document> document_;
  EContentVisibility state_ = EContentVisibility::kVisible;

  // A struct to keep track of forced unlocks, and reasons for it.
  struct UpdateForcedInfo {
    bool is_forced(ForcedPhase phase) const {
      switch (phase) {
        case ForcedPhase::kNone:
          NOTREACHED();
        case ForcedPhase::kStyleAndLayoutTree:
          return style_update_forced_ || layout_update_forced_ ||
                 prepaint_update_forced_;
        case ForcedPhase::kLayout:
          return layout_update_forced_ || prepaint_update_forced_;
        case ForcedPhase::kPrePaint:
          return prepaint_update_forced_;
      }
    }

    void start(ForcedPhase phase) {
      switch (phase) {
        case ForcedPhase::kNone:
          break;
        case ForcedPhase::kStyleAndLayoutTree:
          ++style_update_forced_;
          break;
        case ForcedPhase::kLayout:
          ++layout_update_forced_;
          break;
        case ForcedPhase::kPrePaint:
          ++prepaint_update_forced_;
      }
    }

    void end(ForcedPhase phase) {
      switch (phase) {
        case ForcedPhase::kNone:
          break;
        case ForcedPhase::kStyleAndLayoutTree:
          DCHECK(style_update_forced_);
          --style_update_forced_;
          break;
        case ForcedPhase::kLayout:
          DCHECK(layout_update_forced_);
          --layout_update_forced_;
          break;
        case ForcedPhase::kPrePaint:
          DCHECK(prepaint_update_forced_);
          --prepaint_update_forced_;
      }
    }

   private:
    // Each of the forced modes includes forcing phases before. For instance,
    // layout_update_forced_ == 1 would also ensure that style and layout tree
    // are up to date.
    int style_update_forced_ = 0;
    int layout_update_forced_ = 0;
    int prepaint_update_forced_ = 0;
  };

  UpdateForcedInfo forced_info_;

  StyleRecalcChange blocked_child_recalc_change_;

  bool needs_effective_allowed_touch_action_update_ = false;
  bool needs_blocking_wheel_event_handler_update_ = false;
  bool needs_prepaint_subtree_walk_ = false;
  bool needs_compositing_dependent_flag_update_ = false;

  // Will be true if child traversal was blocked on a previous layout run on the
  // locked element. We need to keep track of this to ensure that on the next
  // layout run where the descendants of the locked element are allowed to be
  // traversed into, we will traverse to the children of the locked element.
  bool child_layout_was_blocked_ = false;

  // Tracks whether the element associated with this lock is being tracked by a
  // document level intersection observer.
  bool is_observed_ = false;

  uint16_t activatable_mask_ =
      static_cast<uint16_t>(DisplayLockActivationReason::kAny);

  // Is set to true if we are registered for lifecycle notifications.
  bool is_registered_for_lifecycle_notifications_ = false;

  // This is set to true when we have delayed locking ourselves due to viewport
  // intersection (or lack thereof) because we were nested in a locked subtree.
  // In that case, we register for lifecycle notifications and check every time
  // if we are still nested.
  bool needs_deferred_not_intersecting_signal_ = false;

  // Lock has been requested.
  bool is_locked_ = false;

  // If true, this lock is kept unlocked at least until the beginning of the
  // lifecycle. If nothing else is keeping it unlocked, then it will be locked
  // again at the start of the lifecycle.
  bool keep_unlocked_until_lifecycle_ = false;

  // This is set to true if we're in the 'auto' mode and had our first
  // intersection / non-intersection notification. This is reset to false if the
  // 'auto' mode is added again (after being removed).
  bool had_any_viewport_intersection_notifications_ = false;

  enum class RenderAffectingState : int {
    kLockRequested,
    kIntersectsViewport,
    kSubtreeHasFocus,
    kSubtreeHasSelection,
    kAutoStateUnlockedUntilLifecycle,
    kAutoUnlockedForPrint,
    kSubtreeHasTopLayerElement,
    kDescendantIsViewTransitionElement,
    kDescendantIsAnchorTarget,
    kHasScrollerWithScrollMarkerGroup,
    kNumRenderAffectingStates
  };
  void SetRenderAffectingState(RenderAffectingState state, bool flag);
  void NotifyRenderAffectingStateChanged();
  const char* RenderAffectingStateName(int state) const;

  std::array<bool,
             static_cast<int>(RenderAffectingState::kNumRenderAffectingStates)>
      render_affecting_state_ = {false};
  int keep_unlocked_count_ = 0;

  bool had_lifecycle_update_since_last_unlock_ = false;

  // Tracks whether we're updating requested state, which can only happen from
  // the style adjuster. Note that this is different from a InStyleRecalc check
  // since we can also force update style outside of this call (via ensure
  // computed style).
  bool set_requested_state_scope_ = false;

  std::optional<ScrollOffset> stashed_scroll_offset_;

  // When we use content-visibility:hidden for the <details> element's content
  // slot or the hidden=until-found attribute, then this lock must activate
  // during find-in-page.
  bool is_details_slot_ = false;

  // When an element has the hidden=until-found attribute, it gets the a
  // presentational style of content-visibility:hidden, and we also want to
  // activate this lock during find-in-page.
  bool is_hidden_until_found_ = false;

  // If we have pending subtree checks, it means we should check for selection
  // and focus at the start of the next frame.
  bool has_pending_subtree_checks_ = false;

  // If true, we need to clear the fact that we have a top layer at the start of
  // the next frame.
  bool has_pending_clear_has_top_layer_ = false;

  // If true, we need to check if this subtree has any top layer elements at the
  // start of the next frame.
  bool has_pending_top_layer_check_ = false;

  // This is set to the last value for which ContentVisibilityAutoStateChange
  // event has been dispatched (if any).
  std::optional<bool> last_notified_skipped_state_;

  // If true, there is a pending task that will dispatch a state change event if
  // needed.
  bool state_change_task_pending_ = false;

  // True if this lock needs to recompute whether kDescendantIsAnchorTarget
  // applies. If so, after layout is complete it's necessary to actually
  // compute whether that is the case.
  bool anchor_positioning_render_state_may_have_changed_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_DISPLAY_LOCK_CONTEXT_H_
