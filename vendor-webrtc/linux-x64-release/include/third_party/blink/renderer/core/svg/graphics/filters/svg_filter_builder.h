/*
 * Copyright (C) 2008 Alex Mathews <possessedpenguinbob@gmail.com>
 * Copyright (C) 2009 Dirk Schulze <krit@webkit.org>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FILTER_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FILTER_BUILDER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/graphics/interpolation_space.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace gfx {
class RectF;
}

namespace blink {

class Filter;
class FilterEffect;
class SVGFilterElement;
class SVGFilterPrimitiveStandardAttributes;

// A map from SVGFilterPrimitiveStandardAttributes -> FilterEffect and
// FilterEffect -> dependent (downstream) FilterEffects ("reverse DAG"). Used
// during invalidations for changes to the primitives (graph nodes).
class SVGFilterGraphNodeMap final
    : public GarbageCollected<SVGFilterGraphNodeMap> {
 public:
  SVGFilterGraphNodeMap();

  using FilterEffectSet = GCedHeapHashSet<Member<FilterEffect>>;

  void AddBuiltinEffect(FilterEffect*);
  void AddPrimitive(SVGFilterPrimitiveStandardAttributes&, FilterEffect*);

  // Required to change the attributes of a filter during an
  // SvgAttributeChanged.
  FilterEffect* EffectForElement(
      SVGFilterPrimitiveStandardAttributes& primitive) {
    auto it = effect_element_.find(&primitive);
    return it != effect_element_.end() ? it->value.Get() : nullptr;
  }

  void InvalidateDependentEffects(FilterEffect*);

  void Trace(Visitor*) const;

 private:
  FilterEffectSet& EffectReferences(FilterEffect* effect) {
    // Only allowed for effects that are part of this node map.
    DCHECK(effect_references_.Contains(effect));
    return *effect_references_.find(effect)->value;
  }

  // The value is the set of filter effects that depend on the key
  // filter effect.
  HeapHashMap<Member<FilterEffect>, Member<FilterEffectSet>> effect_references_;
  HeapHashMap<WeakMember<SVGFilterPrimitiveStandardAttributes>,
              Member<FilterEffect>>
      effect_element_;
};

class SVGFilterBuilder {
  STACK_ALLOCATED();

 public:
  SVGFilterBuilder(FilterEffect* source_graphic,
                   SVGFilterGraphNodeMap* = nullptr,
                   const cc::PaintFlags* fill_flags = nullptr,
                   const cc::PaintFlags* stroke_flags = nullptr);

  void BuildGraph(Filter*, SVGFilterElement&, const gfx::RectF&);

  FilterEffect* GetEffectById(const AtomicString& id) const;
  FilterEffect* LastEffect() const { return last_effect_; }

  static InterpolationSpace ResolveInterpolationSpace(EColorInterpolation);

 private:
  void Add(const AtomicString& id, FilterEffect*);
  void AddBuiltinEffects();

  typedef HeapHashMap<AtomicString, Member<FilterEffect>> NamedFilterEffectMap;

  NamedFilterEffectMap builtin_effects_;
  NamedFilterEffectMap named_effects_;

  FilterEffect* last_effect_ = nullptr;
  SVGFilterGraphNodeMap* node_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FILTER_BUILDER_H_
