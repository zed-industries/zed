// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_H_

#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// The Interpolation class is an abstract class representing an animation effect
// between two keyframe values for the same property (CSS property, SVG
// attribute, etc), for example animating the CSS property 'left' from '100px'
// to '200px'.
//
// This is represented by a pair of keyframes, referred to as the start and end
// keyframes. Each keyframe contains a value for the given property, plus the
// value's 'composite mode', which indicates how the value applies on top of the
// element's existing value. For example, consider an element whose property
// 'left' computes to 50px before animations are applied, and which has an
// interpolation with start keyframe
//     {'left': '100px', composite: 'add'}
// and end keyframe
//     {'left': '200px', composite: 'replace'}
// This will produce an interpolated animation effect of the 'left' property of
// the element between the values 150px (i.e. 50px + 100px) and 200px.
//
// The subclasses of Interpolation store the start and end keyframes in
// different forms; see the description of each subclass for appropriate usage.
//
// At any given point in a keyframe animation effect, multiple interpolations
// may be in effect. These interpolations are referred to as the active
// interpolations, and are what are returned when sampling the effect model of
// an animation at a given local time (see EffectModel::Sample). Typically,
// there is only one active interpolation per property affected by the
// animation.
//
// Interpolations are used in two phases of animation computation:
//
// 1. Timing update phase:
//    To determine the value of a property at the current point in time, the
//    code calls the Interpolate function for each item in the list of active
//    interpolations. The arguments to this function specify how far through the
//    interpolation the animation currently is, and the function calculates the
//    value of the property at this point, storing the result in the
//    Interpolation object.
//
// 2. Effect application phase:
//    The interpolation's effect at its current timing state is applied to the
//    element. How this is done depends on the subclass of Interpolation. See
//    the subclass documentation for more.
class CORE_EXPORT Interpolation : public GarbageCollected<Interpolation> {
 public:
  Interpolation(const Interpolation&) = delete;
  Interpolation& operator=(const Interpolation&) = delete;
  virtual ~Interpolation() {}

  virtual void Interpolate(int iteration, double fraction) = 0;

  virtual bool IsInvalidatableInterpolation() const { return false; }
  virtual bool IsTransitionInterpolation() const { return false; }

  virtual const PropertyHandle& GetProperty() const = 0;

  // Indicates whether the cached current value (as calculated by Interpolate)
  // incorporates or replaces the property value that underlies the animation,
  // as in the case of additive animations. This tells us whether we can
  // optimise away computing underlying values.
  virtual bool DependsOnUnderlyingValue() const { return false; }

  virtual void Trace(Visitor*) const {}

 protected:
  Interpolation() = default;
};

using ActiveInterpolations = GCedHeapVector<Member<Interpolation>, 1>;
using ActiveInterpolationsMap =
    HeapHashMap<PropertyHandle, Member<ActiveInterpolations>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_H_
