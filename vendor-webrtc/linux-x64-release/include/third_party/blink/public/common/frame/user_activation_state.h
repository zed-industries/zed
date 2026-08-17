// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_STATE_H_

#include "base/time/time.h"
#include "base/time/time_override.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/frame/user_activation_notification_type.mojom-forward.h"

namespace blink {

#if defined(MEMORY_SANITIZER)
// MSan runs can be very slow, so don't time out user gestures quickly.  See
// https://crbug.com/1433572 for one example.
inline constexpr base::TimeDelta kActivationLifespan = base::Seconds(60);
#else
// The expiry time should be long enough to allow network round trips even in a
// very slow connection (to support xhr-like calls with user activation), yet
// not too long to make an "unattended" page feel activated.
inline constexpr base::TimeDelta kActivationLifespan = base::Seconds(5);
#endif

// This class represents the user activation state of a frame.  It maintains two
// bits of information: whether this frame has ever seen an activation in its
// lifetime (the sticky bit), and whether this frame has a current activation
// that is neither expired nor consumed (the transient bit).  See User
// Activation v2 (UAv2) links below for more info.
//
//
// Tracking User Activation across the Frame Tree
// ==============================================
//
// This state changes in three ways: activation (of both bits) through user
// input notifications, and deactivation (of transient bit only) through expiry
// and consumption.
//
// - A notification update at a frame activates all ancestor frames (sets both
// bits to true).  Also see "Same-origin Visibility" below.
//
// - A consumption call deactivates the transient bits in the whole frame tree.
// This exhaustive consumption guarantees that a malicious subframe can't embed
// sub-subframes in a way that could allow multiple consumptions per user
// activation.
//
// - After a certain time limit (few seconds), the transient bit is deactivated.
// (Internally, the class doens't store the transient bit, but stores the bit's
// expiry time instead.)
//
//
// Same-origin Visibility of User Activation
// =========================================
//
// We launched UAv2 with a relaxed visibility model that a user activation is
// visible to all same-origin frames (w.r.t. the originally-activated frame), in
// addition to the ancestor frames as per UAv2.  We will remove this relaxation
// after implementing a mechanism for activation transfer
// (https://crbug.com/928838).
//
// Details: Before UAv2, user activation was visible to all same-process frames
// and no cross-process frames (undocumented).  The ancestor-only restriction
// with UAv2 caused a few breakages because of reliance on the old assumption,
// see the Type=Bug-Regression bugs blocking the above bug.  Once we have a
// workaround for those broken cases (most likely through a postMessage
// transfer), we will go back to the ancestor-only visibility model.
//
//
// State Replication in Browser and Renderers
// ==========================================
//
// The user activation state is replicated in the browser process (in
// |RenderFrameHostImpl|) and in the renderer processes (in |LocalFrame| and
// |RemoteFrame|).  The replicated states across the browser and renderer
// processes are kept in sync as follows:
//
// [A] Consumption of activation state for popups starts in the frame tree of
// the browser process and propagate to the renderer trees through direct IPCs
// (one IPC sent to each renderer).
//
// [B] Consumption calls from JS/blink side (e.g. video picture-in-picture)
// update the originating renderer's local frame tree and send an IPC to the
// browser; the browser updates its frame tree and sends IPCs to all other
// renderers each of which then updates its local frame tree.
//
// [B'] Notification updates on user inputs is like [B] (renderer first).  These
// should really be like [A] (browser first), see https://crbug.com/848778.
//
// [C] Expiration of an active state is tracked independently in each process.
//
//
// More Info
// =========
//
// - UAv2 explainer: https://mustaqahmed.github.io/user-activation-v2
// - Main design:
//   https://docs.google.com/a/chromium.org/document/d/1erpl1yqJlc1pH0QvVVmi1s3WzqQLsEXTLLh6VuYp228
// - Browser-side replication for OOPIFs:
//   https://docs.google.com/document/d/1XL3vCedkqL65ueaGVD-kfB5RnnrnTaxLc7kmU91oerg
class BLINK_COMMON_EXPORT UserActivationState {
 public:
  UserActivationState();

  // Marks the user activation state as active, which sets the sticky state to
  // true and updates the transient state timestamp to "now".
  //
  // The |notification_type| parameter is used for histograms only.
  //
  // TODO(mustaq): When removing |notification_type|, explicitly pass
  // |is_restricted| as a parameter here.
  void Activate(mojom::UserActivationNotificationType notification_type);

  // Used when propagating user activation state across cross-process
  // navigations.
  void SetHasBeenActive();

  void Clear();

  // Returns the sticky activation state, which is |true| if the frame has ever
  // seen an activation.
  bool HasBeenActive() const;

  // Returns the transient activation state, which is |true| if the frame has
  // recently been activated and the transient state hasn't been consumed yet.
  bool IsActive() const;

  // Consumes the transient activation state if available, and returns |true| if
  // successfully consumed.
  bool ConsumeIfActive();

  // Indicates if the last user activation notification was restricted in
  // nature.  This is a non-spec-compliant state, added only for compat reasons.
  //
  // Please don't add any new dependency to it!
  //
  // More details: A user activation on a frame is marked as restricted when the
  // frame is neither an ancestor nor of the same-origin w.r.t. the frame where
  // user interaction happened.  In other words, the restricted activation does
  // not follow the tracking mechanism mentioned in the HTML spec and above.
  // This non-standard activation in Chrome prevents breaking old extensions
  // that (historically) expect a synthetic user activation to be available in
  // an "unexposed" script-context (say in an extension's background script)
  // after receiving an extension message under certain conditions.
  bool LastActivationWasRestricted() const;

  // Records UMA stats related to consumption.  Must be called:
  // - before |ConsumeIfActive()| to record correct stats, and
  // - only once during consumption propagation to suppress over-counting.
  void RecordPreconsumptionUma() const;

 private:
  void ActivateTransientState();
  void DeactivateTransientState();

  bool IsActiveInternal() const;

  mojom::UserActivationNotificationType EffectiveNotificationType() const;

  bool has_been_active_ = false;
  base::TimeTicks transient_state_expiry_time_;

  bool last_activation_was_restricted_ = false;

  // Tracks the expiry of |kInteraction| notification for UMA data.
  base::TimeTicks transient_state_expiry_time_for_interaction_;

  // Tracks the type of notification for UMA data.
  mojom::UserActivationNotificationType first_notification_type_;
  mojom::UserActivationNotificationType last_notification_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_USER_ACTIVATION_STATE_H_
