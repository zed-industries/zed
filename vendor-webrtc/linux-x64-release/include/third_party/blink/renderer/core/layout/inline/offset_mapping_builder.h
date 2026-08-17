// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_BUILDER_H_

#include "base/auto_reset.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/offset_mapping.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LayoutObject;
class LayoutText;

// This is the helper class for constructing the DOM-to-TextContent offset
// mapping. It holds an offset mapping, and provides APIs to modify the mapping
// step by step until the construction is finished.
// Design doc: https://goo.gl/CJbxky
class CORE_EXPORT OffsetMappingBuilder {
  STACK_ALLOCATED();

 public:
  // A scope-like object that, mappings appended inside the scope are marked as
  // from the given source node. When multiple scopes nest, only the inner-most
  // scope is effective. Note that at most one of the nested scopes may have a
  // non-null node.
  //
  // Example:
  //
  // OffsetMappingBuilder builder;
  //
  // {
  //   OffsetMappingBuilder::SourceNodeScope scope(&builder, node);
  //
  //   // These 3 characters are marked as from source node |node|.
  //   builder.AppendIdentity(3);
  //
  //   {
  //     OffsetMappingBuilder::SourceNodeScope unset_scope(&builder, nullptr);
  //
  //     // This character is marked as having no source node.
  //     builder.AppendCollapsed(1);
  //   }
  //
  //   // These 2 characters are marked as from source node |node|.
  //   builder.AppendIdentity(2);
  //
  //   // Not allowed.
  //   // OffsetMappingBuilder::SourceNodeScope scope(&builder, node2);
  // }
  //
  // // This character is marked as having no source node.
  // builder.AppendIdentity(1);
  class SourceNodeScope {
    STACK_ALLOCATED();

   public:
    SourceNodeScope(OffsetMappingBuilder* builder, const LayoutObject* node);
    SourceNodeScope(const SourceNodeScope&) = delete;
    SourceNodeScope& operator=(const SourceNodeScope&) = delete;
    ~SourceNodeScope();

   private:
    OffsetMappingBuilder* const builder_ = nullptr;
    base::AutoReset<const LayoutObject*> layout_object_auto_reset_;
    base::AutoReset<unsigned> appended_length_auto_reset_;
  };

  OffsetMappingBuilder();
  OffsetMappingBuilder(const OffsetMappingBuilder&) = delete;
  ~OffsetMappingBuilder() {
    mapping_units_.clear();
    unit_ranges_.clear();
  }
  OffsetMappingBuilder& operator=(const OffsetMappingBuilder&) = delete;

  void ReserveCapacity(unsigned capacity);

  // Append an identity offset mapping of the specified length with null
  // annotation to the builder.
  void AppendIdentityMapping(unsigned length);

  // Cancel the last AppendIdentityMapping(1) call.
  // This works only for kOpenRubyColumn.
  void RevertIdentityMapping1();

  // Append a collapsed offset mapping from the specified length with null
  // annotation to the builder.
  void AppendCollapsedMapping(unsigned length);

  // Append a variable offset mapping from the specified `dom_length` to the
  // specified `text_content_length`.
  // Either of `dom_length` or `text_content_length` should be 1.
  void AppendVariableMapping(unsigned dom_length, unsigned text_content_length);

  // TODO(xiaochengh): Add the following API when we start to fix offset mapping
  // for text-transform.
  // Append an expanded offset mapping to the specified length with null
  // annotation to the builder.
  // void AppendExpandedMapping(unsigned length);

  // This function should only be called by InlineItemsBuilder during
  // whitespace collapsing, and in the case that the target string of the
  // currently held mapping:
  // (i)  has at least |space_offset + 1| characters,
  // (ii) character at |space_offset| in destination string is a collapsible
  //      whitespace,
  // This function changes the space into collapsed.
  void CollapseTrailingSpace(unsigned space_offset);

  // Concatenate the offset mapping held by another builder to this builder.
  // TODO(xiaochengh): Implement when adding support for 'text-transform'
  // void Concatenate(const OffsetMappingBuilder&);

  // Composite the offset mapping held by another builder to this builder.
  // TODO(xiaochengh): Implement when adding support for 'text-transform'
  // void Composite(const OffsetMappingBuilder&);

  // Restore a trailing collapsible space at |offset| of text content. The space
  // is associated with |layout_text|.
  void RestoreTrailingCollapsibleSpace(const LayoutText& layout_text,
                                       unsigned offset);

  // Set the destination string of the offset mapping.
  // Returns false if the specified string is inconsistent with
  // `destination_length_`. We can't build an OffstMapping in such case.
  bool SetDestinationString(const String&);

  // Finalize and return the offset mapping.
  // This method can only be called once, as it can invalidate the stored data.
  OffsetMapping* Build();

 private:
  const LayoutObject* current_layout_object_ = nullptr;
  unsigned current_offset_ = 0;
  bool has_open_unit_ = false;
#if DCHECK_IS_ON()
  bool has_nonnull_node_scope_ = false;
#endif

  // Length of the current destination string.
  unsigned destination_length_ = 0;

  // Mapping units of the current mapping function.
  HeapVector<OffsetMappingUnit> mapping_units_;

  // Unit ranges of the current mapping function.
  OffsetMapping::RangeMap unit_ranges_;

  // The destination string of the offset mapping.
  String destination_string_;

  friend class SourceNodeScope;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_OFFSET_MAPPING_BUILDER_H_
