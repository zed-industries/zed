// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MOBILE_METRICS_TAP_FRIENDLINESS_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MOBILE_METRICS_TAP_FRIENDLINESS_CHECKER_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class Element;
class LocalFrameView;

// Evaluates friendliness on actually tapped targets, especially friendliness on
// smart phone devices are checked. Tap events will be sent as UKM.
class CORE_EXPORT TapFriendlinessChecker
    : public GarbageCollected<TapFriendlinessChecker> {
  using PassKey = base::PassKey<TapFriendlinessChecker>;

 public:
  explicit TapFriendlinessChecker(LocalFrameView& view,
                                  base::PassKey<TapFriendlinessChecker>)
      : view_(&view) {}
  static TapFriendlinessChecker* CreateIfMobile(LocalFrameView& view);
  virtual ~TapFriendlinessChecker() = default;

  void RegisterTapEvent(Element* target);

  void Trace(Visitor* visitor) const;

 private:
  Member<LocalFrameView> view_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MOBILE_METRICS_TAP_FRIENDLINESS_CHECKER_H_
