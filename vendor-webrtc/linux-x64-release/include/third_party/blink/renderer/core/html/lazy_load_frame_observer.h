// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_FRAME_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_FRAME_OBSERVER_H_

#include <memory>

#include "base/time/time.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class IntersectionObserver;
class IntersectionObserverEntry;
class HTMLFrameOwnerElement;
class ResourceRequestHead;

class LazyLoadFrameObserver final
    : public GarbageCollected<LazyLoadFrameObserver> {
 public:
  // The loading pipeline for an iframe differs depending on whether the
  // navigation is its first, or a dynamic / subsequent one. Since the iframe
  // loading path differs, LazyLoadFrameObserver must also account for this
  // difference when loading a deferred frame. This enum helps us keep track of
  // that so we can do the right thing.
  enum class LoadType {
    kFirst,
    kSubsequent,
  };

  LazyLoadFrameObserver(HTMLFrameOwnerElement&, LoadType);
  ~LazyLoadFrameObserver();

  void DeferLoadUntilNearViewport(const ResourceRequestHead&, WebFrameLoadType);
  bool IsLazyLoadPending() const {
    return lazy_load_intersection_observer_ != nullptr;
  }
  void CancelPendingLazyLoad();

  void LoadImmediately();

  void Trace(Visitor*) const;

 private:
  struct LazyLoadRequestInfo;

  void LoadIfHiddenOrNearViewport(
      const HeapVector<Member<IntersectionObserverEntry>>&);

  const Member<HTMLFrameOwnerElement> element_;

  // The intersection observer responsible for loading the frame once it's near
  // the viewport.
  Member<IntersectionObserver> lazy_load_intersection_observer_;

  // Keeps track of the resource request and other info needed to load in the
  // deferred frame. This is only non-null if there's a lazy load pending.
  std::unique_ptr<LazyLoadRequestInfo> lazy_load_request_info_;

  LoadType load_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LAZY_LOAD_FRAME_OBSERVER_H_
