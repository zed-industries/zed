// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_CHANGE_EVENT_H_

#include <utility>

#include "net/cookies/canonical_cookie.h"
#include "services/network/public/mojom/cookie_manager.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_cookie_list_item.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace net {
class CanonicalCookie;
}  // namespace net

namespace blink {

class CookieChangeEventInit;

class CookieChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CookieChangeEvent* Create() {
    return MakeGarbageCollected<CookieChangeEvent>();
  }

  // Used by Blink.
  //
  // The caller is expected to create HeapVectors and std::move() them into this
  // method.
  static CookieChangeEvent* Create(const AtomicString& type,
                                   HeapVector<Member<CookieListItem>> changed,
                                   HeapVector<Member<CookieListItem>> deleted) {
    return MakeGarbageCollected<CookieChangeEvent>(type, std::move(changed),
                                                   std::move(deleted));
  }

  // Used by JavaScript, via the V8 bindings.
  static CookieChangeEvent* Create(const AtomicString& type,
                                   const CookieChangeEventInit* initializer) {
    return MakeGarbageCollected<CookieChangeEvent>(type, initializer);
  }

  CookieChangeEvent();
  CookieChangeEvent(const AtomicString& type,
                    HeapVector<Member<CookieListItem>> changed,
                    HeapVector<Member<CookieListItem>> deleted);
  CookieChangeEvent(const AtomicString& type,
                    const CookieChangeEventInit* initializer);
  ~CookieChangeEvent() override;

  const HeapVector<Member<CookieListItem>>& changed() const { return changed_; }
  const HeapVector<Member<CookieListItem>>& deleted() const { return deleted_; }

  // Event
  const AtomicString& InterfaceName() const override;

  // GarbageCollected
  void Trace(Visitor*) const override;

  static CookieListItem* ToCookieListItem(
      const net::CanonicalCookie& canonical_cookie,
      const network::mojom::blink::CookieEffectiveSameSite&,
      bool is_deleted);  // True for information from a cookie deletion event.

  // Helper for converting backend event information into a CookieChangeEvent.
  static void ToEventInfo(
      const network::mojom::blink::CookieChangeInfoPtr& change_info,
      HeapVector<Member<CookieListItem>>& changed,
      HeapVector<Member<CookieListItem>>& deleted);

 private:
  HeapVector<Member<CookieListItem>> changed_;
  HeapVector<Member<CookieListItem>> deleted_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_CHANGE_EVENT_H_
