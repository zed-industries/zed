// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_DRAWING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_DRAWING_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_handwriting_hints.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class ExecutionContext;
class HandwritingPrediction;
class HandwritingRecognizer;
class HandwritingStroke;
class ScriptState;

class HandwritingDrawing final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HandwritingDrawing(ExecutionContext* context,
                              HandwritingRecognizer* recognizer,
                              const HandwritingHints* hints);

  HandwritingDrawing(const HandwritingDrawing&) = delete;
  HandwritingDrawing& operator=(const HandwritingDrawing&) = delete;

  ~HandwritingDrawing() override;

  // IDL Interface:
  void addStroke(HandwritingStroke* stroke);
  void removeStroke(const HandwritingStroke* stroke);
  void clear();
  ScriptPromise<IDLSequence<HandwritingPrediction>> getPrediction(
      ScriptState* script_state);
  const HeapVector<Member<HandwritingStroke>>& getStrokes();

  void Trace(Visitor* visitor) const override;

 private:
  bool IsValid() const;

  Member<const HandwritingHints> hints_;

  HeapVector<Member<HandwritingStroke>> strokes_;

  WeakMember<HandwritingRecognizer> recognizer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_DRAWING_H_
