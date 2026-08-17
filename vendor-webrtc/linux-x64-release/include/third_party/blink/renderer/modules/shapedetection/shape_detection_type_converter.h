// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_SHAPE_DETECTION_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_SHAPE_DETECTION_TYPE_CONVERTER_H_

#include "base/notreached.h"
#include "mojo/public/cpp/bindings/type_converter.h"
#include "services/shape_detection/public/mojom/facedetection.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace mojo {

// TypeConverter to translate from shape_detection::mojom::blink::LandmarkType
// to String.
template <>
struct TypeConverter<String, shape_detection::mojom::blink::LandmarkType> {
  static String Convert(shape_detection::mojom::blink::LandmarkType input) {
    switch (input) {
      case shape_detection::mojom::blink::LandmarkType::EYE:
        return "eye";
      case shape_detection::mojom::blink::LandmarkType::MOUTH:
        return "mouth";
      case shape_detection::mojom::blink::LandmarkType::NOSE:
        return "nose";
    }

    NOTREACHED();
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHAPEDETECTION_SHAPE_DETECTION_TYPE_CONVERTER_H_
