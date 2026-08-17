// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_HISTORY_USER_ACTIVATION_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_HISTORY_USER_ACTIVATION_STATE_H_

#include "base/time/time.h"

namespace blink {

// Used to decide whether to allow web pages to prevent history traversal, and
// to ensure they are not doing so twice in a row without an intervening user
// activation.
// This is used for a similar purpose to the history manipulation intervention
// (src/docs/history_manipulation_intervention.md), but at a different
// point in time. The intervention is used to make history entries skippable
// when they are navigated away from e.g. by creating a new entry, while
// HistoryUserActivationState is used to determine whether the web page is
// allowed to block when traversing to an already created history entry.
// HistoryUserActivationState records activation at the same time as
// UserActivationState, but consume behaves differently.
// `HistoryUserActivationState::Consume()` is only called on the target window,
// not descendants, and is only triggered by specific APIs that block history
// traversals. Therefore there will be cases where
// `HistoryUserActivationState::IsActive()` is true but
// `UserActivationState::IsActive()` is false, and vice versa.
//
// State Replication in Browser and Renderers
// ==========================================
//
// The history user activation state is replicated in the browser process (in
// `RenderFrameHostImpl`) and in the renderer processes (in `LocalFrame`).
// The replicated states across the browser and renderer processes are kept in
// sync as follows:
//
// [A] Activation and Clear nearly identical to UserActivationState, except that
// they are not passed to RemoteFrames in the renderer.
//
// [B] Consumption of activation state already occurs in the renderer, and
// the browser is notified via mojo message (`DidConsumeHistoryUserActivation`);
class HistoryUserActivationState {
 public:
  HistoryUserActivationState() = default;
  ~HistoryUserActivationState() = default;

  void Activate() { user_activation_time_ = base::TimeTicks::Now(); }
  void Consume() { last_used_user_activation_time_ = user_activation_time_; }
  bool IsActive() const {
    return last_used_user_activation_time_ != user_activation_time_;
  }
  // Clear() is called at the same time as UserActivationState::Clear(). It does
  // the same thing as Consume(), but add this helper for clarity at the
  // callsites.
  void Clear() { Consume(); }

 private:
  base::TimeTicks user_activation_time_;
  base::TimeTicks last_used_user_activation_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_HISTORY_USER_ACTIVATION_STATE_H_
