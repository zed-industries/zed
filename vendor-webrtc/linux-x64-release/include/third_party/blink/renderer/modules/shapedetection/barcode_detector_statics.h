// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_STATICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_STATICS_H_

#include "services/shape_detection/public/mojom/barcodedetection_provider.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_barcode_format.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;

// This class owns the BarcodeDetectionProvider connection used to create the
// BarcodeDetector instances for this ExecutionContext.
class BarcodeDetectorStatics final
    : public GarbageCollected<BarcodeDetectorStatics>,
      public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  static BarcodeDetectorStatics* From(ExecutionContext*);

  explicit BarcodeDetectorStatics(ExecutionContext&);
  ~BarcodeDetectorStatics();

  void CreateBarcodeDetection(
      mojo::PendingReceiver<shape_detection::mojom::blink::BarcodeDetection>,
      shape_detection::mojom::blink::BarcodeDetectorOptionsPtr);
  ScriptPromise<IDLSequence<V8BarcodeFormat>> EnumerateSupportedFormats(
      ScriptState*);

  void Trace(Visitor*) const override;

 private:
  void EnsureServiceConnection();
  void OnEnumerateSupportedFormats(
      ScriptPromiseResolver<IDLSequence<V8BarcodeFormat>>*,
      const Vector<shape_detection::mojom::blink::BarcodeFormat>&);
  void OnConnectionError();

  HeapMojoRemote<shape_detection::mojom::blink::BarcodeDetectionProvider>
      service_;

  // Holds Promises returned by EnumerateSupportedFormats() so that they can be
  // resolve in the case of a Mojo connection error.
  HeapHashSet<Member<ScriptPromiseResolver<IDLSequence<V8BarcodeFormat>>>>
      get_supported_format_requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_BARCODE_DETECTOR_STATICS_H_
