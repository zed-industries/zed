// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNIZER_H_

#include "third_party/blink/public/mojom/handwriting/handwriting.mojom-blink.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class HandwritingDrawing;
class HandwritingHints;
class ScriptState;

class HandwritingRecognizer final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HandwritingRecognizer(
      ExecutionContext* context,
      mojo::PendingRemote<handwriting::mojom::blink::HandwritingRecognizer>
          pending_remote);

  HandwritingRecognizer(const HandwritingRecognizer&) = delete;
  HandwritingRecognizer& operator=(const HandwritingRecognizer&) = delete;

  ~HandwritingRecognizer() override;

  // Used by the drawing to see if the recognizer is valid.
  bool IsValid();

  // Gets prediction, called by `HandwritingDrawing`.
  void GetPrediction(
      Vector<handwriting::mojom::blink::HandwritingStrokePtr> strokes,
      handwriting::mojom::blink::HandwritingHintsPtr hints,
      handwriting::mojom::blink::HandwritingRecognizer::GetPredictionCallback
          callback);

  // IDL Interface:
  HandwritingDrawing* startDrawing(ScriptState* script_state,
                                   const HandwritingHints* hints,
                                   ExceptionState& exception_state);
  void finish(ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override;

 private:
  // This function will invalidate the recognizer and its drawings;
  void Invalidate();

  HeapMojoRemote<handwriting::mojom::blink::HandwritingRecognizer>
      remote_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNIZER_H_
