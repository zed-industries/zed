// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNITION_SERVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNITION_SERVICE_H_

#include "third_party/blink/public/mojom/handwriting/handwriting.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExecutionContext;
class HandwritingModelConstraint;
class HandwritingRecognizer;
class HandwritingRecognizerQueryResult;
class ScriptState;

class HandwritingRecognitionService final
    : public GarbageCollected<HandwritingRecognitionService>,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  explicit HandwritingRecognitionService(Navigator&);

  static HandwritingRecognitionService& From(Navigator&);

  // IDL Interface:
  static ScriptPromise<HandwritingRecognizer> createHandwritingRecognizer(
      ScriptState*,
      Navigator&,
      const HandwritingModelConstraint*,
      ExceptionState&);
  static ScriptPromise<IDLNullable<HandwritingRecognizerQueryResult>>
  queryHandwritingRecognizer(ScriptState*,
                             Navigator&,
                             const HandwritingModelConstraint*,
                             ExceptionState&);

  void Trace(Visitor* visitor) const override;

 private:
  // Bind the Mojo connection to browser process if needed.
  // Returns false when the execution context is not valid (e.g., the frame is
  // detached) and an exception will be thrown.
  // Otherwise returns true.
  bool BootstrapMojoConnectionIfNeeded(ScriptState*, ExceptionState&);
  ScriptPromise<HandwritingRecognizer> CreateHandwritingRecognizer(
      ScriptState*,
      const HandwritingModelConstraint*,
      ExceptionState&);

  ScriptPromise<IDLNullable<HandwritingRecognizerQueryResult>>
  QueryHandwritingRecognizer(ScriptState*,
                             const HandwritingModelConstraint* constraint,
                             ExceptionState&);

  HeapMojoRemote<handwriting::mojom::blink::HandwritingRecognitionService>
      remote_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_RECOGNITION_SERVICE_H_
