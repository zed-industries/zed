// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_REVEAL_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_REVEAL_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class DOMViewTransition;
class PageRevealEventInit;

// Implementation for the pagereveal event. Fired before the first
// rendering update after a Document is activated (loaded, restored from
// BFCache, prerender activated).
// TODO(bokan): Update spec link once it's settled.
// https://drafts.csswg.org/css-view-transitions-2/#reveal-event
class PageRevealEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PageRevealEvent();
  PageRevealEvent(const AtomicString&, const PageRevealEventInit*);
  ~PageRevealEvent() override;

  static PageRevealEvent* Create(const AtomicString& type,
                                 const PageRevealEventInit* initializer) {
    return MakeGarbageCollected<PageRevealEvent>(type, initializer);
  }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

  DOMViewTransition* viewTransition() const;
  void SetViewTransition(DOMViewTransition*);

 private:
  Member<DOMViewTransition> dom_view_transition_;
};

template <>
struct DowncastTraits<PageRevealEvent> {
  static bool AllowFrom(const Event& event) {
    return event.type() == event_type_names::kPagereveal;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_REVEAL_EVENT_H_
