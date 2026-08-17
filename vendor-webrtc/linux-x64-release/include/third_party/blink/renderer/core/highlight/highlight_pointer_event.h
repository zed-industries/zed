// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_POINTER_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_POINTER_EVENT_H_

#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/events/pointer_event.h"

namespace blink {

class HighlightPointerEventInit;

class CORE_EXPORT HighlightPointerEvent : public PointerEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static HighlightPointerEvent* Create(
      const AtomicString& type,
      const HighlightPointerEventInit* initializer,
      base::TimeTicks platform_time_stamp = base::TimeTicks::Now(),
      MouseEvent::SyntheticEventType synthetic_event_type =
          kRealOrIndistinguishable,
      WebMenuSourceType menu_source_type = kMenuSourceNone) {
    return MakeGarbageCollected<HighlightPointerEvent>(
        type, initializer, platform_time_stamp, synthetic_event_type,
        menu_source_type);
  }

  explicit HighlightPointerEvent(
      const AtomicString&,
      const HighlightPointerEventInit*,
      base::TimeTicks platform_time_stamp,
      MouseEvent::SyntheticEventType synthetic_event_type,
      WebMenuSourceType menu_source_type = kMenuSourceNone);

  Range* range() const { return range_.Get(); }

  bool IsHighlightPointerEvent() const override;

  void Trace(blink::Visitor*) const override;

 private:
  Member<Range> range_;
};

template <>
struct DowncastTraits<HighlightPointerEvent> {
  static bool AllowFrom(const Event& event) {
    return event.IsHighlightPointerEvent();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_POINTER_EVENT_H_
