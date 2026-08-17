// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_CHILD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_CHILD_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/css/cssom/prepopulated_computed_style_property_map.h"
#include "third_party/blink/renderer/core/layout/layout_input_node.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CSSLayoutDefinition;
class CustomIntrinsicSizes;
class CustomLayoutConstraintsOptions;
class CustomLayoutFragment;
class CustomLayoutToken;
class ExceptionState;

// Represents a "CSS box" for use by a web developer. This is passed into the
// web developer defined layout and intrinsicSizes functions so that they can
// perform layout on these children.
//
// The represent all inflow children, out-of-flow children (fixed/absolute) do
// not appear in the children list.
class CustomLayoutChild : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CustomLayoutChild(const CSSLayoutDefinition&, LayoutInputNode);
  CustomLayoutChild(const CustomLayoutChild&) = delete;
  CustomLayoutChild& operator=(const CustomLayoutChild&) = delete;
  ~CustomLayoutChild() override = default;

  // LayoutChild.idl
  PrepopulatedComputedStylePropertyMap* styleMap() const {
    return style_map_.Get();
  }
  ScriptPromise<CustomIntrinsicSizes> intrinsicSizes(ScriptState*,
                                                     ExceptionState&);
  ScriptPromise<CustomLayoutFragment> layoutNextFragment(
      ScriptState*,
      const CustomLayoutConstraintsOptions*,
      ExceptionState&);

  const LayoutInputNode& GetLayoutNode() const {
    DCHECK(node_);
    return node_;
  }
  void ClearLayoutNode() { node_ = nullptr; }

  void SetCustomLayoutToken(CustomLayoutToken* token) { token_ = token; }

  void Trace(Visitor*) const override;

 private:
  LayoutInputNode node_;
  Member<PrepopulatedComputedStylePropertyMap> style_map_;
  Member<CustomLayoutToken> token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_CHILD_H_
