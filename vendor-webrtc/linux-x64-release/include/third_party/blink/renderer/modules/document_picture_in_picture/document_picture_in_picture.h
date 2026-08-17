// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class DocumentPictureInPictureOptions;
class DOMWindow;
class ExceptionState;
class LocalDOMWindow;
class ScriptState;

class MODULES_EXPORT DocumentPictureInPicture
    : public EventTarget,
      public Supplement<LocalDOMWindow> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DocumentPictureInPicture* From(LocalDOMWindow& window);

  explicit DocumentPictureInPicture(LocalDOMWindow&);

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  ScriptPromise<DOMWindow> requestWindow(ScriptState*,
                                         DocumentPictureInPictureOptions*,
                                         ExceptionState&);

  DOMWindow* window(ScriptState*) const;

  static DocumentPictureInPicture* documentPictureInPicture(LocalDOMWindow&);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(enter, kEnter)

  static const char kSupplementName[];

  void Trace(Visitor*) const override;

 protected:
  // EventTarget implementation.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_DOCUMENT_PICTURE_IN_PICTURE_H_
