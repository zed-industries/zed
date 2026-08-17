// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_IMAGE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_IMAGE_OBSERVER_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Document;
class Element;
class IntersectionObserver;
class IntersectionObserverEntry;

class LazyLoadImageObserver final
    : public GarbageCollected<LazyLoadImageObserver> {
 public:
  LazyLoadImageObserver() = default;

  void StartMonitoringNearViewport(Document*, Element*);
  void StopMonitoring(Element*);

  void Trace(Visitor*) const;

  // Loads all currently known lazy-loaded images. Returns whether any
  // resources started loading as a result.
  bool LoadAllImagesAndBlockLoadEvent(Document& for_document);

 private:
  void LoadIfNearViewport(const HeapVector<Member<IntersectionObserverEntry>>&);

  int GetLazyLoadingImageMarginPx(const Document& document);

  // The intersection observer responsible for loading the image once it's near
  // the viewport.
  Member<IntersectionObserver> lazy_load_intersection_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_IMAGE_OBSERVER_H_
