// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_FRAGMENT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_FRAGMENT_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
class Element;
class KURL;
class LocalFrame;
class ScrollIntoViewOptions;

// This class is an interface for the concept of a "fragment anchor". A
// fragment anchor allows a page to link to a specific part of a page by using
// the URL fragment. The fragment is the part after the '#' character. E.g.
// navigating to www.example.com/index.html#section3 will find the element with
// id "section3" and focus and scroll it into view.
//
// This class provides an interface that different types of fragment anchors
// can implement, allowing fragments to specify different kinds of anchors.
// Callers should use the TryCreate static method to create and return the
// appropriate type of base class.
class CORE_EXPORT FragmentAnchor : public GarbageCollected<FragmentAnchor> {
 public:
  // Parses the fragment string and tries to create a FragmentAnchor object of
  // the appropriate derived type. If no anchor could be created from the given
  // url, this returns nullptr. In either case, side-effects on the document
  // will be performed, for example, setting/clearing :target and svgView().
  static FragmentAnchor* TryCreate(const KURL& url,
                                   LocalFrame& frame,
                                   bool should_scroll);

  explicit FragmentAnchor(LocalFrame& frame) : frame_(frame) {}
  FragmentAnchor() = default;
  virtual ~FragmentAnchor() = default;

  // Invoking the fragment anchor scrolls it into view and performs any other
  // desired actions. This is called repeatedly during loading as the lifecycle
  // is updated to keep the element in view. If true, the anchor should be kept
  // alive and invoked again. Otherwise it may be disposed.
  virtual bool Invoke() = 0;

  // This should be called when the anchor is "installed". In other words, when
  // the caller receives the FragmentAnchor and stores it. This allows the
  // anchor to perform some initialization.
  virtual void Installed() = 0;

  virtual void DidScroll(mojom::blink::ScrollType type) = 0;

  virtual void Trace(Visitor*) const;

  virtual bool IsTextFragmentAnchor() { return false; }

  virtual bool IsSelectorFragmentAnchor() { return false; }

  virtual void ScrollElementIntoViewWithOptions(Element* element_to_scroll,
                                                ScrollIntoViewOptions* options);

  // Called when additional searchable content may have become available in the
  // document.
  virtual void NewContentMayBeAvailable() {}

  Member<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_FRAGMENT_ANCHOR_H_
