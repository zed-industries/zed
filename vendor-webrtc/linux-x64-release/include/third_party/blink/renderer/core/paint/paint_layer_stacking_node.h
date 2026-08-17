/*
 * Copyright (C) 2003, 2009, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Intel Corporation. All rights reserved.
 *
 * Portions are Copyright (C) 1998 Netscape Communications Corporation.
 *
 * Other contributors:
 *   Robert O'Callahan <roc+@cs.cmu.edu>
 *   David Baron <dbaron@dbaron.org>
 *   Christian Biesinger <cbiesinger@web.de>
 *   Randall Jesup <rjesup@wgate.com>
 *   Roland Mainz <roland.mainz@informatik.med.uni-giessen.de>
 *   Josh Soref <timeless@mac.com>
 *   Boris Zbarsky <bzbarsky@mit.edu>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA
 *
 * Alternatively, the contents of this file may be used under the terms
 * of either the Mozilla Public License Version 1.1, found at
 * http://www.mozilla.org/MPL/ (the "MPL") or the GNU General Public
 * License Version 2.0, found at http://www.fsf.org/copyleft/gpl.html
 * (the "GPL"), in which case the provisions of the MPL or the GPL are
 * applicable instead of those above.  If you wish to allow use of your
 * version of this file only under the terms of one of those two
 * licenses (the MPL or the GPL) and not to allow others to use your
 * version of this file under the LGPL, indicate your decision by
 * deletingthe provisions above and replace them with the notice and
 * other provisions required by the MPL or the GPL, as the case may be.
 * If you do not delete the provisions above, a recipient may use your
 * version of this file under any of the LGPL, the MPL or the GPL.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_STACKING_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_STACKING_NODE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_box_model_object.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PaintLayer;
class ComputedStyle;

// This class is only for PaintLayer, PaintLayerPaintOrderIterator and
// PaintLayerPaintOrderReverseIterator. Other classes should not use this class.
//
// PaintLayerStackingNode represents a stacked element which is either a
// stacking context or a positioned element.
// See
// https://chromium.googlesource.com/chromium/src.git/+/main/third_party/blink/renderer/core/paint/README.md
// for more details of stacked elements.
//
// Stacked elements are the basis for the CSS painting algorithm. The paint
// order is determined by walking stacked elements in an order defined by
// ‘z-index’. This walk is interleaved with non-stacked contents.
// See CSS 2.1 appendix E for the actual algorithm
// http://www.w3.org/TR/CSS21/zindex.html
// See also PaintLayerPainter (in particular paintLayerContents) for
// our implementation of the walk.
//
// Stacked elements form a subtree over the layout tree. Ideally we would want
// objects of this class to be a node in this tree but there are potential
// issues with stale pointers so we rely on PaintLayer's tree structure.
//
// This class's purpose is to represent a node in the stacked element tree
// (aka paint tree). It currently caches the z-order lists for painting and
// hit-testing.
//
// To implement any paint order iterations, use PaintLayerPaintOrderIterator and
// PaintLayerZOrderReverseIterator.
//
// We create PaintLayerStackingNode only for real stacking contexts with stacked
// children. PaintLayerPaintOrder[Reverse]Iterator can iterate normal flow
// children in paint order with or without a stacking node.
class CORE_EXPORT PaintLayerStackingNode
    : public GarbageCollected<PaintLayerStackingNode> {
 public:
  explicit PaintLayerStackingNode(PaintLayer*);
  PaintLayerStackingNode(const PaintLayerStackingNode&) = delete;
  PaintLayerStackingNode& operator=(const PaintLayerStackingNode&) = delete;
  ~PaintLayerStackingNode() = default;

  void DirtyZOrderLists();
  void UpdateZOrderLists();

  // Returns whether a relevant style changed.
  static bool StyleDidChange(PaintLayer& paint_layer,
                             const ComputedStyle* old_style);

  using PaintLayers = HeapVector<Member<PaintLayer>>;
  using GCedPaintLayers = GCedHeapVector<Member<PaintLayer>>;

  const PaintLayers& PosZOrderList() const {
    DCHECK(!z_order_lists_dirty_);
    return pos_z_order_list_;
  }
  const PaintLayers& NegZOrderList() const {
    DCHECK(!z_order_lists_dirty_);
    return neg_z_order_list_;
  }

  const GCedPaintLayers* LayersPaintingOverlayOverflowControlsAfter(
      const PaintLayer* layer) const {
    DCHECK(!z_order_lists_dirty_);
    auto it = layer_to_overlay_overflow_controls_painting_after_.find(layer);
    return it == layer_to_overlay_overflow_controls_painting_after_.end()
               ? nullptr
               : it->value.Get();
  }

  void ClearNeedsReorderOverlayOverflowControls();

  void Trace(Visitor* visitor) const;

 private:
  void RebuildZOrderLists();

  struct HighestLayers;
  void CollectLayers(PaintLayer&, HighestLayers*);

  // Holds a sorted list of all the descendant nodes within that have z-indices
  // of 0 (or is treated as 0 for positioned objects) or greater.
  PaintLayers pos_z_order_list_;
  // Holds descendants within our stacking context with negative z-indices.
  PaintLayers neg_z_order_list_;

  // Overlay overflow controls(scrollbar or resizer) need to be painted above
  // all child contents, even if the contents are stacked in a stacking context
  // which is an ancestor of the scrolling or resizing layer, for example:
  //   <div id="stacking-context" style="opacity: 0.5">
  //     <div id="other" style="position: relative; z-index: 10></div>
  //     <div id="target" style="overflow: scroll; resize: both">
  //       <div id="child" style="position: relative">CHILD</div>
  //     </div>
  //   </div>
  // and
  //   <div id="stacking-context" style="opacity: 0.5">
  //     <div id="other" style="position: relative; z-index: 10></div>
  //     <div id="target" style="overflow: scroll; position: relative">
  //       <div id="child" style="position: absolute; z-index: 5">CHILD</div>
  //     </div>
  //   </div>
  //
  // The paint order without reordering overlay overflow controls would be:
  //              stacking-context
  //                 /      |    \
  //              target  child  other
  //                |
  //    overlay overflow controls
  // where the overlay overflow controls would be painted incorrectly below
  // |child| which is the sub content of |target|.
  //
  // To paint the overlay overflow controls above all child contents, we need to
  // reorder the z-order of overlay scrollbars in the stacking context:
  //              stacking-context
  //              /      |    |   \
  //           target  child  |  other
  //                          |
  //               overlay overflow controls
  //
  // This map records the PaintLayers (the values of the map) that have overlay
  // overflow controls that should paint after the given PaintLayer (the key of
  // the map). The value of the map is a list of PaintLayers because there may
  // be more than one scrolling or resizing container in the same stacking
  // context with overlay overflow controls.
  // For the above example, this map has one entry {child: target} which means
  // that |target|'s overlay overflow controls should be painted after |child|.
  HeapHashMap<Member<const PaintLayer>, Member<GCedPaintLayers>>
      layer_to_overlay_overflow_controls_painting_after_;

  Member<PaintLayer> layer_;

  // Indicates whether the z-order lists above are dirty.
  bool z_order_lists_dirty_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_STACKING_NODE_H_
