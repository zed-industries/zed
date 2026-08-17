// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_media_query_list_event_init.h"
#include "third_party/blink/renderer/core/css/media_query_list.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_interface_names.h"

namespace blink {

class MediaQueryListEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaQueryListEvent* Create(
      const AtomicString& event_type,
      const MediaQueryListEventInit* initializer) {
    return MakeGarbageCollected<MediaQueryListEvent>(event_type, initializer);
  }

  MediaQueryListEvent(const String& media, bool matches)
      : Event(event_type_names::kChange, Bubbles::kNo, Cancelable::kNo),
        media_(media),
        matches_(matches) {}

  explicit MediaQueryListEvent(MediaQueryList* list)
      : Event(event_type_names::kChange, Bubbles::kNo, Cancelable::kNo),
        media_query_list_(list),
        matches_(false) {}

  MediaQueryListEvent(const AtomicString& event_type,
                      const MediaQueryListEventInit* initializer)
      : Event(event_type, initializer), matches_(false) {
    if (initializer->hasMedia()) {
      media_ = initializer->media();
    }
    if (initializer->hasMatches()) {
      matches_ = initializer->matches();
    }
  }

  String media() const {
    return media_query_list_ ? media_query_list_->media() : media_;
  }
  bool matches() const {
    return media_query_list_ ? media_query_list_->matches() : matches_;
  }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kMediaQueryListEvent;
  }

  // beforeprint/afterprint events need to be dispatched while the execution
  // context is paused.  When printing, window.print() invoked by beforeprint/
  // afterprint event listeners should have no effect, hence the event dispatch
  // needs to be done during the pause.
  // Accordingly, MediaQueryListEvent is also expected to be dispatched while
  // printing.
  bool ShouldDispatchEvenWhenExecutionContextIsPaused() const override {
    // TODO(thestig,yukishiino): Probably it's better to return true only when
    // we're actually printing.  It's possible that execution contexts are
    // paused for other reasons (e.g. other modal dialogs).
    return true;
  }

  void Trace(Visitor* visitor) const override {
    Event::Trace(visitor);
    visitor->Trace(media_query_list_);
  }

 private:
  // We have media_/matches_ for JS-created events; we use media_query_list_
  // for events that blink generates.
  Member<MediaQueryList> media_query_list_;
  String media_;
  bool matches_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_EVENT_H_
