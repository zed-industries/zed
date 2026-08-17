// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_COOKIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_COOKIES_H_

#include "services/network/public/mojom/cookie_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_internal_cookie.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_internal_cookie_same_site.h"
#include "v8/include/v8-isolate.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {

InternalCookie* CookieMojomToInternalCookie(
    const network::mojom::blink::CookieWithAccessResultPtr& cookie,
    v8::Isolate* isolate);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_COOKIES_H_
