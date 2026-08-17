// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INL_H_

#include "third_party/blink/renderer/core/layout/layout_box_model_object.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/paint/paint_layer.h"

namespace blink {

void LayoutObject::MarkContainerChainForOverflowRecalcIfNeeded(
    bool mark_container_chain_scrollable_overflow_recalc) {
  NOT_DESTROYED();
  LayoutObject* object = this;
  do {
    // Cell and row need to propagate the flag to their containing section and
    // row as their containing block is the table wrapper.
    // This enables us to only recompute overflow the modified sections / rows.
    object = object->IsTableCell() || object->IsTableRow()
                 ? object->Parent()
                 : object->Container();
    if (object) {
      bool already_needs_scrollable_overflow_recalc = false;
      if (mark_container_chain_scrollable_overflow_recalc) {
        already_needs_scrollable_overflow_recalc =
            object->ChildNeedsScrollableOverflowRecalc();
        if (!already_needs_scrollable_overflow_recalc) {
          object->SetChildNeedsScrollableOverflowRecalc();
        }
      }

      if (object->HasLayer()) {
        auto* box_model_object = To<LayoutBoxModelObject>(object);
        if (box_model_object->HasSelfPaintingLayer()) {
          auto* layer = box_model_object->Layer();
          if (layer->NeedsVisualOverflowRecalc()) {
            if (already_needs_scrollable_overflow_recalc) {
              return;
            }
          } else {
            layer->SetNeedsVisualOverflowRecalc();
          }
        }
      }
    }

  } while (object);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INL_H_
