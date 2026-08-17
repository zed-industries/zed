// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Mapping from one layer to another (obviously). This is used in two places:
//
//  - When building superrulesets, we merge the RuleSets' layers
//    to new CascadeLayer objects in the superruleset. Normally,
//    we also map values in the RuleSet::Intervals, but occasionally,
//    we need to look up @page rule etc. in the original RuleSets
//    (which are not mapped), so we need to also be able to look up
//    by the old layers, so we store and use the mapping.
//
//  - When building CascadeLayerMap (cascade_layer_map.h), we similarly combine
//    layers from all active RuleSets (the superruleset's layers
//    will be used in place of the layers of all RuleSets it is
//    subsuming), into one grouping so give them a canonical numbering.
//    For clarity, we use the typedef CanonicalLayerMap there.
using LayerMap = HeapHashMap<Member<const CascadeLayer>, Member<CascadeLayer>>;

// A CascadeLayer object represents a node in the ordered tree of cascade layers
// in the sorted layer ordering.
// https://www.w3.org/TR/css-cascade-5/#layer-ordering
class CORE_EXPORT CascadeLayer final : public GarbageCollected<CascadeLayer> {
 public:
  explicit CascadeLayer(const AtomicString& name = g_empty_atom)
      : name_(name) {}
  ~CascadeLayer() = default;

  const AtomicString& GetName() const { return name_; }
  const HeapVector<Member<CascadeLayer>>& GetDirectSubLayers() const {
    return direct_sub_layers_;
  }

  // Getting or setting the order of a layer is only valid for canonical cascade
  // layers i.e. the unique layer representation for a particular tree scope.
  const std::optional<uint16_t> GetOrder() const { return order_; }
  void SetOrder(uint16_t order) { order_ = order; }

  CascadeLayer* GetOrAddSubLayer(const StyleRuleBase::LayerName& name);

  // Recursive merge, used during creation of superrulesets.
  // The hash set gets filled/appended with a map from the old to the new
  // layers, where applicable (no sub-CascadeLayer objects from “other”
  // are ever reused, so that they are unchanged even after future merges).
  //
  // This merges only the sub-layer structure and creates the mapping;
  // it does not touch order_, which is updated during creation of the
  // CascadeLayerMap.
  void Merge(const CascadeLayer& other, LayerMap& mapping);

  void Trace(blink::Visitor*) const;

 private:
  friend class CascadeLayerTest;
  friend class RuleSetCascadeLayerTest;

  String ToStringForTesting() const;
  void ToStringInternal(StringBuilder&, const String&) const;

  CascadeLayer* FindDirectSubLayer(const AtomicString&) const;
  void ComputeLayerOrderInternal(unsigned* next);

  std::optional<uint16_t> order_;
  AtomicString name_;
  HeapVector<Member<CascadeLayer>> direct_sub_layers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CASCADE_LAYER_H_
