// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LayoutBlockFlow;
class LayoutObject;

enum class OffsetMappingUnitType { kIdentity, kCollapsed, kVariable };

// An OffsetMappingUnit indicates a "simple" offset mapping between dom offset
// range [dom_start, dom_end] on node |owner| and text content offset range
// [text_content_start, text_content_end]. The mapping between them falls in one
// of the following categories, depending on |type|:
// - kIdentity: The mapping between the two ranges is the identity mapping. In
//   other words, the two ranges have the same length, and the offsets are
//   mapped one-to-one.
// - kCollapsed: The mapping is collapsed, namely, |text_content_start| and
//   |text_content_end| are the same, and characters in the dom range are
//   collapsed.
// - kVariable: The mapping is expanded or shrunk, namely.
//   -- |dom_end == dom_start + 1|, and
//      |text_content_end > text_content_start + 1|, indicating that the
//      character in the dom range is expanded into multiple characters, or
//   -- |dom_end > dom_start + 1|, and
//      |text_content_end == text_content_start + 1|, indicating that multiple
//      characters in the dom range is shrunk into a single character.
// See design doc https://goo.gl/CJbxky for details.
class CORE_EXPORT OffsetMappingUnit {
  DISALLOW_NEW();

 public:
  OffsetMappingUnit(OffsetMappingUnitType,
                    const LayoutObject&,
                    unsigned dom_start,
                    unsigned dom_end,
                    unsigned text_content_start,
                    unsigned text_content_end);

  // Returns associated node for this unit or null if this unit is associated
  // to generated content.
  const Node* AssociatedNode() const;

  OffsetMappingUnitType GetType() const { return type_; }
  // Returns true if GetType() is OffsetMappingUnitType::kCollapsed.
  bool IsCollapsed() const {
    return type_ == OffsetMappingUnitType::kCollapsed;
  }

  const LayoutObject& GetLayoutObject() const { return *layout_object_; }
  // Returns |Node| for this unit. If this unit comes from CSS generated
  // content, we can't use this function.
  // TODO(yosin): We should rename |GetOwner()| to |NonPseudoNode()|.
  const Node& GetOwner() const;
  unsigned DOMStart() const { return dom_start_; }
  unsigned DOMEnd() const { return dom_end_; }
  unsigned TextContentStart() const { return text_content_start_; }
  unsigned TextContentEnd() const { return text_content_end_; }

  // If the passed unit can be concatenated to |this| to create a bigger unit,
  // replaces |this| by the result and returns true; Returns false otherwise.
  bool Concatenate(const OffsetMappingUnit&);

  unsigned ConvertDOMOffsetToTextContent(unsigned) const;

  unsigned ConvertTextContentToFirstDOMOffset(unsigned) const;
  unsigned ConvertTextContentToLastDOMOffset(unsigned) const;

  void AssertValid() const;

  void Trace(Visitor*) const;

 private:
  OffsetMappingUnitType type_ = OffsetMappingUnitType::kIdentity;

  Member<const LayoutObject> layout_object_;
  // TODO(yosin): We should rename |dom_start_| and |dom_end_| to appropriate
  // names since |layout_object_| is for generated text, these offsets are
  // offset in |LayoutText::text_| instead of DOM node.
  unsigned dom_start_;
  unsigned dom_end_;

  // |text_content_start_| and |text_content_end_| are offsets in
  // |OffsetMapping::text_|. These values are in [0, |text_.length()] to
  // represent collapsed spaces at the end of block.
  unsigned text_content_start_;
  unsigned text_content_end_;

  friend class OffsetMappingBuilder;
};

// Each inline formatting context laid out with LayoutNG has an OffsetMapping
// object that stores the mapping information between DOM positions and offsets
// in the text content string of the context.
// See design doc https://goo.gl/CJbxky for details.
class CORE_EXPORT OffsetMapping final : public GarbageCollected<OffsetMapping> {
 public:
  using UnitVector = HeapVector<OffsetMappingUnit>;
  using RangeMap =
      HeapHashMap<Member<const Node>, std::pair<unsigned, unsigned>>;

  OffsetMapping(UnitVector&&, RangeMap&&, String);
  OffsetMapping(const OffsetMapping&) = delete;
  OffsetMapping& operator=(const OffsetMapping&) = delete;
  ~OffsetMapping();

  const UnitVector& GetUnits() const { return units_; }
  const RangeMap& GetRanges() const { return ranges_; }
  const String& GetText() const { return text_; }

  /// A utility class that converts offsets of `LayoutObject` to offsets of text
  /// content.
  class CORE_EXPORT LayoutObjectConverter {
    STACK_ALLOCATED();

   public:
    // The `offset_mapping` must be non-null and outlive this instance.
    LayoutObjectConverter(const OffsetMapping* offset_mapping,
                          const LayoutObject& layout_object)
        : units_(offset_mapping->GetMappingUnitsForLayoutObject(layout_object)),
          last_unit_(units_.begin()) {}

    unsigned TextContentOffset(unsigned offset) const;

   private:
    const base::span<const OffsetMappingUnit> units_;
    // These are the cache of the last used `offset` and the result, to make
    // the next search faster when `offset` increases.
    mutable base::span<const OffsetMappingUnit>::iterator last_unit_;
    mutable unsigned last_offset_ = 0;
  };

  // ------ Static getters for offset mapping objects  ------

  // TODO(xiaochengh): Unify the following getters and make them work on both
  // legacy and LayoutNG.

  // OffsetMapping APIs only accept the following positions:
  // 1. Offset-in-anchor in a text node;
  // 2. Before/After-anchor of an atomic inline or a text-like node like <br>.
  static bool AcceptsPosition(const Position&);

  // Returns the mapping object of the inline formatting context laying out the
  // given position.
  static const OffsetMapping* GetFor(const Position&);

  // Returns the mapping object of the inline formatting context laying out the
  // given position even if legacy layout tree.
  // TODO(yosin): Once we get rid of legacy layout, we should get rid of
  // |ForceGetFor()|.
  static const OffsetMapping* ForceGetFor(const Position&);

  // Returns the mapping object of the inline formatting context containing the
  // given LayoutObject, if it's laid out with LayoutNG. If the LayoutObject is
  // itself an inline formatting context, returns its own offset mapping object.
  // This makes the retrieval of the mapping object easier when we already have
  // a LayoutObject at hand.
  static const OffsetMapping* GetFor(const LayoutObject*);

  // Returns the inline formatting context (which is a block flow) where the
  // given object is laid out -- this is the block flow whose offset mapping
  // contains the given object. Note that the object can be in either legacy or
  // NG layout, while OffsetMapping is supported on both of them.
  static LayoutBlockFlow* GetInlineFormattingContextOf(const LayoutObject&);

  // Variants taking position instead of |LayoutObject|.
  static LayoutBlockFlow* GetInlineFormattingContextOf(const Position&);

  // ------ Mapping APIs from DOM to text content ------

  // Returns the OffsetMappingUnit whose DOM range contains the position.
  // If there are multiple qualifying units, returns the last one.
  const OffsetMappingUnit* GetMappingUnitForPosition(const Position&) const;

  // Returns all OffsetMappingUnits whose DOM ranges has non-empty (but
  // possibly collapsed) intersections with the passed in DOM range. If a unit
  // partially intersects the range, it is clamped with only the part within the
  // range returned. This API only accepts ranges whose start and end have the
  // same anchor node.
  UnitVector GetMappingUnitsForDOMRange(const EphemeralRange&) const;

  // Returns all OffsetMappingUnits associated to |node|. When |node| is
  // laid out with ::first-letter, this function returns both first-letter part
  // and remaining part. Note: |node| should have associated mapping.
  base::span<const OffsetMappingUnit> GetMappingUnitsForNode(
      const Node& node) const;

  // Returns all OffsetMappingUnits associated to |layout_object|. This
  // function works even if |layout_object| is for CSS generated content
  // ("content" property in ::before/::after, etc.)
  // Note: Unlike |GetMappingUnitsForNode()|, this function returns units
  // for first-letter or remaining part only instead of both parts.
  // Note: |layout_object| should have associated mapping.
  base::span<const OffsetMappingUnit> GetMappingUnitsForLayoutObject(
      const LayoutObject& layout_object) const;

  // Returns the text content offset corresponding to the given position.
  // Returns nullopt when the position is not laid out in this context.
  std::optional<unsigned> GetTextContentOffset(const Position&) const;

  // Starting from the given position, searches for non-collapsed content in
  // the anchor node in forward/backward direction and returns the position
  // before/after it; Returns null if there is no more non-collapsed content in
  // the anchor node.
  Position StartOfNextNonCollapsedContent(const Position&) const;
  Position EndOfLastNonCollapsedContent(const Position&) const;

  // Returns true if the position is right before/after non-collapsed content in
  // the anchor node. Note that false is returned if the position is already at
  // the end/start of the anchor node.
  bool IsBeforeNonCollapsedContent(const Position&) const;
  bool IsAfterNonCollapsedContent(const Position&) const;

  // Maps the given position to a text content offset, and then returns the text
  // content character before the offset. Returns nullopt if it does not exist.
  std::optional<UChar> GetCharacterBefore(const Position&) const;

  // ------ Mapping APIs from text content to DOM ------

  // These APIs map a text content offset to DOM positions, or return null when
  // both characters next to the offset are in generated content (list markers,
  // ::before/after, generated BiDi control characters, ...). The returned
  // position is either offset in a text node, or before/after an atomic inline
  // (IMG, BR, ...).
  // Note 1: there can be multiple positions mapped to the same offset when,
  // for example, there are collapsed whitespaces. Hence, we have two APIs to
  // return the first/last one of them.
  // Note 2: there is a corner case where Shadow DOM changes the ordering of
  // nodes in the flat tree, so that they are not laid out in the same order as
  // in the DOM tree. In this case, "first" and "last" position are defined on
  // the layout order, aka the flat tree order.
  Position GetFirstPosition(unsigned) const;
  Position GetLastPosition(unsigned) const;

  // Returns all OffsetMappingUnits whose text content ranges has non-empty
  // (but possibly collapsed) intersection with (start, end). Note that units
  // that only "touch" |start| or |end| are excluded.
  // Note: Returned range may include units for generated content.
  base::span<const OffsetMappingUnit> GetMappingUnitsForTextContentOffsetRange(
      unsigned start,
      unsigned end) const;

  // Returns the first |OffsetMappingUnit| where |TextContentStart() >=
  // offset| including unit for generated content.
  const OffsetMappingUnit* GetFirstMappingUnit(unsigned offset) const;

  // Returns the last |OffsetMappingUnit| where |TextContentStart() >= offset|
  // including unit for generated content.
  const OffsetMappingUnit* GetLastMappingUnit(unsigned offset) const;

  // ------ APIs inspecting the text content string ------

  // Returns false if all characters in [start, end) of |text_| are bidi
  // control characters. Returns true otherwise.
  bool HasBidiControlCharactersOnly(unsigned start, unsigned end) const;

  void Trace(Visitor* visitor) const {
    visitor->Trace(units_);
    visitor->Trace(ranges_);
  }

 private:
  // The OffsetMappingUnits of the inline formatting context in osrted order.
  UnitVector units_;

  // Stores the unit range for each node in inline formatting context.
  RangeMap ranges_;

  // The text content string of the inline formatting context. Same string as
  // |InlineNodeData::text_content_|.
  String text_;
};

CORE_EXPORT LayoutBlockFlow* NGInlineFormattingContextOf(const Position&);

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::OffsetMappingUnit>
    : VectorTraitsBase<blink::OffsetMappingUnit> {
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  static constexpr bool kCanTraceConcurrently = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_H_
