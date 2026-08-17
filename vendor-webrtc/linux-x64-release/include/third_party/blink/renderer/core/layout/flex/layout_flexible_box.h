// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_LAYOUT_FLEXIBLE_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_LAYOUT_FLEXIBLE_BOX_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

struct DevtoolsFlexInfo;

// Devtools uses this info to highlight lines and items on its flexbox overlay.
// Devtools usually reads such info from the layout or fragment trees. But
// Layout doesn't store this flex line -> flex items hierarchy there, or
// anywhere, because neither paint nor ancestor layout needs it. So the NG flex
// layout algorithm will fill one of these in when devtools requests it.

class CORE_EXPORT LayoutFlexibleBox : public LayoutBlock {
 public:
  explicit LayoutFlexibleBox(Element*);

  bool HasTopOverflow() const override;
  bool HasLeftOverflow() const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    // This string can affect a production behavior.
    // See tool_highlight.ts in devtools-frontend.
    return "LayoutFlexibleBox";
  }

  const DevtoolsFlexInfo* FlexLayoutData() const;
  // Once this is set to true it is never set back to false. This is maybe okay,
  // but could make devtools use too much memory after a lot of flexboxes have
  // been inspected.
  void SetNeedsLayoutForDevtools();

 protected:
  bool IsChildAllowed(LayoutObject* object,
                      const ComputedStyle& style) const override;
  void RemoveChild(LayoutObject*) override;

  bool IsFlexibleBox() const final {
    NOT_DESTROYED();
    return true;
  }
};

template <>
struct DowncastTraits<LayoutFlexibleBox> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsFlexibleBox();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_LAYOUT_FLEXIBLE_BOX_H_
