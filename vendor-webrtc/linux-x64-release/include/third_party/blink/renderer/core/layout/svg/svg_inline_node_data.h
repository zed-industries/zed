// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_INLINE_NODE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_INLINE_NODE_DATA_H_

#include "third_party/blink/renderer/core/layout/layout_text.h"
#include "third_party/blink/renderer/core/layout/svg/svg_character_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct SvgTextContentRange {
  DISALLOW_NEW();

  void Trace(Visitor* visitor) const { visitor->Trace(layout_object); }

  // This must be a LayoutSVGTextPath for |SvgInlineNodeData::
  // text_path_range_list|, and must be a LayoutObject for SVGTextContentElement
  // for SvgInlineNodeData::text_length_range_list
  Member<const LayoutObject> layout_object;
  unsigned start_index;
  unsigned end_index;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::SvgTextContentRange)

namespace blink {

using SvgTextChunkOffsets =
    HeapHashMap<Member<const LayoutText>, Vector<unsigned>>;

// SVG-specific data stored in InlineNodeData.
struct SvgInlineNodeData final : public GarbageCollected<SvgInlineNodeData> {
  void Trace(Visitor* visitor) const {
    visitor->Trace(text_length_range_list);
    visitor->Trace(text_path_range_list);
    visitor->Trace(chunk_offsets);
  }
  Vector<std::pair<unsigned, SvgCharacterData>> character_data_list;
  HeapVector<SvgTextContentRange> text_length_range_list;
  HeapVector<SvgTextContentRange> text_path_range_list;
  SvgTextChunkOffsets chunk_offsets;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_INLINE_NODE_DATA_H_
