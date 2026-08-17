// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SVG_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SVG_RESOURCE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_resource.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class SVGResource;
class SVGResourceClient;

class StyleSVGResource : public GarbageCollected<StyleSVGResource> {
 public:
  StyleSVGResource(SVGResource* resource, const AtomicString& url);
  CORE_EXPORT ~StyleSVGResource();

  void Trace(Visitor* visitor) const { visitor->Trace(resource_); }

  bool operator==(const StyleSVGResource& other) const {
    return resource_.Get() == other.resource_.Get();
  }

  void AddClient(SVGResourceClient& client);
  void RemoveClient(SVGResourceClient& client);

  SVGResource* Resource() const { return resource_.Get(); }
  const AtomicString& Url() const { return url_; }

 private:
  Member<SVGResource> resource_;
  const AtomicString url_;

  StyleSVGResource(const StyleSVGResource&) = delete;
  StyleSVGResource& operator=(const StyleSVGResource&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SVG_RESOURCE_H_
