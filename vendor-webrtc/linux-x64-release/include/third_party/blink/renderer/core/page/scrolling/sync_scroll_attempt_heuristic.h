// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SYNC_SCROLL_ATTEMPT_HEURISTIC_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SYNC_SCROLL_ATTEMPT_HEURISTIC_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class Frame;

// SyncScrollAttemptHeuristic handles detecting cases where a page attempts to
// synchronize an effect with scrolling (eg, using JavaScript to implement a
// parallax scroller). While a vended Scope instance exists, the heuristic
// observes when script accesses scroll position and then also modifies inline
// style or scroll offset in order to estimate when script is attempting to
// synchronize content with scrolling (these style/scroll-offset updates may
// not actually be implementing a sync-scroll effect, so this can be
// inaccurate).
//
// This is complicated somewhat by rAF. It can happen that a scroll handler
// will use rAF to realize the update. Our heuristic is imprecise in this case.
// We consider it to be a sync scroll attempt if
//   a) a rAF callback was scheduled via a scroll handler and,
//   b) any rAF callback then mutates inline style or scroll position.
// I.e., we do not distinguish the rAF callback scheduled during the scroll
// handler from any other rAF callback, which can lead to incorrectly flagging a
// rAF-induced mutation as a sync scroll attempt.
//
// More precisely,
//   - A Scope |A| is created before step 6.9 of the html processing model[1]
//     and it is destroyed before step 6.10. This scope is always active.
//   - We then create a Scope |B| before step 6.14 that is destroyed before step
//     6.15 [1]. This scope is only active (i.e., listening for updates), if
//     a rAF callback was scheduled during Scope |A|'s lifetime.
// If, while Scopes |A| and |B| are active, scroll offset was accessed and
// either inline style or scroll position was modified, the heuristic considers
// this to have been an attempt at creating a scroll-synchronized effect.
//
// We only attempt to detect sync scroll attempts if the given frame is the
// outermost main frame and UKM is not recorded when the given frame is remote.
//
// [1] https://html.spec.whatwg.org/#event-loop-processing-model
//
// TODO(crbug.com/1499981): This should be removed once synchronized scrolling
// impact is understood.
class CORE_EXPORT SyncScrollAttemptHeuristic final {
  STACK_ALLOCATED();

 public:
  explicit SyncScrollAttemptHeuristic(Frame* frame);
  SyncScrollAttemptHeuristic(const SyncScrollAttemptHeuristic&) = delete;
  SyncScrollAttemptHeuristic& operator=(const SyncScrollAttemptHeuristic&) =
      delete;
  ~SyncScrollAttemptHeuristic();

  class Scope {
    STACK_ALLOCATED();

   public:
    explicit Scope(bool enable_observation);
    ~Scope();

   private:
    bool enable_observation_ = false;
  };

  [[nodiscard]] static Scope GetScrollHandlerScope();
  [[nodiscard]] static Scope GetRequestAnimationFrameScope();
  static void DidAccessScrollOffset();
  static void DidSetScrollOffset();
  static void DidSetStyle();
  static void DidRequestAnimationFrame();

 private:
  static void EnableObservation();
  static void DisableObservation();

  Frame* frame_ = nullptr;
  SyncScrollAttemptHeuristic* last_instance_ = nullptr;
  bool did_access_scroll_offset_ = false;
  bool did_set_scroll_offset_ = false;
  bool did_set_style_ = false;
  bool did_request_animation_frame_ = false;
  bool is_observing_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SYNC_SCROLL_ATTEMPT_HEURISTIC_H_
