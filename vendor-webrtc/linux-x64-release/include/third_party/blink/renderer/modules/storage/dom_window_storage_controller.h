// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_CONTROLLER_H_

#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class MODULES_EXPORT DOMWindowStorageController final
    : public GarbageCollected<DOMWindowStorageController>,
      public Supplement<LocalDOMWindow>,
      public LocalDOMWindow::EventListenerObserver {
 public:
  static const char kSupplementName[];

  explicit DOMWindowStorageController(LocalDOMWindow&);
  static DOMWindowStorageController& From(LocalDOMWindow&);

  void Trace(Visitor*) const override;

  // Inherited from LocalDOMWindow::EventListenerObserver
  void DidAddEventListener(LocalDOMWindow*, const AtomicString&) override;
  void DidRemoveEventListener(LocalDOMWindow*, const AtomicString&) override {}
  void DidRemoveAllEventListeners(LocalDOMWindow*) override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_CONTROLLER_H_
