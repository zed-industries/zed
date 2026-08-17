// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASON_DETAILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASON_DETAILS_H_

#include "third_party/blink/public/mojom/back_forward_cache_not_restored_reasons.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class CORE_EXPORT NotRestoredReasonDetails : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit NotRestoredReasonDetails(String reason);
  NotRestoredReasonDetails(const NotRestoredReasonDetails&);

  const String reason() { return reason_; }

  ScriptObject toJSON(ScriptState* script_state) const;

 private:
  String reason_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASON_DETAILS_H_
