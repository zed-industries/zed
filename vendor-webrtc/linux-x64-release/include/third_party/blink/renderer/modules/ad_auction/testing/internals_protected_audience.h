// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_TESTING_INTERNALS_PROTECTED_AUDIENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_TESTING_INTERNALS_PROTECTED_AUDIENCE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class ScriptState;

class InternalsProtectedAudience {
  STATIC_ONLY(InternalsProtectedAudience);

 public:
  static ScriptPromise<IDLUndefined> setProtectedAudienceKAnonymity(
      ScriptState* script_state,
      Internals&,
      const String& owner_origin_str,
      const String& name,
      const Vector<String>& hashes_base64);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_TESTING_INTERNALS_PROTECTED_AUDIENCE_H_
