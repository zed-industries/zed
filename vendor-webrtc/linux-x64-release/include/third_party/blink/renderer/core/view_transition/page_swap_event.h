// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_SWAP_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_SWAP_EVENT_H_

#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
class Document;
class DOMViewTransition;
class NavigationActivation;
class PageSwapEventInit;

// Implementation for the pageswap event. Fired before the Document is hidden
// and unloaded or placed in the BFCache.
// TODO(khushalsagar): Update spec link once it's settled.
class PageSwapEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PageSwapEvent(Document&,
                   mojom::blink::PageSwapEventParamsPtr,
                   DOMViewTransition*);
  PageSwapEvent(const AtomicString&, const PageSwapEventInit*);

  static PageSwapEvent* Create(const AtomicString& type,
                               const PageSwapEventInit* initializer) {
    return MakeGarbageCollected<PageSwapEvent>(type, initializer);
  }

  ~PageSwapEvent() override;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

  DOMViewTransition* viewTransition() const;
  NavigationActivation* activation() const;

 private:
  Member<NavigationActivation> activation_;
  Member<DOMViewTransition> dom_view_transition_;
};

template <>
struct DowncastTraits<PageSwapEvent> {
  static bool AllowFrom(const Event& event) {
    return event.type() == event_type_names::kPageswap;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_PAGE_SWAP_EVENT_H_
