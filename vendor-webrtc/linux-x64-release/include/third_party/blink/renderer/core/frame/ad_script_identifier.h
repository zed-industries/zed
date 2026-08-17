// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_SCRIPT_IDENTIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_SCRIPT_IDENTIFIER_H_

#include "base/containers/span.h"
#include "base/hash/hash.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "v8/include/v8-inspector.h"

namespace blink {

// Used to uniquely identify ad script on the stack.
struct CORE_EXPORT AdScriptIdentifier {
  static constexpr int kEmptyId = -1;

  // Creates an empty/unspecified identifier.
  AdScriptIdentifier();

  AdScriptIdentifier(const v8_inspector::V8DebuggerId& context_id, int id);

  bool operator==(const AdScriptIdentifier& other) const;

  // v8's debugging id for the v8::Context.
  v8_inspector::V8DebuggerId context_id;

  // The script's v8 identifier.
  int id;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::AdScriptIdentifier>
    : GenericHashTraits<blink::AdScriptIdentifier> {
  static unsigned GetHash(const blink::AdScriptIdentifier& script_id) {
    std::pair<int64_t, int64_t> p = script_id.context_id.pair();
    int64_t arr[] = {p.first, p.second, script_id.id};
    return base::FastHash(base::as_byte_span(arr));
  }

  static void ConstructDeletedValue(blink::AdScriptIdentifier& script_id) {
    script_id = blink::AdScriptIdentifier();
  }

  static bool IsDeletedValue(const blink::AdScriptIdentifier& script_id) {
    return script_id.id == blink::AdScriptIdentifier::kEmptyId;
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_AD_SCRIPT_IDENTIFIER_H_
