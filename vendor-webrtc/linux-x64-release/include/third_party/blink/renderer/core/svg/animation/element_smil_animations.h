// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_ELEMENT_SMIL_ANIMATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_ELEMENT_SMIL_ANIMATIONS_H_

#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class SMILAnimationSandwich;
class SMILTime;
class SVGAnimationElement;

// This class manages all the SMIL animations (sandwiches) that apply to a
// certain SVGElement. It is created and updated by the SVGAnimationElements
// (or subclasses) that contribute to it, and is stored in SVGElementRareData.
class ElementSMILAnimations : public GarbageCollected<ElementSMILAnimations> {
 public:
  ElementSMILAnimations();

  void AddAnimation(const QualifiedName& attribute, SVGAnimationElement*);
  void RemoveAnimation(const QualifiedName& attribute, SVGAnimationElement*);
  bool HasAnimations() const { return !sandwiches_.empty(); }

  bool Apply(SMILTime elapsed);

  void Trace(Visitor*) const;

 private:
  HeapHashMap<QualifiedName, Member<SMILAnimationSandwich>> sandwiches_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_ELEMENT_SMIL_ANIMATIONS_H_
