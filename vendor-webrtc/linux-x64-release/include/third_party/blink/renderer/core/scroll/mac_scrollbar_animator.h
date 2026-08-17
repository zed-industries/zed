// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {
class ScrollableArea;
class Scrollbar;

// This class stores the state for an individual scrollbar (in contrast with
// MacScrollbarAnimator which has state for the full ScrollableArea).
class CORE_EXPORT MacScrollbar {
 public:
  static MacScrollbar* GetForScrollbar(const Scrollbar&);
  virtual ~MacScrollbar() = default;

  virtual void SetEnabled(bool) = 0;
  virtual float GetKnobAlpha() = 0;
  virtual float GetTrackAlpha() = 0;
  virtual int GetTrackBoxWidth() = 0;
};

// This is a base class for MacScrollbarAnimatorImpl. This is required because
// mac_scrollbar_animator_impl.h has some #include that can't be included in
// most platform-agnostic code.
class CORE_EXPORT MacScrollbarAnimator
    : public GarbageCollected<MacScrollbarAnimator> {
 public:
  // Create method that returns a MacScrollAnimatorImpl when on Mac, and isn't
  // implemented on other platforms. This is a workaround to instatiating
  // ScrollAnimatorMac directly on it's callers (e.g. ScrollableArea).
  static MacScrollbarAnimator* Create(ScrollableArea*);
  virtual void Trace(Visitor* visitor) const {}

  virtual void MouseEnteredScrollbar(Scrollbar&) const = 0;
  virtual void MouseExitedScrollbar(Scrollbar&) const = 0;

  virtual void DidAddVerticalScrollbar(Scrollbar&) = 0;
  virtual void WillRemoveVerticalScrollbar(Scrollbar&) = 0;
  virtual void DidAddHorizontalScrollbar(Scrollbar&) = 0;
  virtual void WillRemoveHorizontalScrollbar(Scrollbar&) = 0;

  virtual void DidChangeUserVisibleScrollOffset(const ScrollOffset&) = 0;

  virtual void Dispose() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_H_
