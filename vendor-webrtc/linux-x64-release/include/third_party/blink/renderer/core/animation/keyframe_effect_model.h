/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_MODEL_H_

#include <memory>

#include "base/functional/function_ref.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/animation/effect_model.h"
#include "third_party/blink/renderer/core/animation/interpolation_effect.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/animation/string_keyframe.h"
#include "third_party/blink/renderer/core/animation/timeline_range.h"
#include "third_party/blink/renderer/core/animation/transition_keyframe.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ComputedStyle;
class Element;
class KeyframeEffectModelTest;

class CORE_EXPORT KeyframeEffectModelBase : public EffectModel {
 public:
  // FIXME: Implement accumulation.

  using PropertySpecificKeyframeVector =
      HeapVector<Member<Keyframe::PropertySpecificKeyframe>>;

  class CORE_EXPORT KeyframeProperties {
    STACK_ALLOCATED();

   public:
    struct EndIterator {};

    class CORE_EXPORT Iterator {
      STACK_ALLOCATED();

     public:
      explicit Iterator(const KeyframeEffectModelBase* model)
          : keyframes_(model->GetFrames()) {
        AdvanceToNextKeyframeWithProperties();
      }

      Iterator& operator++();
      PropertyHandle operator*() const { return *current_property_.value(); }
      bool operator==(EndIterator) const { return keyframes_.empty(); }

     private:
      void AdvanceToNextKeyframeWithProperties();

      base::span<const Member<Keyframe>> keyframes_;
      const Keyframe::IterableProperties* keyframe_properties_ = nullptr;
      std::optional<Keyframe::PropertyIteratorWrapper> current_property_;
    };

    explicit KeyframeProperties(const KeyframeEffectModelBase* model)
        : model_(model) {}
    Iterator begin() const { return Iterator(model_); }
    EndIterator end() const { return EndIterator(); }
    bool empty() const { return begin() == end(); }
    PropertyHandleSet UniqueProperties() const;

   private:
    const KeyframeEffectModelBase* model_;
  };

  class PropertySpecificKeyframeGroup
      : public GarbageCollected<PropertySpecificKeyframeGroup> {
   public:
    void AppendKeyframe(Keyframe::PropertySpecificKeyframe*);
    const PropertySpecificKeyframeVector& Keyframes() const {
      return keyframes_;
    }

    bool IsStatic() const { return has_static_value_; }

    void Trace(Visitor* visitor) const { visitor->Trace(keyframes_); }

   private:
    void RemoveRedundantKeyframes();
    void CheckIfStatic();
    bool AddSyntheticKeyframeIfRequired(
        scoped_refptr<TimingFunction> zero_offset_easing);

    PropertySpecificKeyframeVector keyframes_;

    // TODO(kevers): Store CSS value if static in order to short-circuit
    // applying the effect if set as we don't need to determine the bounding
    // keyframes.
    bool has_static_value_ = false;

    friend class KeyframeEffectModelBase;
  };

  using KeyframeGroupMap =
      GCedHeapHashMap<PropertyHandle, Member<PropertySpecificKeyframeGroup>>;
  class CORE_EXPORT IterableDynamicProperties {
    STACK_ALLOCATED();

   public:
    struct EndIterator {};

    class CORE_EXPORT Iterator {
      STACK_ALLOCATED();

     public:
      explicit Iterator(const KeyframeEffectModelBase* model)
          : model_(model),
            current_keyframe_group_(model_->keyframe_groups_->begin()) {
        AdvanceToNextGroup();
      }

      Iterator& operator++() {
        current_keyframe_group_++;
        AdvanceToNextGroup();
        return *this;
      }
      PropertyHandle operator*() const { return current_keyframe_group_->key; }
      bool operator==(EndIterator) const {
        return current_keyframe_group_ == model_->keyframe_groups_->end();
      }

     private:
      void AdvanceToNextGroup();

      const KeyframeEffectModelBase* model_;
      KeyframeGroupMap::const_iterator current_keyframe_group_;
    };

    explicit IterableDynamicProperties(const KeyframeEffectModelBase* model)
        : model_(model) {}
    Iterator begin() const { return Iterator(model_); }
    EndIterator end() const { return EndIterator(); }
    bool empty() const { return begin() == end(); }
    bool Contains(const PropertyHandle& property) const;

   private:
    const KeyframeEffectModelBase* model_;
  };

  bool AffectedByUnderlyingAnimations() const final { return !IsReplaceOnly(); }
  bool IsReplaceOnly() const;

  // Returns an iterable collection over the properties that are animated by
  // keyframes in this effect. This includes duplicates of properties
  // specified in multiple keyframes.
  KeyframeProperties Properties() const;

  // Returns an iterable collection over the properties with changing
  // values that are animated by keyframes in this effect.
  IterableDynamicProperties DynamicProperties() const;

  bool HasStaticProperty() const;

  using KeyframeVector = HeapVector<Member<Keyframe>>;
  const KeyframeVector& GetFrames() const { return keyframes_; }
  bool HasFrames() const { return !keyframes_.empty(); }
  template <class K>
  void SetFrames(HeapVector<K>& keyframes);

  // Keyframes for CSS animations require additional processing to lazy
  // evaluate computed values.
  virtual KeyframeVector GetComputedKeyframes(Element*) { return keyframes_; }

  CompositeOperation Composite() const { return composite_; }
  void SetComposite(CompositeOperation composite);

  const PropertySpecificKeyframeVector* GetPropertySpecificKeyframes(
      const PropertyHandle& property) const {
    EnsureKeyframeGroups();
    const auto keyframe_group_iter = keyframe_groups_->find(property);
    if (keyframe_group_iter == keyframe_groups_->end())
      return nullptr;
    return &keyframe_group_iter->value->Keyframes();
  }

  const KeyframeGroupMap& GetPropertySpecificKeyframeGroups() const {
    EnsureKeyframeGroups();
    return *keyframe_groups_;
  }

  // EffectModel implementation.
  bool Sample(int iteration,
              double fraction,
              TimingFunction::LimitDirection,
              AnimationTimeDelta iteration_duration,
              HeapVector<Member<Interpolation>>&) const override;

  bool IsKeyframeEffectModel() const override { return true; }

  virtual bool IsStringKeyframeEffectModel() const { return false; }
  virtual bool IsTransitionKeyframeEffectModel() const { return false; }
  virtual bool IsCssKeyframeEffectModel() { return false; }

  bool HasSyntheticKeyframes() const {
    EnsureKeyframeGroups();
    return has_synthetic_keyframes_;
  }

  void InvalidateCompositorKeyframesSnapshot() const {
    needs_compositor_keyframes_snapshot_ = true;
  }

  bool SnapshotNeutralCompositorKeyframes(
      Element&,
      const ComputedStyle& old_style,
      const ComputedStyle& new_style,
      const ComputedStyle* parent_style) const;

  bool SnapshotAllCompositorKeyframesIfNecessary(
      Element&,
      const ComputedStyle& base_style,
      const ComputedStyle* parent_style) const;

  template <class K>
  static Vector<double> GetComputedOffsets(const HeapVector<K>& keyframes);

  bool Affects(const PropertyHandle& property) const override {
    EnsureKeyframeGroups();
    return keyframe_groups_->Contains(property);
  }

  bool HasRevert() const {
    EnsureKeyframeGroups();
    return has_revert_;
  }

  bool HasNamedRangeKeyframes() { return has_named_range_keyframes_; }

  bool RequiresPropertyNode() const;

  bool IsTransformRelatedEffect() const override;

  // Update properties used in resolving logical properties. Returns true if
  // one or more keyframes changed as a result of the update.
  bool SetLogicalPropertyResolutionContext(
      WritingDirectionMode writing_direction);

  virtual KeyframeEffectModelBase* Clone() = 0;

  // Ensure timeline offsets are properly resolved. If any of the offsets
  // changed, the keyframes are resorted and cached data is cleared. Returns
  // true if one or more offsets were affected.
  bool ResolveTimelineOffsets(const TimelineRange&,
                              double range_start,
                              double range_end);

  void Trace(Visitor*) const override;

 protected:
  friend class IterableDynamicProperties;
  friend class IterableDynamicProperties::Iterator;

  KeyframeEffectModelBase(CompositeOperation composite,
                          scoped_refptr<TimingFunction> default_keyframe_easing)
      : interpolation_effect_(MakeGarbageCollected<InterpolationEffect>()),
        last_iteration_(0),
        last_fraction_(std::numeric_limits<double>::quiet_NaN()),
        last_iteration_duration_(AnimationTimeDelta()),
        composite_(composite),
        default_keyframe_easing_(std::move(default_keyframe_easing)) {}

  // Lazily computes the groups of property-specific keyframes.
  void EnsureKeyframeGroups() const;
  void EnsureInterpolationEffectPopulated() const;

  // Clears the various bits of cached data that this class has.
  void ClearCachedData();

  using ShouldSnapshotPropertyFunction =
      base::FunctionRef<bool(const PropertyHandle&)>;
  using ShouldSnapshotKeyframeFunction =
      base::FunctionRef<bool(const PropertySpecificKeyframe&)>;

  bool SnapshotCompositableProperties(
      Element& element,
      const ComputedStyle& computed_style,
      const ComputedStyle* parent_style,
      ShouldSnapshotPropertyFunction should_process_property,
      ShouldSnapshotKeyframeFunction should_process_keyframe) const;

  bool SnapshotCompositorKeyFrames(
      const PropertyHandle& property,
      Element& element,
      const ComputedStyle& computed_style,
      const ComputedStyle* parent_style,
      ShouldSnapshotPropertyFunction should_process_property,
      ShouldSnapshotKeyframeFunction should_process_keyframe) const;

  // Keyframes require tracking of the original position in the list and
  // resolution of computed offsets for sorting. As timeline offsets are layout
  // dependent, keyframes require shuffling whenever a timeline offset resolves
  // to a new value. Different ordering rules are needed for generation of
  // property specific keyframes and for reporting in a getKeyframes calls.
  // In both cases, the ordering rules depend on a combination of the computed
  // offset and original index.
  void IndexKeyframesAndResolveComputedOffsets();

  KeyframeVector keyframes_;
  // The spec describes filtering the normalized keyframes at sampling time
  // to get the 'property-specific keyframes'. For efficiency, we cache the
  // property-specific lists.
  mutable Member<KeyframeGroupMap> keyframe_groups_;
  mutable Member<InterpolationEffect> interpolation_effect_;
  mutable int last_iteration_;
  mutable double last_fraction_;
  mutable AnimationTimeDelta last_iteration_duration_;
  CompositeOperation composite_;
  scoped_refptr<TimingFunction> default_keyframe_easing_;

  mutable bool has_synthetic_keyframes_ = false;
  mutable bool needs_compositor_keyframes_snapshot_ = true;
  mutable bool has_revert_ = false;
  mutable bool has_named_range_keyframes_ = false;

  // The timeline and animation ranges last used to resolve
  // named range offsets. (See ResolveTimelineOffsets).
  std::optional<TimelineRange> last_timeline_range_;
  std::optional<double> last_range_start_;
  std::optional<double> last_range_end_;

  friend class KeyframeEffectModelTest;
};

// Time independent representation of an Animation's keyframes.
template <class K>
class KeyframeEffectModel : public KeyframeEffectModelBase {
 public:
  using KeyframeVector = HeapVector<Member<K>>;
  KeyframeEffectModel(
      const KeyframeVector& keyframes,
      CompositeOperation composite = kCompositeReplace,
      scoped_refptr<TimingFunction> default_keyframe_easing = nullptr,
      bool has_named_range_keyframes = false)
      : KeyframeEffectModelBase(composite, std::move(default_keyframe_easing)) {
    keyframes_.AppendVector(keyframes);
    IndexKeyframesAndResolveComputedOffsets();
    has_named_range_keyframes_ = has_named_range_keyframes;
  }

  KeyframeEffectModelBase* Clone() override {
    KeyframeVector keyframes;
    for (const auto& keyframe : GetFrames()) {
      Keyframe* new_keyframe = keyframe->Clone();
      keyframes.push_back(static_cast<K*>(new_keyframe));
    }
    return MakeGarbageCollected<KeyframeEffectModel<K>>(
        keyframes, composite_, default_keyframe_easing_);
  }

  KeyframeEffectModel<StringKeyframe>* CloneAsEmptyStringKeyframeModel() {
    HeapVector<Member<StringKeyframe>> empty_keyframes;
    return MakeGarbageCollected<KeyframeEffectModel<StringKeyframe>>(
        empty_keyframes, composite_, default_keyframe_easing_);
  }

 private:
  bool IsStringKeyframeEffectModel() const override { return false; }
  bool IsTransitionKeyframeEffectModel() const override { return false; }
};

using KeyframeVector = KeyframeEffectModelBase::KeyframeVector;
using PropertySpecificKeyframeVector =
    KeyframeEffectModelBase::PropertySpecificKeyframeVector;

using StringKeyframeEffectModel = KeyframeEffectModel<StringKeyframe>;
using StringKeyframeVector = StringKeyframeEffectModel::KeyframeVector;
using StringPropertySpecificKeyframeVector =
    StringKeyframeEffectModel::PropertySpecificKeyframeVector;

using TransitionKeyframeEffectModel = KeyframeEffectModel<TransitionKeyframe>;
using TransitionKeyframeVector = TransitionKeyframeEffectModel::KeyframeVector;
using TransitionPropertySpecificKeyframeVector =
    TransitionKeyframeEffectModel::PropertySpecificKeyframeVector;

template <>
struct DowncastTraits<KeyframeEffectModelBase> {
  static bool AllowFrom(const EffectModel& value) {
    return value.IsKeyframeEffectModel();
  }
};
template <>
struct DowncastTraits<StringKeyframeEffectModel> {
  static bool AllowFrom(const KeyframeEffectModelBase& value) {
    return value.IsStringKeyframeEffectModel();
  }
};
template <>
struct DowncastTraits<TransitionKeyframeEffectModel> {
  static bool AllowFrom(const KeyframeEffectModelBase& value) {
    return value.IsTransitionKeyframeEffectModel();
  }
};

inline const StringKeyframeEffectModel* ToStringKeyframeEffectModel(
    const EffectModel* base) {
  return To<StringKeyframeEffectModel>(To<KeyframeEffectModelBase>(base));
}

inline StringKeyframeEffectModel* ToStringKeyframeEffectModel(
    EffectModel* base) {
  return To<StringKeyframeEffectModel>(To<KeyframeEffectModelBase>(base));
}

inline TransitionKeyframeEffectModel* ToTransitionKeyframeEffectModel(
    EffectModel* base) {
  return To<TransitionKeyframeEffectModel>(To<KeyframeEffectModelBase>(base));
}

template <>
inline bool KeyframeEffectModel<StringKeyframe>::IsStringKeyframeEffectModel()
    const {
  return true;
}

template <>
inline bool KeyframeEffectModel<
    TransitionKeyframe>::IsTransitionKeyframeEffectModel() const {
  return true;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_MODEL_H_
