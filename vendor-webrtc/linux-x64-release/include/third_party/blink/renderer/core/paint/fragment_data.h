// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_DATA_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/paint/object_paint_properties.h"
#include "third_party/blink/renderer/platform/graphics/paint/cull_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PaintLayer;
struct StickyPositionScrollingConstraints;

// Represents the data for a particular fragment of a LayoutObject.
// See README.md.
class CORE_EXPORT FragmentData : public GarbageCollected<FragmentData> {
 public:
  // Physical offset of this fragment's local border box's top-left position
  // from the origin of the transform node of the fragment's property tree
  // state.
  PhysicalOffset PaintOffset() const { return paint_offset_; }
  void SetPaintOffset(const PhysicalOffset& paint_offset) {
    paint_offset_ = paint_offset;
  }

  // An id for this object that is unique for the lifetime of the WebView.
  UniqueObjectId UniqueId() const {
    DCHECK(rare_data_);
    return rare_data_->unique_id;
  }

  // The PaintLayer associated with this LayoutBoxModelObject. This can be null
  // depending on the return value of LayoutBoxModelObject::LayerTypeRequired().
  PaintLayer* Layer() const {
    AssertIsFirst();
    return rare_data_ ? rare_data_->layer.Get() : nullptr;
  }
  void SetLayer(PaintLayer*);

  StickyPositionScrollingConstraints* StickyConstraints() const {
    AssertIsFirst();
    return rare_data_ ? rare_data_->sticky_constraints.Get() : nullptr;
  }
  void SetStickyConstraints(StickyPositionScrollingConstraints* constraints) {
    AssertIsFirst();
    if (!rare_data_ && !constraints)
      return;
    EnsureRareData().sticky_constraints = constraints;
  }

  // Holds references to the paint property nodes created by this object.
  const ObjectPaintProperties* PaintProperties() const {
    return rare_data_ ? rare_data_->paint_properties.Get() : nullptr;
  }
  ObjectPaintProperties* PaintProperties() {
    return rare_data_ ? rare_data_->paint_properties.Get() : nullptr;
  }
  ObjectPaintProperties& EnsurePaintProperties() {
    EnsureRareData();
    if (!rare_data_->paint_properties) {
      rare_data_->paint_properties =
          MakeGarbageCollected<ObjectPaintProperties>();
    }
    return *rare_data_->paint_properties;
  }
  void ClearPaintProperties() {
    if (rare_data_)
      rare_data_->paint_properties = nullptr;
  }
  void EnsureId() { EnsureRareData().EnsureId(); }
  bool HasUniqueId() const { return rare_data_ && rare_data_->unique_id; }

  // This is a complete set of property nodes that should be used as a
  // starting point to paint a LayoutObject. This data is cached because some
  // properties inherit from the containing block chain instead of the
  // painting parent and cannot be derived in O(1) during the paint walk.
  // LocalBorderBoxProperties() includes fragment clip.
  //
  // For example: <div style='opacity: 0.3;'/>
  //   The div's local border box properties would have an opacity 0.3 effect
  //   node. Even though the div has no transform, its local border box
  //   properties would have a transform node that points to the div's
  //   ancestor transform space.
  PropertyTreeStateOrAlias LocalBorderBoxProperties() const {
    if (!HasLocalBorderBoxProperties()) [[unlikely]] {
      return LocalBorderBoxPropertiesFallback();
    }
    return PropertyTreeStateOrAlias(rare_data_->local_border_box_properties);
  }
  bool HasLocalBorderBoxProperties() const {
    return rare_data_ &&
           rare_data_->local_border_box_properties.IsInitialized();
  }
  void ClearLocalBorderBoxProperties() {
    if (rare_data_) {
      rare_data_->local_border_box_properties.SetUninitialized();
    }
  }
  void SetLocalBorderBoxProperties(const PropertyTreeStateOrAlias& state) {
    DCHECK(state.IsInitialized());
    EnsureRareData().local_border_box_properties = state;
  }

  void SetCullRect(const CullRect& cull_rect) {
    EnsureRareData().cull_rect_ = cull_rect;
  }
  CullRect GetCullRect() const {
    return rare_data_ ? rare_data_->cull_rect_ : CullRect();
  }
  void SetContentsCullRect(const CullRect& contents_cull_rect) {
    EnsureRareData().contents_cull_rect_ = contents_cull_rect;
  }
  CullRect GetContentsCullRect() const {
    return rare_data_ ? rare_data_->contents_cull_rect_ : CullRect();
  }

  // This is the complete set of property nodes that can be used to paint the
  // contents of this fragment. It is similar to LocalBorderBoxProperties()
  // but includes properties (e.g., overflow clip, scroll translation,
  // isolation nodes) that apply to contents.
  PropertyTreeStateOrAlias ContentsProperties() const {
    return PropertyTreeStateOrAlias(ContentsTransform(), ContentsClip(),
                                    ContentsEffect());
  }

  const TransformPaintPropertyNodeOrAlias& PreTransform() const;
  const ClipPaintPropertyNodeOrAlias& PreClip() const;
  const EffectPaintPropertyNodeOrAlias& PreEffect() const;

  const TransformPaintPropertyNodeOrAlias& ContentsTransform() const;
  const ClipPaintPropertyNodeOrAlias& ContentsClip() const;
  const EffectPaintPropertyNodeOrAlias& ContentsEffect() const;

#if DCHECK_IS_ON()
  void SetIsFirst() { is_first_ = true; }
#endif

  ~FragmentData() = default;
  void Trace(Visitor* visitor) const { visitor->Trace(rare_data_); }

 protected:
  friend class FragmentDataTest;

  NOINLINE PropertyTreeStateOrAlias LocalBorderBoxPropertiesFallback() const;

#if DCHECK_IS_ON()
  void AssertIsFirst() const { DCHECK(is_first_); }
#else
  void AssertIsFirst() const {}
#endif

  // Contains rare data that that is not needed on all fragments.
  struct CORE_EXPORT RareData final : public GarbageCollected<RareData> {
   public:
    RareData();
    RareData(const RareData&) = delete;
    RareData& operator=(const RareData&) = delete;
    ~RareData();

    void EnsureId();
    void SetLayer(PaintLayer*);

    void Trace(Visitor* visitor) const;

    // The following data fields are not fragment specific. Placed here just to
    // avoid separate data structure for them. They are only to be accessed in
    // the first fragment.
    Member<PaintLayer> layer;
    Member<StickyPositionScrollingConstraints> sticky_constraints;
    HeapVector<Member<FragmentData>> additional_fragments;

    // Fragment specific data.
    Member<ObjectPaintProperties> paint_properties;
    TraceablePropertyTreeStateOrAlias local_border_box_properties{
        TraceablePropertyTreeStateOrAlias::kUninitialized};
    CullRect cull_rect_;
    CullRect contents_cull_rect_;
    UniqueObjectId unique_id = 0;
  };

  RareData& EnsureRareData();

  PhysicalOffset paint_offset_;
  Member<RareData> rare_data_;

#if DCHECK_IS_ON()
  bool is_first_ = false;
#endif
};

// The first FragmentData entry associated with a LayoutObject. Provides some
// list functionality, to manipulate the list of FragmentData entries.
// Invariant: There's always at least one FragmentData entry. As such, Shrink(0)
// is forbidden, for instance. It's very common to have just one FragmentData
// entry. So the the first one is stored directly in FragmentData(Head). Any
// additional entries are stored in the first FragmentData's
// rare_data_.additional_fragments.
class CORE_EXPORT FragmentDataList final : public FragmentData {
 public:
  FragmentData& AppendNewFragment();
  void Shrink(wtf_size_t);

  FragmentData& front() {
    AssertIsFirst();
    return *this;
  }
  const FragmentData& front() const {
    AssertIsFirst();
    return *this;
  }
  FragmentData& back();
  const FragmentData& back() const;
  FragmentData& at(wtf_size_t idx);
  const FragmentData& at(wtf_size_t idx) const;
  wtf_size_t size() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_DATA_H_
