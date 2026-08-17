// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TESTING_INTERNALS_STORAGE_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TESTING_INTERNALS_STORAGE_ACCESS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExceptionState;
class Internals;
class ScriptState;

class InternalsStorageAccess {
  STATIC_ONLY(InternalsStorageAccess);

 public:
  static ScriptPromise<IDLUndefined> setStorageAccess(
      ScriptState*,
      Internals&,
      const WTF::String& origin,
      const WTF::String& embedding_origin,
      const bool blocked,
      ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TESTING_INTERNALS_STORAGE_ACCESS_H_
