// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_BEFOREUNLOAD_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_BEFOREUNLOAD_EVENT_LISTENER_H_

#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class Document;
class Event;
class ExecutionContext;

// Helper class used to setup beforeunload listener for certain documents which
// include plugins that are handled externally and need user verification before
// before closing the page.
class BeforeUnloadEventListener : public NativeEventListener {
 public:
  explicit BeforeUnloadEventListener(Document*);

  void SetShowBeforeUnloadDialog(bool show_dialog) {
    show_dialog_ = show_dialog;
  }

  void Trace(Visitor* visitor) const override;

 private:
  void Invoke(ExecutionContext*, Event* event) override;

  Member<Document> doc_;
  bool show_dialog_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_BEFOREUNLOAD_EVENT_LISTENER_H_
