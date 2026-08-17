// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_EXTENDABLE_COOKIE_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_EXTENDABLE_COOKIE_CHANGE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_cookie_list_item.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExtendableCookieChangeEventInit;
class WaitUntilObserver;

class ExtendableCookieChangeEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Used by Blink.
  //
  // The caller is expected to create HeapVectors and std::move() them into this
  // method.
  static ExtendableCookieChangeEvent* Create(
      const AtomicString& type,
      HeapVector<Member<CookieListItem>> changed,
      HeapVector<Member<CookieListItem>> deleted,
      WaitUntilObserver* wait_until_observer) {
    return MakeGarbageCollected<ExtendableCookieChangeEvent>(
        type, std::move(changed), std::move(deleted), wait_until_observer);
  }

  // Used by JavaScript, via the V8 bindings.
  static ExtendableCookieChangeEvent* Create(
      const AtomicString& type,
      const ExtendableCookieChangeEventInit* initializer) {
    return MakeGarbageCollected<ExtendableCookieChangeEvent>(type, initializer);
  }

  ExtendableCookieChangeEvent(const AtomicString& type,
                              HeapVector<Member<CookieListItem>> changed,
                              HeapVector<Member<CookieListItem>> deleted,
                              WaitUntilObserver*);
  ExtendableCookieChangeEvent(
      const AtomicString& type,
      const ExtendableCookieChangeEventInit* initializer);
  ~ExtendableCookieChangeEvent() override;

  const HeapVector<Member<CookieListItem>>& changed() const { return changed_; }
  const HeapVector<Member<CookieListItem>>& deleted() const { return deleted_; }

  // Event
  const AtomicString& InterfaceName() const override;

  // GarbageCollected
  void Trace(Visitor*) const override;

 private:

  HeapVector<Member<CookieListItem>> changed_;
  HeapVector<Member<CookieListItem>> deleted_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_EXTENDABLE_COOKIE_CHANGE_EVENT_H_
