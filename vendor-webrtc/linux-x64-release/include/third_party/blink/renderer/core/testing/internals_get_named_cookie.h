// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_NAMED_COOKIE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_NAMED_COOKIE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {
class InternalCookie;
class Internals;
class ScriptState;

class InternalsGetNamedCookie {
  STATIC_ONLY(InternalsGetNamedCookie);

 public:
  static ScriptPromise<IDLNullable<InternalCookie>> getNamedCookie(
      ScriptState* script_state,
      Internals& internals,
      const WTF::String& name);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_GET_NAMED_COOKIE_H_
