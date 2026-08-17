// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_picture_in_picture_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/picture_in_picture/picture_in_picture_window.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// An PictureInPictureEvent is a subclass of Event with an additional
// attribute that points to the Picture-in-Picture window.
class MODULES_EXPORT PictureInPictureEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PictureInPictureEvent* Create(const AtomicString&,
                                       PictureInPictureWindow*);
  static PictureInPictureEvent* Create(const AtomicString&,
                                       const PictureInPictureEventInit*);

  PictureInPictureEvent(AtomicString const&, PictureInPictureWindow*);
  PictureInPictureEvent(AtomicString const&, const PictureInPictureEventInit*);

  PictureInPictureWindow* pictureInPictureWindow() const;

  void Trace(Visitor*) const override;

 private:
  Member<PictureInPictureWindow> picture_in_picture_window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_EVENT_H_
