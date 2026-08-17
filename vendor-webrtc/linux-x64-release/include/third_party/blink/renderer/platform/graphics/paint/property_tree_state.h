// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PROPERTY_TREE_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PROPERTY_TREE_STATE_H_

#include "base/dcheck_is_on.h"
#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/platform/graphics/paint/clip_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/transform_paint_property_node.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PropertyTreeStateOrAlias;
class TraceablePropertyTreeStateOrAlias;
class PropertyTreeState;
class TraceablePropertyTreeState;

class PropertyTreeStateOrAliasData {
  STACK_ALLOCATED();

 protected:
  PropertyTreeStateOrAliasData() = default;
  PropertyTreeStateOrAliasData(
      const TransformPaintPropertyNodeOrAlias& transform,
      const ClipPaintPropertyNodeOrAlias& clip,
      const EffectPaintPropertyNodeOrAlias& effect)
      : transform_(&transform), clip_(&clip), effect_(&effect) {
    DCHECK(transform_);
    DCHECK(clip_);
    DCHECK(effect_);
  }

  const TransformPaintPropertyNodeOrAlias* transform_ = nullptr;
  const ClipPaintPropertyNodeOrAlias* clip_ = nullptr;
  const EffectPaintPropertyNodeOrAlias* effect_ = nullptr;
};

class TraceablePropertyTreeStateOrAliasData {
  DISALLOW_NEW();

 public:
  void Trace(Visitor* visitor) const {
    visitor->Trace(transform_);
    visitor->Trace(clip_);
    visitor->Trace(effect_);
  }

 protected:
  TraceablePropertyTreeStateOrAliasData() = default;
  TraceablePropertyTreeStateOrAliasData(
      const TransformPaintPropertyNodeOrAlias& transform,
      const ClipPaintPropertyNodeOrAlias& clip,
      const EffectPaintPropertyNodeOrAlias& effect)
      : transform_(&transform), clip_(&clip), effect_(&effect) {
    DCHECK(transform_);
    DCHECK(clip_);
    DCHECK(effect_);
  }

  Member<const TransformPaintPropertyNodeOrAlias> transform_;
  Member<const ClipPaintPropertyNodeOrAlias> clip_;
  Member<const EffectPaintPropertyNodeOrAlias> effect_;
};

// A complete set of paint properties including those that are inherited from
// other objects.
template <typename Data>
class PLATFORM_EXPORT PropertyTreeStateOrAliasBase : public Data {
 public:
  PropertyTreeStateOrAliasBase(
      const TransformPaintPropertyNodeOrAlias& transform,
      const ClipPaintPropertyNodeOrAlias& clip,
      const EffectPaintPropertyNodeOrAlias& effect)
      : Data(transform, clip, effect) {}

  template <typename Other>
  explicit PropertyTreeStateOrAliasBase(
      const PropertyTreeStateOrAliasBase<Other>& other)
      : PropertyTreeStateOrAliasBase(other.Transform(),
                                     other.Clip(),
                                     other.Effect()) {}

  template <typename Other>
  void operator=(const PropertyTreeStateOrAliasBase<Other>& other) {
    SetTransform(other.Transform());
    SetClip(other.Clip());
    SetEffect(other.Effect());
  }

  static PropertyTreeState Root();

  // This is used as the initial value of uninitialized property tree state.
  // Access to the nodes are not allowed.
  enum UninitializedTag { kUninitialized };
  explicit PropertyTreeStateOrAliasBase(UninitializedTag) {}

  void SetUninitialized() {
    this->transform_ = nullptr;
    this->clip_ = nullptr;
    this->effect_ = nullptr;
  }

  // Returns true if all fields are initialized.
  bool IsInitialized() const {
    return this->transform_ && this->clip_ && this->effect_;
  }

  // Returns an unaliased property tree state.
  PropertyTreeState Unalias() const;

  const TransformPaintPropertyNodeOrAlias& Transform() const {
    DCHECK(this->transform_);
    return *this->transform_;
  }
  void SetTransform(const TransformPaintPropertyNodeOrAlias& node) {
    this->transform_ = &node;
    DCHECK(this->transform_);
  }

  const ClipPaintPropertyNodeOrAlias& Clip() const {
    DCHECK(this->clip_);
    return *this->clip_;
  }
  void SetClip(const ClipPaintPropertyNodeOrAlias& node) {
    this->clip_ = &node;
    DCHECK(this->clip_);
  }

  const EffectPaintPropertyNodeOrAlias& Effect() const {
    DCHECK(this->effect_);
    return *this->effect_;
  }
  void SetEffect(const EffectPaintPropertyNodeOrAlias& node) {
    this->effect_ = &node;
    DCHECK(this->effect_);
  }

  void ClearChangedToRoot(int sequence_number) const {
    Transform().ClearChangedToRoot(sequence_number);
    Clip().ClearChangedToRoot(sequence_number);
    Effect().ClearChangedToRoot(sequence_number);
  }

  // Returns true if any property tree state change is >= |change| relative to
  // |relative_to|. Note that this is O(|nodes|).
  bool Changed(PaintPropertyChangeType,
               const PropertyTreeState& relative_to) const;
  bool ChangedToRoot(PaintPropertyChangeType) const;

  String ToString() const;
#if DCHECK_IS_ON()
  // Dumps the tree from this state up to the root as a string.
  String ToTreeString() const;
#endif
  std::unique_ptr<JSONObject> ToJSON() const;

  template <typename Other>
  bool operator==(const PropertyTreeStateOrAliasBase<Other>& other) const {
    return this->transform_ == other.transform_ && this->clip_ == other.clip_ &&
           this->effect_ == other.effect_;
  }

 protected:
  template <typename Other>
  friend class PropertyTreeStateOrAliasBase;
};

class PLATFORM_EXPORT PropertyTreeStateOrAlias
    : public PropertyTreeStateOrAliasBase<PropertyTreeStateOrAliasData> {
 public:
  using PropertyTreeStateOrAliasBase::PropertyTreeStateOrAliasBase;
  using PropertyTreeStateOrAliasBase::operator=;

  String ToString() const;
#if DCHECK_IS_ON()
  // Dumps the tree from this state up to the root as a string.
  String ToTreeString() const;
#endif
  std::unique_ptr<JSONObject> ToJSON() const;
};

class TraceablePropertyTreeStateOrAlias
    : public PropertyTreeStateOrAliasBase<
          TraceablePropertyTreeStateOrAliasData> {
 public:
  using PropertyTreeStateOrAliasBase::PropertyTreeStateOrAliasBase;
  using PropertyTreeStateOrAliasBase::operator=;
};

template <typename Super>
class PLATFORM_EXPORT PropertyTreeStateBase : public Super {
 public:
  PropertyTreeStateBase(const TransformPaintPropertyNode& transform,
                        const ClipPaintPropertyNode& clip,
                        const EffectPaintPropertyNode& effect)
      : Super(transform, clip, effect) {}

  explicit PropertyTreeStateBase(Super::UninitializedTag)
      : Super(Super::kUninitialized) {}

  template <typename Other>
  explicit PropertyTreeStateBase(const PropertyTreeStateBase<Other>& other)
      : PropertyTreeStateBase(other.Transform(), other.Clip(), other.Effect()) {
  }

  template <typename Other>
  void operator=(const PropertyTreeStateBase<Other>& other) {
    SetTransform(other.Transform());
    SetClip(other.Clip());
    SetEffect(other.Effect());
  }

  PropertyTreeState Unalias() const = delete;

  const TransformPaintPropertyNode& Transform() const {
    const auto& node = Super::Transform();
    DCHECK(!node.IsParentAlias());
    return static_cast<const TransformPaintPropertyNode&>(node);
  }
  void SetTransform(const TransformPaintPropertyNode& node) {
    Super::SetTransform(node);
  }
  const ClipPaintPropertyNode& Clip() const {
    const auto& node = Super::Clip();
    DCHECK(!node.IsParentAlias());
    return static_cast<const ClipPaintPropertyNode&>(node);
  }
  void SetClip(const ClipPaintPropertyNode& node) { Super::SetClip(node); }
  const EffectPaintPropertyNode& Effect() const {
    const auto& node = Super::Effect();
    DCHECK(!node.IsParentAlias());
    return static_cast<const EffectPaintPropertyNode&>(node);
  }
  void SetEffect(const EffectPaintPropertyNode& node) {
    Super::SetEffect(node);
  }
};

class PLATFORM_EXPORT PropertyTreeState
    : public PropertyTreeStateBase<PropertyTreeStateOrAlias> {
 public:
  using PropertyTreeStateBase::PropertyTreeStateBase;
  using PropertyTreeStateBase::operator=;

  // Determines whether drawings based on the 'guest' state can be painted into
  // a layer with the 'home' state, and if yes, returns the common ancestor
  // state to which both layer will be upcasted.
  using IsCompositedScrollFunction =
      base::FunctionRef<bool(const TransformPaintPropertyNode&)>;
  std::optional<PropertyTreeState> CanUpcastWith(
      const PropertyTreeState& guest,
      IsCompositedScrollFunction) const;
};

class PLATFORM_EXPORT TraceablePropertyTreeState
    : public PropertyTreeStateBase<TraceablePropertyTreeStateOrAlias> {
 public:
  using PropertyTreeStateBase::PropertyTreeStateBase;
  using PropertyTreeStateBase::operator=;
};

template <typename Data>
inline PropertyTreeState PropertyTreeStateOrAliasBase<Data>::Root() {
  return PropertyTreeState(TransformPaintPropertyNode::Root(),
                           ClipPaintPropertyNode::Root(),
                           EffectPaintPropertyNode::Root());
}

template <typename Data>
inline PropertyTreeState PropertyTreeStateOrAliasBase<Data>::Unalias() const {
  return PropertyTreeState(Transform().Unalias(), Clip().Unalias(),
                           Effect().Unalias());
}

template <typename Data>
inline bool PropertyTreeStateOrAliasBase<Data>::Changed(
    PaintPropertyChangeType change,
    const PropertyTreeState& relative_to) const {
  return Transform().Changed(change, relative_to.Transform()) ||
         Clip().Changed(change, relative_to, &Transform()) ||
         Effect().Changed(change, relative_to, &Transform());
}

template <typename Data>
inline bool PropertyTreeStateOrAliasBase<Data>::ChangedToRoot(
    PaintPropertyChangeType change) const {
  return Changed(change, Root());
}

template <typename Data>
inline String PropertyTreeStateOrAliasBase<Data>::ToString() const {
  return PropertyTreeStateOrAlias(Transform(), Clip(), Effect()).ToString();
}

#if DCHECK_IS_ON()
template <typename Data>
inline String PropertyTreeStateOrAliasBase<Data>::ToTreeString() const {
  return PropertyTreeStateOrAlias(Transform(), Clip(), Effect()).ToTreeString();
}
#endif

template <typename Data>
inline std::unique_ptr<JSONObject> PropertyTreeStateOrAliasBase<Data>::ToJSON()
    const {
  return PropertyTreeStateOrAlias(Transform(), Clip(), Effect()).ToJSON();
}

template <typename Data>
inline std::ostream& operator<<(
    std::ostream& os,
    const PropertyTreeStateOrAliasBase<Data>& state) {
  return os << state.ToString().Utf8();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PROPERTY_TREE_STATE_H_
