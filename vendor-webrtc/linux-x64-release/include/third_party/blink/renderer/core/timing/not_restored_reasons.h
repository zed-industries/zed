// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASONS_H_

#include "third_party/blink/public/mojom/back_forward_cache_not_restored_reasons.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/timing/not_restored_reason_details.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class CORE_EXPORT NotRestoredReasons : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit NotRestoredReasons(
      String src,
      String id,
      String name,
      String url,
      HeapVector<Member<NotRestoredReasonDetails>>* reasons,
      HeapVector<Member<NotRestoredReasons>>* children);

  NotRestoredReasons(const NotRestoredReasons&);

  const String src() const { return src_; }

  const String id() const { return id_; }

  const String name() const { return name_; }

  const String url() const { return url_; }

  const std::optional<HeapVector<Member<NotRestoredReasonDetails>>> reasons()
      const;

  const std::optional<HeapVector<Member<NotRestoredReasons>>> children() const;

  ScriptObject toJSON(ScriptState* script_state) const;

  void Trace(Visitor* visitor) const override;

 private:
  String src_;
  String id_;
  String name_;
  String url_;
  HeapVector<Member<NotRestoredReasonDetails>> reasons_;
  HeapVector<Member<NotRestoredReasons>> children_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_NOT_RESTORED_REASONS_H_
