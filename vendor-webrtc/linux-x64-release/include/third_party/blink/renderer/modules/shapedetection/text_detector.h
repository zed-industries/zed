// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_TEXT_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_TEXT_DETECTOR_H_

#include "services/shape_detection/public/mojom/textdetection.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class DetectedText;
class ExecutionContext;

class MODULES_EXPORT TextDetector final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextDetector* Create(ExecutionContext*);

  explicit TextDetector(ExecutionContext*);
  ~TextDetector() override = default;

  ScriptPromise<IDLSequence<DetectedText>> detect(ScriptState*,
                                                  const V8ImageBitmapSource*,
                                                  ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  void OnDetectText(
      ScriptPromiseResolver<IDLSequence<DetectedText>>*,
      Vector<shape_detection::mojom::blink::TextDetectionResultPtr>);
  void OnTextServiceConnectionError();

  HeapMojoRemote<shape_detection::mojom::blink::TextDetection> text_service_;

  HeapHashSet<Member<ScriptPromiseResolverBase>> text_service_requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_TEXT_DETECTOR_H_
