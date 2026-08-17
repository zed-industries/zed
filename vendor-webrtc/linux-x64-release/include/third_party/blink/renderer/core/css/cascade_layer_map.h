// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/active_style_sheets.h"
#include "third_party/blink/renderer/core/css/cascade_layer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

// Gathers cascade layers from all style sheets in a tree scope, sorts them
// into the cascade layer ordering as per spec, and creates a mapping from
// layers in each sheet to the sorted layer order number.
class CORE_EXPORT CascadeLayerMap : public GarbageCollected<CascadeLayerMap> {
 public:
  static constexpr uint16_t kImplicitOuterLayerOrder =
      std::numeric_limits<uint16_t>::max();

  CascadeLayerMap(const ActiveStyleSheetVector& sheets);

  uint16_t GetLayerOrder(const CascadeLayer& layer) const {
    return layer_order_map_.at(&layer);
  }

  // Compare the layer orders of two CascadeLayer objects, possibly from
  // different sheets. Callers may pass nullptr to represent the implicit outer
  // layer.
  int CompareLayerOrder(const CascadeLayer* lhs, const CascadeLayer* rhs) const;

  void Trace(blink::Visitor*) const;

  const CascadeLayer* GetRootLayer() const;

 private:
  Member<const CascadeLayer> canonical_root_layer_;
  HeapHashMap<Member<const CascadeLayer>, uint16_t> layer_order_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_MAP_H_
