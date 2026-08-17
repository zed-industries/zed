/*
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ELEMENT_RARE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ELEMENT_RARE_DATA_H_

#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"

namespace blink {

class ElementSMILAnimations;
class SVGElementResourceClient;

class SVGElementRareData final : public GarbageCollected<SVGElementRareData> {
 public:
  SVGElementRareData()
      : corresponding_element_(nullptr),
        needs_override_computed_style_update_(false) {}
  SVGElementRareData(const SVGElementRareData&) = delete;
  SVGElementRareData& operator=(const SVGElementRareData&) = delete;

  SVGElementSet& OutgoingReferences() { return outgoing_references_; }
  const SVGElementSet& OutgoingReferences() const {
    return outgoing_references_;
  }
  SVGElementSet& IncomingReferences() { return incoming_references_; }
  const SVGElementSet& IncomingReferences() const {
    return incoming_references_;
  }

  HeapHashSet<WeakMember<SVGElement>>& ElementInstances() {
    return element_instances_;
  }
  const HeapHashSet<WeakMember<SVGElement>>& ElementInstances() const {
    return element_instances_;
  }

  SVGElement* CorrespondingElement() const {
    return corresponding_element_.Get();
  }
  void SetCorrespondingElement(SVGElement* corresponding_element) {
    corresponding_element_ = corresponding_element;
  }

  ElementSMILAnimations* GetSMILAnimations() { return smil_animations_.Get(); }
  ElementSMILAnimations& EnsureSMILAnimations();

  MutableCSSPropertyValueSet* AnimatedSMILStyleProperties() const {
    return animated_smil_style_properties_.Get();
  }
  MutableCSSPropertyValueSet* EnsureAnimatedSMILStyleProperties();

  const ComputedStyle* OverrideComputedStyle(Element*, const ComputedStyle*);
  void ClearOverriddenComputedStyle();

  void SetNeedsOverrideComputedStyleUpdate() {
    needs_override_computed_style_update_ = true;
  }

  SVGElementResourceClient* GetSVGResourceClient() {
    return resource_client_.Get();
  }
  SVGElementResourceClient& EnsureSVGResourceClient(SVGElement*);

  SVGResourceTarget& EnsureResourceTarget(SVGElement& element);
  bool HasResourceTarget() const;

  AffineTransform* AnimateMotionTransform();

  void Trace(Visitor*) const;

 private:
  SVGElementSet outgoing_references_;
  SVGElementSet incoming_references_;
  HeapHashSet<WeakMember<SVGElement>> element_instances_;
  Member<SVGElement> corresponding_element_;
  Member<SVGElementResourceClient> resource_client_;
  Member<ElementSMILAnimations> smil_animations_;
  bool needs_override_computed_style_update_ : 1;
  Member<MutableCSSPropertyValueSet> animated_smil_style_properties_;
  Member<const ComputedStyle> override_computed_style_;
  WeakMember<SVGResourceTarget> resource_target_;
  // Used by <animateMotion>
  AffineTransform animate_motion_transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ELEMENT_RARE_DATA_H_
