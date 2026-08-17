// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GENERATED_CHILDREN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GENERATED_CHILDREN_H_

#include "third_party/blink/renderer/core/html/forms/html_select_element.h"
#include "third_party/blink/renderer/core/html_element_type_helpers.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"

namespace blink {

// We only create "generated" child layoutObjects like one for first-letter if:
// - the firstLetterBlock can have children in the DOM and
// - the block doesn't have any special assumption on its text children.
// This correctly prevents form controls from having such layoutObjects.
static bool CanHaveGeneratedChildren(const LayoutObject& layout_object) {
  // FIXME: LayoutMedia::layout makes assumptions about what children are
  // allowed so we can't support generated content.
  if (layout_object.IsMedia() || layout_object.IsTextControl() ||
      layout_object.IsMenuList() || layout_object.IsInputButton()) {
    if (HTMLSelectElement::CustomizableSelectEnabled(layout_object.GetNode()) &&
        layout_object.IsMenuList() &&
        To<HTMLSelectElement>(layout_object.GetNode())
            ->IsAppearanceBaseButton()) {
      // appearance:base-select <select>s should be allowed to have ::after etc.
      return true;
    }
    return false;
  }

  return layout_object.CanHaveChildren();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GENERATED_CHILDREN_H_
