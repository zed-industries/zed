// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_DELETE_ALL_COOKIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_DELETE_ALL_COOKIES_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Internals;
class ScriptState;

class InternalsDeleteAllCookies {
  STATIC_ONLY(InternalsDeleteAllCookies);

 public:
  static ScriptPromise<IDLUndefined> deleteAllCookies(ScriptState* script_state,
                                                      Internals& internals);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNALS_DELETE_ALL_COOKIES_H_
