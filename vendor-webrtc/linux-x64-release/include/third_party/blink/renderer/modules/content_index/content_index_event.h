// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_content_index_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class WaitUntilObserver;

class MODULES_EXPORT ContentIndexEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ContentIndexEvent* Create(const AtomicString& type,
                                   ContentIndexEventInit* init) {
    return MakeGarbageCollected<ContentIndexEvent>(type, init,
                                                   /* observer= */ nullptr);
  }

  ContentIndexEvent(const AtomicString& type,
                    ContentIndexEventInit* init,
                    WaitUntilObserver* observer);
  ~ContentIndexEvent() override;

  // Web exposed attribute defined in the IDL file.
  const String& id() const;

  // ExtendableEvent interface.
  const AtomicString& InterfaceName() const override;

 private:
  String id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_EVENT_H_
