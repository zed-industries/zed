// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_document_picture_in_picture_event_init.h"
#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// A DocumentPictureInPictureEvent is a subclass of Event with an additional
// attribute that points to a Document Picture-in-Picture window.
class MODULES_EXPORT DocumentPictureInPictureEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DocumentPictureInPictureEvent* Create(const AtomicString&, DOMWindow*);
  static DocumentPictureInPictureEvent* Create(
      const AtomicString&,
      const DocumentPictureInPictureEventInit*);

  DocumentPictureInPictureEvent(AtomicString const&, DOMWindow*);
  DocumentPictureInPictureEvent(AtomicString const&,
                                const DocumentPictureInPictureEventInit*);

  DOMWindow* window() const;

  void Trace(Visitor*) const override;

 private:
  Member<DOMWindow> document_picture_in_picture_window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_EVENT_H_
