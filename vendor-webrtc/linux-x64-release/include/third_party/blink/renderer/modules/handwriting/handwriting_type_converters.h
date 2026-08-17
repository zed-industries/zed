// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_TYPE_CONVERTERS_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/handwriting/handwriting.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {
class HandwritingDrawingSegment;
class HandwritingHints;
class HandwritingPoint;
class HandwritingPrediction;
class HandwritingSegment;
class HandwritingStroke;
class HandwritingRecognizerQueryResult;
class HandwritingModelConstraint;
}  // namespace blink

namespace mojo {

// Converters from IDL to Mojo.

template <>
struct MODULES_EXPORT
    TypeConverter<handwriting::mojom::blink::HandwritingPointPtr,
                  blink::HandwritingPoint*> {
  static handwriting::mojom::blink::HandwritingPointPtr Convert(
      const blink::HandwritingPoint* input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<handwriting::mojom::blink::HandwritingStrokePtr,
                  blink::HandwritingStroke*> {
  static handwriting::mojom::blink::HandwritingStrokePtr Convert(
      const blink::HandwritingStroke* input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<handwriting::mojom::blink::HandwritingHintsPtr,
                  blink::HandwritingHints*> {
  static handwriting::mojom::blink::HandwritingHintsPtr Convert(
      const blink::HandwritingHints* input);
};

// Converters from Mojo to IDL.

template <>
struct MODULES_EXPORT
    TypeConverter<blink::HandwritingPoint*,
                  handwriting::mojom::blink::HandwritingPointPtr> {
  static blink::HandwritingPoint* Convert(
      const handwriting::mojom::blink::HandwritingPointPtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::HandwritingStroke*,
                  handwriting::mojom::blink::HandwritingStrokePtr> {
  static blink::HandwritingStroke* Convert(
      const handwriting::mojom::blink::HandwritingStrokePtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::HandwritingDrawingSegment*,
                  handwriting::mojom::blink::HandwritingDrawingSegmentPtr> {
  static blink::HandwritingDrawingSegment* Convert(
      const handwriting::mojom::blink::HandwritingDrawingSegmentPtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::HandwritingSegment*,
                  handwriting::mojom::blink::HandwritingSegmentPtr> {
  static blink::HandwritingSegment* Convert(
      const handwriting::mojom::blink::HandwritingSegmentPtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::HandwritingPrediction*,
                  handwriting::mojom::blink::HandwritingPredictionPtr> {
  static blink::HandwritingPrediction* Convert(
      const handwriting::mojom::blink::HandwritingPredictionPtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<handwriting::mojom::blink::HandwritingModelConstraintPtr,
                  blink::HandwritingModelConstraint*> {
  static handwriting::mojom::blink::HandwritingModelConstraintPtr Convert(
      const blink::HandwritingModelConstraint* input);
};

template <>
struct MODULES_EXPORT TypeConverter<
    blink::HandwritingRecognizerQueryResult*,
    handwriting::mojom::blink::QueryHandwritingRecognizerResultPtr> {
  static blink::HandwritingRecognizerQueryResult* Convert(
      const handwriting::mojom::blink::QueryHandwritingRecognizerResultPtr&
          input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HANDWRITING_HANDWRITING_TYPE_CONVERTERS_H_
