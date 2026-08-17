// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_STROKE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_STROKE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class HandwritingPoint;

class MODULES_EXPORT HandwritingStroke final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HandwritingStroke();

  HandwritingStroke(const HandwritingStroke&) = delete;
  HandwritingStroke& operator=(const HandwritingStroke&) = delete;

  ~HandwritingStroke() override;

  static HandwritingStroke* Create();

  // IDL Interface:
  void addPoint(const HandwritingPoint* point);
  const HeapVector<Member<const HandwritingPoint>>& getPoints() const;
  void clear();

  void Trace(Visitor* visitor) const override;

 private:
  HeapVector<Member<const HandwritingPoint>> points_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_STROKE_H_
