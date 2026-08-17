// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_H_

#include "services/shape_detection/public/mojom/barcodedetection.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_barcode_format.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class DetectedBarcode;
class ExecutionContext;
class BarcodeDetectorOptions;

class MODULES_EXPORT BarcodeDetector final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static BarcodeDetector* Create(ExecutionContext*,
                                 const BarcodeDetectorOptions*,
                                 ExceptionState& exception_state);

  // Barcode Detection API functions.
  static ScriptPromise<IDLSequence<V8BarcodeFormat>> getSupportedFormats(
      ScriptState*);

  static V8BarcodeFormat::Enum BarcodeFormatToEnum(
      const shape_detection::mojom::BarcodeFormat format);

  ScriptPromise<IDLSequence<DetectedBarcode>> detect(ScriptState*,
                                                     const V8ImageBitmapSource*,
                                                     ExceptionState&);

  explicit BarcodeDetector(ExecutionContext*,
                           const BarcodeDetectorOptions*,
                           ExceptionState&);
  ~BarcodeDetector() override = default;

  void Trace(Visitor*) const override;

 private:
  void OnDetectBarcodes(
      ScriptPromiseResolver<IDLSequence<DetectedBarcode>>*,
      Vector<shape_detection::mojom::blink::BarcodeDetectionResultPtr>);

  void OnConnectionError();

  HeapMojoRemote<shape_detection::mojom::blink::BarcodeDetection> service_;

  HeapHashSet<Member<ScriptPromiseResolverBase>> detect_requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_H_
