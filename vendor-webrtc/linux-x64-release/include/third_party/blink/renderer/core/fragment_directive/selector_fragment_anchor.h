// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SELECTOR_FRAGMENT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SELECTOR_FRAGMENT_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/scrolling/fragment_anchor.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"

namespace blink {
class LocalFrame;

class CORE_EXPORT SelectorFragmentAnchor : public FragmentAnchor {
 public:
  explicit SelectorFragmentAnchor(LocalFrame& frame, bool should_scroll)
      : FragmentAnchor(frame), should_scroll_(should_scroll) {}
  SelectorFragmentAnchor(const SelectorFragmentAnchor&) = delete;
  SelectorFragmentAnchor& operator=(const SelectorFragmentAnchor&) = delete;
  ~SelectorFragmentAnchor() override = default;

  void DidScroll(mojom::blink::ScrollType type) override;

  bool Invoke() override;

  void Trace(Visitor*) const override;

 protected:
  // This will be invoked by Invoke() when the page is visible until the
  // fragment has been dismissed. See FragmentAnchor::Invoke for details about
  // usage.
  virtual bool InvokeSelector() = 0;

  // Whether we should scroll the anchor into view. This will be false for
  // history navigations and reloads, where we want to restore the highlight but
  // not scroll into view again.
  bool should_scroll_ = false;
  // Whether the user has scrolled the page
  bool user_scrolled_ = false;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SELECTOR_FRAGMENT_ANCHOR_H_
