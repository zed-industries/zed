// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IMAGE_FALLBACK_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IMAGE_FALLBACK_HELPER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class HTMLElement;
class ComputedStyleBuilder;

class HTMLImageFallbackHelper {
  STATIC_ONLY(HTMLImageFallbackHelper);

 public:
  static void CreateAltTextShadowTree(Element&);
  // Adjusts the style of the shadow host, and also produces a StyleHostData
  // object (stored on the builder) which is used by AdjustChildStyle
  // (in the .cc file) to implement custom inheritance from the host.
  static void AdjustHostStyle(HTMLElement&, ComputedStyleBuilder&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IMAGE_FALLBACK_HELPER_H_
