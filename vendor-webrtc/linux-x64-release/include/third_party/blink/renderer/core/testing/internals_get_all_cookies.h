// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_ALL_COOKIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_ALL_COOKIES_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Internals;
class InternalCookie;
class ScriptState;

class InternalsGetAllCookies {
  STATIC_ONLY(InternalsGetAllCookies);

 public:
  static ScriptPromise<IDLSequence<InternalCookie>> getAllCookies(
      ScriptState* script_state,
      Internals& internals);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_ALL_COOKIES_H_
