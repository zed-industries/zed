// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LAZY_IMAGE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LAZY_IMAGE_HELPER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
class Element;
class HTMLImageElement;
class LocalFrame;

// Contains helper functions to deal with the lazy loading logic of images.
class LazyImageHelper final {
  STATIC_ONLY(LazyImageHelper);

 public:
  static void StartMonitoring(Element* element);
  static void StopMonitoring(Element* element);

  static bool LoadAllImagesAndBlockLoadEvent(Document&);

  static bool ShouldDeferImageLoad(LocalFrame& frame,
                                   HTMLImageElement* html_image);

  static void RecordMetricsOnLoadFinished(HTMLImageElement* image_element);
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LAZY_IMAGE_HELPER_H_
