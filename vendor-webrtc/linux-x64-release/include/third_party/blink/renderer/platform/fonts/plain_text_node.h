// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_NODE_H_

#include "third_party/blink/renderer/platform/fonts/shaping/frame_shape_cache.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class Font;
class FrameShapeCache;
class ShapeResult;
class ShapeResultView;
class TextRun;
struct CharacterRange;

// PlainTextItem represents a sub-segment of a PlainTextNode.
class PLATFORM_EXPORT PlainTextItem {
  DISALLOW_NEW();

 public:
  // `start` - The start offset in the owner text content.
  // `length` - The code unit length of this item.
  //  `dir` - Text direction of this item.
  // `text_content` - The text of the owner PlainTextNode.
  PlainTextItem(wtf_size_t start,
                wtf_size_t length,
                TextDirection dir,
                const String& text_content)
      : text_(text_content.Substring(start, length)),
        start_offset_(start),
        length_(length),
        direction_(dir) {}
  void Trace(Visitor* visitor) const;

  wtf_size_t StartOffset() const { return start_offset_; }
  wtf_size_t EndOffset() const { return start_offset_ + length_; }
  wtf_size_t Length() const { return length_; }
  TextDirection Direction() const { return direction_; }
  const ShapeResult* GetShapeResult() const { return shape_result_; }
  const ShapeResultView* EnsureView() const;
  const String& Text() const { return text_; }
  const gfx::RectF& InkBounds() const { return ink_bounds_; }

 private:
  friend class PlainTextNode;
  friend class FrameShapeCacheTest;

  Member<ShapeResult> shape_result_;
  // This is mutable for on-demand creation.
  mutable Member<ShapeResultView> shape_result_view_;
  gfx::RectF ink_bounds_;
  String text_;
  wtf_size_t start_offset_;
  wtf_size_t length_;
  TextDirection direction_;
};

// We chose "25" so that it's enough for Chars-chartjs suite in Speedometer3.
using PlainTextItemList = HeapVector<PlainTextItem, 25>;

// PlainTextNode class represents the information necessary to render a single
// TextRun instance.
//
// This includes a list of substrings after Bidi reordering and word
// segmentation, as well as their ShapeResult.
//
// Instances of this class are immutable.
class PLATFORM_EXPORT PlainTextNode : public GarbageCollected<PlainTextNode> {
 public:
  // normalize_space - Enables canvas-specific whitespace normalization
  PlainTextNode(const TextRun& run,
                bool normalize_space,
                const Font& font,
                bool supports_bidi,
                FrameShapeCache* cache);
  void Trace(Visitor* visitor) const;

  PlainTextNode(const PlainTextNode&) = delete;
  PlainTextNode& operator=(const PlainTextNode&) = delete;

  float AccumulateInlineSize(gfx::RectF* glyph_bounds) const;
  CharacterRange ComputeCharacterRange(unsigned absolute_from,
                                       unsigned absolute_to) const;

  // The text contains:
  //  - Normalized whitespace
  //  - No BiDi override controls
  const String& TextContent() const { return text_content_; }
  TextDirection BaseDirection() const { return base_direction_; }
  bool ContainsRtlItems() const { return contains_rtl_items_; }
  bool HasVerticalOffsets() const { return has_vertical_offsets_; }
  const PlainTextItemList& ItemList() const { return item_list_; }

 private:
  friend class PlainTextNodeTest;
  friend class FrameShapeCacheTest;

  // Up-converts to UTF-16 as needed and normalizes spaces and Unicode control
  // characters as per the CSS Text Module Level 3 specification.
  // https://drafts.csswg.org/css-text-3/#white-space-processing
  // Also, check if BiDi reorder is necessary.
  static std::pair<String, bool> NormalizeSpacesAndMaybeBidi(
      StringView text,
      bool normalize_canvas_space);

  void SegmentText(const TextRun& run,
                   bool bidi_overridden,
                   const Font& font,
                   bool supports_bidi);
  void SegmentWord(wtf_size_t start_offset,
                   wtf_size_t run_length,
                   TextDirection direction,
                   const Font& font);

  void Shape(const Font& font, FrameShapeCache* cache);

  String text_content_;
  PlainTextItemList item_list_;
  bool normalize_space_ = false;
  TextDirection base_direction_ = TextDirection::kLtr;
  bool contains_rtl_items_ = false;
  bool has_vertical_offsets_ = false;
};

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::PlainTextItem>
    : VectorTraitsBase<blink::PlainTextItem> {
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  static constexpr bool kCanTraceConcurrently = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_NODE_H_
