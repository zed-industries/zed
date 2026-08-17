// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_IMAGE_RESOURCE_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_IMAGE_RESOURCE_TYPE_CONVERTERS_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/common/manifest/manifest.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class ManifestImageResource;
MODULES_EXPORT Manifest::ImageResource ConvertManifestImageResource(
    const ManifestImageResource* image_resource);

}  // namespace blink

namespace mojo {

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::ManifestImageResourcePtr,
                  blink::ManifestImageResource*> {
  static blink::mojom::blink::ManifestImageResourcePtr Convert(
      const blink::ManifestImageResource* image_resource);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_IMAGE_RESOURCE_TYPE_CONVERTERS_H_
