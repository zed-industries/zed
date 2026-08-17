// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_DESCRIPTION_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_DESCRIPTION_TYPE_CONVERTER_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/content_index/content_index.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_content_description.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace mojo {

template <>
struct MODULES_EXPORT TypeConverter<blink::mojom::blink::ContentDescriptionPtr,
                                    const blink::ContentDescription*> {
  static blink::mojom::blink::ContentDescriptionPtr Convert(
      const blink::ContentDescription* description);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::ContentDescription*,
                  blink::mojom::blink::ContentDescriptionPtr> {
  static blink::ContentDescription* Convert(
      const blink::mojom::blink::ContentDescriptionPtr& description);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_DESCRIPTION_TYPE_CONVERTER_H_
