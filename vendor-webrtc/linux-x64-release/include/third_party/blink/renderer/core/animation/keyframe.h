// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/animation/effect_model.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

using PropertyHandleSet = HashSet<PropertyHandle>;

class Element;
class ComputedStyle;
class CompositorKeyframeValue;
class TimelineRange;
class V8ObjectBuilder;

// A base class representing an animation keyframe.
//
// Generically a keyframe is a set of (property, value) pairs. In the
// web-animations spec keyframes have a few additional properties:
//
//   * A possibly-null keyframe offset, which represents the keyframe's position
//     relative to other keyframes in the same effect.
//   * A non-null timing function, which applies to the period of time between
//     this keyframe and the next keyframe in the same effect and influences
//     the interpolation between them.
//   * An keyframe-specific composite operation, which specifies a specific
//     composite operation used to combine values in this keyframe with an
//     underlying value. If this is 'auto', the keyframe effect composite
//     operation is used instead.
//
// For spec details, refer to: https://w3.org/TR/web-animations-1/#keyframe
//
// Implementation-wise the base Keyframe class captures the offset, composite
// operation, and timing function. It is left to subclasses to define and store
// the set of (property, value) pairs.
//
// === PropertySpecificKeyframes ===
//
// When calculating the effect value of a keyframe effect, the web-animations
// spec requires that a set of 'property-specific' keyframes are created.
// Property-specific keyframes resolve any unspecified offsets in the keyframes,
// calculate computed values for the specified properties, convert shorthand
// properties to multiple longhand properties, and resolve any conflicting
// shorthand properties.
//
// In this implementation property-specific keyframes are created only once and
// cached for subsequent calls, rather than re-computing them for every sample
// from the keyframe effect. See KeyframeEffectModelBase::EnsureKeyframeGroups.
//
// FIXME: Make Keyframe immutable
class CORE_EXPORT Keyframe : public GarbageCollected<Keyframe> {
 public:
  struct EndIterator {};

  class VirtualPropertyIterator {
   public:
    virtual ~VirtualPropertyIterator() = default;
    virtual void Advance(const Keyframe* keyframe) = 0;
    virtual PropertyHandle Deref(const Keyframe* keyframe) const = 0;
    virtual bool AtEnd(const Keyframe* keyframe) const = 0;
  };

  class CORE_EXPORT PropertyIteratorWrapper {
    STACK_ALLOCATED();

   public:
    explicit PropertyIteratorWrapper(
        const Keyframe* keyframe,
        std::unique_ptr<VirtualPropertyIterator> impl)
        : keyframe_(keyframe), impl_(std::move(impl)) {}

    bool operator==(EndIterator other) const { return impl_->AtEnd(keyframe_); }

    PropertyIteratorWrapper& operator++() {
      impl_->Advance(keyframe_);
      return *this;
    }

    PropertyHandle operator*() const { return impl_->Deref(keyframe_); }

   private:
    const Keyframe* keyframe_;
    std::unique_ptr<VirtualPropertyIterator> impl_;
  };

  class CORE_EXPORT IterableProperties
      : public GarbageCollected<IterableProperties> {
   public:
    IterableProperties() = default;
    virtual ~IterableProperties() = default;
    virtual PropertyIteratorWrapper begin() const = 0;
    EndIterator end() const { return EndIterator(); }
    virtual size_t size() const = 0;
    bool empty() const { return begin() == end(); }
    virtual void Trace(Visitor*) const {}
    virtual bool IsTransitionProperties() const { return false; }
  };

  Keyframe(const Keyframe&) = delete;
  Keyframe& operator=(const Keyframe&) = delete;
  virtual ~Keyframe() = default;

  static const double kNullComputedOffset;

  // TODO(smcgruer): The keyframe offset should be immutable.
  void SetOffset(std::optional<double> offset) { offset_ = offset; }
  std::optional<double> Offset() const { return offset_; }

  // Offsets are computed for programmatic keyframes that do not have a
  // specified offset (either as a percentage or timeline offset). These are
  // explicitly stored in the keyframe rather than computed on demand since
  // keyframes can be reordered to accommodate changes to the resolved timeline
  // offsets and computed offsets need to be sorted into the correct position.
  void SetComputedOffset(std::optional<double> offset) {
    computed_offset_ = offset;
  }
  std::optional<double> ComputedOffset() const { return computed_offset_; }

  // In order to have a valid computed offset, it must be evaluated and finite.
  // NaN Is used as the null value for computed offset. Note as NaN != NaN we
  // cannot check that the value matches kNullComputedOffset.
  bool HasComputedOffset() const {
    return computed_offset_ && !std::isnan(computed_offset_.value());
  }

  double CheckedOffset() const { return offset_.value_or(-1); }

  void SetTimelineOffset(std::optional<TimelineOffset> timeline_offset) {
    timeline_offset_ = timeline_offset;
  }
  const std::optional<TimelineOffset>& GetTimelineOffset() const {
    return timeline_offset_;
  }

  // TODO(smcgruer): The keyframe composite operation should be immutable.
  void SetComposite(EffectModel::CompositeOperation composite) {
    composite_ = composite;
  }
  std::optional<EffectModel::CompositeOperation> Composite() const {
    return composite_;
  }

  void SetEasing(scoped_refptr<TimingFunction> easing) {
    if (easing)
      easing_ = std::move(easing);
    else
      easing_ = LinearTimingFunction::Shared();
  }
  TimingFunction& Easing() const { return *easing_; }
  void CopyEasing(const Keyframe& other) { SetEasing(other.easing_); }

  // Track the original positioning in the list for tiebreaking during sort
  // when two keyframes have the same offset.
  void SetIndex(int index) { original_index_ = index; }
  std::optional<int> Index() { return original_index_; }

  // Returns an iterable collection of the properties represented in this
  // keyframe.
  const IterableProperties& Properties() const { return *properties_; }

  // Returns a copy of the properties represented in this keyframe.
  // Prefer iterating over Properties() when a copy is not needed.
  Vector<PropertyHandle> PropertiesVector() const;

  // Creates a clone of this keyframe.
  //
  // The clone should have the same (property, value) pairs, offset value,
  // composite operation, and timing function, as well as any other
  // subclass-specific data.
  virtual Keyframe* Clone() const = 0;

  // Comparator for stable sorting keyframes by offset. In the event of a tie
  // we sort by original index of the keyframe if specified.
  static bool LessThan(const Member<Keyframe>& a, const Member<Keyframe>& b);

  // Compute the offset if dependent on a timeline range.  Returns true if the
  // offset changed.
  bool ResolveTimelineOffset(const TimelineRange&,
                             double range_start,
                             double range_end);

  // Add the properties represented by this keyframe to the given V8 object.
  //
  // Subclasses should override this to add the (property, value) pairs they
  // store, and call into the base version to add the basic Keyframe properties.
  virtual void AddKeyframePropertiesToV8Object(V8ObjectBuilder&,
                                               Element*) const;

  virtual bool IsStringKeyframe() const { return false; }
  virtual bool IsTransitionKeyframe() const { return false; }

  virtual void Trace(Visitor* visitor) const { visitor->Trace(properties_); }

  // Represents a property-specific keyframe as defined in the spec. Refer to
  // the Keyframe class-level documentation for more details.
  class CORE_EXPORT PropertySpecificKeyframe
      : public GarbageCollected<PropertySpecificKeyframe> {
   public:
    PropertySpecificKeyframe(double offset,
                             scoped_refptr<TimingFunction> easing,
                             EffectModel::CompositeOperation);
    PropertySpecificKeyframe(const PropertySpecificKeyframe&) = delete;
    PropertySpecificKeyframe& operator=(const PropertySpecificKeyframe&) =
        delete;
    virtual ~PropertySpecificKeyframe() = default;
    double Offset() const { return offset_; }
    TimingFunction& Easing() const { return *easing_; }
    EffectModel::CompositeOperation Composite() const { return composite_; }
    double UnderlyingFraction() const {
      return composite_ == EffectModel::kCompositeReplace ? 0 : 1;
    }
    virtual bool IsNeutral() const = 0;
    virtual bool IsRevert() const = 0;
    virtual bool IsRevertLayer() const = 0;

    // FIXME: Remove this once CompositorAnimations no longer depends on
    // CompositorKeyframeValues
    virtual bool PopulateCompositorKeyframeValue(
        const PropertyHandle&,
        Element&,
        const ComputedStyle& base_style,
        const ComputedStyle* parent_style) const {
      return false;
    }

    virtual const CompositorKeyframeValue* GetCompositorKeyframeValue()
        const = 0;

    virtual bool IsCSSPropertySpecificKeyframe() const { return false; }
    virtual bool IsTransitionPropertySpecificKeyframe() const { return false; }

    virtual PropertySpecificKeyframe* NeutralKeyframe(
        double offset,
        scoped_refptr<TimingFunction> easing) const = 0;
    virtual Interpolation* CreateInterpolation(
        const PropertyHandle&,
        const Keyframe::PropertySpecificKeyframe& end) const;

    virtual void Trace(Visitor*) const {}

   protected:
    double offset_;
    scoped_refptr<TimingFunction> easing_;
    EffectModel::CompositeOperation composite_;
  };

  // Construct and return a property-specific keyframe for this keyframe.
  //
  // The 'effect_composite' parameter is the composite operation of the effect
  // that owns the keyframe. If the keyframe has a keyframe-specific composite
  // operation it should ignore this value when creating the property specific
  // keyframe.
  //
  // The 'offset' parameter is the offset to use in the resultant
  // PropertySpecificKeyframe. For CSS Transitions and CSS Animations, this is
  // the normal offset from the keyframe itself. However in web-animations this
  // will be a computed offset value which may differ from the keyframe offset.
  virtual PropertySpecificKeyframe* CreatePropertySpecificKeyframe(
      const PropertyHandle&,
      EffectModel::CompositeOperation effect_composite,
      double offset) const = 0;

 protected:
  explicit Keyframe(IterableProperties* properties)
      : properties_(properties), easing_(LinearTimingFunction::Shared()) {}
  Keyframe(IterableProperties* properties,
           std::optional<double> offset,
           std::optional<TimelineOffset> timeline_offset,
           std::optional<EffectModel::CompositeOperation> composite,
           scoped_refptr<TimingFunction> easing)
      : properties_(properties),
        offset_(offset),
        timeline_offset_(timeline_offset),
        composite_(composite),
        easing_(std::move(easing)) {
    if (!easing_)
      easing_ = LinearTimingFunction::Shared();
  }

  Member<IterableProperties> properties_;

  // Either the specified offset or the offset resolved from a timeline offset.
  std::optional<double> offset_;
  // The computed offset will equal the specified or resolved timeline offset
  // if non-null. The computed offset is null if the keyframe has an unresolved
  // timeline offset. Otherwise, it is calculated based on a rule to equally
  // space within an anchored range.
  // See KeyframeEffectModelBase::GetComputedOffsets.
  std::optional<double> computed_offset_;
  // Offsets of the form <name> <percent>. These offsets are layout depending
  // and need to be re-resolved on a style change affecting the corresponding
  // timeline range. If the effect is not associated with an animation that is
  // attached to a timeline with a non-empty timeline range,
  // then the offset and computed offset will be null.
  std::optional<TimelineOffset> timeline_offset_;

  // The original index in the keyframe list is used to resolve ties in the
  // offset when sorting, and to conditionally recover the original order when
  // reporting.
  std::optional<int> original_index_;

  // To avoid having multiple CompositeOperation enums internally (one with
  // 'auto' and one without), we use a std::optional for composite_. A
  // std::nullopt value represents 'auto'.
  std::optional<EffectModel::CompositeOperation> composite_;
  scoped_refptr<TimingFunction> easing_;
};

using PropertySpecificKeyframe = Keyframe::PropertySpecificKeyframe;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_H_
