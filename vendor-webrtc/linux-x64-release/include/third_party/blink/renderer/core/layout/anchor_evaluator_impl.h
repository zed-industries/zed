// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_EVALUATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_EVALUATOR_IMPL_H_

#include <optional>
#include <variant>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/anchor_evaluator.h"
#include "third_party/blink/renderer/core/css/css_anchor_query_enums.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/layout/anchor_scope.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

class AnchorSpecifierValue;
class Element;
class LayoutBox;
class LayoutObject;
class StitchedAnchorQueries;
class PaintLayer;

using AnchorKey = std::variant<const AnchorScopedName*, const Element*>;

// This class is conceptually a concatenation of two hash maps with different
// key types but the same value type. To save memory, we don't implement it as
// one hash map with a unified key type; Otherwise, the size of each key will be
// increased by at least one pointer, which is undesired.
template <typename AnchorReference>
class AnchorQueryBase : public GarbageCollectedMixin {
  using NamedAnchorMap =
      HeapHashMap<Member<const AnchorScopedName>, Member<AnchorReference>>;
  using ImplicitAnchorMap =
      HeapHashMap<Member<const Element>, Member<AnchorReference>>;

 public:
  bool IsEmpty() const {
    return named_anchors_.empty() && implicit_anchors_.empty();
  }

  const AnchorReference* GetAnchorReference(const AnchorKey& key) const {
    if (const AnchorScopedName* const* name =
            std::get_if<const AnchorScopedName*>(&key)) {
      return GetAnchorReference(named_anchors_, *name);
    }
    return GetAnchorReference(implicit_anchors_, std::get<const Element*>(key));
  }

  struct AddResult {
    Member<AnchorReference>* stored_value;
    bool is_new_entry;
    STACK_ALLOCATED();
  };
  AddResult insert(const AnchorKey& key, AnchorReference* reference) {
    if (const AnchorScopedName* const* name =
            std::get_if<const AnchorScopedName*>(&key)) {
      return insert(named_anchors_, *name, reference);
    }
    return insert(implicit_anchors_, std::get<const Element*>(key), reference);
  }

  class Iterator {
    STACK_ALLOCATED();

    using NamedAnchorMap = typename AnchorQueryBase::NamedAnchorMap;
    using ImplicitAnchorMap = typename AnchorQueryBase::ImplicitAnchorMap;

   public:
    Iterator(const AnchorQueryBase* anchor_query,
             typename NamedAnchorMap::const_iterator named_map_iterator,
             typename ImplicitAnchorMap::const_iterator implicit_map_iterator)
        : anchor_query_(anchor_query),
          named_map_iterator_(named_map_iterator),
          implicit_map_iterator_(implicit_map_iterator) {}

    struct Entry {
      AnchorKey key;
      AnchorReference* value;
      STACK_ALLOCATED();
    };
    Entry operator*() const {
      if (named_map_iterator_ != anchor_query_->named_anchors_.end())
        return Entry{named_map_iterator_->key, named_map_iterator_->value};
      return Entry{implicit_map_iterator_->key, implicit_map_iterator_->value};
    }

    bool operator==(const Iterator& other) const {
      return named_map_iterator_ == other.named_map_iterator_ &&
             implicit_map_iterator_ == other.implicit_map_iterator_;
    }
    bool operator!=(const Iterator& other) const { return !operator==(other); }

    Iterator& operator++() {
      if (named_map_iterator_ != anchor_query_->named_anchors_.end())
        ++named_map_iterator_;
      else
        ++implicit_map_iterator_;
      return *this;
    }

   private:
    const AnchorQueryBase* anchor_query_;
    typename NamedAnchorMap::const_iterator named_map_iterator_;
    typename ImplicitAnchorMap::const_iterator implicit_map_iterator_;
  };
  Iterator begin() const {
    return Iterator{this, named_anchors_.begin(), implicit_anchors_.begin()};
  }
  Iterator end() const {
    return Iterator{this, named_anchors_.end(), implicit_anchors_.end()};
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(named_anchors_);
    visitor->Trace(implicit_anchors_);
  }

 private:
  friend class Iterator;

  template <typename AnchorMapType, typename KeyType>
  static const AnchorReference* GetAnchorReference(const AnchorMapType& anchors,
                                                   const KeyType& key) {
    auto it = anchors.find(key);
    return it != anchors.end() ? it->value : nullptr;
  }

  template <typename AnchorMapType, typename KeyType>
  static AddResult insert(AnchorMapType& anchors,
                          const KeyType& key,
                          AnchorReference* reference) {
    auto result = anchors.insert(key, reference);
    return AddResult{&result.stored_value->value, result.is_new_entry};
  }

  NamedAnchorMap named_anchors_;
  ImplicitAnchorMap implicit_anchors_;
};

struct CORE_EXPORT PhysicalAnchorReference
    : public GarbageCollected<PhysicalAnchorReference> {
  PhysicalAnchorReference(const Element& element,
                          const PhysicalRect& rect,
                          bool is_out_of_flow,
                          GCedHeapHashSet<Member<Element>>* display_locks)
      : rect(rect),
        element(&element),
        display_locks(display_locks),
        is_out_of_flow(is_out_of_flow) {}

  LayoutObject* GetLayoutObject() const { return element->GetLayoutObject(); }

  // Insert |this| into the given singly linked list in the reverse tree order.
  void InsertInReverseTreeOrderInto(Member<PhysicalAnchorReference>* head_ptr);

  void Trace(Visitor* visitor) const;

  PhysicalRect rect;
  Member<const Element> element;
  // A singly linked list in the reverse tree order. There can be at most one
  // in-flow reference, which if exists must be at the end of the list.
  Member<PhysicalAnchorReference> next;
  Member<GCedHeapHashSet<Member<Element>>> display_locks;
  bool is_out_of_flow = false;
};

class CORE_EXPORT PhysicalAnchorQuery
    : public GarbageCollected<PhysicalAnchorQuery>,
      public AnchorQueryBase<PhysicalAnchorReference> {
 public:
  using Base = AnchorQueryBase<PhysicalAnchorReference>;

  enum class SetOptions {
    // An in-flow entry.
    kInFlow,
    // An out-of-flow entry.
    kOutOfFlow,
  };

  // Find and return a valid anchor reference for the specified anchor key.
  // Unless nullptr is returned, the returned anchor reference is guaranteed to
  // have a valid LayoutObject.
  const PhysicalAnchorReference* AnchorReference(const LayoutBox& query_box,
                                                 const AnchorKey&) const;
  const LayoutObject* AnchorLayoutObject(const LayoutBox& query_box,
                                         const AnchorKey&) const;

  // If the element owning this object has a display lock, the element should be
  // passed as |element_for_display_lock|.
  void Set(const AnchorKey&,
           const LayoutObject& layout_object,
           const PhysicalRect& rect,
           SetOptions,
           Element* element_for_display_lock);
  void Set(const AnchorKey&, PhysicalAnchorReference* reference);
  // If the element owning this object has a display lock, the element should be
  // passed as |element_for_display_lock|.
  void SetFromChild(const PhysicalAnchorQuery& physical_query,
                    PhysicalOffset additional_offset,
                    SetOptions,
                    Element* element_for_display_lock);

  // Evaluate the |anchor_value| for the given reference. Returns |nullopt| if
  // the query is invalid (due to wrong axis).
  std::optional<LayoutUnit> EvaluateAnchor(
      const PhysicalAnchorReference& reference,
      CSSAnchorValue anchor_value,
      float percentage,
      LayoutUnit available_size,
      WritingDirectionMode container_writing_direction,
      WritingDirectionMode self_writing_direction,
      const PhysicalOffset& offset_to_padding_box,
      bool is_y_axis,
      bool is_right_or_bottom) const;
  LayoutUnit EvaluateSize(const PhysicalAnchorReference& reference,
                          CSSAnchorSizeValue anchor_size_value,
                          WritingMode container_writing_mode,
                          WritingMode self_writing_mode) const;
};

class CORE_EXPORT AnchorEvaluatorImpl : public AnchorEvaluator {
  STACK_ALLOCATED();

 public:
  // An empty evaluator that always return `nullopt`. This instance can still
  // compute `HasAnchorFunctions()`.
  AnchorEvaluatorImpl() = default;

  AnchorEvaluatorImpl(const LayoutBox& query_box,
                      const PhysicalAnchorQuery& anchor_query,
                      const LayoutObject* implicit_anchor,
                      WritingDirectionMode container_writing_direction,
                      const PhysicalOffset& offset_to_padding_box,
                      const PhysicalSize& available_size)
      : query_box_(&query_box),
        anchor_query_(&anchor_query),
        implicit_anchor_(implicit_anchor),
        container_writing_direction_(container_writing_direction),
        containing_block_rect_(offset_to_padding_box, available_size),
        display_locks_affected_by_anchors_(
            MakeGarbageCollected<GCedHeapHashSet<Member<Element>>>()) {
    DCHECK(anchor_query_);
  }

  // This constructor takes |StitchedAnchorQueries| and |containing_block|
  // instead of |PhysicalAnchorQuery|.
  AnchorEvaluatorImpl(const LayoutBox& query_box,
                      const StitchedAnchorQueries& anchor_queries,
                      const LayoutObject* implicit_anchor,
                      const LayoutObject& containing_block,
                      WritingDirectionMode container_writing_direction,
                      const PhysicalOffset& offset_to_padding_box,
                      const PhysicalSize& available_size)
      : query_box_(&query_box),
        anchor_queries_(&anchor_queries),
        implicit_anchor_(implicit_anchor),
        containing_block_(&containing_block),
        container_writing_direction_(container_writing_direction),
        containing_block_rect_(offset_to_padding_box, available_size),
        display_locks_affected_by_anchors_(
            MakeGarbageCollected<GCedHeapHashSet<Member<Element>>>()) {
    DCHECK(anchor_queries_);
    DCHECK(containing_block_);
  }

  // Returns true if any anchor reference in the axis is in the same scroll
  // container as the default anchor, in which case we need scroll adjustment in
  // the axis after layout.
  bool NeedsScrollAdjustmentInX() const {
    return needs_scroll_adjustment_in_x_;
  }
  bool NeedsScrollAdjustmentInY() const {
    return needs_scroll_adjustment_in_y_;
  }

  // Evaluates the given anchor query. Returns nullopt if the query invalid
  // (e.g., no target or wrong axis).
  std::optional<LayoutUnit> Evaluate(
      const class AnchorQuery&,
      const ScopedCSSName* position_anchor,
      const std::optional<PositionAreaOffsets>&) override;

  std::optional<PositionAreaOffsets> ComputePositionAreaOffsetsForLayout(
      const ScopedCSSName* position_anchor,
      PositionArea position_area) override;

  std::optional<PhysicalOffset> ComputeAnchorCenterOffsets(
      const ComputedStyleBuilder&) override;

  const PhysicalAnchorQuery* AnchorQuery() const;

  // Returns the most recent anchor evaluated. If more than one anchor has been
  // evaluated so far, nullptr is returned. This is done to avoid extra noise
  // for assistive tech.
  Element* AccessibilityAnchor() const;
  void ClearAccessibilityAnchor();

  GCedHeapHashSet<Member<Element>>* GetDisplayLocksAffectedByAnchors() const {
    return display_locks_affected_by_anchors_;
  }

 private:
  // Unless nullptr is returned, the returned anchor reference is guaranteed to
  // have a valid LayoutObject.
  const PhysicalAnchorReference* ResolveAnchorReference(
      const AnchorSpecifierValue& anchor_specifier,
      const ScopedCSSName* position_anchor) const;

  bool ShouldUseScrollAdjustmentFor(const LayoutObject* anchor,
                                    const ScopedCSSName* position_anchor) const;

  std::optional<LayoutUnit> EvaluateAnchor(
      const AnchorSpecifierValue& anchor_specifier,
      CSSAnchorValue anchor_value,
      float percentage,
      const ScopedCSSName* position_anchor,
      const std::optional<PositionAreaOffsets>&) const;
  std::optional<LayoutUnit> EvaluateAnchorSize(
      const AnchorSpecifierValue& anchor_specifier,
      CSSAnchorSizeValue anchor_size_value,
      const ScopedCSSName* position_anchor) const;
  void UpdateAccessibilityAnchor(const LayoutObject* anchor) const;

  const LayoutObject* DefaultAnchor(const ScopedCSSName* position_anchor) const;
  const PaintLayer* DefaultAnchorScrollContainerLayer(
      const ScopedCSSName* position_anchor) const;

  bool AllowAnchor() const;
  bool AllowAnchorSize() const;
  bool IsYAxis() const;
  bool IsRightOrBottom() const;

  LayoutUnit AvailableSizeAlongAxis(
      const PhysicalRect& position_area_modified_containing_block_rect) const {
    return IsYAxis() ? position_area_modified_containing_block_rect.Height()
                     : position_area_modified_containing_block_rect.Width();
  }

  // Returns the containing block, further constrained by the position-area.
  // Not to be confused with the inset-modified containing block.
  PhysicalRect PositionAreaModifiedContainingBlock(
      const std::optional<PositionAreaOffsets>&) const;

  const LayoutBox* query_box_ = nullptr;
  mutable const PhysicalAnchorQuery* anchor_query_ = nullptr;
  mutable const StitchedAnchorQueries* anchor_queries_ = nullptr;
  const LayoutObject* implicit_anchor_ = nullptr;
  const LayoutObject* containing_block_ = nullptr;
  WritingDirectionMode container_writing_direction_{WritingMode::kHorizontalTb,
                                                    TextDirection::kLtr};

  // Either width or height will be used, depending on IsYAxis().
  PhysicalRect containing_block_rect_;

  // A single-value cache. If a call to Get has the same key as the last call,
  // then the cached result it returned. Otherwise, the value is created using
  // CreationFunc, then returned.
  template <typename KeyType, typename ValueType>
  class CachedValue {
    STACK_ALLOCATED();

    template <typename T>
    static bool Equals(const T* a, const T* b) {
      return base::ValuesEquivalent(a, b);
    }

    template <typename T>
    static bool Equals(const T& a, const T& b) {
      return a == b;
    }

   public:
    template <typename CreationFunc>
    ValueType Get(KeyType key, CreationFunc create) {
      if (value_.has_value() && Equals(key_, key)) {
        DCHECK_EQ(value_.value(), create());
        return value_.value();
      }
      key_ = key;
      value_ = create();
      return value_.value();
    }

   private:
    KeyType key_{};
    std::optional<ValueType> value_;
  };

  // Caches most recent result of PositionAreaModifiedContainingBlock.
  mutable CachedValue<std::optional<PositionAreaOffsets>, PhysicalRect>
      cached_position_area_modified_containing_block_;

  // Caches most recent result of DefaultAnchor.
  mutable CachedValue<const ScopedCSSName*, const LayoutObject*>
      cached_default_anchor_;

  // Caches most recent result of DefaultAnchorScrollContainerLayer.
  mutable CachedValue<const ScopedCSSName*, const PaintLayer*>
      cached_default_anchor_scroll_container_layer_;

  mutable bool needs_scroll_adjustment_in_x_ = false;
  mutable bool needs_scroll_adjustment_in_y_ = false;

  // Most recent anchor evaluated, used for accessibility. This value is cleared
  // before a @position-try rule is applied.
  mutable Element* accessibility_anchor_ = nullptr;

  // True if more than one anchor has been evaluated so far. This value is
  // cleared before a @position-try rule is applied.
  mutable bool has_multiple_accessibility_anchors_ = false;

  // A set of elements whose display locks' skipping status are potentially
  // impacted by anchors found by this evaluator.
  mutable GCedHeapHashSet<Member<Element>>* display_locks_affected_by_anchors_ =
      nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_EVALUATOR_IMPL_H_
