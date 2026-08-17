// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_SCOPED_BLINK_AX_EVENT_INTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_SCOPED_BLINK_AX_EVENT_INTENT_H_

#include "third_party/blink/renderer/core/accessibility/blink_ax_event_intent.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Annotates all accessibility events that are raised while an instance of this
// class is alive with a specific intent or intents. Multiple instances with
// different intents could be alive at the same time.
//
// An event intent is a description of what caused an accessibility event and
// may include user actions such as paste, or page actions such as a text
// insertion. Before firing a bunch of accessibility events that have the same
// cause, create an instance of ScopedBlinkAXEventIntent. Multiple event intents
// could be in effect at the same time, such as "selection move" and "text
// insertion". As long as an instance of ScopedBlinkAXEventIntent is alive, all
// accessibility events that are raised will be annotated with the same intent
// or intents.
class CORE_EXPORT ScopedBlinkAXEventIntent final {
  STACK_ALLOCATED();

 public:
  ScopedBlinkAXEventIntent(const BlinkAXEventIntent& intent,
                           Document* document);
  ScopedBlinkAXEventIntent(const Vector<BlinkAXEventIntent>& intents,
                           Document* document);
  ~ScopedBlinkAXEventIntent();
  ScopedBlinkAXEventIntent(const ScopedBlinkAXEventIntent& intent) = delete;
  ScopedBlinkAXEventIntent& operator=(const ScopedBlinkAXEventIntent& intent) =
      delete;

  const Vector<BlinkAXEventIntent>& intents() const { return intents_; }

 private:
  Vector<BlinkAXEventIntent> intents_;

  // This class is stack allocated, and therefore no WeakPersistent handle is
  // required for |document_| and no trace method is necessary.
  Document* const document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_SCOPED_BLINK_AX_EVENT_INTENT_H_
