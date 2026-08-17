// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ELEMENT_FRAGMENT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ELEMENT_FRAGMENT_ANCHOR_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/scrolling/fragment_anchor.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LocalFrame;
class KURL;
class Node;

// An element fragment anchor is a FragmentAnchor based on a single element.
// This is the traditional fragment anchor of the web. For example, the fragment
// string will be used to find an element in the page with a matching id and use
// that to scroll into view and focus.
//
// While the page is loading, the fragment anchor tries to repeatedly scroll
// the element into view since it's position may change as a result of layouts.
// TODO(bokan): Maybe we no longer need the repeated scrolling since that
// should be handled by scroll-anchoring?
class CORE_EXPORT ElementFragmentAnchor final : public FragmentAnchor {
 public:
  // Parses the URL fragment and, if possible, creates and returns a fragment
  // based on an Element in the page. Returns nullptr otherwise. Produces side
  // effects related to fragment targeting in the page in either case.
  static ElementFragmentAnchor* TryCreate(const KURL& url,
                                          LocalFrame& frame,
                                          bool should_scroll);

  ElementFragmentAnchor(Node& anchor_node, LocalFrame& frame);
  ElementFragmentAnchor(const ElementFragmentAnchor&) = delete;
  ElementFragmentAnchor& operator=(const ElementFragmentAnchor&) = delete;
  ~ElementFragmentAnchor() override = default;

  // Will attempt to scroll the anchor into view.
  bool Invoke() override;

  // Will attempt to focus the anchor.
  void Installed() override;

  // Used to let the anchor know the frame's been scrolled and so we should
  // abort keeping the fragment target in view to avoid fighting with user
  // scrolls.
  void DidScroll(mojom::blink::ScrollType type) override;

  void Trace(Visitor*) const override;

 private:
  FRIEND_TEST_ALL_PREFIXES(ElementFragmentAnchorTest,
                           AnchorRemovedBeforeBeginFrameCrash);

  void ApplyFocusIfNeeded();

  WeakMember<Node> anchor_node_;
  bool needs_focus_;

  // While this is true, the fragment is still "active" in the sense that we
  // want the owner to continue calling Invoke(). Once this is false, calling
  // Invoke has no effect and the fragment can be disposed (unless focus is
  // still needed).
  bool needs_invoke_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ELEMENT_FRAGMENT_ANCHOR_H_
