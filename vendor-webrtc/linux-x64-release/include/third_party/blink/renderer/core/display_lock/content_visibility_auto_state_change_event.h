// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_CONTENT_VISIBILITY_AUTO_STATE_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_CONTENT_VISIBILITY_AUTO_STATE_CHANGE_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class ContentVisibilityAutoStateChangeEventInit;

class ContentVisibilityAutoStateChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ContentVisibilityAutoStateChangeEvent* Create() {
    return MakeGarbageCollected<ContentVisibilityAutoStateChangeEvent>();
  }
  static ContentVisibilityAutoStateChangeEvent* Create(const AtomicString& type,
                                                       bool skipped) {
    return MakeGarbageCollected<ContentVisibilityAutoStateChangeEvent>(type,
                                                                       skipped);
  }
  static ContentVisibilityAutoStateChangeEvent* Create(
      const AtomicString& type,
      const ContentVisibilityAutoStateChangeEventInit* initializer) {
    return MakeGarbageCollected<ContentVisibilityAutoStateChangeEvent>(
        type, initializer);
  }

  ContentVisibilityAutoStateChangeEvent();
  ContentVisibilityAutoStateChangeEvent(const AtomicString& type, bool skipped);
  ContentVisibilityAutoStateChangeEvent(
      const AtomicString&,
      const ContentVisibilityAutoStateChangeEventInit*);
  ~ContentVisibilityAutoStateChangeEvent() override;

  bool skipped() const;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  bool skipped_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DISPLAY_LOCK_CONTENT_VISIBILITY_AUTO_STATE_CHANGE_EVENT_H_
