/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_SANDWICH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_SANDWICH_H_

#include "third_party/blink/renderer/core/svg/animation/smil_time.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SVGAnimationElement;

// This class implements/helps with implementing the "sandwich model" from SMIL.
// https://www.w3.org/TR/SMIL3/smil-animation.html#animationNS-AnimationSandwichModel
//
// A "sandwich" contains all the animation elements that targets a specific
// attribute (or property) on a certain element.
//
// Consider the following simple example:
//
// <svg>
//   <rect id="foo" width="100" height="100" fill="yellow">
//     <set id="s1" attributeName="fill" to="blue" begin="1s; 3s" dur="1s"/>
//     <set id="s2" attributeName="fill" to="lightblue" begin="1.5s" dur="2s"/>
//   </rect>
// </svg>
//
// In this case there is only one sandwich: <#foo, "fill">
//
// The sandwich is priority-sorted with the priority being derived from when
// the currently active interval began - later is higher. In the above example
// there are three intervals: [1s 2s) and [3s 4s) for the first <set> element
// (in tree-order) and [1.5s 3.5s) for the second <set> element. The animation
// elements are only active within the intervals defined (no fill="freeze").
//
// When the first interval of the first <set> starts (at 1s), it is the only
// active animation and thus the only one to apply. When the second interval
// starts (at 1.5s) its animation gets a higher priority and replaces the lower
// priority animation from the first <set>. The first <set> then ends at 2s,
// leaving the second <set> as the only active animation. When the first <set>
// then starts again at 3s it gets a higher priority because of the later begin
// time and replaces the animation from the second <set>. When the second <set>
// ends at 3.5s nothing changes because the first <set> is still active. When
// the second <set> ends again at 4s, no animation apply and the target reverts
// to the base value (yellow) again.
//
// Schematically (right hand side exclusive):
//
// 0s -> 1s:   No animations apply (fill=yellow)
//             Sandwich order: (s1) (s2) [both inactive]
// 1s -> 1.5s: The first <set> apply (fill=blue)
//             Sandwich order: s1 (s2)   [only s1 active]
// 1.5s -> 2s: The second <set> apply (fill=lightblue)
//             Sandwich order: s1 s2
// 2s -> 3s:   The second <set> apply (fill=lightblue)
//             Sandwich order: (s1) s2
// 3s -> 3.5s: The first <set> apply (fill=blue)
//             Sandwich order: s2 s1
// 3.5s -> 4s: The first <set> apply (fill=blue)
//             Sandwich order: (s2) s1
// 4s -> ...:  No animations apply (fill=yellow)
//
// -----
//
// Implementation details:
//
// UpdateActiveAnimationStack() handles the sorting described above and
// constructs a vector containing only the active elements.
//
// ApplyAnimationValues() computes the actual animation value based on the
// vector of active elements and applies it to the target element.
//
class SMILAnimationSandwich : public GarbageCollected<SMILAnimationSandwich> {
 public:
  SMILAnimationSandwich();

  void Add(SVGAnimationElement* animation);
  void Remove(SVGAnimationElement* animation);

  void UpdateActiveAnimationStack(SMILTime presentation_time);
  bool ApplyAnimationValues();

  bool IsEmpty() { return sandwich_.empty(); }

  void Trace(Visitor*) const;

 private:
  using AnimationsVector = HeapVector<Member<SVGAnimationElement>>;

  // All the animation (really: timed) elements that make up the sandwich,
  // sorted according to priority.
  AnimationsVector sandwich_;
  // The currently active animation elements in the sandwich. Retains the
  // ordering of elements from |sandwich_| when created. This is the animation
  // elements from which the animation value is computed.
  AnimationsVector active_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_SANDWICH_H_
