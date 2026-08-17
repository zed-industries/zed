// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_FRAGMENT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_FRAGMENT_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/fragment_directive/selector_fragment_anchor.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrame;
class KURL;
class Element;

// This class is responsible for finding the selector directive part of the
// directive string in fragment if there are any, and scrolling it into the
// middle of the viewport.
// https://github.com/WICG/scroll-to-text-fragment/blob/main/EXTENSIONS.md#proposed-solution
class CORE_EXPORT CssSelectorFragmentAnchor final
    : public SelectorFragmentAnchor {
 public:
  static CssSelectorFragmentAnchor* TryCreate(const KURL& url,
                                              LocalFrame& frame,
                                              bool should_scroll);

  CssSelectorFragmentAnchor(Element& anchor_node,
                            LocalFrame& frame,
                            bool should_scroll);
  CssSelectorFragmentAnchor(const CssSelectorFragmentAnchor&) = delete;
  CssSelectorFragmentAnchor& operator=(const CssSelectorFragmentAnchor&) =
      delete;
  ~CssSelectorFragmentAnchor() override = default;

  void Installed() override;

  bool InvokeSelector() override;

  void Trace(Visitor*) const override;

  bool IsSelectorFragmentAnchor() override { return true; }

 private:
  Member<Element> anchor_node_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_FRAGMENT_ANCHOR_H_
