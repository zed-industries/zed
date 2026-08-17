// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_CLIENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class QualifiedName;
class SVGFilterPrimitiveStandardAttributes;
class SVGResource;

class CORE_EXPORT SVGResourceClient : public GarbageCollectedMixin {
 public:
  virtual ~SVGResourceClient() = default;

  virtual void ResourceContentChanged(SVGResource*) = 0;

  virtual void FilterPrimitiveChanged(
      SVGResource* resource,
      SVGFilterPrimitiveStandardAttributes& primitive,
      const QualifiedName& attribute) {
    ResourceContentChanged(resource);
  }

 protected:
  SVGResourceClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_CLIENT_H_
