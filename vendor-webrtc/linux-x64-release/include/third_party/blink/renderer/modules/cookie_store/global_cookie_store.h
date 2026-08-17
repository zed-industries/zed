// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_GLOBAL_COOKIE_STORE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_GLOBAL_COOKIE_STORE_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CookieStore;
class LocalDOMWindow;
class ServiceWorkerGlobalScope;

// The "cookieStore" attribute on the Window global and SW global scope.
class GlobalCookieStore {
  STATIC_ONLY(GlobalCookieStore);

 public:
  static CookieStore* cookieStore(LocalDOMWindow&);
  static CookieStore* cookieStore(ServiceWorkerGlobalScope&);

  // Event listener only available on the SW global scope.
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(cookiechange, kCookiechange)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_GLOBAL_COOKIE_STORE_H_
