/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_LIFECYCLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_LIFECYCLE_H_

#include "base/auto_reset.h"
#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

#if DCHECK_IS_ON()
namespace WTF {
class String;
}  // namespace WTF
#endif

namespace blink {

class CORE_EXPORT DocumentLifecycle {
  DISALLOW_NEW();

 public:
  enum LifecycleState {
    kUninitialized,
    kInactive,

    // When the document is active, it traverses these states.

    kVisualUpdatePending,

    kInStyleRecalc,
    kStyleClean,

    kInPerformLayout,
    kAfterPerformLayout,
    kLayoutClean,

    kInCompositingInputsUpdate,
    kCompositingInputsClean,

    // In InPrePaint step, any data needed by painting are prepared.
    // Paint property trees are built and paint invalidations are issued.
    kInPrePaint,
    kPrePaintClean,

    // In InPaint step, paint artifacts are generated and raster invalidations
    // are issued.
    // In CAP, composited layers are generated/updated.
    kInPaint,
    kPaintClean,

    // Once the document starts shutting down, we cannot return
    // to the style/layout/compositing states.
    kStopping,
    kStopped,
  };

  class Scope {
    STACK_ALLOCATED();

   public:
    Scope(DocumentLifecycle&, LifecycleState final_state);
    Scope(const Scope&) = delete;
    Scope& operator=(const Scope&) = delete;
    ~Scope();

   private:
    DocumentLifecycle& lifecycle_;
    LifecycleState final_state_;
  };

  class DeprecatedTransition {
    DISALLOW_NEW();

   public:
    DeprecatedTransition(LifecycleState from, LifecycleState to);
    DeprecatedTransition(const DeprecatedTransition&) = delete;
    DeprecatedTransition& operator=(const DeprecatedTransition&) = delete;
    ~DeprecatedTransition();

    LifecycleState From() const { return from_; }
    LifecycleState To() const { return to_; }

   private:
    DeprecatedTransition* previous_;
    LifecycleState from_;
    LifecycleState to_;
  };

  // Within this scope, state transitions are not allowed.
  // Any attempts to advance or rewind will result in a DCHECK.
  class DisallowTransitionScope {
    STACK_ALLOCATED();

   public:
    explicit DisallowTransitionScope(DocumentLifecycle& document_lifecycle)
        : document_lifecycle_(document_lifecycle) {
      document_lifecycle_.IncrementNoTransitionCount();
    }
    DisallowTransitionScope(const DisallowTransitionScope&) = delete;
    DisallowTransitionScope& operator=(const DisallowTransitionScope&) = delete;

    ~DisallowTransitionScope() {
      document_lifecycle_.DecrementNoTransitionCount();
    }

   private:
    DocumentLifecycle& document_lifecycle_;
  };

  class DetachScope {
    STACK_ALLOCATED();

   public:
    explicit DetachScope(DocumentLifecycle& document_lifecycle)
        : document_lifecycle_(document_lifecycle) {
      document_lifecycle_.IncrementDetachCount();
    }
    DetachScope(const DetachScope&) = delete;
    DetachScope& operator=(const DetachScope&) = delete;

    ~DetachScope() { document_lifecycle_.DecrementDetachCount(); }

   private:
    DocumentLifecycle& document_lifecycle_;
  };

  // If we hit a devtool break point in the middle of document lifecycle, for
  // example, https://crbug.com/788219, this scope is triggered and no more
  // layout or style computation is allowed.
  // This class should never be used outside of debugging.
  class PostponeTransitionScope {
    USING_FAST_MALLOC(PostponeTransitionScope);

   public:
    explicit PostponeTransitionScope(DocumentLifecycle& document_lifecycle)
        : document_lifecycle_(document_lifecycle) {
      document_lifecycle_.SetLifecyclePostponed();
    }
    ~PostponeTransitionScope() {
      document_lifecycle_.ResetLifecyclePostponed();
    }

   private:
    DocumentLifecycle& document_lifecycle_;
  };

  class CheckNoTransitionScope {
    STACK_ALLOCATED();

   public:
    explicit CheckNoTransitionScope(DocumentLifecycle& document_lifecycle)
        : auto_reset_(&document_lifecycle.check_no_transition_, true) {}

   private:
    base::AutoReset<bool> auto_reset_;
  };

  DocumentLifecycle();
  DocumentLifecycle(const DocumentLifecycle&) = delete;
  DocumentLifecycle& operator=(const DocumentLifecycle&) = delete;

  bool IsActive() const { return state_ > kInactive && state_ < kStopping; }
  LifecycleState GetState() const { return state_; }

  bool StateAllowsTreeMutations() const;
  bool StateAllowsLayoutTreeMutations() const;
  bool StateAllowsDetach() const;

  void AdvanceTo(LifecycleState);
  void EnsureStateAtMost(LifecycleState);

  bool StateTransitionDisallowed() const { return disallow_transition_count_; }
  void IncrementNoTransitionCount() { disallow_transition_count_++; }
  void DecrementNoTransitionCount() {
    DCHECK_GT(disallow_transition_count_, 0);
    disallow_transition_count_--;
  }

  bool InDetach() const { return detach_count_; }
  void IncrementDetachCount() { detach_count_++; }
  void DecrementDetachCount() {
    DCHECK_GT(detach_count_, 0);
    detach_count_--;
  }

  bool LifecyclePostponed() const { return life_cycle_postponed_; }

#if DCHECK_IS_ON()
  WTF::String ToString() const;
#endif
 private:
  friend class PostponeTransitionScope;
  friend class CheckNoTransitionScope;
#if DCHECK_IS_ON()
  bool CanAdvanceTo(LifecycleState) const;
  bool CanRewindTo(LifecycleState) const;
#endif

  void SetLifecyclePostponed() { life_cycle_postponed_ = true; }
  void ResetLifecyclePostponed() { life_cycle_postponed_ = false; }

  LifecycleState state_;
  int detach_count_;
  int disallow_transition_count_;
  bool life_cycle_postponed_;
  bool check_no_transition_;
};

inline bool DocumentLifecycle::StateAllowsTreeMutations() const {
  // TODO: We should not allow mutations in AfterPerformLayout
  // either, but we need to fix MediaList listeners and plugins first.
  return state_ != kInStyleRecalc && state_ != kInPerformLayout &&
         state_ != kInCompositingInputsUpdate && state_ != kInPrePaint &&
         state_ != kInPaint;
}

inline bool DocumentLifecycle::StateAllowsLayoutTreeMutations() const {
  return detach_count_ || state_ == kInStyleRecalc;
}

inline bool DocumentLifecycle::StateAllowsDetach() const {
  return state_ == kVisualUpdatePending || state_ == kInStyleRecalc ||
         state_ == kStyleClean || state_ == kLayoutClean ||
         state_ == kCompositingInputsClean || state_ == kPrePaintClean ||
         state_ == kPaintClean || state_ == kStopping || state_ == kInactive;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_LIFECYCLE_H_
