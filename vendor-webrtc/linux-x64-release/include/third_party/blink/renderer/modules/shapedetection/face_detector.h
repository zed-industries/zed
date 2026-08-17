// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_FACE_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_FACE_DETECTOR_H_

#include "services/shape_detection/public/mojom/facedetection.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class DetectedFace;
class ExecutionContext;
class FaceDetectorOptions;

class MODULES_EXPORT FaceDetector final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static FaceDetector* Create(ExecutionContext*, const FaceDetectorOptions*);

  FaceDetector(ExecutionContext*, const FaceDetectorOptions*);
  ~FaceDetector() override = default;

  ScriptPromise<IDLSequence<DetectedFace>> detect(
      ScriptState* script_state,
      const V8ImageBitmapSource* image_source,
      ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  void OnDetectFaces(
      ScriptPromiseResolver<IDLSequence<DetectedFace>>*,
      Vector<shape_detection::mojom::blink::FaceDetectionResultPtr>);
  void OnFaceServiceConnectionError();

  HeapMojoRemote<shape_detection::mojom::blink::FaceDetection> face_service_;

  HeapHashSet<Member<ScriptPromiseResolver<IDLSequence<DetectedFace>>>>
      face_service_requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_FACE_DETECTOR_H_
